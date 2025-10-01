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

async fn basic_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Basic Example ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "What is 2 + 2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and receive responses
    // client.query("What is 2 + 2?").await?;
    // while let Ok(message) = client.get_next_message().await {
    //     if let Message::Assistant(assistant_msg) = message {
    //         for block in assistant_msg.content {
    //             if let ContentBlock::Text(text_block) = block {
    //                 println!("Claude: {}", text_block.text);
    //             }
    //         }
    //     }
    // }
    println!("Would send query: \"What is 2 + 2?\" and receive responses");
    println!();

    Ok(())
}

async fn with_options_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== With Options Example ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "Explain what Python is in one sentence.".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query with options
    // client.query("Explain what Python is in one sentence.").await?;
    // while let Ok(message) = client.get_next_message().await {
    //     if let Message::Assistant(assistant_msg) = message {
    //         for block in assistant_msg.content {
    //             if let ContentBlock::Text(text_block) = block {
    //                 println!("Claude: {}", text_block.text);
    //             }
    //         }
    //     }
    // }
    println!("Would send query: \"Explain what Python is in one sentence.\" with custom options");
    println!();

    Ok(())
}

async fn with_tools_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== With Tools Example ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    let user_message = UserMessage {
        content: "Create a file called hello.txt with 'Hello, World!' in it".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query with tools allowed
    // client.query("Create a file called hello.txt with 'Hello, World!' in it").await?;
    // while let Ok(message) = client.get_next_message().await {
    //     match message {
    //         Message::Assistant(assistant_msg) => {
    //             for block in assistant_msg.content {
    //                 if let ContentBlock::Text(text_block) = block {
    //                     println!("Claude: {}", text_block.text);
    //                 }
    //             }
    //         },
    //         Message::Result(result_msg) => {
    //             if let Some(cost) = result_msg.total_cost_usd {
    //                 if cost > 0.0 {
    //                     println!("\nCost: ${:.4}", cost);
    //                 }
    //             }
    //         },
    //         _ => {}
    //     }
    // }
    println!("Would send query: \"Create a file called hello.txt with 'Hello, World!' in it\" with file tools allowed");
    println!();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    basic_example().await?;
    with_options_example().await?;
    with_tools_example().await?;
    Ok(())
}
