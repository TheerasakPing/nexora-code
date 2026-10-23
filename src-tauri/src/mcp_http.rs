//! Auth-gated read-only MCP HTTP discovery transport. NOT exposed to Tauri
//! commands until an independently reviewed native operator consent flow is
//! available. No tool invocation, proxy, redirect or credential injection.
#![allow(dead_code)]

use crate::{
    mcp_protocol::MAX_RPC_FRAME,
    mcp_security::{PinnedHttpTarget, VerifiedOperatorConsent},
};
use reqwest::{header, redirect::Policy, Client};
use serde_json::Value;
use std::time::Duration;

pub struct ReadOnlyHttp {
    target: PinnedHttpTarget,
    client: Client,
}

impl ReadOnlyHttp {
    /// Only a trusted native consent can instantiate a transport. A registered
    /// or enabled MCP server is not sufficient. DNS peers are already validated
    /// by the restrictive R2 policy and explicitly pinned into reqwest.
    pub fn new(target: PinnedHttpTarget, _consent: VerifiedOperatorConsent) -> Result<Self, &'static str> {
        if target.peers().is_empty() {
            return Err("No approved MCP peers");
        }
        // This crate intentionally uses reqwest's rustls-no-provider build.
        // Initialize the audited ring provider before building a client.
        if rustls::crypto::CryptoProvider::get_default().is_none() {
            let _ = rustls::crypto::ring::default_provider().install_default();
        }
        let client = Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            .resolve_to_addrs(target.host(), target.peers())
            .connect_timeout(Duration::from_secs(3))
            .timeout(Duration::from_secs(8))
            .pool_max_idle_per_host(0)
            .build()
            .map_err(|_| "Could not initialize secure MCP HTTP client")?;
        Ok(Self { target, client })
    }

    /// No call/execute/read operations until a separate action-scoped approval
    /// is added at this executor boundary. This handler is discovery only.
    pub async fn request_catalog(&self, frame: Value) -> Result<Vec<u8>, &'static str> {
        let method = frame["method"].as_str().ok_or("Invalid MCP request method")?;
        if !matches!(method,
            "server/discover" | "initialize" | "tools/list" |
            "resources/list" | "prompts/list"
        ) || frame["jsonrpc"] != "2.0" || frame["id"].as_u64().is_none() {
            return Err("MCP tool invocation or unsupported method is not authorized");
        }
        let bytes = serde_json::to_vec(&frame).map_err(|_| "Cannot encode MCP request")?;
        if bytes.len() > MAX_RPC_FRAME {
            return Err("MCP request is too large");
        }
        let response = self.client
            .post(self.target.original_url().clone())
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json")
            .header("Mcp-Method", method)
            .header("MCP-Protocol-Version",
                if frame["params"]["_meta"]["io.modelcontextprotocol/protocolVersion"] == "2026-07-28"
                { "2026-07-28" } else { "2025-11-25" })
            .body(bytes)
            .send().await.map_err(|_| "MCP HTTP request failed")?;
        let peer = response.remote_addr().ok_or("MCP HTTP connected peer unavailable")?;
        self.target.verify_connected_peer(peer)
            .map_err(|_| "MCP HTTP peer is outside approved IP set")?;
        if !response.status().is_success() {
            return Err("MCP HTTP request was not successful (no redirects permitted)");
        }
        let content_type = response.headers().get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()).unwrap_or("");
        if !content_type.split(';').next().is_some_and(|value| value.trim().eq_ignore_ascii_case("application/json")) {
            return Err("MCP HTTP requires application/json; SSE requires separate reviewed transport");
        }
        if response.content_length().is_some_and(|value| value > MAX_RPC_FRAME as u64) {
            return Err("MCP HTTP response exceeds its size limit");
        }
        let mut response = response;
        let mut result = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| "MCP HTTP stream failed")? {
            if result.len().saturating_add(chunk.len()) > MAX_RPC_FRAME {
                return Err("MCP HTTP response exceeds its size limit");
            }
            result.extend_from_slice(&chunk);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp_security::{fixture_operator_consent, plan_http_connection};
    use std::net::{IpAddr, Ipv4Addr};
    use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

    async fn server(status: &str, content_type: &str, body: &str) -> (u16, tokio::task::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let status = status.to_owned();
        let content_type = content_type.to_owned();
        let body = body.to_owned();
        let task = tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buf=[0u8; 4096];
                let n=socket.read(&mut buf).await.unwrap_or(0);
                let request = String::from_utf8_lossy(&buf[..n]);
                assert!(request.starts_with("POST /mcp HTTP/1.1"));
                let response = format!("HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len());
                socket.write_all(response.as_bytes()).await.unwrap();
            }
        });
        (port, task)
    }
    #[tokio::test]
    async fn pinned_loopback_http_catalog_returns_bounded_json_without_redirects() {
        let response=r#"{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"tools":{}}}}"#;
        let (port, task)=server("200 OK", "application/json", response).await;
        let url=format!("http://127.0.0.1:{port}/mcp");
        let target=plan_http_connection(&url,&[IpAddr::V4(Ipv4Addr::LOCALHOST)]).unwrap();
        let client=ReadOnlyHttp::new(target,fixture_operator_consent()).unwrap();
        let result=client.request_catalog(serde_json::json!({"jsonrpc":"2.0","id":1,"method":"server/discover","params":{}})).await.unwrap();
        assert_eq!(result,response.as_bytes());
        task.await.unwrap();
    }
    #[tokio::test]
    async fn http_transport_rejects_redirect_and_call_without_network() {
        let (port,task)=server("302 Found", "application/json", "{}").await;
        let url=format!("http://127.0.0.1:{port}/mcp");
        let client=ReadOnlyHttp::new(plan_http_connection(&url,&[IpAddr::V4(Ipv4Addr::LOCALHOST)]).unwrap(),
            fixture_operator_consent()).unwrap();
        assert!(client.request_catalog(serde_json::json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"danger"}})).await.is_err());
        let error=client.request_catalog(serde_json::json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}})).await.unwrap_err();
        assert!(error.contains("redirects"));
        task.await.unwrap();
    }
    #[tokio::test]
    async fn http_transport_denies_non_json_content_type() {
        let (port,task)=server("200 OK", "text/event-stream", "data: unauthorized").await;
        let url=format!("http://127.0.0.1:{port}/mcp");
        let client=ReadOnlyHttp::new(plan_http_connection(&url,&[IpAddr::V4(Ipv4Addr::LOCALHOST)]).unwrap(),
            fixture_operator_consent()).unwrap();
        assert!(client.request_catalog(serde_json::json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}})).await.is_err());
        task.await.unwrap();
    }
}
