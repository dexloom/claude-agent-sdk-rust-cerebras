# Claude Agent SDK for Rust

The Claude Agent SDK for Rust provides a powerful interface for interacting with Claude agents through various transports, enabling rich, interactive conversations with Claude Code.

## Features

The Rust SDK offers functionality equivalent to the Python SDK:

- **Streaming Support**: Full streaming capabilities with real-time message processing
- **Client Connection Management**: Robust connection handling with Claude Code CLI
- **Bidirectional Communication**: Send and receive messages at any time during a conversation
- **Stateful Conversations**: Maintains conversation context across multiple turns
- **Error Handling**: Comprehensive error types matching the Python SDK
- **Tool Permission Control**: Dynamically manage tool permissions during conversations
- **Model Selection**: Switch between different Claude models during a session
- **Hook System**: Register callbacks for various events in the conversation lifecycle
- **MCP Server Integration**: Connect to Model Context Protocol servers (stdio, SSE, HTTP, SDK)
- **Message Parsing**: Parse and handle different message types from Claude (user, assistant, system, result)

## Prerequisites

You must have Claude Code CLI installed on your system. If you haven't installed it yet:

1. Download Claude Code from [https://claude.ai/download](https://claude.ai/download)
2. Ensure the CLI is in your PATH

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
claude-agent-sdk = "0.1.0"
```

Or run:

```bash
cargo add claude-agent-sdk
```

## Usage Examples

### Basic Query Functionality

```rust
use claude_agent_sdk::{query, Message, TextBlock};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut messages = Vec::new();
    
    // Simple one-off question
    let mut stream = query("What is the capital of France?").await?;
    
    while let Some(message) = stream.next().await {
        match message {
            Message::Assistant(assistant_msg) => {
                for block in assistant_msg.content {
                    if let ContentBlock::Text(TextBlock { text }) = block {
                        println!("Claude: {}", text);
                    }
                }
            }
            Message::Result(result_msg) => {
                if let Some(cost) = result_msg.total_cost_usd {
                    println!("Cost: ${:.4}", cost);
                }
                break; // End of response
            }
            _ => {}
        }
        messages.push(message);
    }
    
    Ok(())
}
```

### Streaming Mode Usage

```rust
use claude_agent_sdk::{ClaudeSDKClient, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ClaudeSDKClient::new(); // Using default SubprocessCLITransport
    
    // Connect to Claude with initial prompt
    client.connect(Some("Help me analyze this codebase")).await?;
    
    // Send a follow-up message
    client.send_user_message("Can you explain the architecture?").await?;
    
    // Receive messages
    while let Some(message) = client.receive_messages().await? {
        match message {
            Message::Assistant(assistant_msg) => {
                // Process assistant message content
                for block in assistant_msg.content {
                    match block {
                        ContentBlock::Text(text_block) => {
                            println!("Claude: {}", text_block.text);
                        }
                        ContentBlock::Thinking(thinking_block) => {
                            println!("Claude is thinking: {}", thinking_block.thinking);
                        }
                        ContentBlock::ToolUse(tool_use_block) => {
                            println!("Claude wants to use tool: {}", tool_use_block.name);
                        }
                        ContentBlock::ToolResult(tool_result_block) => {
                            println!("Tool result received");
                        }
                    }
                }
            }
            Message::Result(result_msg) => {
                if let Some(cost) = result_msg.total_cost_usd {
                    println!("Total cost: ${:.4}", cost);
                }
                break; // End of response
            }
            _ => {}
        }
    }
    
    client.disconnect().await?;
    Ok(())
}
```

### Client Connection and Management

```rust
use claude_agent_sdk::{ClaudeSDKClient, SubprocessCLITransport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a transport
    let transport = SubprocessCLITransport::new("claude", &["--streaming"])?;
    
    // Create client with custom transport
    let mut client = ClaudeSDKClient::new_with_transport(transport);
    
    // Connect to Claude
    client.connect(None).await?;
    
    // Query Claude
    client.query("What are design patterns in Rust?").await?;
    
    // Change permission mode
    client.set_permission_mode("acceptEdits").await?;
    
    // Switch model
    client.set_model(Some("claude-sonnet-4-5")).await?;
    
    // Disconnect
    client.disconnect().await?;
    
    Ok(())
}
```

### Error Handling

```rust
use claude_agent_sdk::{ClaudeSDKClient, AgentError};

#[tokio::main]
async fn main() {
    let mut client = ClaudeSDKClient::new();
    
    match client.connect(None).await {
        Ok(()) => println!("Connected successfully"),
        Err(AgentError::TransportError(e)) => {
            eprintln!("Transport error: {}", e);
        }
        Err(AgentError::ProcessError(e)) => {
            eprintln!("Process error: {}", e);
        }
        Err(AgentError::MessageParsingError(e)) => {
            eprintln!("Message parsing error: {}", e);
        }
        Err(e) => {
            eprintln!("Other error: {}", e);
        }
    }
}
```

### Integration with Claude Code CLI

The SDK integrates seamlessly with Claude Code CLI, automatically managing the subprocess lifecycle:

```rust
use claude_agent_sdk::{SubprocessCLITransport, ClaudeSDKClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The SDK will automatically create a SubprocessCLITransport
    let mut client = ClaudeSDKClient::new();
    
    // Connect with a prompt
    client.connect(Some("Generate a Rust web server using Axum")).await?;
    
    // Send additional messages as needed
    client.send_user_message("Add authentication middleware").await?;
    
    // Process responses
    while let Some(message) = client.receive_messages().await? {
        // Handle messages...
    }
    
    client.disconnect().await?;
    Ok(())
}
```
## Examples

The examples directory contains various Rust implementations demonstrating different features and use cases of the Claude Agent SDK. Each example shows how to use specific functionalities of the SDK in a practical context.

Below is documentation for each of the examples provided:

## API Reference

### Examples Directory Overview

The examples directory contains Rust implementations that demonstrate various features and capabilities of the Claude Agent SDK:

- **agents.rs** - Shows how to define and use multiple specialized agents with different prompts and tool configurations
- **hooks.rs** - Demonstrates how to use hooks to customize behavior at different points in the conversation lifecycle
- **include_partial_messages.rs** - Illustrates how to enable and handle partial message streaming
- **mcp_calculator.rs** - Example of implementing an MCP (Model Context Protocol) server with calculator tools
- **quick_start.rs** - Simple examples showing basic SDK usage patterns
- **setting_sources.rs** - Shows how to configure different setting sources for project-specific slash commands
- **stderr_callback_example.rs** - Demonstrates how to capture and handle stderr output from Claude Code CLI
- **streaming_mode.rs** - Examples of streaming mode functionality with the Claude Agent SDK
- **streaming_mode_ipython.rs** - IPython friendly examples of Claude Agent SDK streaming mode
- **streaming_mode_trio.rs** - Example of multi-turn conversation using async patterns
- **system_prompt.rs** - Shows different system prompt configurations and their effects
- **tool_permission_callback.rs** - Demonstrates how to control tool usage through permission callbacks

### ClaudeSDKClient Methods

- `new()` - Create a new client with default transport
- `new_with_transport(transport)` - Create a client with a custom transport
- `connect(prompt: Option<&str>)` - Connect to Claude with an optional initial prompt
- `disconnect()` - Disconnect from Claude
- `query(prompt: &str)` - Send a new request

### quick_start.rs

This example demonstrates basic usage patterns of the Claude Agent SDK.

**Key Features:**
- Simple query example asking "What is 2 + 2?"
- Query with options example explaining Python in one sentence
- Query with tools example showing file creation

**How to Run:**
```bash
cargo run --example quick_start
```
- `send_user_message(content: &str)` - Send a user message

### streaming_mode.rs

This example demonstrates the streaming mode functionality of the Claude Agent SDK.

**Key Features:**
- Basic streaming example with a simple math question
- Multi-turn conversation example showing stateful interactions
- Custom options example with allowed tools and system prompt configuration

**How to Run:**
```bash

### hooks.rs

This example demonstrates how to use hooks to customize behavior at different points in the conversation lifecycle.

**Key Features:**
- PreToolUse hook example that blocks certain bash commands while allowing others
- UserPromptSubmit hook example that adds custom context to user prompts

**How to Run:**
```bash
cargo run --example hooks

### tool_permission_callback.rs

This example demonstrates how to control tool usage through permission callbacks.

**Key Features:**
- Tool permission callback that allows all tools automatically
- Tool permission callback that denies dangerous tools and redirects file writes to safe directories
- Examples of handling various tools including Write, Edit, MultiEdit, and Bash tools

**How to Run:**
```bash

### system_prompt.rs

This example demonstrates different system prompt configurations and their effects.

**Key Features:**
- No system prompt example (vanilla Claude behavior)
- String system prompt example
- Preset system prompt example
- Preset system prompt with append example

**How to Run:**

### include_partial_messages.rs

This example illustrates how to enable and handle partial message streaming.

**Key Features:**
- Configuration of ClaudeAgentOptions with include_partial_messages set to true
- Example using a model that supports partial responses
- Demonstration of handling streaming responses with partial message inclusion

**How to Run:**
```bash

### setting_sources.rs

This example shows how to configure different setting sources for project-specific slash commands.

**Key Features:**
- Default behavior example with no custom setting sources
- User settings only example
- Project and user settings example demonstrating how to enable project-specific slash commands

**How to Run:**
```bash

### stderr_callback_example.rs

This example demonstrates how to capture and handle stderr output from Claude Code CLI.

**Key Features:**
- Configuration of stderr callback function
- Example with debug mode enabled
- Error detection in stderr output

**How to Run:**
```bash

### mcp_calculator.rs

This example demonstrates implementing an MCP (Model Context Protocol) server with calculator tools.

**Key Features:**
- Implementation of calculator tools (add, subtract, multiply, divide, sqrt, power)
- Configuration of Claude to use the calculator server with allowed tools
- Pre-approved calculator MCP tools for usage without permission prompts
- Examples of complex calculations using multiple tools

**How to Run:**

### agents.rs

This example demonstrates how to define and use multiple specialized agents with different prompts and tool configurations.

**Key Features:**
- Code reviewer agent example with Read and Grep tools
- Documentation writer agent example with Read, Write, and Edit tools
- Multiple agents example showing how to use several specialized agents in one session
- Agent definitions with custom prompts, tools, and models

**How to Run:**

### streaming_mode_ipython.rs

This example contains IPython-friendly code snippets for ClaudeSDKClient streaming mode.

**Key Features:**
- Basic streaming example with a simple math question
- Streaming with real-time display example
- Persistent client example for multi-turn conversations

**How to Run:**
These examples are designed to be copy-pasted directly into IPython and run independently.

### streaming_mode_trio.rs

This example demonstrates multi-turn conversation using async patterns with the Claude SDK.

**Key Features:**
- Multi-turn conversation example showing stateful interactions
- Sequential calculations where each step builds on the previous one
- Handling of conversation context between multiple queries

**How to Run:**
```bash
cargo run --example streaming_mode_trio
```
```bash
cargo run --example streaming_mode_ipython
```
```bash
cargo run --example agents
```
```bash
cargo run --example mcp_calculator
```
cargo run --example stderr_callback_example
```
cargo run --example setting_sources
```
cargo run --example include_partial_messages
```
```bash
cargo run --example system_prompt
```
cargo run --example tool_permission_callback
```
```
cargo run --example streaming_mode
```
- `send_assistant_message(content: Vec<ContentBlock>, model: &str)` - Send an assistant message
- `send_system_message(subtype: &str, data: HashMap<String, Value>)` - Send a system message
- `send_result_message(result_msg: ResultMessage)` - Send a result message
- `send_stream_event(event: StreamEvent)` - Send a stream event
- `receive_messages()` - Receive messages from Claude
- `get_next_message()` - Get the next parsed message
- `send_message(message: Value)` - Send a raw JSON message
- `receive_message()` - Receive a raw JSON message
- `interrupt()` - Send interrupt signal
- `set_permission_mode(mode: &str)` - Change permission mode
- `set_model(model: Option<&str>)` - Change the AI model

### Query Function

- `query(prompt: &str)` - One-shot query function that returns a stream of messages

### Message Types

- `UserMessage` - Messages from the user
- `AssistantMessage` - Messages from Claude with content blocks
- `SystemMessage` - System-level messages with metadata
- `ResultMessage` - Final result message with cost and usage information
- `StreamEvent` - Stream events for partial message updates
- `TextBlock` - Text content blocks
- `ThinkingBlock` - Thinking content blocks
- `ToolUseBlock` - Tool use content blocks
- `ToolResultBlock` - Tool result content blocks

### Error Types

- `AgentError::TransportError` - Errors related to transport mechanisms
- `AgentError::SerializationError` - JSON serialization/deserialization errors
- `AgentError::IOError` - IO-related errors
- `AgentError::ProcessError` - CLI process errors
- `AgentError::MessageParsingError` - Errors parsing messages from Claude
- `AgentError::InvalidMessageType` - Invalid message type received
- `AgentError::ToolExecutionError` - Errors during tool execution
- `AgentError::CustomError` - Custom errors

## Architecture and Implementation Details

The Rust Claude Agent SDK follows a modular architecture:

1. **Transport Layer** (`transport.rs`) - Handles communication with Claude Code CLI through subprocess stdin/stdout
2. **Client Layer** (`client.rs`) - Provides high-level interface for interactive conversations
3. **Query Layer** (`query.rs`, `internal/query.rs`) - Manages the control protocol for streaming mode features
4. **Message Types** (`types.rs`) - Defines all message types and content blocks used in communication
5. **Error Handling** (`error.rs`) - Comprehensive error types for different failure modes
6. **Message Parsing** (`message_parser.rs`, `internal/message_parser.rs`) - Utilities for parsing JSON messages into Rust types

In streaming mode, the SDK uses a control protocol to manage features like interrupts, permission changes, and hooks. Control requests are sent through the transport with a request ID, and responses are matched accordingly.

## Testing

Run the test suite with:

```bash
cargo test
```

The tests are organized into modules:

- `client_tests.rs` - Client functionality tests
- `error_tests.rs` - Error handling tests
- `integration_tests.rs` - Integration tests with Claude Code CLI
- `internal_query_tests.rs` - Internal query implementation tests
- `message_parser_tests.rs` - Message parsing tests
- `streaming_tests.rs` - Streaming mode tests
- `transport_tests.rs` - Transport layer tests

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Add your changes and tests
4. Ensure all tests pass with `cargo test`
5. Submit a pull request with a clear description of your changes

For major changes, please open an issue first to discuss what you'd like to change.