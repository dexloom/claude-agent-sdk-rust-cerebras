use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

use claude_agent_sdk::transport::Transport;
use claude_agent_sdk::types::{
    AssistantMessage, Message, ResultMessage, StreamEvent, SystemMessage, UserMessage,
};
use claude_agent_sdk::{Client, SubprocessCLITransport};

// Define ClaudeAgentOptions struct
#[derive(Debug, Clone)]
struct ClaudeAgentOptions {
    include_partial_messages: Option<bool>,
    model: Option<String>,
    max_turns: Option<u32>,
    env: Option<HashMap<String, String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Enable partial message streaming
    let mut env = HashMap::new();
    env.insert("MAX_THINKING_TOKENS".to_string(), "8000".to_string());

    let options = ClaudeAgentOptions {
        include_partial_messages: Some(true),
        model: Some("claude-sonnet-4-5".to_string()),
        max_turns: Some(2),
        env: Some(env),
    };

    // Create client
    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Send a prompt that will generate a streaming response
    // prompt = "Run a bash command to sleep for 5 seconds"
    let prompt = "Think of three jokes, then tell one";
    println!("Prompt: {}\n", prompt);
    println!("==================================================");

    let user_message = UserMessage {
        content: prompt.to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and receive responses
    // client.query(prompt).await?;
    // while let Ok(message) = client.get_next_message().await {
    //     println!("{:?}", message);
    // }
    println!("Would send query with client and receive streaming responses");

    Ok(())
}
