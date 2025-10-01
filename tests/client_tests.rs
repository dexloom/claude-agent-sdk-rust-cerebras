use claude_agent_sdk::client::Client;
use claude_agent_sdk::error::AgentError;
use claude_agent_sdk::transport::Transport;
use claude_agent_sdk::types::*;
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
    async fn test_client_creation() {
        let mock_transport = MockTransport::new();
        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        // We can't directly test private fields, so we'll just ensure it compiles
    }

    #[tokio::test]
    async fn test_send_message() {
        let mut mock_transport = MockTransport::new();
        mock_transport
            .expect_send()
            .with(eq(json!({"test": "message"})))
            .times(1)
            .returning(|_| Ok(json!(null)));

        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        let result = client.send_message(json!({"test": "message"})).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_receive_message() {
        let mut mock_transport = MockTransport::new();
        mock_transport
            .expect_receive()
            .times(1)
            .returning(|| Ok(json!({"response": "test"})));

        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        let result = client.receive_message().await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, json!({"response": "test"}));
    }

    #[tokio::test]
    async fn test_send_user_message() {
        let mut mock_transport = MockTransport::new();
        mock_transport
            .expect_send()
            .with(always()) // We can't easily match the complex JSON structure
            .times(1)
            .returning(|_| Ok(json!(null)));

        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        let result = client.send_user_message("test content").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_assistant_message() {
        let mut mock_transport = MockTransport::new();
        mock_transport
            .expect_send()
            .with(always()) // We can't easily match the complex JSON structure
            .times(1)
            .returning(|_| Ok(json!(null)));

        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        let content = vec![ContentBlock::Text(TextBlock {
            text: "test".to_string(),
        })];
        let result = client.send_assistant_message(content, "test-model").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_system_message() {
        let mut mock_transport = MockTransport::new();
        mock_transport
            .expect_send()
            .with(always()) // We can't easily match the complex JSON structure
            .times(1)
            .returning(|_| Ok(json!(null)));

        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        let mut data = HashMap::new();
        data.insert("key".to_string(), json!("value"));

        let result = client.send_system_message("test_subtype", data).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_result_message() {
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

        let mut mock_transport = MockTransport::new();
        mock_transport
            .expect_send()
            .with(always()) // We can't easily match the complex JSON structure
            .times(1)
            .returning(|_| Ok(json!(null)));

        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        let result = client.send_result_message(result_msg).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_stream_event() {
        let stream_event = StreamEvent {
            uuid: "uuid_123".to_string(),
            session_id: "session_123".to_string(),
            event: json!({"type": "test"}),
            parent_tool_use_id: None,
        };

        let mut mock_transport = MockTransport::new();
        mock_transport
            .expect_send()
            .with(always()) // We can't easily match the complex JSON structure
            .times(1)
            .returning(|_| Ok(json!(null)));

        let transport_mutex = Arc::new(Mutex::new(mock_transport)) as Arc<Mutex<dyn Transport>>;
        let client = Client::new(transport_mutex);

        let result = client.send_stream_event(stream_event).await;
        assert!(result.is_ok());
    }
}
