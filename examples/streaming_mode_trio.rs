// Example of multi-turn conversation using async patterns with the Claude SDK.
// This demonstrates how to use the ClaudeSDKClient for interactive,
// stateful conversations where you can send follow-up messages based on
// Claude's responses.

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

fn display_message(msg: Message) {
    match msg {
        Message::User(user_msg) => {
            println!("User: {}", user_msg.content);
        }
        Message::Assistant(assistant_msg) => {
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

async fn multi_turn_conversation() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    println!("=== Multi-turn Conversation ===\n");

    // First turn: Simple math question
    println!("User: What's 15 + 27?");
    let user_message = UserMessage {
        content: "What's 15 + 27?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];
    println!("Would send query and receive response");
    println!();

    // Second turn: Follow-up calculation
    println!("User: Now multiply that result by 2");
    let user_message = UserMessage {
        content: "Now multiply that result by 2".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];
    println!("Would send follow-up query and receive response");
    println!();

    // Third turn: One more operation
    println!("User: Divide that by 7 and round to 2 decimal places");
    let user_message = UserMessage {
        content: "Divide that by 7 and round to 2 decimal places".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];
    println!("Would send another query and receive response");

    println!("\nConversation complete!");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    multi_turn_conversation().await?;
    Ok(())
}
