//! R2 operator-approved, ephemeral read-only MCP catalog inspection.
//! Never exposes a reusable approval token, process handle or tool executor
//! to JS. UI input identifies only a persisted server and protocol era.
//! One native modal, one config snapshot, one bounded discovery session.
use crate::{
    mcp_http::ReadOnlyHttp,
    mcp_protocol::{Catalog, Era, Inspector, ProtocolFailure},
    mcp_registry::{McpServer, Registry, Transport},
    mcp_security::{self, VerifiedOperatorConsent},
    mcp_stdio::ReadOnlyStdio,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    net::IpAddr,
    sync::Arc,
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
