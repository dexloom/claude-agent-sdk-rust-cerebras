use claude_agent_sdk::message_parser::*;
use serde_json::Value;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_message_creation() {
        let message = TextMessage {
            message_id: "test_id".to_string(),
            content: "test content".to_string(),
            role: "user".to_string(),
            metadata: HashMap::new(),
        };

        assert_eq!(message.message_id, "test_id");
        assert_eq!(message.content, "test content");
        assert_eq!(message.role, "user");
    }

    #[test]
    fn test_tool_use_message_creation() {
        let mut tool_input = HashMap::new();
        tool_input.insert("key".to_string(), Value::String("value".to_string()));

        let message = ToolUseMessage {
            message_id: "test_id".to_string(),
            tool_name: "test_tool".to_string(),
            tool_input,
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        };

        assert_eq!(message.message_id, "test_id");
        assert_eq!(message.tool_name, "test_tool");
        assert_eq!(message.role, "assistant");
        assert_eq!(
            message.tool_input.get("key").unwrap(),
            &Value::String("value".to_string())
        );
    }

    #[test]
    fn test_tool_result_message_creation() {
        let message = ToolResultMessage {
            message_id: "test_id".to_string(),
            tool_name: "test_tool".to_string(),
            tool_result: Value::String("result".to_string()),
            role: "user".to_string(),
            metadata: HashMap::new(),
        };

        assert_eq!(message.message_id, "test_id");
        assert_eq!(message.tool_name, "test_tool");
        assert_eq!(message.role, "user");
        assert_eq!(message.tool_result, Value::String("result".to_string()));
    }

    #[test]
    fn test_partial_text_message_creation() {
        let message = PartialTextMessage {
            message_id: "test_id".to_string(),
            content: "partial content".to_string(),
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        };

        assert_eq!(message.message_id, "test_id");
        assert_eq!(message.content, "partial content");
        assert_eq!(message.role, "assistant");
    }

    #[test]
    fn test_partial_tool_use_message_creation() {
        let mut tool_input = HashMap::new();
        tool_input.insert("key".to_string(), Value::String("value".to_string()));

        let message = PartialToolUseMessage {
            message_id: "test_id".to_string(),
            tool_name: "test_tool".to_string(),
            tool_input,
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        };

        assert_eq!(message.message_id, "test_id");
        assert_eq!(message.tool_name, "test_tool");
        assert_eq!(message.role, "assistant");
        assert_eq!(
            message.tool_input.get("key").unwrap(),
            &Value::String("value".to_string())
        );
    }

    #[test]
    fn test_message_id_extraction() {
        let text_message = Message::Text(TextMessage {
            message_id: "text_id".to_string(),
            content: "test".to_string(),
            role: "user".to_string(),
            metadata: HashMap::new(),
        });
        assert_eq!(text_message.message_id(), "text_id");

        let tool_use_message = Message::ToolUse(ToolUseMessage {
            message_id: "tool_use_id".to_string(),
            tool_name: "test_tool".to_string(),
            tool_input: HashMap::new(),
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        });
        assert_eq!(tool_use_message.message_id(), "tool_use_id");

        let tool_result_message = Message::ToolResult(ToolResultMessage {
            message_id: "tool_result_id".to_string(),
            tool_name: "test_tool".to_string(),
            tool_result: Value::String("result".to_string()),
            role: "user".to_string(),
            metadata: HashMap::new(),
        });
        assert_eq!(tool_result_message.message_id(), "tool_result_id");

        let partial_text_message = Message::PartialText(PartialTextMessage {
            message_id: "partial_text_id".to_string(),
            content: "partial".to_string(),
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        });
        assert_eq!(partial_text_message.message_id(), "partial_text_id");

        let partial_tool_use_message = Message::PartialToolUse(PartialToolUseMessage {
            message_id: "partial_tool_use_id".to_string(),
            tool_name: "test_tool".to_string(),
            tool_input: HashMap::new(),
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        });
        assert_eq!(partial_tool_use_message.message_id(), "partial_tool_use_id");
    }

    #[test]
    fn test_message_serialization_deserialization() {
        // Test TextMessage serialization
        let text_message = TextMessage {
            message_id: "text_id".to_string(),
            content: "test content".to_string(),
            role: "user".to_string(),
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&text_message).unwrap();
        let deserialized: TextMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(text_message.message_id, deserialized.message_id);

        // Test ToolUseMessage serialization
        let mut tool_input = HashMap::new();
        tool_input.insert("key".to_string(), Value::String("value".to_string()));

        let tool_use_message = ToolUseMessage {
            message_id: "tool_use_id".to_string(),
            tool_name: "test_tool".to_string(),
            tool_input: tool_input.clone(),
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&tool_use_message).unwrap();
        let deserialized: ToolUseMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(tool_use_message.message_id, deserialized.message_id);
        assert_eq!(tool_use_message.tool_name, deserialized.tool_name);

        // Test ToolResultMessage serialization
        let tool_result_message = ToolResultMessage {
            message_id: "tool_result_id".to_string(),
            tool_name: "test_tool".to_string(),
            tool_result: Value::String("result".to_string()),
            role: "user".to_string(),
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&tool_result_message).unwrap();
        let deserialized: ToolResultMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(tool_result_message.message_id, deserialized.message_id);
        assert_eq!(tool_result_message.tool_result, deserialized.tool_result);

        // Test PartialTextMessage serialization
        let partial_text_message = PartialTextMessage {
            message_id: "partial_text_id".to_string(),
            content: "partial content".to_string(),
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&partial_text_message).unwrap();
        let deserialized: PartialTextMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(partial_text_message.message_id, deserialized.message_id);
        assert_eq!(partial_text_message.content, deserialized.content);

        // Test PartialToolUseMessage serialization
        let partial_tool_use_message = PartialToolUseMessage {
            message_id: "partial_tool_use_id".to_string(),
            tool_name: "test_tool".to_string(),
            tool_input,
            role: "assistant".to_string(),
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&partial_tool_use_message).unwrap();
        let deserialized: PartialToolUseMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(partial_tool_use_message.message_id, deserialized.message_id);
        assert_eq!(partial_tool_use_message.tool_name, deserialized.tool_name);
    }
}
