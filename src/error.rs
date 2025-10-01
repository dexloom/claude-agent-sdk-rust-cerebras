use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Transport error: {0}")]
    TransportError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Process error: {0}")]
    ProcessError(String),

    #[error("Message parsing error: {0}")]
    MessageParsingError(String),

    #[error("Invalid message type: {0}")]
    InvalidMessageType(String),

    #[error("Tool execution error: {0}")]
    ToolExecutionError(String),

    #[error("Custom error: {0}")]
    CustomError(String),
}

impl AgentError {
    pub fn custom(message: &str) -> Self {
        AgentError::CustomError(message.to_string())
    }
}
