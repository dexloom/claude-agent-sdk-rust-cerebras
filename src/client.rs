use crate::error::AgentError;
use crate::transport::Transport;
use crate::types::{
    AssistantMessage, Message, ResultMessage, StreamEvent, SystemMessage, UserMessage,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Client {
    transport: Arc<Mutex<dyn Transport>>,
}

impl Client {
    pub fn new(transport: Arc<Mutex<dyn Transport>>) -> Self {
        Self { transport }
    }

    pub async fn query(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Value>>,
        system: Option<Value>,
        stream: Option<bool>,
        on_message: Option<fn(Message)>,
        _tool_permissions: Option<HashMap<String, bool>>,
    ) -> Result<Value, AgentError> {
        // Build the query request
        let request = json!({
            "type": "query",
            "messages": messages,
            "tools": tools,
            "system": system,
            "stream": stream.unwrap_or(false),
        });

        // Send the query request through the transport
        self.send_message(request).await?;

        // If stream is enabled and on_message callback is provided, handle streaming
        if stream.unwrap_or(false) {
            if let Some(callback) = on_message {
                loop {
                    let json_value = self.receive_message().await?;
                    // Try to parse as Message, if it fails just continue
                    if let Ok(message) = serde_json::from_value::<Message>(json_value.clone()) {
                        callback(message.clone());

                        // Check if this is a result message which indicates the end of the stream
                        match message {
                            Message::Result(_) => break,
                            _ => continue,
                        }
                    } else {
                        // If it's not a Message variant, it might be a response that we should return
                        return Ok(json_value);
                    }
                }
            }
        }

        // Receive and return the response
        let response = self.receive_message().await?;
        Ok(response)
    }

    pub async fn send_message(&self, message: Value) -> Result<(), AgentError> {
        self.transport.lock().await.send(message).await?;
        Ok(())
    }

    pub async fn receive_message(&self) -> Result<Value, AgentError> {
        let result = self.transport.lock().await.receive().await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(AgentError::TransportError(e)),
        }
    }

    // Additional methods based on the Python SDK implementation
    pub async fn get_next_message(&self) -> Result<Message, AgentError> {
        let json_value = self.receive_message().await?;
        // Parse the JSON into the appropriate Message variant
        let message: Message = serde_json::from_value(json_value)?;
        Ok(message)
    }

    pub async fn send_user_message(&self, content: &str) -> Result<(), AgentError> {
        let message = UserMessage {
            content: content.to_string(),
            parent_tool_use_id: None,
        };
        let json_value = serde_json::to_value(message)?;
        self.send_message(json_value).await
    }

    pub async fn send_assistant_message(
        &self,
        content: Vec<crate::types::ContentBlock>,
        model: &str,
    ) -> Result<(), AgentError> {
        let message = AssistantMessage {
            content,
            model: model.to_string(),
            parent_tool_use_id: None,
        };
        let json_value = serde_json::to_value(message)?;
        self.send_message(json_value).await
    }

    pub async fn send_system_message(
        &self,
        subtype: &str,
        data: HashMap<String, Value>,
    ) -> Result<(), AgentError> {
        let message = SystemMessage {
            subtype: subtype.to_string(),
            data,
        };
        let json_value = serde_json::to_value(message)?;
        self.send_message(json_value).await
    }

    pub async fn send_result_message(&self, result_msg: ResultMessage) -> Result<(), AgentError> {
        let json_value = serde_json::to_value(result_msg)?;
        self.send_message(json_value).await
    }

    pub async fn send_stream_event(&self, event: StreamEvent) -> Result<(), AgentError> {
        let json_value = serde_json::to_value(event)?;
        self.send_message(json_value).await
    }
}
