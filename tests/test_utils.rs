use claude_agent_sdk::transport::Transport;
use mockall::mock;
use serde_json::Value;

// Mock transport for testing
mock! {
    pub Transport {}
    #[async_trait::async_trait]
    impl Transport for Transport {
        async fn send(&self, payload: Value) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
        async fn receive(&self) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
    }
}

// Utility functions for testing
pub fn create_test_user_message(content: &str) -> claude_agent_sdk::types::UserMessage {
    claude_agent_sdk::types::UserMessage {
        content: content.to_string(),
        parent_tool_use_id: None,
    }
}

pub fn create_test_assistant_message(text: &str) -> claude_agent_sdk::types::AssistantMessage {
    claude_agent_sdk::types::AssistantMessage {
        content: vec![claude_agent_sdk::types::ContentBlock::Text(
            claude_agent_sdk::types::TextBlock {
                text: text.to_string(),
            },
        )],
        model: "claude-3-haiku".to_string(),
        parent_tool_use_id: None,
    }
}

pub fn create_test_system_message(subtype: &str) -> claude_agent_sdk::types::SystemMessage {
    claude_agent_sdk::types::SystemMessage {
        subtype: subtype.to_string(),
        data: std::collections::HashMap::new(),
    }
}
