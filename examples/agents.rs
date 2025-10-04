use serde_json::Value;
use std::collections::HashMap;
use std::process::Command;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

use claude_agent_sdk::transport::Transport;
use claude_agent_sdk::types::{Message, UserMessage};
use claude_agent_sdk::{Client, SubprocessCLITransport};

// Define types for AgentDefinition
#[derive(Debug, Clone)]
struct AgentDefinition {
    description: String,
    prompt: String,
    tools: Vec<String>,
    model: Option<String>,
}

#[derive(Debug, Clone)]
struct ClaudeAgentOptions {
    agents: HashMap<String, AgentDefinition>,
}

fn check_claude_cli() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("which").arg("claude").output()?;
    if !output.stdout.is_empty() {
        Ok(())
    } else {
        Err("Claude CLI not found. Please install it before running this example.".into())
    }
}

async fn code_reviewer_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Code Reviewer Agent Example ===");

    // Check if Claude CLI is available
    if let Err(e) = check_claude_cli() {
        println!("Skipping example: {}", e);
        return Ok(());
    }

    // Create transport (need to specify the actual CLI command)
    let agents_json = r#"{"code-reviewer": {"description": "Reviews Rust code", "prompt": "You are a Rust code reviewer agent. Review the provided Rust code and provide feedback on best practices, potential bugs, and improvements.", "tools": ["Edit"]}}"#;
    let transport = SubprocessCLITransport::new(
        "claude",
        &["--mode", "streaming", "--print", "--agents", agents_json],
    )?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Send query
    let user_message = UserMessage {
        content: "Review the code in src/transport.rs".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    match client
        .query(
            messages, None, // tools
            None, // system
            None, // stream
            None, // on_message
            None, // tool_permissions
        )
        .await
    {
        Ok(response) => {
            println!("Response: {:?}", response);
            println!();
            Ok(())
        }
        Err(e) => {
            println!("Error in code reviewer example: {}", e);
            println!();
            Ok(()) // Continue with other examples
        }
    }
}

async fn documentation_writer_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Documentation Writer Agent Example ===");

    // Check if Claude CLI is available
    if let Err(e) = check_claude_cli() {
        println!("Skipping example: {}", e);
        return Ok(());
    }

    // Create transport (need to specify the actual CLI command)
    let agents_json = r#"{"doc-writer": {"description": "Writes documentation", "prompt": "You are a documentation writer agent. Explain code concepts and create documentation.", "tools": []}}"#;
    let transport = SubprocessCLITransport::new(
        "claude",
        &["--mode", "streaming", "--print", "--agents", agents_json],
    )?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Send query
    let user_message = UserMessage {
        content: "Use the doc-writer agent to explain what SubprocessCLITransport is used for"
            .to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    match client
        .query(
            messages, None, // tools
            None, // system
            None, // stream
            None, // on_message
            None, // tool_permissions
        )
        .await
    {
        Ok(response) => {
            println!("Response: {:?}", response);
            println!();
            Ok(())
        }
        Err(e) => {
            println!("Error in documentation writer example: {}", e);
            println!();
            Ok(()) // Continue with other examples
        }
    }
}

async fn multiple_agents_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Multiple Agents Example ===");

    // Check if Claude CLI is available
    if let Err(e) = check_claude_cli() {
        println!("Skipping example: {}", e);
        return Ok(());
    }

    // Create transport (need to specify the actual CLI command)
    let agents_json = r#"{"analyzer": {"description": "File system analyzer", "prompt": "You are a file system analyzer. Find files and report on their contents.", "tools": ["Bash(find:*)"]}}"#;
    let transport = SubprocessCLITransport::new(
        "claude",
        &["--mode", "streaming", "--print", "--agents", agents_json],
    )?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Send query
    let user_message = UserMessage {
        content: "Use the analyzer agent to find all Rust files in the examples/ directory"
            .to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    match client
        .query(
            messages, None, // tools
            None, // system
            None, // stream
            None, // on_message
            None, // tool_permissions
        )
        .await
    {
        Ok(response) => {
            println!("Response: {:?}", response);
            println!();
            Ok(())
        }
        Err(e) => {
            println!("Error in multiple agents example: {}", e);
            println!();
            Ok(()) // Continue with other examples
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    code_reviewer_example().await?;
    documentation_writer_example().await?;
    multiple_agents_example().await?;
    Ok(())
}
