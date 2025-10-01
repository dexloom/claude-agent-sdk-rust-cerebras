use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

use claude_agent_sdk::transport::Transport;
use claude_agent_sdk::types::{
    AssistantMessage, ContentBlock, Message, ResultMessage, StreamEvent, SystemMessage, TextBlock,
    ToolResultBlock, ToolUseBlock, UserMessage,
};
use claude_agent_sdk::{Client, SubprocessCLITransport};

// Define ClaudeAgentOptions struct
#[derive(Debug, Clone)]
struct ClaudeAgentOptions {
    allowed_tools: Option<Vec<String>>,
    system_prompt: Option<String>,
    env: Option<HashMap<String, String>>,
    model: Option<String>,
}

fn display_message(msg: Message) {
    match msg {
        Message::User(user_msg) => {
            // Handle different content block types
            println!("User: {}", user_msg.content);
        }
        Message::Assistant(assistant_msg) => {
            // Handle different content block types
            for block in assistant_msg.content {
                match block {
                    ContentBlock::Text(text_block) => {
                        println!("Claude: {}", text_block.text);
                    }
                    _ => {}
                }
            }
        }
        Message::System(system_msg) => {
            // Ignore system messages
        }
        Message::Result(result_msg) => {
            println!("Result ended");
        }
        _ => {}
    }
}

async fn example_basic_streaming() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Basic Streaming Example ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    println!("User: What is 2+2?");

    let user_message = UserMessage {
        content: "What is 2+2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and receive responses
    // client.query("What is 2+2?").await?;
    // while let Ok(msg) = client.get_next_message().await {
    //     display_message(msg);
    // }
    println!("Would send query and receive streaming responses");

    println!("\n");
    Ok(())
}

async fn example_multi_turn_conversation() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Multi-Turn Conversation Example ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // First turn
    println!("User: What's the capital of France?");
    let user_message = UserMessage {
        content: "What's the capital of France?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];
    println!("Would send query and receive response");

    // Second turn - follow-up
    println!("\nUser: What's the population of that city?");
    let user_message = UserMessage {
        content: "What's the population of that city?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];
    println!("Would send follow-up query and receive response");

    println!("\n");
    Ok(())
}

async fn example_with_options() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Custom Options Example ===");

    // Configure options
    let options = ClaudeAgentOptions {
        allowed_tools: Some(vec!["Read".to_string(), "Write".to_string()]),
        system_prompt: Some("You are a helpful coding assistant.".to_string()),
        env: None,
        model: None,
    };

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    println!("User: Create a simple hello.txt file with a greeting message");
    let user_message = UserMessage {
        content: "Create a simple hello.txt file with a greeting message".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and receive responses
    println!("Would send query with custom options and receive responses");

    println!("\n");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    example_basic_streaming().await?;
    example_multi_turn_conversation().await?;
    example_with_options().await?;
    Ok(())
}
