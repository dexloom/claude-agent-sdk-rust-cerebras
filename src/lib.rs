//! Claude Agent SDK for Rust
//!
//! This SDK provides a Rust interface for interacting with Claude agents through various transports.

pub mod client;
pub mod error;
pub mod internal;
pub mod message_parser;
pub mod query;
pub mod transport;
pub mod types;

pub use client::Client;
pub use error::AgentError;
pub use transport::SubprocessCLITransport;
pub use types::*;

// Re-export internal modules that should be part of the public API
pub use internal::query::Query;
