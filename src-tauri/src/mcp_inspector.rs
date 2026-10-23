//! R2 operator-approved, ephemeral read-only MCP catalog inspection.
//! Never exposes a reusable approval token, process handle or tool executor
//! to JS. UI input identifies only a persisted server and protocol era.
//! One native modal, one config snapshot, one bounded discovery session.
use crate::{
    mcp_http::ReadOnlyHttp,
    mcp_protocol::{Catalog, Era, Inspector, ProtocolFailure},
    mcp_registry::{McpServer, Registry, Transport},
    mcp_security::{self, Capability, McpPermissionLedger, ToolIntent, VerifiedOperatorConsent},
    mcp_stdio::ReadOnlyStdio,
};
use serde::Serialize;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::{
    net::IpAddr,
    sync::Arc,
    time::Instant,
};
use tauri::{Manager, State, WebviewWindow};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct InspectionGate(Arc<Mutex<()>>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CatalogItem {
    kind: &'static str,
    name: String,
    label: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CatalogSnapshot {
    pub server: String,
    pub protocol: &'static str,
    pub items: Vec<CatalogItem>,
    pub status: &'static str,
}

fn error(value: ProtocolFailure) -> String {
    format!("MCP protocol rejected an invalid/unsupported response: {value:?}")
}
fn era_name(era: Era) -> &'static str {
    match era {
        Era::Modern20260728 => "2026-07-28",
        Era::Legacy20251125 => "2025-11-25",
    }
}
fn protocol(value: &str) -> Result<Era, String> {
    match value {
        "2026-07-28" => Ok(Era::Modern20260728),
        "2025-11-25" => Ok(Era::Legacy20251125),
        _ => Err("Unsupported MCP protocol version".into()),
    }
}

fn snapshot(name: &str, inspector: &Inspector, era: Era) -> CatalogSnapshot {
    let mut items = Vec::new();
    for (category, label) in [
        (Catalog::Tools, "tool"),
        (Catalog::Resources, "resource"),
        (Catalog::Prompts, "prompt"),
    ] {
        for entry in inspector.entries(category) {
            items.push(CatalogItem {
                kind: label,
                name: entry.name.clone(),
                label: entry.label.clone(),
            });
        }
    }
    CatalogSnapshot {
        server: name.to_owned(),
        protocol: era_name(era),
        items,
        status: "inspected_disconnect",
    }
}

async fn inspect_stdio(
    server: &McpServer,
    inspector: &mut Inspector,
    era: Era,
    consent: VerifiedOperatorConsent,
) -> Result<CatalogSnapshot, String> {
    let mut process = ReadOnlyStdio::spawn(server, consent)
        .await.map_err(str::to_owned)?;
    let outcome = async {
        let bytes = process.request_catalog(inspector.discover_request())
            .await.map_err(str::to_owned)?;
        inspector.accept_discovery(&bytes).map_err(error)?;
        if era == Era::Legacy20251125 {
            process.notify_initialized().await.map_err(str::to_owned)?;
        }
        for category in [Catalog::Tools, Catalog::Resources, Catalog::Prompts] {
            loop {
                let query = match inspector.request_page(category) {
                    Ok(query) => query,
                    Err(ProtocolFailure::CapabilityUnavailable) => break,
                    Err(failure) => return Err(error(failure)),
                };
                let reply = process.request_catalog(query).await.map_err(str::to_owned)?;
                if inspector.accept_page(category, &reply).map_err(error)? {
                    break;
                }
            }
        }
        Ok(snapshot(&server.name, inspector, era))
    }.await;
    process.terminate().await;
    outcome
}

async fn inspect_http(
    server: &McpServer,
    inspector: &mut Inspector,
    era: Era,
    consent: VerifiedOperatorConsent,
) -> Result<CatalogSnapshot, String> {
    if era != Era::Modern20260728 {
        return Err("Legacy stateful HTTP handshake needs separate audited session transport".into());
    }
    let url = match &server.transport {
        Transport::StreamableHttp { url } => url,
        _ => return Err("Expected HTTP MCP configuration".into()),
    };
    let parsed = tauri::Url::parse(url).map_err(|_| "Invalid MCP endpoint URL")?;
    let host = parsed.host_str().ok_or("MCP endpoint has no host")?;
    let port = parsed.port_or_known_default().ok_or("Invalid MCP port")?;
    let addresses: Vec<IpAddr> = tokio::time::timeout(
        std::time::Duration::from_secs(3), tokio::net::lookup_host((host.trim_matches(['[', ']']), port))
    ).await.map_err(|_| "MCP DNS lookup timed out")?
        .map_err(|_| "MCP DNS lookup failed")?
        .take(17).map(|value| value.ip()).collect();
    let approved_target = mcp_security::plan_http_connection(url, &addresses)
        .map_err(|_| "MCP network endpoint failed SSRF/peer policy")?;
    let client = ReadOnlyHttp::new(approved_target, consent).map_err(str::to_owned)?;
    let reply = client.request_catalog(inspector.discover_request()).await.map_err(str::to_owned)?;
    inspector.accept_discovery(&reply).map_err(error)?;
    for category in [Catalog::Tools, Catalog::Resources, Catalog::Prompts] {
        loop {
            let query = match inspector.request_page(category) {
                Ok(query) => query,
                Err(ProtocolFailure::CapabilityUnavailable) => break,
                Err(failure) => return Err(error(failure)),
            };
            let reply = client.request_catalog(query).await.map_err(str::to_owned)?;
            if inspector.accept_page(category, &reply).map_err(error)? {
                break;
            }
        }
    }
    Ok(snapshot(&server.name, inspector, era))
}

/// Discard a native approval whenever the persisted server differs from
/// what the operator reviewed. This is a byte-structured configuration check,
/// not a comparison of a renderer-supplied name or display label.
fn approved_config_unchanged(registry: &Registry, approved: &McpServer) -> Result<bool, String> {
    let matching = registry.list()?
        .into_iter().find(|entry| entry.name == approved.name);
    Ok(matching.as_ref() == Some(approved))
}

/// Trusted native-only entry point for reading the configured server catalog.
/// All actionable tools/resources/prompts remain blocked: no tool invocation
/// is available through this command. Every call ends by disconnecting.
async fn read_catalog(
    server: &McpServer, era: Era, consent: VerifiedOperatorConsent,
) -> Result<CatalogSnapshot, String> {
    if !server.enabled {
        return Err("MCP server is disabled".into());
    }
    if server.credential_ref.is_some() {
        return Err("Authenticated MCP discovery requires reviewed OS-vault integration".into());
    }
    let mut inspector = Inspector::new(era);
    match server.transport {
        Transport::Stdio { .. } => inspect_stdio(server, &mut inspector, era, consent).await,
        Transport::StreamableHttp { .. } => inspect_http(server, &mut inspector, era, consent).await,
    }
}

/// Manual tool calls are user-facing, always bounded, and never issued by
/// background agents. A renderer-supplied name/arguments are DATA, not a
/// permission grant. Every call gets its own native OS approval and ticket.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolExecutionResult {
    server: String,
    tool: String,
    result: Value,
    status: &'static str,
}

fn validate_manual_tool_input(
    tool: &str, arguments: &Value,
) -> Result<(Map<String, Value>, String), String> {
    if tool.is_empty() || tool.len() > 128 || tool.chars().any(char::is_control) {
        return Err("MCP tool name must be a bounded, non-control string".into());
    }
    let argument_map = arguments.as_object().ok_or("MCP tool arguments must be a JSON object")?;
    let raw = serde_json::to_string(arguments).map_err(|_| "Invalid MCP tool arguments")?;
    if raw.len() > 2048 || raw.chars().any(char::is_control) {
        return Err("MCP tool arguments are too large for full native approval".into());
    }
    Ok((argument_map.clone(), raw))
}

async fn perform_approved_tool(
    server: &McpServer, era: Era, tool: &str,
    arguments: Map<String, Value>, consent: VerifiedOperatorConsent,
) -> Result<Value, String> {
    // Require the actual connected server's tool catalog to advertise this
    // exact name BEFORE sending tools/call. No arbitrary method dispatch.
    let mut inspector = Inspector::new(era);
    match &server.transport {
        Transport::Stdio { .. } => {
            let mut connection = ReadOnlyStdio::spawn(server, consent)
                .await.map_err(str::to_owned)?;
            let output = async {
                let bytes = connection.request_catalog(inspector.discover_request())
                    .await.map_err(str::to_owned)?;
                inspector.accept_discovery(&bytes).map_err(error)?;
                if era == Era::Legacy20251125 {
                    connection.notify_initialized().await.map_err(str::to_owned)?;
                }
                loop {
                    let request = inspector.request_page(Catalog::Tools).map_err(error)?;
                    let bytes = connection.request_catalog(request).await.map_err(str::to_owned)?;
                    if inspector.accept_page(Catalog::Tools, &bytes).map_err(error)? {
                        break;
                    }
                }
                if !inspector.entries(Catalog::Tools).iter().any(|entry| entry.name == tool) {
                    return Err("Requested MCP tool was not advertised by the connected server".into());
                }
                connection.invoke_approved_tool(tool, arguments).await.map_err(str::to_owned)
            }.await;
            connection.terminate().await;
            output
        }
        Transport::StreamableHttp { url } => {
            if era != Era::Modern20260728 {
                return Err("Manual HTTP tool calls support only 2026-07-28 stateless MCP".into());
            }
            let parsed = tauri::Url::parse(url).map_err(|_| "Invalid MCP endpoint URL")?;
            let host = parsed.host_str().ok_or("Invalid MCP endpoint host")?;
            let port = parsed.port_or_known_default().ok_or("Invalid MCP endpoint port")?;
            let addresses: Vec<IpAddr> = tokio::time::timeout(
                std::time::Duration::from_secs(3),
                tokio::net::lookup_host((host.trim_matches(['[', ']']), port)),
            ).await.map_err(|_| "MCP DNS lookup timed out")?
                .map_err(|_| "MCP DNS lookup failed")?
                .take(17).map(|peer| peer.ip()).collect();
            let approved_target = mcp_security::plan_http_connection(url, &addresses)
                .map_err(|_| "MCP target denied by network security policy")?;
            let connection = ReadOnlyHttp::new(approved_target, consent).map_err(str::to_owned)?;
            let response = connection.request_catalog(inspector.discover_request())
                .await.map_err(str::to_owned)?;
            inspector.accept_discovery(&response).map_err(error)?;
            loop {
                let request = inspector.request_page(Catalog::Tools).map_err(error)?;
                let response = connection.request_catalog(request).await.map_err(str::to_owned)?;
                if inspector.accept_page(Catalog::Tools, &response).map_err(error)? {
                    break;
                }
            }
            if !inspector.entries(Catalog::Tools).iter().any(|entry| entry.name == tool) {
                return Err("Requested MCP tool was not advertised by the connected server".into());
            }
            connection.invoke_approved_tool(tool, arguments).await.map_err(str::to_owned)
        }
    }
}

/// Explicit, operator-driven, per-tool opt-in. Not a general MCP agent
/// executor, no credential-backed server access, no silent retry.
#[tauri::command]
pub async fn mcp_execute_tool_once(
    window: WebviewWindow,
    registry: State<'_, Registry>,
    gate: State<'_, InspectionGate>,
    server_name: String,
    protocol_version: String,
    tool_name: String,
    arguments: Value,
) -> Result<ToolExecutionResult, String> {
    if window.label() != "main" {
        return Err("Only the trusted main window can request manual MCP execution".into());
    }
    let era = protocol(&protocol_version)?;
    let (arguments, argument_string) = validate_manual_tool_input(&tool_name, &arguments)?;
    let _guard = Arc::clone(&gate.0).try_lock_owned()
        .map_err(|_| "Another MCP operation is in progress")?;
    let reg = registry.inner().clone();
    let lookup_name = server_name.clone();
    let server = tokio::task::spawn_blocking(move || reg.list())
        .await.map_err(|_| "MCP registry unavailable")??
        .into_iter().find(|entry| entry.name == lookup_name)
        .ok_or("MCP server not found")?;
    if !server.enabled || server.credential_ref.is_some() {
        return Err("Only enabled, non-credentialed MCP servers can execute manually".into());
    }
    let bytes = serde_json::to_vec(&server).map_err(|_| "MCP config is invalid")?;
    let hash = format!("{:x}", Sha256::digest(&bytes));
    let tool_digest = format!("{:x}", Sha256::digest(
        serde_json::to_vec(&(&tool_name, &arguments)).map_err(|_| "Invalid arguments")?,
    ));
    let intent = ToolIntent::from_config(
        &format!("mcp-manual-{}", uuid::Uuid::new_v4()),
        &server.name, &bytes, Capability::MutatingTool(format!("tool:{tool_digest}")),
    ).map_err(|_| "Invalid bounded MCP tool intent")?;
    let transport = match &server.transport {
        Transport::Stdio { command, args } =>
            format!("stdio {command} ({} configured arguments)", args.len()),
        Transport::StreamableHttp { url } => format!("HTTP {url}"),
    };
    let mut tickets = McpPermissionLedger::default();
    let issued_at = Instant::now();
    let ticket = tickets.request(intent.clone(), issued_at)
        .map_err(|_| "Could not issue scoped MCP approval ticket")?;
    let consent = match mcp_security::confirm_tool_execution(
        window.app_handle().clone(), server.name.clone(), transport,
        tool_name.clone(), argument_string, hash,
    ).await {
        Ok(consent) => consent,
        Err(error) => { tickets.deny(ticket); return Err(error); }
    };
    let reg = registry.inner().clone();
    let expected = server.clone();
    let unchanged = tokio::task::spawn_blocking(move || approved_config_unchanged(&reg, &expected))
        .await.map_err(|_| "MCP config revalidation unavailable")??;
    if !unchanged {
        tickets.deny(ticket);
        return Err("MCP server configuration changed during tool approval".into());
    }
    let after_dialog = Instant::now();
    tickets.approve(ticket, consent, after_dialog)
        .map_err(|_| "MCP tool approval expired or invalid")?;
    // Exact single-use ticket is consumed before ANY tool executable request;
    // the only resulting capability stays within this Rust stack.
    let execution_consent = tickets.take_operator_consent(ticket, &intent, Instant::now())
        .map_err(|_| "MCP tool authorization was already used or expired")?;
    let result = perform_approved_tool(&server, era, &tool_name, arguments, execution_consent).await?;
    Ok(ToolExecutionResult {
        server: server.name, tool: tool_name, result, status: "completed_disconnect",
    })
}

#[tauri::command]
pub async fn mcp_inspect_catalog(
    window: WebviewWindow,
    registry: State<'_, Registry>,
    gate: State<'_, InspectionGate>,
    server_name: String,
    protocol_version: String,
) -> Result<CatalogSnapshot, String> {
    if window.label() != "main" {
        return Err("MCP inspector requires the main trusted workspace window".into());
    }
    let era = protocol(&protocol_version)?;
    let lock = Arc::clone(&gate.0);
    let _guard = lock.try_lock_owned()
        .map_err(|_| "Another MCP inspection is in progress")?;
    // Registry is authoritative. Browser-supplied transport/config and raw
    // approval flags are not accepted.
    let reg = registry.inner().clone();
    let server_name_copy = server_name.clone();
    let server = tokio::task::spawn_blocking(move || reg.list())
        .await.map_err(|_| "MCP Registry unavailable")??
        .into_iter().find(|server| server.name == server_name_copy)
        .ok_or("MCP server not found")?;
    if !server.enabled || server.credential_ref.is_some() {
        return Err("MCP inspection requires an enabled, unauthenticated server".into());
    }
    let config_bytes = serde_json::to_vec(&server).map_err(|_| "MCP config not serializable")?;
    let fingerprint = format!("{:x}", Sha256::digest(&config_bytes));
    let description = match &server.transport {
        Transport::Stdio { command, args } => {
            format!("local stdio: {command} ({} configured arguments)", args.len())
        }
        Transport::StreamableHttp { url } => format!("HTTP: {url}"),
    };
    let consent = mcp_security::confirm_catalog_inspection(
        window.app_handle().clone(), server.name.clone(), description, fingerprint
    ).await?;
    // Config may change while the native dialog is open. Verify exact stored
    // bytes and enabled flag before touching any executable/network address.
    let reg = registry.inner().clone();
    let approved_server = server.clone();
    let still_matches = tokio::task::spawn_blocking(move || {
        approved_config_unchanged(&reg, &approved_server)
    }).await.map_err(|_| "MCP Registry worker failed")??;
    if !still_matches {
        return Err("MCP configuration changed while awaiting native approval".into());
    }
    // The consent value never leaves this Rust call. It cannot be replayed by
    // the caller. The short-lived transport is dropped after inspection.
    read_catalog(&server, era, consent).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mcp_registry::Transport, mcp_security::fixture_operator_consent};

    #[test]
    fn tool_arguments_and_names_are_bounded_and_invalid_shapes_denied() {
        assert!(validate_manual_tool_input("safe_search", &serde_json::json!({"query":"hello"})).is_ok());
        assert!(validate_manual_tool_input("tool/subpath", &serde_json::json!({})).is_ok());
        assert!(validate_manual_tool_input("safe_search", &serde_json::json!(["array"])).is_err());
        assert!(validate_manual_tool_input("safe_search", &serde_json::json!({"x":"z".repeat(3000)})).is_err());
        assert!(validate_manual_tool_input("bad
name", &serde_json::json!({})).is_err());
        let raw = serde_json::json!({"name":"safe_search", "arguments":{"query":"literal"}});
        assert!(serde_json::to_string(&raw).unwrap().contains("literal"));
    }

    #[test]
    fn one_use_ticket_binds_config_and_exact_arguments() {
        let bytes = b"approved server config";
        let now = Instant::now();
        let digest = format!("{:x}", Sha256::digest(br#"["search",{"input":"a"}]"#));
        let intent = ToolIntent::from_config(
            &format!("mcp-manual-{}",uuid::Uuid::new_v4()),
            "fixture", bytes, Capability::MutatingTool(format!("tool:{digest}")),
        ).unwrap();
        let changed = ToolIntent::from_config(
            &intent.session_id, "fixture", bytes,
            Capability::MutatingTool(format!("tool:{}", Sha256::digest(b"other").iter()
                .map(|byte| format!("{byte:02x}")).collect::<String>())),
        ).unwrap();
        let mut ledger = McpPermissionLedger::default();
        let ticket = ledger.request(intent.clone(), now).unwrap();
        ledger.approve(ticket, fixture_operator_consent(), now).unwrap();
        assert!(ledger.take_operator_consent(ticket, &changed, now).is_err());
        assert!(ledger.take_operator_consent(ticket, &intent, now).is_err());
        let ticket = ledger.request(intent.clone(), now).unwrap();
        ledger.approve(ticket, fixture_operator_consent(), now).unwrap();
        assert!(ledger.take_operator_consent(ticket, &intent, now).is_ok());
        assert!(ledger.take_operator_consent(ticket, &intent, now).is_err());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn manual_tool_executes_once_on_real_local_stdio_fixture_only_if_advertised() {
        use std::{fs, os::unix::fs::PermissionsExt};
        let folder = std::env::temp_dir().join(format!("nexora-r2-manual-exec-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&folder).unwrap();
        let executable = folder.join("mock");
        let mark = folder.join("called");
        let script = format!(r##"#!/bin/sh
read discover
echo '{{"jsonrpc":"2.0","id":1,"result":{{"capabilities":{{"tools":{{}}}}}}}}'
read listed
echo '{{"jsonrpc":"2.0","id":2,"result":{{"tools":[{{"name":"safe_echo","inputSchema":{{"type":"object"}}}}]}}}}'
read call
case "$call" in *tools/call*safe_echo*) printf 'one' > '{}';; *) exit 7;; esac
echo '{{"jsonrpc":"2.0","id":9471,"result":{{"content":[{{"type":"text","text":"echo-complete"}}]}}}}'
"##, mark.display());
        fs::write(&executable, script).unwrap();
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
        let server = McpServer {
            name: "mock".into(), description: "".into(), enabled: true, credential_ref: None,
            transport: Transport::Stdio {command:executable.to_string_lossy().into_owned(),args:vec![]},
        };
        let result = tokio::time::timeout(std::time::Duration::from_secs(5),
            perform_approved_tool(&server, Era::Modern20260728, "safe_echo",
                serde_json::json!({"message":"hi"}).as_object().unwrap().clone(), fixture_operator_consent())
        ).await.unwrap().unwrap();
        assert_eq!(result["content"][0]["text"], "echo-complete");
        assert_eq!(fs::read_to_string(&mark).unwrap(), "one");
        fs::remove_file(&mark).unwrap();
        let error = perform_approved_tool(&server, Era::Modern20260728, "not_advertised",
            serde_json::json!({}).as_object().unwrap().clone(), fixture_operator_consent()).await.unwrap_err();
        assert!(error.contains("not advertised"));
        assert!(!mark.exists(), "unknown tool request unexpectedly executed");
        fs::remove_dir_all(folder).unwrap();
    }

    #[tokio::test]
    async fn manual_tool_calls_real_local_pinned_http_only_for_advertised_name() {
        use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
        let listener=TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port=listener.local_addr().unwrap().port();
        let task=tokio::spawn(async move {
            for (method, id, result) in [
                ("server/discover",1,serde_json::json!({"capabilities":{"tools":{}}})),
                ("tools/list",2,serde_json::json!({"tools":[{"name":"say_hi","inputSchema":{"type":"object"}}]})),
                ("tools/call",9471,serde_json::json!({"content":[{"type":"text","text":"done"}]})),
            ] {
                let (mut socket, _)=listener.accept().await.unwrap();
                let mut chunk = [0u8; 8192];
                let read = tokio::time::timeout(
                    std::time::Duration::from_secs(2), socket.read(&mut chunk)
                ).await.unwrap().unwrap();
                let request = String::from_utf8_lossy(&chunk[..read]);
                assert!(request.contains(method), "unexpected RPC method: {method}");
                let payload=serde_json::json!({"jsonrpc":"2.0","id":id,"result":result}).to_string();
                let response=format!("HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: {}
Connection: close

{payload}",payload.len());
                socket.write_all(response.as_bytes()).await.unwrap();
            }
        });
        let server=McpServer{
            name:"fixture".into(),description:"".into(),enabled:true,credential_ref:None,
            transport:Transport::StreamableHttp{url:format!("http://127.0.0.1:{port}/mcp")},
        };
        let result=tokio::time::timeout(std::time::Duration::from_secs(6),
            perform_approved_tool(&server, Era::Modern20260728, "say_hi",
                serde_json::json!({"name":"friend"}).as_object().unwrap().clone(), fixture_operator_consent())
        ).await.unwrap().unwrap();
        assert_eq!(result["content"][0]["text"],"done");
        task.await.unwrap();
    }

    #[test]
    fn config_change_and_removal_invalidate_native_inspector_approval() {
        use std::fs;
        let root = std::env::temp_dir()
            .join(format!("nexora-r2-approved-snapshot-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&root).unwrap();
        let registry = Registry::new(&root);
        let server = McpServer {
            name: "fixture".into(), description: "reviewed".into(), enabled: true,
            credential_ref: None,
            transport: Transport::StreamableHttp { url: "http://127.0.0.1:34111/mcp".into() },
        };
        registry.upsert(server.clone()).unwrap();
        assert!(approved_config_unchanged(&registry, &server).unwrap());
        registry.set_enabled("fixture", false).unwrap();
        assert!(!approved_config_unchanged(&registry, &server).unwrap());
        registry.upsert(server.clone()).unwrap();
        let mut changed = server.clone();
        changed.transport = Transport::StreamableHttp {
            url: "http://127.0.0.1:34112/mcp".into(),
        };
        registry.upsert(changed).unwrap();
        assert!(!approved_config_unchanged(&registry, &server).unwrap());
        registry.remove("fixture").unwrap();
        assert!(!approved_config_unchanged(&registry, &server).unwrap());
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn inspector_gate_rejects_concurrent_approval_dialogs() {
        let gate = InspectionGate::default();
        let active = Arc::clone(&gate.0).try_lock_owned().unwrap();
        assert!(Arc::clone(&gate.0).try_lock_owned().is_err());
        drop(active);
        assert!(Arc::clone(&gate.0).try_lock_owned().is_ok());
    }

    #[test]
    fn inspector_protocol_is_strict_and_empty_snapshots_reveal_no_config() {
        assert!(protocol("2025-12-01").is_err());
        let inspect = Inspector::new(Era::Modern20260728);
        let dto = snapshot("safe_name", &inspect, Era::Modern20260728);
        assert!(dto.items.is_empty());
        assert_eq!(dto.status, "inspected_disconnect");
        let text = serde_json::to_string(&dto).unwrap();
        assert!(!text.contains("credentialRef"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn complete_offline_legacy_stdio_inspection_requires_scoped_native_consent() {
        use std::{fs, os::unix::fs::PermissionsExt};
        let folder = std::env::temp_dir().join(format!("nexora-r2-inspector-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&folder).unwrap();
        let executable = folder.join("mock-mcp");
        let script = r##"#!/bin/sh
read initial
echo '{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-11-25","serverInfo":{"name":"fixture","version":"1"},"capabilities":{"tools":{},"resources":{},"prompts":{}}}}'
read initialized
case "$initialized" in *notifications/initialized*) ;; *) exit 6 ;; esac
read list
echo '{"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"safe_search","inputSchema":{"type":"object"}}]}}'
read list
echo '{"jsonrpc":"2.0","id":3,"result":{"resources":[{"uri":"resource://test/1"}]}}'
read list
echo '{"jsonrpc":"2.0","id":4,"result":{"prompts":[{"name":"summarize"}]}}'
"##;
        fs::write(&executable, script).unwrap();
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
        let server = McpServer {
            name: "fixture".into(), description: "".into(), enabled: true,
            credential_ref: None,
            transport: Transport::Stdio {
                command: executable.to_string_lossy().into_owned(), args: vec![],
            },
        };
        let output = read_catalog(&server, Era::Legacy20251125, fixture_operator_consent())
            .await.unwrap();
        assert_eq!(output.status, "inspected_disconnect");
        assert_eq!(output.items.len(), 3);
        assert_eq!(output.items[0].name, "safe_search");
        assert_eq!(output.items[1].name, "resource://test/1");
        assert_eq!(output.items[2].name, "summarize");
        fs::remove_dir_all(folder).unwrap();
    }

    #[tokio::test]
    async fn complete_pinned_http_inspection_returns_catalog_without_executing_tools() {
        use std::net::{IpAddr, Ipv4Addr};
        use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server_task = tokio::spawn(async move {
            for (id, data) in [
                (1, serde_json::json!({"capabilities":{"tools":{},"resources":{},"prompts":{}}})),
                (2, serde_json::json!({"tools":[{"name":"allowed_metadata","inputSchema":{"type":"object"}}]})),
                (3, serde_json::json!({"resources":[{"uri":"resource://fixture"}]})),
                (4, serde_json::json!({"prompts":[{"name":"review"}]})),
            ] {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut bytes = [0u8; 8192];
                let request_len = stream.read(&mut bytes).await.unwrap();
                assert!(String::from_utf8_lossy(&bytes[..request_len]).starts_with("POST /mcp HTTP/1.1"));
                let payload = serde_json::json!({"jsonrpc":"2.0","id":id,"result":data}).to_string();
                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
                    payload.len()
                );
                stream.write_all(reply.as_bytes()).await.unwrap();
            }
        });
        let url = format!("http://127.0.0.1:{port}/mcp");
        let server = McpServer {
            name: "fixture".into(), description: "".into(), enabled: true,
            credential_ref: None,
            transport: Transport::StreamableHttp { url },
        };
        let snapshot = tokio::time::timeout(
            std::time::Duration::from_secs(6),
            read_catalog(&server, Era::Modern20260728, fixture_operator_consent()),
        ).await.unwrap().unwrap();
        assert_eq!(snapshot.items.len(), 3);
        assert!(snapshot.items.iter().all(|entry| !entry.name.contains("secret")));
        server_task.await.unwrap();
        // The ephemeral read-only inspector is disconnected after returning.
        assert_eq!(snapshot.status, "inspected_disconnect");
        let _ = (IpAddr::V4(Ipv4Addr::LOCALHOST), port);
    }

    #[tokio::test]
    async fn server_must_be_enabled_and_uncredentialed_before_inspection() {
        let server = McpServer {
            name: "mock".into(), description: String::new(), enabled: false,
            credential_ref: None,
            transport: Transport::Stdio { command: "/bin/false".into(), args: vec![] },
        };
        assert!(read_catalog(&server, Era::Modern20260728, fixture_operator_consent()).await.is_err());
        let mut enabled = server;
        enabled.enabled = true;
        enabled.credential_ref = Some(format!("vault:v1:mock:{}", uuid::Uuid::new_v4()));
        assert!(read_catalog(&enabled, Era::Modern20260728, fixture_operator_consent()).await.is_err());
    }
}
