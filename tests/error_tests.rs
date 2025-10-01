use claude_agent_sdk::error::AgentError;
use serde_json::{Error, Value};
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_error() {
        let error: Box<dyn std::error::Error + Send + Sync> = "test error".into();
        let agent_error = AgentError::TransportError(error);
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Transport error"));
    }

    #[test]
    fn test_serialization_error() {
        let json_error = serde_json::from_str::<Value>("invalid json").unwrap_err();
        let agent_error = AgentError::SerializationError(json_error);
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Serialization error"));
    }

    #[test]
    fn test_io_error() {
        let io_error = io::Error::new(io::ErrorKind::Other, "test error");
        let agent_error = AgentError::IOError(io_error);
        let msg = format!("{}", agent_error);
        assert!(msg.contains("IO error"));
    }

    #[test]
    fn test_process_error() {
        let agent_error = AgentError::ProcessError("test error".to_string());
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Process error"));
        assert!(msg.contains("test error"));
    }

    #[test]
    fn test_message_parsing_error() {
        let agent_error = AgentError::MessageParsingError("test error".to_string());
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Message parsing error"));
        assert!(msg.contains("test error"));
    }

    #[test]
    fn test_invalid_message_type_error() {
        let agent_error = AgentError::InvalidMessageType("test error".to_string());
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Invalid message type"));
        assert!(msg.contains("test error"));
    }

    #[test]
    fn test_tool_execution_error() {
        let agent_error = AgentError::ToolExecutionError("test error".to_string());
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Tool execution error"));
        assert!(msg.contains("test error"));
    }

    #[test]
    fn test_custom_error() {
        let agent_error = AgentError::CustomError("test error".to_string());
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Custom error"));
        assert!(msg.contains("test error"));
    }

    #[test]
    fn test_custom_error_method() {
        let agent_error = AgentError::custom("test error");
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Custom error"));
        assert!(msg.contains("test error"));
    }

    #[test]
    fn test_error_from_transport() {
        let error: Box<dyn std::error::Error + Send + Sync> = "test transport error".into();
        let agent_error = AgentError::from(error);
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Transport error"));
    }

    #[test]
    fn test_error_from_serialization() {
        let json_error = serde_json::from_str::<Value>("invalid json").unwrap_err();
        let agent_error = AgentError::from(json_error);
        let msg = format!("{}", agent_error);
        assert!(msg.contains("Serialization error"));
    }

    #[test]
    fn test_error_from_io() {
        let io_error = io::Error::new(io::ErrorKind::Other, "test io error");
        let agent_error = AgentError::from(io_error);
        let msg = format!("{}", agent_error);
        assert!(msg.contains("IO error"));
    }
}
