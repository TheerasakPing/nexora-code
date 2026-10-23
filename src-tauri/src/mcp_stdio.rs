//! Fail-closed, read-only MCP stdio transport for reviewed, absolute native
//! binaries. No UI entrypoint: live execution is blocked until the native
//! operator consent path and executable ownership audit have been accepted.
#![allow(dead_code)]

use crate::{
    mcp_protocol::MAX_RPC_FRAME,
    mcp_registry::{self, McpServer, Transport},
    mcp_security::VerifiedOperatorConsent,
    providers::ProcessGroup,
};
use serde_json::Value;
use std::{path::Path, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
    time,
};

pub struct ReadOnlyStdio {
    child: Child,
    _group: ProcessGroup,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl ReadOnlyStdio {
    pub async fn spawn(server: &McpServer, _consent: VerifiedOperatorConsent) -> Result<Self, &'static str> {
        mcp_registry::validate(server).map_err(|_| "Invalid MCP stdio configuration")?;
        if !server.enabled {
            return Err("MCP server is disabled");
        }
        let (path, args) = match &server.transport {
            Transport::Stdio { command, args } => (Path::new(command), args),
            _ => return Err("MCP server is not a stdio transport"),
        };
        // A relative tool name may run arbitrary binaries through PATH or
        // installer shims. Only use a specific absolute executable here.
        if !path.is_absolute() ||
            !path.symlink_metadata().map_err(|_| "MCP executable is unavailable")?
                .file_type().is_file()
        {
            return Err("MCP stdio requires an approved absolute regular executable");
        }
        let mut command = Command::new(path);
        command.args(args)
            .env_clear()
            // None of the caller's tokens, SSH_AUTH_SOCK, session credentials,
            // HOME, cloud vars or arbitrary user-defined environment survive.
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        #[cfg(windows)]
        command.creation_flags(0x08000000);
        let mut child = command.spawn().map_err(|_| "Could not start reviewed MCP executable")?;
        let group = ProcessGroup::register(child.id());
        let stdin = child.stdin.take().ok_or("MCP stdin missing")?;
        let stdout = child.stdout.take().ok_or("MCP stdout missing")?;
        Ok(Self { child, _group: group, stdin, stdout: BufReader::new(stdout) })
    }

    /// Only catalog/discovery calls are accepted. Tool invocation and resource
    /// reads require a separate, per-action trust path not implemented here.
    pub async fn request_catalog(&mut self, request: Value) -> Result<Vec<u8>, &'static str> {
        if request["jsonrpc"] != "2.0" || request["id"].as_u64().is_none() ||
           !request["method"].as_str().is_some_and(|m| matches!(
                m, "server/discover" | "initialize" | "tools/list" | "resources/list" | "prompts/list"))
        {
            return Err("MCP stdio request is not an approved read-only catalog operation");
        }
        let mut bytes = serde_json::to_vec(&request).map_err(|_| "Invalid MCP request")?;
        if bytes.len() + 1 > MAX_RPC_FRAME {
            return Err("MCP request exceeds maximum frame length");
        }
        bytes.push(b'\n');
        time::timeout(Duration::from_secs(5), self.stdin.write_all(&bytes))
            .await.map_err(|_| "MCP stdio write deadline expired")?
            .map_err(|_| "MCP stdio write failed")?;
        self.read_bounded_frame().await
    }

    async fn read_bounded_frame(&mut self) -> Result<Vec<u8>, &'static str> {
        time::timeout(Duration::from_secs(5), async {
            let mut result = Vec::new();
            loop {
                let chunk = self.stdout.fill_buf().await
                    .map_err(|_| "MCP stdio read failed")?;
                if chunk.is_empty() {
                    return Err("MCP stdio server exited without a response");
                }
                let stop = chunk.iter().position(|byte| *byte == b'\n')
                    .map(|pos| pos + 1).unwrap_or(chunk.len());
                if result.len().saturating_add(stop) > MAX_RPC_FRAME {
                    return Err("MCP stdio response exceeded frame limit");
                }
                result.extend_from_slice(&chunk[..stop]);
                let complete = result.ends_with(b"\n");
                self.stdout.consume(stop);
                if complete {
                    return Ok(result);
                }
            }
        }).await.map_err(|_| "MCP stdio read deadline expired")?
    }

    pub async fn terminate(mut self) {
        // Dropping the group guard also stops descendant workers even if their
        // parent exited; never just kill the original PID.
        let _ = self.child.start_kill();
        let _ = time::timeout(Duration::from_secs(1), self.child.wait()).await;
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::mcp_security::fixture_operator_consent;
    use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};

    fn temp_executable(source: &str) -> (PathBuf, PathBuf) {
        let folder = std::env::temp_dir().join(format!("nexora-r2-mcp-stdio-{}",uuid::Uuid::new_v4()));
        fs::create_dir(&folder).unwrap();
        let executable = folder.join("fixture");
        fs::write(&executable, format!("#!/bin/sh\n{source}\n")).unwrap();
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
        (folder, executable)
    }
    fn server(path: &Path) -> McpServer {
        McpServer{
            name:"fixture".into(),description:String::new(),enabled:true,credential_ref:None,
            transport:Transport::Stdio{command:path.to_string_lossy().to_string(),args:vec![]},
        }
    }
    #[tokio::test]
    async fn trusted_offline_stdio_catalog_roundtrip_and_denied_tool_call() {
        let (folder,exe)=temp_executable("read line; printf '%s\\n' '{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"capabilities\":{\"tools\":{}}}}'");
        let mut transport=ReadOnlyStdio::spawn(&server(&exe),fixture_operator_consent()).await.unwrap();
        let request=serde_json::json!({"jsonrpc":"2.0","id":1,"method":"server/discover","params":{}});
        let response=transport.request_catalog(request).await.unwrap();
        assert_eq!(serde_json::from_slice::<Value>(&response).unwrap()["result"]["capabilities"]["tools"],serde_json::json!({}));
        assert!(transport.request_catalog(serde_json::json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{}})).await.is_err());
        transport.terminate().await;
        fs::remove_dir_all(folder).unwrap();
    }
    #[tokio::test]
    async fn malformed_or_oversized_stdio_output_is_bounded() {
        let (folder,exe)=temp_executable("read line; head -c 150000 /dev/zero | tr '\\000' 'x'; echo");
        let mut transport=ReadOnlyStdio::spawn(&server(&exe),fixture_operator_consent()).await.unwrap();
        let request=serde_json::json!({"jsonrpc":"2.0","id":1,"method":"server/discover","params":{}});
        assert!(transport.request_catalog(request).await.unwrap_err().contains("frame limit"));
        transport.terminate().await;
        fs::remove_dir_all(folder).unwrap();
    }
    #[tokio::test]
    async fn no_shell_or_relative_binary_can_be_started_without_consent() {
        let item=server(Path::new("npx"));
        assert!(ReadOnlyStdio::spawn(&item,fixture_operator_consent()).await.is_err());
        let item=server(Path::new("/bin/sh"));
        assert!(ReadOnlyStdio::spawn(&item,fixture_operator_consent()).await.is_err());
    }
}
