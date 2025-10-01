use async_trait::async_trait;
use serde_json::Value;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::process::Command;
use tokio::sync::Mutex;

#[async_trait]
pub trait Transport {
    async fn send(&self, payload: Value)
        -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
    async fn receive(&self) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct SubprocessCLITransport {
    child: Arc<Mutex<tokio::process::Child>>,
    stdout_lines: Arc<Mutex<Lines<BufReader<tokio::process::ChildStdout>>>>,
    stdin: Arc<Mutex<tokio::process::ChildStdin>>,
}

impl SubprocessCLITransport {
    pub fn new(
        command: &str,
        args: &[&str],
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let stdout_lines = BufReader::new(stdout).lines();

        let stdin = child.stdin.take().ok_or("Failed to capture stdin")?;

        Ok(SubprocessCLITransport {
            child: Arc::new(Mutex::new(child)),
            stdout_lines: Arc::new(Mutex::new(stdout_lines)),
            stdin: Arc::new(Mutex::new(stdin)),
        })
    }

    pub async fn is_alive(&self) -> bool {
        let mut child = self.child.lock().await;
        match child.try_wait() {
            Ok(Some(_)) => false, // Process has exited
            Ok(None) => true,     // Process is still running
            Err(_) => false,      // Error checking process
        }
    }
}

#[async_trait]
impl Transport for SubprocessCLITransport {
    async fn send(
        &self,
        payload: Value,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let json_string = serde_json::to_string(&payload)?;
        let mut stdin = self.stdin.lock().await;
        stdin.write_all(json_string.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;
        Ok(Value::Null)
    }

    async fn receive(&self) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let mut stdout_lines = self.stdout_lines.lock().await;
        if let Some(line) = stdout_lines.next_line().await? {
            if let Ok(value) = serde_json::from_str::<Value>(&line) {
                return Ok(value);
            }
        }
        Err("Process ended before message was received".into())
    }
}
