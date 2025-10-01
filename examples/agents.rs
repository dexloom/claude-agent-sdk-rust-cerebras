use serde_json::Value;
use std::collections::HashMap;
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

async fn code_reviewer_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Code Reviewer Agent Example ===");

    // Define agent options
    let mut agents = HashMap::new();
    agents.insert(
        "code-reviewer".to_string(),
        AgentDefinition {
            description: "Reviews code for best practices and potential issues".to_string(),
            prompt: "You are a code reviewer. Analyze code for bugs, performance issues, \
                 security vulnerabilities, and adherence to best practices. \
                 Provide constructive feedback."
                .to_string(),
            tools: vec!["Read".to_string(), "Grep".to_string()],
            model: Some("sonnet".to_string()),
        },
    );

    let options = ClaudeAgentOptions { agents };

    // Create transport (need to specify the actual CLI command)
    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Send query
    let user_message = UserMessage {
        content: "Use the doc-writer agent to explain what AgentDefinition is used for".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // Since the query method is not fully implemented, we'll demonstrate the intended usage
    // let response = client.query(
    //     messages,
    //     None, // tools
    //     None, // system
    //     None, // stream
    //     None, // on_message
    //     None, // tool_permissions
    // ).await?;

    println!("Would send query: \"Use the code-reviewer agent to review the code in src/claude_agent_sdk/types.rs\"");
    println!();

    Ok(())
}

async fn documentation_writer_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Documentation Writer Agent Example ===");

    // Define agent options
    let mut agents = HashMap::new();
    agents.insert(
        "doc-writer".to_string(),
        AgentDefinition {
            description: "Writes comprehensive documentation".to_string(),
            prompt: "You are a technical documentation expert. Write clear, comprehensive \
                 documentation with examples. Focus on clarity and completeness."
                .to_string(),
            tools: vec!["Read".to_string(), "Write".to_string(), "Edit".to_string()],
            model: Some("sonnet".to_string()),
        },
    );

    let options = ClaudeAgentOptions { agents };

    // Create transport (need to specify the actual CLI command)
    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Send query
    let user_message = UserMessage {
        content: "Use the code-reviewer agent to review the code in src/claude_agent_sdk/types.rs"
            .to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // Since the query method is not fully implemented, we'll demonstrate the intended usage
    // let response = client.query(
    //     messages,
    //     None, // tools
    //     None, // system
    //     None, // stream
    //     None, // on_message
    //     None, // tool_permissions
    // ).await?;

    println!("Would send query: \"Use the doc-writer agent to explain what AgentDefinition is used for\"");
    println!();

    Ok(())
}

async fn multiple_agents_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Multiple Agents Example ===");

    // Define agent options
    let mut agents = HashMap::new();
    agents.insert(
        "analyzer".to_string(),
        AgentDefinition {
            description: "Analyzes code structure and patterns".to_string(),
            prompt: "You are a code analyzer. Examine code structure, patterns, and architecture."
                .to_string(),
            tools: vec!["Read".to_string(), "Grep".to_string(), "Glob".to_string()],
            model: None,
        },
    );
    agents.insert(
        "tester".to_string(),
        AgentDefinition {
            description: "Creates and runs tests".to_string(),
            prompt: "You are a testing expert. Write comprehensive tests and ensure code quality."
                .to_string(),
            tools: vec!["Read".to_string(), "Write".to_string(), "Bash".to_string()],
            model: Some("sonnet".to_string()),
        },
    );

    let options = ClaudeAgentOptions { agents };

    // Create transport (need to specify the actual CLI command)
    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Send query
    let user_message = UserMessage {
        content: "Use the analyzer agent to find all Rust files in the examples/ directory"
            .to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // Since the query method is not fully implemented, we'll demonstrate the intended usage
    // let response = client.query(
    //     messages,
    //     None, // tools
    //     None, // system
    //     None, // stream
    //     None, // on_message
    //     None, // tool_permissions
    // ).await?;

    println!("Would send query: \"Use the analyzer agent to find all Rust files in the examples/ directory\"");
    println!();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    code_reviewer_example().await?;
    documentation_writer_example().await?;
    multiple_agents_example().await?;
    Ok(())
}
