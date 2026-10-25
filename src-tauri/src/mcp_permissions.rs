//! MCP policy preflight: deliberately no tool dispatch, process creation,
//! authentication, network access, or persistent grants.
//! Registry enabled flag is NOT authorization for any action.
use crate::mcp_registry::{McpServer, Registry};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedAction {
    Connect,
    ListTools,
    ListResources,
    ListPrompts,
    InvokeTool,
    ReadResource,
    GetPrompt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyDecision {
    pub allowed: bool,
    pub code: &'static str,
    pub message: &'static str,
    pub requires_approval: bool,
}

fn deny(code: &'static str, message: &'static str) -> PolicyDecision {
    PolicyDecision {
        allowed: false,
        code,
        message,
        requires_approval: true,
    }
}

/// Read-only decision. Returning a false result is mandatory even when
/// a server has been enabled in the Nexora registry.
pub fn evaluate(
    entries: &[McpServer],
    server_name: &str,
    _action: RequestedAction,
) -> PolicyDecision {
    let Some(server) = entries.iter().find(|server| server.name == server_name) else {
        return deny("not_found", "MCP server is not configured.");
    };
    if !server.enabled {
        return deny("disabled", "MCP server is disabled in the registry.");
    }
    deny(
        "approval_system_pending",
        "MCP execution is disabled until operator approval, isolation and secure transport are implemented.",
    )
}

#[tauri::command]
pub async fn mcp_policy_preflight(
    state: State<'_, Registry>,
    server_name: String,
    action: RequestedAction,
) -> Result<PolicyDecision, String> {
    // Do not let a malformed / overly long name request cause unnecessary disk I/O.
    if server_name.len() > 64
        || server_name.is_empty()
        || !server_name
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' || c == b'-')
    {
        return Ok(deny("invalid_name", "Invalid MCP server name."));
    }
    let reg = state.inner().clone();
    let entries = tauri::async_runtime::spawn_blocking(move || reg.list())
        .await
        .map_err(|_| "MCP policy worker failed.".to_owned())??;
    Ok(evaluate(&entries, &server_name, action))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp_registry::Transport;

    fn server(enabled: bool) -> McpServer {
        McpServer {
            name: "safe_server".to_owned(),
            description: String::new(),
            enabled,
            credential_ref: None,
            transport: Transport::Stdio {
                command: "npx".to_owned(),
                args: vec![],
            },
        }
    }

    #[test]
    fn all_mcp_actions_require_real_policy_implementation() {
        let actions = [
            RequestedAction::Connect,
            RequestedAction::ListTools,
            RequestedAction::ListResources,
            RequestedAction::ListPrompts,
            RequestedAction::InvokeTool,
            RequestedAction::ReadResource,
            RequestedAction::GetPrompt,
        ];
        for action in actions {
            let enabled = evaluate(&[server(true)], "safe_server", action);
            assert!(!enabled.allowed, "{action:?} must be denied");
            assert!(enabled.requires_approval);
            assert_eq!(enabled.code, "approval_system_pending");
            let disabled = evaluate(&[server(false)], "safe_server", action);
            assert!(!disabled.allowed);
            assert_eq!(disabled.code, "disabled");
        }
    }

    #[test]
    fn missing_server_is_never_permitted() {
        let decision = evaluate(&[], "unknown", RequestedAction::ListTools);
        assert!(!decision.allowed);
        assert_eq!(decision.code, "not_found");
    }

    #[test]
    fn malformed_actions_fail_schema_decoding() {
        for raw in [r#""run_shell""#, r#""ignore_permissions""#, r#"null"#] {
            assert!(serde_json::from_str::<RequestedAction>(raw).is_err());
        }
        assert!(serde_json::from_str::<RequestedAction>(r#""invoke_tool""#).is_ok());
    }

    #[test]
    fn decision_does_not_expose_sensitive_configuration() {
        let mut existing = server(true);
        existing.description = "internal-only configuration".into();
        let rendered = serde_json::to_string(&evaluate(
            &[existing],
            "safe_server",
            RequestedAction::Connect,
        ))
        .unwrap();
        assert!(!rendered.contains("internal-only"));
        assert!(!rendered.contains("npx"));
        assert!(rendered.contains("requiresApproval"));
    }
}
