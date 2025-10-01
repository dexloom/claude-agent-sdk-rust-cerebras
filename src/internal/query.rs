use crate::error::AgentError;
use crate::transport::Transport;
use crate::types::Message;
use async_trait::async_trait;
use futures::future::BoxFuture;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ToolPermissionContext {
    pub signal: Option<String>,             // Future: abort signal support
    pub suggestions: Vec<PermissionUpdate>, // Permission suggestions from CLI
}

#[derive(Debug, Clone)]
pub enum PermissionResult {
    Allow(PermissionResultAllow),
    Deny(PermissionResultDeny),
}

#[derive(Debug, Clone)]
pub struct PermissionResultAllow {
    pub behavior: String, // "allow"
    pub updated_input: Option<HashMap<String, Value>>,
    pub updated_permissions: Option<Vec<PermissionUpdate>>,
}

#[derive(Debug, Clone)]
pub struct PermissionResultDeny {
    pub behavior: String, // "deny"
    pub message: String,
    pub interrupt: bool,
}

#[derive(Debug, Clone)]
pub struct PermissionRuleValue {
    pub tool_name: String,
    pub rule_content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PermissionUpdate {
    pub r#type: String, // "addRules", "replaceRules", "removeRules", "setMode", "addDirectories", "removeDirectories"
    pub rules: Option<Vec<PermissionRuleValue>>,
    pub behavior: Option<String>, // PermissionBehavior
    pub mode: Option<String>,     // PermissionMode
    pub directories: Option<Vec<String>>,
    pub destination: Option<String>, // PermissionUpdateDestination
}
// Type alias for hook callbacks
pub type HookCallback = Arc<
    dyn Fn(
            HashMap<String, Value>,
            Option<String>,
            HookContext,
        ) -> BoxFuture<'static, Result<Value, AgentError>>
        + Send
        + Sync,
>;

pub type CanUseTool = Arc<
    dyn Fn(
            String,
            HashMap<String, Value>,
            ToolPermissionContext,
        ) -> BoxFuture<'static, Result<PermissionResult, AgentError>>
        + Send
        + Sync,
>;

pub struct Query {
    transport: Arc<dyn Transport + Send + Sync>,
    is_streaming_mode: bool,
    can_use_tool: Option<CanUseTool>,
    hooks: Option<HashMap<String, Vec<HookMatcher>>>,
    sdk_mcp_servers: Option<HashMap<String, String>>, // Simplified for now

    // Control protocol state
    pending_control_responses: Arc<Mutex<HashMap<String, Arc<tokio::sync::Notify>>>>,
    pending_control_results: Arc<Mutex<HashMap<String, Result<Value, String>>>>,
    hook_callbacks: Arc<Mutex<HashMap<String, HookCallback>>>,
    next_callback_id: Arc<Mutex<u32>>,
    request_counter: Arc<Mutex<u32>>,

    // Message stream
    message_queue: Arc<Mutex<Vec<Message>>>,
    initialized: bool,
    closed: bool,
}

#[derive(Debug, Clone)]
pub struct HookMatcher {
    pub matcher: Option<String>,
    pub hooks: Vec<String>, // Simplified for now
}

#[derive(Debug, Clone)]
pub struct HookContext {
    pub signal: Option<String>, // Future: abort signal support
}

// Add proper transport integration for receiving messages
#[async_trait]
pub trait MessageHandler {
    async fn handle_message(&self, message: Message) -> Result<(), AgentError>;
}

#[async_trait]
impl MessageHandler for Query {
    async fn handle_message(&self, message: Message) -> Result<(), AgentError> {
        // Add message to queue
        self.message_queue.lock().await.push(message);
        Ok(())
    }
}

impl Query {
    pub fn new(
        transport: Arc<dyn Transport + Send + Sync>,
        is_streaming_mode: bool,
        can_use_tool: Option<CanUseTool>,
        hooks: Option<HashMap<String, Vec<HookMatcher>>>,
        sdk_mcp_servers: Option<HashMap<String, String>>,
    ) -> Self {
        Query {
            transport,
            is_streaming_mode,
            can_use_tool,
            hooks,
            sdk_mcp_servers,
            pending_control_responses: Arc::new(Mutex::new(HashMap::new())),
            pending_control_results: Arc::new(Mutex::new(HashMap::new())),
            hook_callbacks: Arc::new(Mutex::new(HashMap::new())),
            next_callback_id: Arc::new(Mutex::new(0)),
            request_counter: Arc::new(Mutex::new(0)),
            message_queue: Arc::new(Mutex::new(Vec::new())),
            initialized: false,
            closed: false,
        }
    }

    pub async fn initialize(&mut self) -> Result<Option<Value>, AgentError> {
        if !self.is_streaming_mode {
            return Ok(None);
        }

        // Build hooks configuration for initialization
        let mut hooks_config: HashMap<String, Vec<Value>> = HashMap::new();
        if let Some(hooks) = &self.hooks {
            for (event, matchers) in hooks {
                if !matchers.is_empty() {
                    hooks_config.insert(event.clone(), Vec::new());
                    for matcher in matchers {
                        let mut callback_ids = Vec::new();
                        for _callback in &matcher.hooks {
                            let mut next_callback_id = self.next_callback_id.lock().await;
                            let callback_id = format!("hook_{}", next_callback_id);
                            *next_callback_id += 1;
                            callback_ids.push(Value::String(callback_id));
                        }
                        hooks_config.get_mut(event).unwrap().push(json!({
                            "matcher": matcher.matcher,
                            "hookCallbackIds": callback_ids,
                        }));
                    }
                }
            }
        }

        // Send initialize request
        let request = json!({
            "subtype": "initialize",
            "hooks": if hooks_config.is_empty() { None } else { Some(hooks_config) },
        });

        let response = self.send_control_request(&request).await?;
        self.initialized = true;
        Ok(Some(response))
    }

    async fn send_control_request(&self, request: &Value) -> Result<Value, AgentError> {
        if !self.is_streaming_mode {
            return Err(AgentError::ProcessError(
                "Control requests require streaming mode".to_string(),
            ));
        }

        // Generate unique request ID
        let mut request_counter = self.request_counter.lock().await;
        *request_counter += 1;
        let request_id = format!("req_{}_{}", request_counter, uuid::Uuid::new_v4());

        // Create event for response
        let event = Arc::new(tokio::sync::Notify::new());
        self.pending_control_responses
            .lock()
            .await
            .insert(request_id.clone(), event.clone());

        // Build and send request
        let control_request = json!({
            "type": "control_request",
            "request_id": request_id,
            "request": request,
        });

        self.transport.send(control_request).await?;

        // Wait for response with timeout
        let timeout_duration = std::time::Duration::from_secs(60);
        match tokio::time::timeout(timeout_duration, event.notified()).await {
            Ok(_) => {
                // Get the result
                let mut results = self.pending_control_results.lock().await;
                if let Some(result) = results.remove(&request_id) {
                    // Remove the response event
                    self.pending_control_responses
                        .lock()
                        .await
                        .remove(&request_id);

                    match result {
                        Ok(value) => Ok(value),
                        Err(error) => Err(AgentError::ProcessError(error)),
                    }
                } else {
                    Err(AgentError::ProcessError(
                        "No result found for control request".to_string(),
                    ))
                }
            }
            Err(_) => {
                // Timeout occurred
                self.pending_control_responses
                    .lock()
                    .await
                    .remove(&request_id);
                self.pending_control_results
                    .lock()
                    .await
                    .remove(&request_id);
                Err(AgentError::ProcessError(format!(
                    "Control request timeout: {}",
                    request
                        .get("subtype")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                )))
            }
        }
    }

    pub async fn interrupt(&self) -> Result<(), AgentError> {
        let request = json!({"subtype": "interrupt"});
        self.send_control_request(&request).await?;
        Ok(())
    }

    pub async fn set_permission_mode(&self, mode: &str) -> Result<(), AgentError> {
        let request = json!({
            "subtype": "set_permission_mode",
            "mode": mode,
        });
        self.send_control_request(&request).await?;
        Ok(())
    }

    pub async fn set_model(&self, model: Option<&str>) -> Result<(), AgentError> {
        let request = json!({
            "subtype": "set_model",
            "model": model,
        });
        self.send_control_request(&request).await?;
        Ok(())
    }

    // Method to add an MCP server
    pub async fn add_mcp_server(&mut self, name: String, uri: String) -> Result<(), AgentError> {
        if let Some(ref mut servers) = self.sdk_mcp_servers {
            servers.insert(name, uri);
        } else {
            let mut servers = HashMap::new();
            servers.insert(name, uri);
            self.sdk_mcp_servers = Some(servers);
        }
        Ok(())
    }

    // Method to list MCP servers
    pub fn list_mcp_servers(&self) -> Option<&HashMap<String, String>> {
        self.sdk_mcp_servers.as_ref()
    }
}

impl Query {
    // Add proper transport integration for receiving messages
    pub async fn receive_message(&self) -> Result<Value, AgentError> {
        self.transport
            .receive()
            .await
            .map_err(AgentError::TransportError)
    }

    // Implement message processing methods
    pub async fn process_messages(&self) -> Result<Vec<Message>, AgentError> {
        let mut messages = Vec::new();
        loop {
            let json_value = self.receive_message().await?;
            if let Ok(message) = serde_json::from_value::<Message>(json_value.clone()) {
                messages.push(message.clone());

                // Check if this is a result message which indicates the end of the stream
                match message {
                    Message::Result(_) => break,
                    _ => continue,
                }
            } else {
                // If it's not a Message variant, it might be a control response
                if let Some(message_type) = json_value.get("type").and_then(|v| v.as_str()) {
                    if message_type == "control_response" {
                        // Handle control response
                        if let Some(request_id) =
                            json_value.get("request_id").and_then(|v| v.as_str())
                        {
                            let mut results = self.pending_control_results.lock().await;
                            if let Some(response) = json_value.get("response") {
                                results.insert(request_id.to_string(), Ok(response.clone()));
                            } else if let Some(error) =
                                json_value.get("error").and_then(|v| v.as_str())
                            {
                                results.insert(request_id.to_string(), Err(error.to_string()));
                            }

                            // Notify the waiting task
                            let pending_responses = self.pending_control_responses.lock().await;
                            if let Some(event) = pending_responses.get(request_id) {
                                event.notify_one();
                            }
                        }
                    }
                }
            }
        }
        Ok(messages)
    }

    // Implement hook callback handling
    pub async fn register_hook_callback(
        &self,
        callback_id: String,
        callback: HookCallback,
    ) -> Result<(), AgentError> {
        let mut callbacks = self.hook_callbacks.lock().await;
        callbacks.insert(callback_id, callback);
        Ok(())
    }

    // Implement hook execution logic
    pub async fn execute_hook(
        &self,
        callback_id: &str,
        data: HashMap<String, Value>,
        tool_use_id: Option<String>,
        context: HookContext,
    ) -> Result<Value, AgentError> {
        let callbacks = self.hook_callbacks.lock().await;
        if let Some(callback) = callbacks.get(callback_id) {
            callback(data, tool_use_id, context).await
        } else {
            Err(AgentError::ProcessError(format!(
                "Hook callback not found: {}",
                callback_id
            )))
        }
    }

    // Handle permission updates correctly
    pub async fn handle_permission_update(
        &self,
        update: PermissionUpdate,
    ) -> Result<(), AgentError> {
        // This is a placeholder implementation
        // In a full implementation, this would update the permission system
        match update.r#type.as_str() {
            "addRules" => {
                // Add permission rules
                println!("Adding permission rules: {:?}", update.rules);
            }
            "replaceRules" => {
                // Replace permission rules
                println!("Replacing permission rules: {:?}", update.rules);
            }
            "removeRules" => {
                // Remove permission rules
                println!("Removing permission rules: {:?}", update.rules);
            }
            "setMode" => {
                // Set permission mode
                if let Some(mode) = update.mode {
                    self.set_permission_mode(&mode).await?;
                }
            }
            "addDirectories" => {
                // Add directories to permission system
                println!("Adding directories: {:?}", update.directories);
            }
            "removeDirectories" => {
                // Remove directories from permission system
                println!("Removing directories: {:?}", update.directories);
            }
            _ => {
                return Err(AgentError::ProcessError(format!(
                    "Unknown permission update type: {}",
                    update.r#type
                )));
            }
        }
        Ok(())
    }

    // Complete query execution methods
    pub async fn execute_query(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Value>>,
        system: Option<Value>,
    ) -> Result<Value, AgentError> {
        let request = json!({
            "type": "query",
            "messages": messages,
            "tools": tools,
            "system": system,
            "stream": false,
        });

        // Send query request
        self.transport
            .send(request)
            .await
            .map_err(AgentError::TransportError)?;

        // Receive and return the response
        self.receive_message().await
    }

    // Implement streaming query execution
    pub async fn execute_query_streaming(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Value>>,
        system: Option<Value>,
    ) -> Result<Vec<Message>, AgentError> {
        let request = json!({
            "type": "query",
            "messages": messages,
            "tools": tools,
            "system": system,
            "stream": true,
        });

        // Send query request
        self.transport
            .send(request)
            .await
            .map_err(AgentError::TransportError)?;

        // Process messages until we get a result
        self.process_messages().await
    }

    // Method to handle tool use requests with permission checking
    pub async fn handle_tool_use(
        &self,
        tool_name: String,
        tool_input: HashMap<String, Value>,
    ) -> Result<PermissionResult, AgentError> {
        if let Some(can_use_tool) = &self.can_use_tool {
            let context = ToolPermissionContext {
                signal: None,
                suggestions: Vec::new(),
            };
            can_use_tool(tool_name, tool_input, context).await
        } else {
            // Default to allow if no permission callback is set
            Ok(PermissionResult::Allow(PermissionResultAllow {
                behavior: "allow".to_string(),
                updated_input: None,
                updated_permissions: None,
            }))
        }
    }

    // Method to get messages from the queue
    pub async fn get_messages(&self) -> Result<Vec<Message>, AgentError> {
        let mut queue = self.message_queue.lock().await;
        let messages = queue.drain(..).collect::<Vec<Message>>();
        Ok(messages)
    }

    // Method to close the query
    pub async fn close(&mut self) -> Result<(), AgentError> {
        self.closed = true;
        Ok(())
    }

    // Method to check if the query is closed
    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
