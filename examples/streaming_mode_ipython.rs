// IPython-friendly code snippets for ClaudeSDKClient streaming mode.
// These examples are designed to be copy-pasted directly into IPython.
// Each example is self-contained and can be run independently.

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

// BASIC STREAMING EXAMPLE
async fn basic_streaming_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    println!("User: What is 2+2?");

    let user_message = UserMessage {
        content: "What is 2+2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and receive responses
    println!("Would send query and receive streaming responses");

    Ok(())
}

// STREAMING WITH REAL-TIME DISPLAY EXAMPLE
async fn streaming_with_display_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    async fn send_and_receive(
        client: &Client,
        prompt: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("User: {}", prompt);

        let user_message = UserMessage {
            content: prompt.to_string(),
            parent_tool_use_id: None,
        };
        let messages = vec![Message::User(user_message)];

        // In a real implementation, we would send the query and receive responses
        println!("Would send query and receive response");

        Ok(())
    }

    send_and_receive(&client, "Tell me a short joke").await?;
    println!("\n---\n");
    send_and_receive(&client, "Now tell me a fun fact").await?;

    Ok(())
}

// PERSISTENT CLIENT EXAMPLE
async fn persistent_client_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    println!("User: What's 2+2?");
    let user_message = UserMessage {
        content: "What's 2+2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];
    println!("Would send query and receive response");

    println!("User: What's 10*10?");
    let user_message = UserMessage {
        content: "What's 10*10?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];
    println!("Would send query and receive response");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    basic_streaming_example().await?;
    println!("\n");

    streaming_with_display_example().await?;
    println!("\n");

    persistent_client_example().await?;
    println!("\n");

    Ok(())
}
