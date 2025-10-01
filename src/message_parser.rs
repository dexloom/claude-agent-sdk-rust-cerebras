use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "text")]
    Text(TextMessage),
    #[serde(rename = "tool_use")]
    ToolUse(ToolUseMessage),
    #[serde(rename = "tool_result")]
    ToolResult(ToolResultMessage),
    #[serde(rename = "partial_text")]
    PartialText(PartialTextMessage),
    #[serde(rename = "partial_tool_use")]
    PartialToolUse(PartialToolUseMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextMessage {
    pub message_id: String,
    pub content: String,
    pub role: String,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolUseMessage {
    pub message_id: String,
    pub tool_name: String,
    pub tool_input: HashMap<String, Value>,
    pub role: String,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolResultMessage {
    pub message_id: String,
    pub tool_name: String,
    pub tool_result: Value,
    pub role: String,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialTextMessage {
    pub message_id: String,
    pub content: String,
    pub role: String,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialToolUseMessage {
    pub message_id: String,
    pub tool_name: String,
    pub tool_input: HashMap<String, Value>,
    pub role: String,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

impl Message {
    pub fn message_id(&self) -> &str {
        match self {
            Message::Text(msg) => &msg.message_id,
            Message::ToolUse(msg) => &msg.message_id,
            Message::ToolResult(msg) => &msg.message_id,
            Message::PartialText(msg) => &msg.message_id,
            Message::PartialToolUse(msg) => &msg.message_id,
        }
    }
}
