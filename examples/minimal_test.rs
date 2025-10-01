use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

use claude_agent_sdk::types::{Message, UserMessage};
use claude_agent_sdk::{Client, SubprocessCLITransport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Testing basic Claude CLI connection...");

    let transport = SubprocessCLITransport::new("claude", &[])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "What is 2+2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    println!("Sending query...");
    let response = client
        .query(messages, None, None, Some(false), None, None)
        .await?;
    println!("Response: {:?}", response);

    Ok(())
}
