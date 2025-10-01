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
    stderr: Option<fn(String)>, // Callback function for stderr
    extra_args: Option<HashMap<String, Option<String>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create options with stderr callback and enable debug mode
    let mut extra_args = HashMap::new();
    extra_args.insert("debug-to-stderr".to_string(), None);

    fn stderr_callback(message: String) {
        // Optionally print specific messages
        if message.contains("[ERROR]") {
            println!("Error detected: {}", message);
        }
    }

    let options = ClaudeAgentOptions {
        stderr: Some(stderr_callback),
        extra_args: Some(extra_args),
    };

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Run a query
    println!("Running query with stderr capture...");

    let user_message = UserMessage {
        content: "What is 2+2?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and capture stderr
    println!("Would send query and capture stderr output");

    Ok(())
}
