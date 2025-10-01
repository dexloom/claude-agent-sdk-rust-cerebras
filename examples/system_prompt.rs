// Example demonstrating different system_prompt configurations.

use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

use claude_agent_sdk::transport::Transport;
use claude_agent_sdk::types::{
    AssistantMessage, ContentBlock, Message, ResultMessage, SystemMessage, TextBlock, UserMessage,
};
use claude_agent_sdk::{Client, SubprocessCLITransport};

async fn no_system_prompt() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== No System Prompt (Vanilla Claude) ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "What is 2 + 2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and receive responses
    println!("Would send query: \"What is 2 + 2?\" and receive responses");
    println!();

    Ok(())
}

async fn string_system_prompt() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== String System Prompt ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "What is 2 + 2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query with a system prompt and receive responses
    println!(
        "Would send query: \"What is 2 + 2?\" with pirate system prompt and receive responses"
    );
    println!();

    Ok(())
}

async fn preset_system_prompt() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Preset System Prompt (Default) ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "What is 2 + 2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query with a preset system prompt and receive responses
    println!(
        "Would send query: \"What is 2 + 2?\" with preset system prompt and receive responses"
    );
    println!();

    Ok(())
}

async fn preset_with_append() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Preset System Prompt with Append ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "What is 2 + 2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query with a preset system prompt and append and receive responses
    println!("Would send query: \"What is 2 + 2?\" with preset system prompt and append and receive responses");
    println!();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    no_system_prompt().await?;
    string_system_prompt().await?;
    preset_system_prompt().await?;
    preset_with_append().await?;
    Ok(())
}
