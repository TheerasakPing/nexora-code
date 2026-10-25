//! Explicit native operator-approved MCP HTTP credential lifecycle.
//! Only an opaque vault reference enters the registry or returns to JS.
//! A password may transiently cross the Tauri IPC from an explicit password
//! input; renderer state is cleared immediately after user submission.
//! No credential reads, tool calls or process invocations are exposed here.
#![allow(dead_code)]

use crate::{
    mcp_inspector::{approved_config_unchanged, InspectionGate},
    mcp_registry::{McpServer, Registry, Transport},
    mcp_security::{self, VerifiedOperatorConsent},
    mcp_vault,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tauri::{Manager, State, WebviewWindow};

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct VaultChangeResult {
    pub server: String,
    pub has_credential: bool,
    pub cleanup_warning: bool,
}

fn server_endpoint(server: &McpServer) -> Result<&str, String> {
    match &server.transport {
        Transport::StreamableHttp { url } => Ok(url),
        Transport::Stdio { .. } =>
            Err("MCP vault Bearer tokens are only available for HTTP servers".into()),
    }
}

fn update_vault(
    registry: &Registry, approved: &McpServer, secret: Option<&str>,
    consent: VerifiedOperatorConsent,
) -> Result<VaultChangeResult, String> {
    if !approved_config_unchanged(registry, approved)? {
        return Err("MCP server changed after approval".into());
    }
    let prior = approved.credential_ref.as_deref();
    match secret {
        Some(value) => {
            // Write the new vault entry first. If the registry cannot link
            // it, clean up the unlinked secret and fail closed.
            let next = mcp_vault::create(&approved.name, value, &consent)?;
            if let Err(error) = registry.set_credential_ref(approved, Some(next.clone())) {
                let cleanup = mcp_vault::delete(&approved.name, &next, &consent);
                if cleanup.is_err() {
                    return Err("MCP registry link failed; vault cleanup also failed".into());
                }
                return Err(error);
            }
            // The new credential is now the only registry-reachable entry.
            // Failure to delete an old OS key is reported, never suppressed.
            let cleanup_warning = prior.is_some_and(|old| {
                mcp_vault::delete(&approved.name, old, &consent).is_err()
            });
            Ok(VaultChangeResult {
                server: approved.name.clone(),
                has_credential: true,
                cleanup_warning,
            })
        }
        None => {
            let old = prior.ok_or("No linked MCP vault credential to revoke")?;
            // Delete before unlinking. If the OS denies removal the registry
            // stays marked protected, and no success is reported.
            mcp_vault::delete(&approved.name, old, &consent)?;
            registry.set_credential_ref(approved, None)?;
            Ok(VaultChangeResult {
                server: approved.name.clone(),
                has_credential: false,
                cleanup_warning: false,
            })
        }
    }
}

#[tauri::command]
pub async fn mcp_vault_change(
    window: WebviewWindow,
    registry: State<'_, Registry>,
    gate: State<'_, InspectionGate>,
    server_name: String,
    operation: String,
    secret: Option<String>,
) -> Result<VaultChangeResult, String> {
    if window.label() != "main" {
        return Err("MCP credential changes require the trusted main window".into());
    }
    if operation != "store" && operation != "revoke" {
        return Err("Unsupported MCP vault operation".into());
    }
    if operation == "store" && secret.as_ref().is_none_or(|value|
        value.is_empty() || value.len() > 4096 ||
        value.chars().any(char::is_control)
    ) {
        return Err("MCP credential must be 1-4096 characters without control bytes".into());
    }
    if operation == "revoke" && secret.is_some() {
        return Err("Revoke requests may not carry a secret".into());
    }
    let _guard = Arc::clone(&gate.0).try_lock_owned()
        .map_err(|_| "Another MCP operation is in progress")?;
    let registry = registry.inner().clone();
    let reg = registry.clone();
    let name = server_name.clone();
    let server = tokio::task::spawn_blocking(move || reg.list())
        .await.map_err(|_| "MCP registry lookup unavailable")??
        .into_iter().find(|entry| entry.name == name)
        .ok_or("MCP server not found")?;
    let endpoint = server_endpoint(&server)?.to_owned();
    if operation == "revoke" && server.credential_ref.is_none() {
        return Err("MCP server has no credential to revoke".into());
    }
    let bytes = serde_json::to_vec(&server).map_err(|_| "MCP config invalid")?;
    let fingerprint = format!("{:x}", Sha256::digest(&bytes));
    let consent = mcp_security::confirm_vault_change(
        window.app_handle().clone(), server.name.clone(), endpoint,
        fingerprint, operation == "revoke",
    ).await?;
    // Recheck after interactive dialog; a different endpoint/config never
    // receives the approved secret. Blocking OS vault I/O stays off WebView.
    tokio::task::spawn_blocking(move || update_vault(
        &registry, &server, secret.as_deref(), consent,
    ))
    .await.map_err(|_| "MCP OS vault worker failed")?
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp_registry::Transport;
    use crate::mcp_security::fixture_operator_consent as consent;
    use std::fs;

    fn setup() -> (Registry, McpServer, std::path::PathBuf) {
        let folder=std::env::temp_dir()
            .join(format!("nexora-vault-lifecycle-{}",uuid::Uuid::new_v4()));
        fs::create_dir_all(&folder).unwrap();
        let registry=Registry::new(&folder);
        let server=McpServer{
            name:"fixture".into(), description:String::new(), enabled:true,
            credential_ref:None,
            transport:Transport::StreamableHttp{url:"https://example.org/mcp".into()},
        };
        registry.upsert(server.clone()).unwrap();
        (registry,server,folder)
    }
    #[test]
    fn vault_store_rotate_revoke_and_registry_hide_secret() {
        let (registry, mut server, folder)=setup();
        let first=update_vault(&registry,&server,Some("first-test-secret"),consent()).unwrap();
        assert!(first.has_credential);
        server=registry.list().unwrap().remove(0);
        let original=server.credential_ref.clone().unwrap();
        assert!(original.starts_with("vault:v1:fixture:"));
        assert!(!fs::read_to_string(folder.join("nexora-mcp-registry-v1.json"))
            .unwrap().contains("first-test-secret"));
        assert_eq!(mcp_vault::resolve("fixture",&original,&consent()).unwrap(),"first-test-secret");
        let rotated=update_vault(&registry,&server,Some("second-test-secret"),consent()).unwrap();
        assert!(rotated.has_credential && !rotated.cleanup_warning);
        let newer=registry.list().unwrap().remove(0);
        let new_ref=newer.credential_ref.clone().unwrap();
        assert_ne!(original,new_ref);
        assert!(mcp_vault::resolve("fixture",&original,&consent()).is_err());
        assert_eq!(mcp_vault::resolve("fixture",&new_ref,&consent()).unwrap(),"second-test-secret");
        assert!(update_vault(&registry,&server,Some("stale"),consent()).is_err());
        let revoked=update_vault(&registry,&newer,None,consent()).unwrap();
        assert!(!revoked.has_credential);
        assert!(registry.list().unwrap()[0].credential_ref.is_none());
        assert!(mcp_vault::resolve("fixture",&new_ref,&consent()).is_err());
        fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn vault_changed_endpoint_denies_link_and_does_not_create_secret() {
        let (registry,server,folder)=setup();
        let mut changed=server.clone();
        changed.transport=Transport::StreamableHttp {url:"https://elsewhere.example/mcp".into()};
        registry.upsert(changed).unwrap();
        assert!(update_vault(&registry,&server,Some("should-not-write"),consent()).is_err());
        assert!(registry.list().unwrap()[0].credential_ref.is_none());
        fs::remove_dir_all(folder).unwrap();
    }
}
