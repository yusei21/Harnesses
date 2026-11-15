//! Explicit Python model adapter: fixed `python -m harness.bridge`, JSON over stdin/stdout.
//! This launches trusted repository Python code, not an OS sandbox. No arbitrary commands.
use crate::core::{Gateway, GatewayFuture, ModelRequest, ModelResponse, RunError};
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub struct PythonGateway {
    pub python_executable: String,
    pub repository_dir: PathBuf,
    pub timeout: Duration,
}
#[derive(Deserialize)]
struct BridgeReply {
    text: Option<String>,
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    #[allow(dead_code)]
    error: Option<String>,
}
impl PythonGateway {
    pub fn repository_default() -> Self {
        Self {
            python_executable: "python3".into(),
            repository_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".."),
            timeout: Duration::from_secs(90),
        }
    }
}
impl Gateway for PythonGateway {
    fn complete(&self, request: ModelRequest) -> GatewayFuture<'_> {
        Box::pin(async move {
            let bytes = serde_json::to_vec(&request).map_err(|_| RunError::WorkerFailure)?;
            if bytes.len() > 128_000 {
                return Err(RunError::ContextTooLarge);
            }
            let mut child = Command::new(&self.python_executable)
                .arg("-m").arg("harness.bridge")
                .current_dir(&self.repository_dir)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .kill_on_drop(true)
                .spawn().map_err(|_| RunError::WorkerFailure)?;
            let mut stdin = child.stdin.take().ok_or(RunError::WorkerFailure)?;
            stdin.write_all(&bytes).await.map_err(|_| RunError::WorkerFailure)?;
            drop(stdin);
            let output = tokio::time::timeout(self.timeout, child.wait_with_output())
                .await.map_err(|_| RunError::WorkerFailure)?
                .map_err(|_| RunError::WorkerFailure)?;
            if !output.status.success() || output.stdout.len() > 64_000 {
                return Err(RunError::WorkerFailure);
            }
            let parsed: BridgeReply = serde_json::from_slice(&output.stdout)
                .map_err(|_| RunError::WorkerFailure)?;
            Ok(ModelResponse {
                text: parsed.text.ok_or(RunError::WorkerFailure)?,
                input_tokens: parsed.input_tokens,
                output_tokens: parsed.output_tokens,
            })
        })
    }
}
