use claude_agent_sdk::transport::Transport;
use claude_agent_sdk::types::*;
use claude_agent_sdk::*;
use mockall::mock;
use mockall::predicate::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// Mock transport for testing
mock! {
    pub Transport {}
    #[async_trait::async_trait]
    impl Transport for Transport {
        async fn send(&self, payload: Value) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
        async fn receive(&self) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sdk_initialization() {
        // Test that we can create a client with a transport
        let mock_transport = MockTransport::new();
        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        // We can't directly test private fields, so we'll just ensure it compiles
    }

    #[tokio::test]
    async fn test_message_workflow() {
        // Test sending different types of messages through the client
        let mut mock_transport = MockTransport::new();
        mock_transport
            .expect_send()
            .returning(|_| Ok(json!(null)))
            .times(5); // We'll test 5 different message types

        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        // Test user message
        assert!(client.send_user_message("Hello, Claude!").await.is_ok());

        // Test assistant message
        let content = vec![ContentBlock::Text(TextBlock {
            text: "Hello, User!".to_string(),
        })];
        assert!(client
            .send_assistant_message(content, "claude-3-haiku")
            .await
            .is_ok());

        // Test system message
        let mut data = HashMap::new();
        data.insert("test_key".to_string(), json!("test_value"));
        assert!(client
            .send_system_message("test_subtype", data)
            .await
            .is_ok());

        // Test result message
        let result_msg = ResultMessage {
            subtype: "test".to_string(),
            duration_ms: 100,
            duration_api_ms: 50,
            is_error: false,
            num_turns: 1,
            session_id: "session_123".to_string(),
            total_cost_usd: None,
            usage: None,
            result: None,
        };
        assert!(client.send_result_message(result_msg).await.is_ok());

        // Test stream event
        let stream_event = StreamEvent {
            uuid: "uuid_123".to_string(),
            session_id: "session_123".to_string(),
            event: json!({"type": "test"}),
            parent_tool_use_id: None,
        };
        assert!(client.send_stream_event(stream_event).await.is_ok());
    }

    #[tokio::test]
    async fn test_message_serialization() {
        // Test that messages can be serialized to JSON
        let user_msg = UserMessage {
            content: "test content".to_string(),
            parent_tool_use_id: None,
        };
        let json_value = serde_json::to_value(user_msg).unwrap();
        assert!(json_value.is_object());

        let text_block = TextBlock {
            text: "test text".to_string(),
        };
        let assistant_msg = AssistantMessage {
            content: vec![ContentBlock::Text(text_block)],
            model: "claude-3-haiku".to_string(),
            parent_tool_use_id: None,
        };
        let json_value = serde_json::to_value(assistant_msg).unwrap();
        assert!(json_value.is_object());
    }

    #[tokio::test]
    async fn test_query_request_response() {
        // Test QueryRequest and QueryResponse serialization
        let query_request = query::QueryRequest {
            query: "test query".to_string(),
            include_partial_messages: Some(true),
            max_tokens: Some(100),
            temperature: Some(0.7),
            tools: None,
            tool_permission_callback: None,
            extra_params: HashMap::new(),
        };
        let json_value = serde_json::to_value(query_request).unwrap();
        assert!(json_value.is_object());

        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), json!("value"));

        let query_response = query::QueryResponse {
            message_id: "response_123".to_string(),
            content: vec![],
            role: "assistant".to_string(),
            metadata,
        };
        let json_value = serde_json::to_value(query_response).unwrap();
        assert!(json_value.is_object());
    }
}
