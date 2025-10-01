use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use claude_agent_sdk::types::{ContentBlock, Message, UserMessage};
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
                    ContentBlock::ToolUse(tool_use_block) => {
                        println!("Using tool: {}", tool_use_block.name);
                        if !tool_use_block.input.is_empty() {
                            println!("  Input: {:?}", tool_use_block.input);
                        }
                    }
                    _ => {}
                }
            }
        }
        Message::Result(result_msg) => {
            println!("Result ended");
            if let Some(cost) = result_msg.total_cost_usd {
                println!("Cost: ${:.6}", cost);
            }
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Example prompts to demonstrate calculator usage
    let prompts = vec![
        "Calculate 15 + 27",
        "What is 100 divided by 7?",
        "Calculate the square root of 144",
        "What is 2 raised to the power of 8?",
        "Calculate (12 + 8) * 3 - 10", // Complex calculation
    ];

    let transport = SubprocessCLITransport::new("claude", &[])?;
    let client = Client::new(Arc::new(Mutex::new(transport)));

    // Create tool definitions for the client
    let tools = Some(vec![
        json!({
            "name": "add",
            "description": "Add two numbers",
            "input_schema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number"},
                    "b": {"type": "number"}
                },
                "required": ["a", "b"]
            }
        }),
        json!({
            "name": "subtract",
            "description": "Subtract one number from another",
            "input_schema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number"},
                    "b": {"type": "number"}
                },
                "required": ["a", "b"]
            }
        }),
        json!({
            "name": "multiply",
            "description": "Multiply two numbers",
            "input_schema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number"},
                    "b": {"type": "number"}
                },
                "required": ["a", "b"]
            }
        }),
        json!({
            "name": "divide",
            "description": "Divide one number by another",
            "input_schema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number"},
                    "b": {"type": "number"}
                },
                "required": ["a", "b"]
            }
        }),
        json!({
            "name": "sqrt",
            "description": "Calculate square root",
            "input_schema": {
                "type": "object",
                "properties": {
                    "n": {"type": "number"}
                },
                "required": ["n"]
            }
        }),
        json!({
            "name": "power",
            "description": "Raise a number to a power",
            "input_schema": {
                "type": "object",
                "properties": {
                    "base": {"type": "number"},
                    "exponent": {"type": "number"}
                },
                "required": ["base", "exponent"]
            }
        }),
    ]);

    for prompt in prompts {
        println!("\n==================================================");
        println!("Prompt: {}", prompt);
        println!("==================================================");

        let user_message = UserMessage {
            content: prompt.to_string(),
            parent_tool_use_id: None,
        };
        let messages = vec![Message::User(user_message)];

        // Send query and process responses
        let _response = client
            .query(messages, tools.clone(), None, Some(true), None, None)
            .await?;

        // Process streaming messages
        loop {
            let message = client.get_next_message().await?;
            display_message(message.clone());

            // Check if this is a result message which indicates the end of the stream
            if let Message::Result(_) = message {
                break;
            }
        }
    }

    Ok(())
}
