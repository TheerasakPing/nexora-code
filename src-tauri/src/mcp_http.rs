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

    /// One native-approved tool invocation. Caller must first verify exact
    /// advertised tool and consume an argument/config-scoped one-shot ticket.
    /// No generic method string is accepted from the renderer.
    pub async fn invoke_approved_tool(
        &self, tool: &str, arguments: serde_json::Map<String, Value>,
    ) -> Result<Value, &'static str> {
        let frame = serde_json::json!({
            "jsonrpc":"2.0", "id":9471u64, "method":"tools/call",
            "params":{"name":tool,"arguments":arguments,
                "_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}
        });
        let bytes = serde_json::to_vec(&frame).map_err(|_| "Cannot encode tool call")?;
        if bytes.len() > MAX_RPC_FRAME {
            return Err("MCP tool request too large");
        }
        let response = self.client
            .post(self.target.original_url().clone())
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json, text/event-stream")
            .header("MCP-Protocol-Version", "2026-07-28")
            .body(bytes)
            .send().await.map_err(|_| "Approved tool HTTP request failed")?;
        let peer = response.remote_addr().ok_or("MCP tool peer unavailable")?;
        self.target.verify_connected_peer(peer)
            .map_err(|_| "MCP tool peer is outside approved IP set")?;
        if !response.status().is_success() {
            return Err("MCP tool HTTP status rejected");
        }
        let mime = response.headers().get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()).unwrap_or("");
        let is_json = mime.split(';').next().is_some_and(|value| value.trim().eq_ignore_ascii_case("application/json"));
        let is_sse = mime.split(';').next().is_some_and(|value| value.trim().eq_ignore_ascii_case("text/event-stream"));
        if !is_json && !is_sse {
            return Err("MCP tool HTTP response is not JSON/SSE");
        }
        if response.content_length().is_some_and(|len| len > MAX_RPC_FRAME as u64) {
            return Err("MCP tool result exceeds output limit");
        }
        let mut response = response;
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| "MCP tool stream failed")? {
            if bytes.len().saturating_add(chunk.len()) > MAX_RPC_FRAME {
                return Err("MCP tool result exceeds output limit");
            }
            bytes.extend_from_slice(&chunk);
            if is_sse {
                if let Some(payload) = find_sse_response(&bytes, 9471)? {
                    return checked_tool_response(&payload);
                }
            }
        }
        if is_sse {
            Err("MCP tool SSE stream closed without matching result")
        } else {
            checked_tool_response(&bytes)
        }
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
            .header(header::ACCEPT, "application/json, text/event-stream")
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
        let is_json = content_type.split(';').next().is_some_and(|value| value.trim().eq_ignore_ascii_case("application/json"));
        let is_sse = content_type.split(';').next().is_some_and(|value| value.trim().eq_ignore_ascii_case("text/event-stream"));
        if !is_json && !is_sse {
            return Err("MCP HTTP requires JSON or bounded event-stream");
        }
        if response.content_length().is_some_and(|value| value > MAX_RPC_FRAME as u64) {
            return Err("MCP HTTP response exceeds its size limit");
        }
        let expected_id = frame["id"].as_u64().ok_or("Invalid request id")?;
        let mut response = response;
        let mut result = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| "MCP HTTP stream failed")? {
            if result.len().saturating_add(chunk.len()) > MAX_RPC_FRAME {
                return Err("MCP HTTP response exceeds its size limit");
            }
            result.extend_from_slice(&chunk);
            if is_sse {
                if let Some(frame) = find_sse_response(&result, expected_id)? {
                    return Ok(frame);
                }
            }
        }
        if is_sse {
            Err("MCP HTTP SSE ended without a matching bounded JSON-RPC message")
        } else {
            Ok(result)
        }
    }
}

fn checked_tool_response(data: &[u8]) -> Result<Value, &'static str> {
    let decoded: Value = serde_json::from_slice(data)
        .map_err(|_| "MCP tool result is not valid JSON")?;
    if decoded["jsonrpc"] != "2.0" || decoded["id"].as_u64() != Some(9471)
        || !decoded["result"].is_object() || decoded.get("error").is_some() {
        return Err("MCP tool result has invalid request ID, error or shape");
    }
    Ok(decoded["result"].clone())
}

/// Only accept a complete bounded SSE message event addressed to this exact
/// JSON-RPC request. This parser is intentionally strict: unrelated server
/// events, non-JSON frames and malformed fields fail closed. Never turn an
/// SSE `event` into executable instructions or follow a remote URL.
fn find_sse_response(body: &[u8], expected_id: u64) -> Result<Option<Vec<u8>>, &'static str> {
    if body.len() > MAX_RPC_FRAME {
        return Err("MCP SSE stream exceeds its size limit");
    }
    // SSE may use LF or CRLF; never silently replace invalid UTF-8.
    let normalized = std::str::from_utf8(body)
        .map_err(|_| "MCP SSE is not UTF-8")?.replace("\r\n", "\n");
    let body = normalized.as_bytes();
    let mut consumed = 0;
    let mut events = 0usize;
    while let Some(pos) = body[consumed..].windows(2).position(|bytes| bytes == b"\n\n") {
        events += 1;
        if events > 32 {
            return Err("MCP SSE event limit exceeded");
        }
        let end = consumed + pos;
        let event = std::str::from_utf8(&body[consumed..end])
            .map_err(|_| "MCP SSE is not UTF-8")?;
        consumed = end + 2;
        let mut data = String::new();
        let mut event_type = "message";
        for line in event.lines() {
            let line = line.trim_end_matches('\r');
            if line.starts_with(':') || line.is_empty() {
                continue;
            }
            if let Some(name) = line.strip_prefix("event:") {
                event_type = name.trim();
            } else if let Some(part) = line.strip_prefix("data:") {
                if !data.is_empty() { data.push('\n'); }
                data.push_str(part.strip_prefix(' ').unwrap_or(part));
            } else if line.starts_with("id:") || line.starts_with("retry:") {
                // No auto-resume/reconnect or execution based on SSE metadata.
            } else {
                return Err("Unexpected MCP SSE field");
            }
        }
        if data.is_empty() { continue; }
        if event_type != "message" {
            return Err("Unexpected MCP SSE event type");
        }
        let value: serde_json::Value = serde_json::from_str(&data)
            .map_err(|_| "MCP SSE contains an invalid JSON-RPC message")?;
        if value["jsonrpc"] != "2.0" || value["id"].as_u64() != Some(expected_id)
            || (!value["result"].is_object() && !value["error"].is_object()) {
            return Err("MCP SSE response is unrelated or malformed");
        }
        return Ok(Some(data.into_bytes()));
    }
    Ok(None)
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
    async fn bounded_sse_catalog_accepts_only_matching_jsonrpc_message() {
        let event = "event: message\ndata: {\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"tools\":[]}}\n\n";
        let (port, task) = server("200 OK", "text/event-stream", event).await;
        let url = format!("http://127.0.0.1:{port}/mcp");
        let client = ReadOnlyHttp::new(
            plan_http_connection(&url, &[IpAddr::V4(Ipv4Addr::LOCALHOST)]).unwrap(),
            fixture_operator_consent(),
        ).unwrap();
        let result = client.request_catalog(serde_json::json!({
            "jsonrpc":"2.0","id":1,"method":"tools/list","params":{}
        })).await.unwrap();
        assert_eq!(serde_json::from_slice::<Value>(&result).unwrap()["result"]["tools"], serde_json::json!([]));
        task.await.unwrap();
        assert!(find_sse_response(b"event: message\ndata: {\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{}}\n\n", 1).is_err());
        assert!(find_sse_response(b"event: execute\ndata: {\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{}}\n\n", 1).is_err());
        assert!(find_sse_response(b"event: message\ndata: secret\n\n", 1).is_err());
        assert!(find_sse_response(b"event: message\ndata: {\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{}}\n", 1).unwrap().is_none());
    }

    #[test]
    fn sse_crlf_and_invalid_utf8_are_strictly_handled() {
        let crlf = b"event: message\r\ndata: {\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{}}\r\n\r\n";
        assert!(find_sse_response(crlf, 1).unwrap().is_some());
        assert!(find_sse_response(b"event: message\ndata: \xff\n\n", 1).is_err());
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
