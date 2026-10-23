//! OS credential-store-backed references for Nexora-owned MCP servers.
//! Internal only. No Tauri IPC and no automatic credential injection.
//! Creating/resolving/deleting a vault credential requires verified native
//! operator consent, currently NOT exposed by the renderer/agent surface.
#![allow(dead_code)]

use crate::{
    mcp_registry::valid_credential_ref,
    mcp_security::VerifiedOperatorConsent,
    secret_store,
};
use uuid::Uuid;

const SERVICE: &str = "com.theerasakping.nexoracode.mcp.v1";

fn account(server: &str, reference: &str) -> Result<String, &'static str> {
    if !valid_credential_ref(server, reference) {
        return Err("Invalid server-bound MCP vault reference");
    }
    let id = reference.rsplit(':').next().ok_or("Invalid MCP vault reference")?;
    Ok(format!("{server}/{id}"))
}

/// Store only in OS secure storage, never in MCP Registry JSON, model context
/// or command-line arguments. The returned reference is NOT authorization.
pub(crate) fn create(
    server: &str, secret: &str, _consent: VerifiedOperatorConsent,
) -> Result<String, String> {
    if server.is_empty() || server.len() > 64 ||
        !server.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' || c == b'-') {
        return Err("Invalid MCP vault server identity".into());
    }
    let reference = format!("vault:v1:{server}:{}", Uuid::new_v4());
    let account = account(server, &reference).map_err(str::to_owned)?;
    secret_store::set(SERVICE, &account, secret)?;
    Ok(reference)
}

/// Reference possession by itself is never a read permission. Only trusted
/// native consent can use this entry point; no Tauri commands call it.
pub(crate) fn resolve(
    server: &str, reference: &str, _consent: VerifiedOperatorConsent,
) -> Result<String, String> {
    let account = account(server, reference).map_err(str::to_owned)?;
    secret_store::get(SERVICE, &account)
        .ok_or_else(|| "MCP credential is not available in OS secure storage".into())
}

pub(crate) fn delete(
    server: &str, reference: &str, _consent: VerifiedOperatorConsent,
) -> Result<(), String> {
    let account = account(server, reference).map_err(str::to_owned)?;
    secret_store::remove(SERVICE, &account);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp_security::fixture_operator_consent as consent;

    #[test]
    fn secure_store_reference_is_server_bound_and_revoke_is_irreversible() {
        let reference = create("filesystem", "example-test-secret", consent()).unwrap();
        assert!(reference.starts_with("vault:v1:filesystem:"));
        assert!(!reference.contains("example-test-secret"));
        assert!(resolve("other", &reference, consent()).is_err());
        assert_eq!(resolve("filesystem", &reference, consent()).unwrap(), "example-test-secret");
        delete("filesystem", &reference, consent()).unwrap();
        assert!(resolve("filesystem", &reference, consent()).is_err());
    }

    #[test]
    fn invalid_secret_inputs_never_create_references() {
        assert!(create("server", "", consent()).is_err());
        assert!(create("server", "hello\nworld", consent()).is_err());
        assert!(create("server", &"x".repeat(4097), consent()).is_err());
        assert!(create("../escape", "test", consent()).is_err());
        assert!(resolve("server", "api-key=do-not-echo", consent()).is_err());
    }
}
