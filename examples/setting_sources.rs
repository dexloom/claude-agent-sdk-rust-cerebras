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

// Define ClaudeAgentOptions struct
#[derive(Debug, Clone)]
struct ClaudeAgentOptions {
    setting_sources: Option<Vec<String>>,
    cwd: Option<String>,
}

fn extract_slash_commands(msg: &SystemMessage) -> Vec<String> {
    if msg.subtype == "init" {
        if let Some(commands) = msg.data.get("slash_commands") {
            if let Some(commands_array) = commands.as_array() {
                return commands_array
                    .iter()
                    .filter_map(|cmd| cmd.as_str())
                    .map(|s| s.to_string())
                    .collect();
            }
        }
    }
    Vec::new()
}

async fn example_default() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Default Behavior Example ===");
    println!("Setting sources: None (default)");
    println!("Expected: No custom slash commands will be available\n");

    let options = ClaudeAgentOptions {
        setting_sources: None,
        cwd: None,
    };

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "What is 2 + 2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and check for system messages
    println!("Would send query: \"What is 2 + 2?\" and check for available slash commands");

    println!();
    Ok(())
}

async fn example_user_only() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== User Settings Only Example ===");
    println!("Setting sources: [\"user\"]");
    println!("Expected: Project slash commands (like /commit) will NOT be available\n");

    let options = ClaudeAgentOptions {
        setting_sources: Some(vec!["user".to_string()]),
        cwd: None,
    };

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "What is 2 + 2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and check for system messages
    println!("Would send query: \"What is 2 + 2?\" and check for available slash commands");

    println!();
    Ok(())
}

async fn example_project_and_user() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Project + User Settings Example ===");
    println!("Setting sources: [\"user\", \"project\"]");
    println!("Expected: Project slash commands (like /commit) WILL be available\n");

    let options = ClaudeAgentOptions {
        setting_sources: Some(vec!["user".to_string(), "project".to_string()]),
        cwd: None,
    };

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "What is 2 + 2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and check for system messages
    println!("Would send query: \"What is 2 + 2?\" and check for available slash commands");

    println!();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting Claude SDK Setting Sources Examples...");
    println!("==================================================\n");

    example_default().await?;
    println!("--------------------------------------------------\n");

    example_user_only().await?;
    println!("--------------------------------------------------\n");

    example_project_and_user().await?;
    println!("--------------------------------------------------\n");

    Ok(())
}
