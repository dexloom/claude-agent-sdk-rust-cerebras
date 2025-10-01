// Example demonstrating tool permission callbacks to control tool usage

use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

use claude_agent_sdk::error::AgentError;
use claude_agent_sdk::internal::query::{
    PermissionResult, PermissionResultAllow, PermissionResultDeny, Query, ToolPermissionContext,
};
use claude_agent_sdk::transport::Transport;
use claude_agent_sdk::types::{
    ContentBlock, Message, TextBlock, ToolResultBlock, ToolUseBlock, UserMessage,
};
use claude_agent_sdk::{Client, SubprocessCLITransport};

async fn allow_all_tools(
    _tool_name: String,
    input_data: HashMap<String, Value>,
    _context: ToolPermissionContext,
) -> Result<PermissionResult, AgentError> {
    println!("\n🔧 Tool Permission Request: {}", _tool_name);
    println!("   Input: {:?}", input_data);

    // Always allow all tools in this example
    println!("   ✅ Automatically allowing {}", _tool_name);
    Ok(PermissionResult::Allow(PermissionResultAllow {
        behavior: "allow".to_string(),
        updated_input: None,
        updated_permissions: None,
    }))
}

async fn deny_dangerous_tools(
    tool_name: String,
    input_data: HashMap<String, Value>,
    _context: ToolPermissionContext,
) -> Result<PermissionResult, AgentError> {
    println!("\n🔧 Tool Permission Request: {}", tool_name);
    println!("   Input: {:?}", input_data);

    // Deny write operations to system directories
    if ["Write", "Edit", "MultiEdit"].contains(&tool_name.as_str()) {
        if let Some(file_path) = input_data.get("file_path").and_then(|v| v.as_str()) {
            if file_path.starts_with("/etc/") || file_path.starts_with("/usr/") {
                println!("   ❌ Denying write to system directory: {}", file_path);
                return Ok(PermissionResult::Deny(PermissionResultDeny {
                    behavior: "deny".to_string(),
                    message: format!("Cannot write to system directory: {}", file_path),
                    interrupt: false,
                }));
            }

            // Redirect writes to a safe directory
            if !file_path.starts_with("/tmp/") && !file_path.starts_with("./") {
                let safe_path = format!(
                    "./safe_output/{}",
                    file_path.split("/").last().unwrap_or("file")
                );
                println!(
                    "   ⚠️  Redirecting write from {} to {}",
                    file_path, safe_path
                );
                let mut modified_input = input_data.clone();
                modified_input.insert("file_path".to_string(), Value::String(safe_path));
                return Ok(PermissionResult::Allow(PermissionResultAllow {
                    behavior: "allow".to_string(),
                    updated_input: Some(modified_input),
                    updated_permissions: None,
                }));
            }
        }
    }

    // Check dangerous bash commands
    if tool_name == "Bash" {
        if let Some(command) = input_data.get("command").and_then(|v| v.as_str()) {
            let dangerous_commands = ["rm -rf", "sudo", "chmod 777", "dd if=", "mkfs"];

            for dangerous in &dangerous_commands {
                if command.contains(dangerous) {
                    println!("   ❌ Denying dangerous command: {}", command);
                    return Ok(PermissionResult::Deny(PermissionResultDeny {
                        behavior: "deny".to_string(),
                        message: format!("Dangerous command pattern detected: {}", dangerous),
                        interrupt: false,
                    }));
                }
            }

            // Allow but log the command
            println!("   ✅ Allowing bash command: {}", command);
            return Ok(PermissionResult::Allow(PermissionResultAllow {
                behavior: "allow".to_string(),
                updated_input: None,
                updated_permissions: None,
            }));
        }
    }

    // For all other tools, allow by default
    println!("   ✅ Allowing tool: {}", tool_name);
    Ok(PermissionResult::Allow(PermissionResultAllow {
        behavior: "allow".to_string(),
        updated_input: None,
        updated_permissions: None,
    }))
}

async fn query_with_tool_permissions() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Tool Permission Callback Examples ===");

    let transport = SubprocessCLITransport::new("claude", &["--mode", "streaming"])?;

    // Create a query with tool permission callback
    // Note: In a full implementation, we would properly integrate the tool permission callback
    // For now, we're just demonstrating how it would be structured
    let user_message = UserMessage {
        content: "Please do the following:\n1. List the files in the current directory\n2. Create a simple Python hello world script at hello.py\n3. Run the script to test it".to_string(),
        parent_tool_use_id: None,
    };
    let messages = vec![Message::User(user_message)];

    println!("Would send query to Claude with tool permission callbacks and process responses");
    println!();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    query_with_tool_permissions().await?;
    Ok(())
}
