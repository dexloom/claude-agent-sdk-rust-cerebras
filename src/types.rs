use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Content block types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextBlock {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThinkingBlock {
    pub thinking: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolUseBlock {
    pub id: String,
    pub name: String,
    pub input: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolResultBlock {
    pub tool_use_id: String,
    pub content: Option<Value>,
    pub is_error: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text(TextBlock),
    #[serde(rename = "thinking")]
    Thinking(ThinkingBlock),
    #[serde(rename = "tool_use")]
    ToolUse(ToolUseBlock),
    #[serde(rename = "tool_result")]
    ToolResult(ToolResultBlock),
}

// Message types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMessage {
    pub content: String,
    pub parent_tool_use_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistantMessage {
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub parent_tool_use_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemMessage {
    pub subtype: String,
    pub data: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultMessage {
    pub subtype: String,
    pub duration_ms: u64,
    pub duration_api_ms: u64,
    pub is_error: bool,
    pub num_turns: u32,
    pub session_id: String,
    pub total_cost_usd: Option<f64>,
    pub usage: Option<HashMap<String, Value>>,
    pub result: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamEvent {
    pub uuid: String,
    pub session_id: String,
    pub event: Value,
    pub parent_tool_use_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "user")]
    User(UserMessage),
    #[serde(rename = "assistant")]
    Assistant(AssistantMessage),
    #[serde(rename = "system")]
    System(SystemMessage),
    #[serde(rename = "result")]
    Result(ResultMessage),
    #[serde(rename = "stream_event")]
    StreamEvent(StreamEvent),
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

impl Message {
    pub fn message_id(&self) -> &str {
        match self {
            Message::User(_) => "user_message",
            Message::Assistant(_) => "assistant_message",
            Message::System(_) => "system_message",
            Message::Result(_) => "result_message",
            Message::StreamEvent(_) => "stream_event",
            Message::Text(msg) => &msg.message_id,
            Message::ToolUse(msg) => &msg.message_id,
            Message::ToolResult(msg) => &msg.message_id,
            Message::PartialText(msg) => &msg.message_id,
            Message::PartialToolUse(msg) => &msg.message_id,
        }
    }
}

// Existing message types from previous implementation
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}
