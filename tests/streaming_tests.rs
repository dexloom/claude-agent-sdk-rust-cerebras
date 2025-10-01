use claude_agent_sdk::internal::query::*;
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
    async fn test_streaming_query_creation() {
        let mock_transport = MockTransport::new();
        let transport_arc = Arc::new(mock_transport) as Arc<dyn Transport + Send + Sync>;

        let query = Query::new(
            transport_arc,
            true, // streaming mode
            None,
            None,
            None,
        );

        // We can't directly test private fields, so we'll just ensure it compiles
    }

    #[tokio::test]
    async fn test_non_streaming_query_creation() {
        let mock_transport = MockTransport::new();
        let transport_arc = Arc::new(mock_transport) as Arc<dyn Transport + Send + Sync>;

        let query = Query::new(
            transport_arc,
            false, // non-streaming mode
            None,
            None,
            None,
        );

        // We can't directly test private fields, so we'll just ensure it compiles
    }

    #[tokio::test]
    async fn test_query_with_hooks() {
        let mock_transport = MockTransport::new();
        let transport_arc = Arc::new(mock_transport) as Arc<dyn Transport + Send + Sync>;

        let mut hooks = HashMap::new();
        hooks.insert(
            "test_event".to_string(),
            vec![HookMatcher {
                matcher: Some("test_matcher".to_string()),
                hooks: vec!["hook1".to_string()],
            }],
        );

        let query = Query::new(transport_arc, true, None, Some(hooks), None);

        // We can't directly test private fields, so we'll just ensure it compiles
    }

    #[tokio::test]
    async fn test_message_enum_creation() {
        // Test all message types in the enum
        let user_message = Message::User(UserMessage {
            content: "user content".to_string(),
            parent_tool_use_id: None,
        });

        let assistant_message = Message::Assistant(AssistantMessage {
            content: vec![ContentBlock::Text(TextBlock {
                text: "assistant content".to_string(),
            })],
            model: "claude-3-haiku".to_string(),
            parent_tool_use_id: None,
        });

        let system_message = Message::System(SystemMessage {
            subtype: "system_subtype".to_string(),
            data: HashMap::new(),
        });

        let result_message = Message::Result(ResultMessage {
            subtype: "result_subtype".to_string(),
            duration_ms: 100,
            duration_api_ms: 50,
            is_error: false,
            num_turns: 1,
            session_id: "session_123".to_string(),
            total_cost_usd: None,
            usage: None,
            result: None,
        });

        let stream_event = Message::StreamEvent(StreamEvent {
            uuid: "uuid_123".to_string(),
            session_id: "session_123".to_string(),
            event: json!({"type": "test_event"}),
            parent_tool_use_id: None,
        });

        let text_message = Message::Text(TextMessage {
            message_id: "text_123".to_string(),
            content: "text content".to_string(),
            role: "user".to_string(),
            metadata: HashMap::new(),
        });

        let tool_use_message = Message::ToolUse(ToolUseMessage {
            message_id: "tool_use_123".to_string(),
            tool_name: "test_tool".to_string(),
            tool_input: HashMap::new(),
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        });

        let tool_result_message = Message::ToolResult(ToolResultMessage {
            message_id: "tool_result_123".to_string(),
            tool_name: "test_tool".to_string(),
            tool_result: json!("result"),
            role: "user".to_string(),
            metadata: HashMap::new(),
        });

        let partial_text_message = Message::PartialText(PartialTextMessage {
            message_id: "partial_text_123".to_string(),
            content: "partial text content".to_string(),
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        });

        let partial_tool_use_message = Message::PartialToolUse(PartialToolUseMessage {
            message_id: "partial_tool_use_123".to_string(),
            tool_name: "test_tool".to_string(),
            tool_input: HashMap::new(),
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        });

        // Test that all message types can be created
        assert!(matches!(user_message, Message::User(_)));
        assert!(matches!(assistant_message, Message::Assistant(_)));
        assert!(matches!(system_message, Message::System(_)));
        assert!(matches!(result_message, Message::Result(_)));
        assert!(matches!(stream_event, Message::StreamEvent(_)));
        assert!(matches!(text_message, Message::Text(_)));
        assert!(matches!(tool_use_message, Message::ToolUse(_)));
        assert!(matches!(tool_result_message, Message::ToolResult(_)));
        assert!(matches!(partial_text_message, Message::PartialText(_)));
        assert!(matches!(
            partial_tool_use_message,
            Message::PartialToolUse(_)
        ));
    }

    #[tokio::test]
    async fn test_message_id_extraction() {
        // Test that all message types can extract their message_id correctly
        let user_message = Message::User(UserMessage {
            content: "user content".to_string(),
            parent_tool_use_id: None,
        });
        assert_eq!(user_message.message_id(), "user_message");

        let assistant_message = Message::Assistant(AssistantMessage {
            content: vec![ContentBlock::Text(TextBlock {
                text: "assistant content".to_string(),
            })],
            model: "claude-3-haiku".to_string(),
            parent_tool_use_id: None,
        });
        assert_eq!(assistant_message.message_id(), "assistant_message");

        let system_message = Message::System(SystemMessage {
            subtype: "system_subtype".to_string(),
            data: HashMap::new(),
        });
        assert_eq!(system_message.message_id(), "system_message");

        let result_message = Message::Result(ResultMessage {
            subtype: "result_subtype".to_string(),
            duration_ms: 100,
            duration_api_ms: 50,
            is_error: false,
            num_turns: 1,
            session_id: "session_123".to_string(),
            total_cost_usd: None,
            usage: None,
            result: None,
        });
        assert_eq!(result_message.message_id(), "result_message");
    }
}
