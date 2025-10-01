use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

use claude_agent_sdk::transport::Transport;
use claude_agent_sdk::types::{
    AssistantMessage, ContentBlock, Message, ResultMessage, SystemMessage, TextBlock,
    ToolResultBlock, ToolUseBlock, UserMessage,
};
use claude_agent_sdk::{Client, SubprocessCLITransport};

// Define ClaudeAgentOptions struct
#[derive(Debug, Clone)]
struct ClaudeAgentOptions {
    allowed_tools: Option<Vec<String>>,
    hooks: Option<HashMap<String, Vec<HookMatcher>>>,
    setting_sources: Option<Vec<String>>,
}

// Define HookMatcher struct
#[derive(Debug, Clone)]
struct HookMatcher {
    matcher: Option<String>,
    hooks: Vec<String>, // In a real implementation, this would be actual hook functions
}

// Define HookContext struct
#[derive(Debug, Clone)]
struct HookContext {
    signal: Option<String>,
}

// Define HookJSONOutput type
type HookJSONOutput = HashMap<String, Value>;

async fn check_bash_command(
    input_data: HashMap<String, Value>,
    tool_use_id: Option<String>,
    context: HookContext,
) -> Result<HookJSONOutput, Box<dyn std::error::Error + Send + Sync>> {
    let tool_name = input_data
        .get("tool_name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let tool_input = match input_data.get("tool_input") {
        Some(Value::Object(obj)) => obj,
        _ => &serde_json::Map::new(),
    };

    if tool_name != "Bash" {
        return Ok(HashMap::new());
    }

    let command = tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let block_patterns = vec!["foo.sh"];

    for pattern in block_patterns {
        if command.contains(pattern) {
            println!("Blocked command: {}", command);
            let mut result = HashMap::new();
            let mut hook_specific_output = serde_json::Map::new();
            hook_specific_output.insert(
                "hookEventName".to_string(),
                Value::String("PreToolUse".to_string()),
            );
            hook_specific_output.insert(
                "permissionDecision".to_string(),
                Value::String("deny".to_string()),
            );
            hook_specific_output.insert(
                "permissionDecisionReason".to_string(),
                Value::String(format!("Command contains invalid pattern: {}", pattern)),
            );
            result.insert(
                "hookSpecificOutput".to_string(),
                Value::Object(hook_specific_output),
            );
            return Ok(result);
        }
    }

    Ok(HashMap::new())
}

async fn add_custom_instructions(
    input_data: HashMap<String, Value>,
    tool_use_id: Option<String>,
    context: HookContext,
) -> Result<HookJSONOutput, Box<dyn std::error::Error + Send + Sync>> {
    let mut result = HashMap::new();
    let mut hook_specific_output = serde_json::Map::new();
    hook_specific_output.insert(
        "hookEventName".to_string(),
        Value::String("SessionStart".to_string()),
    );
    hook_specific_output.insert(
        "additionalContext".to_string(),
        Value::String("My favorite color is hot pink".to_string()),
    );
    result.insert(
        "hookSpecificOutput".to_string(),
        Value::Object(hook_specific_output),
    );
    Ok(result)
}

async fn example_pretooluse() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== PreToolUse Example ===");
    println!(
        "This example demonstrates how PreToolUse can block some bash commands but not others.\n"
    );

    // Configure hooks using ClaudeAgentOptions
    let mut hooks = HashMap::new();
    let mut pretooluse_hooks = Vec::new();

    let mut matcher_hooks = Vec::new();
    matcher_hooks.push("check_bash_command".to_string());

    pretooluse_hooks.push(HookMatcher {
        matcher: Some("Bash".to_string()),
        hooks: matcher_hooks,
    });

    hooks.insert("PreToolUse".to_string(), pretooluse_hooks);

    let options = ClaudeAgentOptions {
        allowed_tools: Some(vec!["Bash".to_string()]),
        hooks: Some(hooks),
        setting_sources: None,
    };

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Test 1: Command with forbidden pattern (will be blocked)
    println!("Test 1: Trying a command that our PreToolUse hook should block...");
    println!("User: Run the bash command: ./foo.sh --help");

    let user_message = UserMessage {
        content: "Run the bash command: ./foo.sh --help".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and receive responses
    // client.query("Run the bash command: ./foo.sh --help").await?;
    println!("Would send query with client\n");

    println!("==================================================\n");

    // Test 2: Safe command that should work
    println!("Test 2: Trying a command that our PreToolUse hook should allow...");
    println!("User: Run the bash command: echo 'Hello from hooks example!'");

    let user_message = UserMessage {
        content: "Run the bash command: echo 'Hello from hooks example!'\n".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and receive responses
    // client.query("Run the bash command: echo 'Hello from hooks example!'").await?;
    // while let Ok(msg) = client.get_next_message().await {
    //     display_message(msg);
    // }
    println!("Would send query with client\n");

    println!("\n");
    Ok(())
}

async fn example_userpromptsubmit() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== UserPromptSubmit Example ===");
    println!("This example shows how a UserPromptSubmit hook can add context.\n");

    let mut hooks = HashMap::new();
    let mut userpromptsubmit_hooks = Vec::new();

    let mut matcher_hooks = Vec::new();
    matcher_hooks.push("add_custom_instructions".to_string());

    userpromptsubmit_hooks.push(HookMatcher {
        matcher: None,
        hooks: matcher_hooks,
    });

    hooks.insert("UserPromptSubmit".to_string(), userpromptsubmit_hooks);

    let options = ClaudeAgentOptions {
        allowed_tools: None,
        hooks: Some(hooks),
        setting_sources: None,
    };

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    println!("User: What's my favorite color?");
    let user_message = UserMessage {
        content: "What's my favorite color?".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    // In a real implementation, we would send the query and receive responses
    // client.query("What's my favorite color?").await?;
    // while let Ok(msg) = client.get_next_message().await {
    //     display_message(msg);
    // }
    println!("Would send query with client\n");

    println!("\n");
    Ok(())
}

fn display_message(msg: Message) {
    match msg {
        Message::User(user_msg) => {
            // In a real implementation, we would handle different block types
            println!("User: {}", user_msg.content);
        }
        Message::Assistant(assistant_msg) => {
            // Handle different content block types
            for block in assistant_msg.content {
                match block {
                    ContentBlock::Text(text_block) => {
                        println!("Claude: {}", text_block.text);
                    }
                    ContentBlock::ToolUse(tool_use_block) => {
                        println!("Using tool: {}", tool_use_block.name);
                    }
                    _ => {}
                }
            }
        }
        Message::Result(result_msg) => {
            println!("Result ended");
        }
        Message::System(system_msg) => {
            // Ignore system messages
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting Claude SDK Hooks Examples...");
    println!("==================================================\n");

    example_pretooluse().await?;
    println!("--------------------------------------------------\n");

    example_userpromptsubmit().await?;
    println!("--------------------------------------------------\n");

    Ok(())
}
