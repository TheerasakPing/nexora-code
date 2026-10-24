//! R2 policy primitives only. No network sockets, process launches, or MCP calls.
//! A future client MUST connect to a pinned validated IP (not re-resolve it), retain
//! the original TLS hostname, reject redirects, and verify the connected peer.
//! Merely storing enabled=true in the MCP registry never grants access.

use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, VecDeque},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    time::{Duration, Instant},
};
use tauri::Url;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Denial {
    InvalidEndpoint,
    ForbiddenTarget,
    EmptyResolution,
    UntrustedDnsAnswer,
    RedirectNotPermitted,
    UnpinnedPeer,
    InvalidIntent,
    ExpiredOrMissingApproval,
}

#[derive(Clone, Debug)]
pub struct PinnedHttpTarget {
    original_url: Url,
    host: String,
    port: u16,
    peers: Vec<SocketAddr>,
}

impl PinnedHttpTarget {
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    /// These are the ONLY socket peers a future HTTP client may connect to.
    /// DNS answers may not be re-resolved after this check.
    pub fn peers(&self) -> &[SocketAddr] {
        &self.peers
    }
    pub fn original_url(&self) -> &Url {
        &self.original_url
    }
    pub fn verify_connected_peer(&self, actual: SocketAddr) -> Result<(), Denial> {
        self.peers
            .contains(&actual)
            .then_some(())
            .ok_or(Denial::UnpinnedPeer)
    }
    /// No implicit Location header following in R2.
    pub fn accept_redirect(&self, _location: &str) -> Result<(), Denial> {
        Err(Denial::RedirectNotPermitted)
    }
}

fn public_v4(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    match octets {
        [0, ..]
        | [10, ..]
        | [127, ..]
        | [169, 254, ..]
        | [192, 168, ..]
        | [192, 0, 0, ..]
        | [192, 0, 2, ..]
        | [192, 88, 99, ..]
        | [192, 175, 48, ..]
        | [198, 51, 100, ..]
        | [203, 0, 113, ..]
        | [192, 52, 193, ..] => false,
        [100, b, ..] if (64..=127).contains(&b) => false, // CGNAT
        [172, b, ..] if (16..=31).contains(&b) => false,
        [198, b, ..] if b == 18 || b == 19 => false, // benchmark
        [a, ..] if a >= 224 => false,                // multicast and reserved
        _ => !ip.is_broadcast() && !ip.is_unspecified(),
    }
}

fn public_v6(ip: Ipv6Addr) -> bool {
    if ip.to_ipv4_mapped().is_some() {
        return false; // IPv4-mapped destinations need their own policy.
    }
    let segments = ip.segments();
    // Conservative global unicast-only: excludes ULA, link-local, multicast,
    // loopback, IPv4 translation, 6to4 relay and non-global reservations.
    if !(0x2000..=0x3fff).contains(&segments[0]) {
        return false;
    }
    if segments[0] == 0x2001 {
        if segments[1] == 0x0db8 || segments[1] == 0x0000 {
            return false; // documentation and Teredo
        }
        if segments[1] == 0x0010 || segments[1] == 0x0020 {
            return false; // ORCHID ranges, conservative
        }
    }
    segments[0] != 0x2002 // 6to4
}

fn is_public_peer(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => public_v4(ip),
        IpAddr::V6(ip) => public_v6(ip),
    }
}

/// Validates a *snapshot* of resolved records. Future network code must bind
/// to one of returned socket addresses to prevent DNS rebind/time-of-check swaps.
/// Rejects the ENTIRE answer set if any record is forbidden.
pub fn plan_http_connection(url: &str, resolved: &[IpAddr]) -> Result<PinnedHttpTarget, Denial> {
    let parsed = Url::parse(url).map_err(|_| Denial::InvalidEndpoint)?;
    let host = parsed.host_str().ok_or(Denial::InvalidEndpoint)?;
    let hostname = host.trim_matches(['[', ']']).to_ascii_lowercase();
    let local = matches!(hostname.as_str(), "localhost" | "127.0.0.1" | "::1");
    if parsed.username() != ""
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || parsed.cannot_be_a_base()
        || !matches!(parsed.scheme(), "https" | "http")
        || (parsed.scheme() == "http" && !local)
        || resolved.is_empty()
        || resolved.len() > 16
    {
        return Err(Denial::InvalidEndpoint);
    }
    if hostname.ends_with(".localhost") {
        return Err(Denial::ForbiddenTarget);
    }
    if let Ok(literal) = hostname.parse::<IpAddr>() {
        // Even local literal addresses must not resolve to a different IP.
        if resolved.iter().any(|ip| *ip != literal) {
            return Err(Denial::UntrustedDnsAnswer);
        }
    }
    // Literal localhost must not unexpectedly resolve to a non-loopback IP.
    if local {
        if resolved.iter().any(|ip| !ip.is_loopback()) {
            return Err(Denial::UntrustedDnsAnswer);
        }
    } else if resolved.iter().any(|ip| !is_public_peer(*ip)) {
        return Err(Denial::ForbiddenTarget);
    }
    let port = parsed
        .port_or_known_default()
        .ok_or(Denial::InvalidEndpoint)?;
    if port == 0 {
        return Err(Denial::InvalidEndpoint);
    }
    let mut peers = Vec::new();
    for ip in resolved {
        let addr = SocketAddr::new(*ip, port);
        if !peers.contains(&addr) {
            peers.push(addr);
        }
    }
    Ok(PinnedHttpTarget {
        original_url: parsed,
        host: hostname,
        port,
        peers,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Capability {
    Connect,
    Discover,
    ReadOnlyTool(String),
    MutatingTool(String),
}

/// Permission is bound to session, server, exact configuration and operation.
/// One approval never automatically grants another capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolIntent {
    pub session_id: String,
    pub server_id: String,
    pub server_config_sha256: [u8; 32],
    pub capability: Capability,
}

// Identity bytes must not carry control sequences, invisible Unicode format
// characters, whitespace or shell/JSON delimiters into later native audit and
// approval surfaces. These are identifiers, not user-facing labels.
fn valid_intent_identifier(value: &str, limit: usize) -> bool {
    !value.is_empty()
        && value.len() <= limit
        && !value
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'/' | b':')
        })
}

impl ToolIntent {
    pub fn from_config(
        session_id: &str,
        server_id: &str,
        server_config: &[u8],
        capability: Capability,
    ) -> Result<Self, Denial> {
        if !valid_intent_identifier(session_id, 128)
            || server_id.is_empty()
            || server_id.len() > 64
            || server_config.is_empty()
            || server_config.len() > 16 * 1024
            || !server_id
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' || c == b'-')
            || matches!(&capability, Capability::ReadOnlyTool(name) | Capability::MutatingTool(name) if !valid_intent_identifier(name, 256))
        {
            return Err(Denial::InvalidIntent);
        }
        Ok(Self {
            session_id: session_id.to_owned(),
            server_id: server_id.to_owned(),
            server_config_sha256: Sha256::digest(server_config).into(),
            capability,
        })
    }
}

/// Deliberately not publicly constructible. A future *trusted native* UI
/// approval path must supply this token; agent/tool/renderer parameters cannot
/// manufacture explicit operator consent via IPC.
pub struct VerifiedOperatorConsent {
    _private: (),
}

/// Only the native desktop dialog may mint production discovery consent.
/// The renderer cannot pass/deserialize this token or approve itself.
/// This is one confirmation for read-only catalog discovery of ONE persisted,
/// snapshot-bound MCP configuration (never a tool/resource execution grant).
pub(crate) async fn confirm_catalog_inspection(
    app: tauri::AppHandle,
    server_name: String,
    transport_description: String,
    config_fingerprint: String,
) -> Result<VerifiedOperatorConsent, String> {
    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
    let approved = tokio::task::spawn_blocking(move || {
        app.dialog()
            .message(format!(
                "Allow Nexora Code to connect for READ-ONLY MCP discovery?\n\nServer: {server_name}\nTransport: {transport_description}\nConfiguration SHA-256: {config_fingerprint}\n\nOnly tools/list, resources/list and prompts/list may be requested. No tools will run. Connections may expose your computer IP to the server.\n\nContinue?"
            ))
            .title("Nexora Code — Approve MCP Catalog Inspection")
            .kind(MessageDialogKind::Warning)
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Approve Once".into(), "Deny".into(),
            ))
            .blocking_show()
    })
    .await
    .map_err(|_| "Native MCP approval dialog unavailable".to_owned())?;
    if approved {
        Ok(VerifiedOperatorConsent { _private: () })
    } else {
        Err("MCP catalog inspection was not approved by the operator".into())
    }
}

/// A native OS modal, separate from tool execution, approves one bounded
/// OS-secure-store update. Never displays or returns the actual secret.
pub(crate) async fn confirm_vault_change(
    app: tauri::AppHandle, server: String, endpoint: String,
    config_hash: String, revoke: bool,
) -> Result<VerifiedOperatorConsent, String> {
    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
    let operation = if revoke { "REVOKE" } else { "STORE / ROTATE" };
    let confirmed = tokio::task::spawn_blocking(move || app.dialog()
        .message(format!(
            "Authorize ONE MCP credential {operation} in OS secure storage?\n\nServer: {server}\nEndpoint: {endpoint}\nConfig SHA-256: {config_hash}\n\nCredentials are never written to the MCP registry. Only approved HTTP MCP requests will receive an Authorization header.\n\nContinue?"
        ))
        .title("Nexora Code — MCP OS Vault")
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::OkCancelCustom("Approve Once".into(), "Deny".into()))
        .blocking_show()
    ).await.map_err(|_| "Native vault approval dialog unavailable".to_owned())?;
    if confirmed {
        Ok(VerifiedOperatorConsent { _private: () })
    } else {
        Err("Operator denied MCP vault change".into())
    }
}

/// Native consent for ONE specific tool invocation, not for all tools on the
/// server. The exact arguments must fit the dialog in full; no truncation.
pub(crate) async fn confirm_tool_execution(
    app: tauri::AppHandle,
    server: String,
    transport: String,
    tool: String,
    arguments: String,
    config_hash: String,
) -> Result<VerifiedOperatorConsent, String> {
    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
    if arguments.len() > 2048 || arguments.chars().any(|value| value == '\u{001b}') {
        return Err("Tool arguments exceed the native approval display limit".into());
    }
    let accepted = tokio::task::spawn_blocking(move || app.dialog()
        .message(format!(
            "Execute ONE MCP tool? This can CHANGE FILES, SEND DATA or RUN COMMANDS depending on the server.\n\nServer: {server}\nTransport: {transport}\nTool: {tool}\nArguments (complete): {arguments}\nConfig SHA-256: {config_hash}\n\nApprove this exact call once?"
        ))
        .title("Nexora Code — MCP Tool Execution Approval")
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Run Once".into(), "Deny".into(),
        ))
        .blocking_show()
    ).await.map_err(|_| "Native MCP tool approval dialog unavailable".to_owned())?;
    if accepted {
        Ok(VerifiedOperatorConsent { _private: () })
    } else {
        Err("Operator denied this MCP tool invocation".into())
    }
}

// Only deterministic native test fixtures can issue simulated operator
// consent; production has no renderer/agent-accessible constructor.
#[cfg(test)]
pub(crate) fn fixture_operator_consent() -> VerifiedOperatorConsent {
    VerifiedOperatorConsent { _private: () }
}


struct PendingApproval {
    intent: ToolIntent,
    expires: Instant,
    approved: bool,
}

/// Audit records deliberately contain only a random ticket ID and a fixed
/// outcome kind: no tool arguments, credential refs, remote error messages,
/// secrets, raw session IDs, config blobs or agent-controlled text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditKind {
    Requested,
    Approved,
    Denied,
    Consumed,
    Rejected,
    Revoked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuditEvent {
    pub ticket_id: Uuid,
    pub kind: AuditKind,
}

/// Ephemeral, one-shot permission ledger; defaults to no access, no persisted
/// blanket approvals, no authorization after restart, bounded outstanding IDs.
#[derive(Default)]
pub struct McpPermissionLedger {
    tickets: HashMap<Uuid, PendingApproval>,
    audit: VecDeque<AuditEvent>,
}

impl McpPermissionLedger {
    const MAX_PENDING: usize = 128;
    const MAX_TTL: Duration = Duration::from_secs(120);
    const MAX_AUDIT: usize = 256;

    fn record(&mut self, id: Uuid, kind: AuditKind) {
        if self.audit.len() >= Self::MAX_AUDIT {
            self.audit.pop_front();
        }
        self.audit.push_back(AuditEvent { ticket_id: id, kind });
    }

    /// Read-only diagnostic snapshot; internal native code only, never
    /// rendered or persisted without a separate privacy/security review.
    pub fn audit(&self) -> Vec<AuditEvent> {
        self.audit.iter().copied().collect()
    }

    pub fn request(&mut self, intent: ToolIntent, now: Instant) -> Result<Uuid, Denial> {
        self.tickets.retain(|_, ticket| ticket.expires > now);
        if self.tickets.len() >= Self::MAX_PENDING {
            return Err(Denial::InvalidIntent);
        }
        let id = Uuid::new_v4();
        self.tickets.insert(
            id,
            PendingApproval {
                intent,
                expires: now + Self::MAX_TTL,
                approved: false,
            },
        );
        self.record(id, AuditKind::Requested);
        Ok(id)
    }

    pub fn approve(
        &mut self,
        id: Uuid,
        _operator: VerifiedOperatorConsent,
        now: Instant,
    ) -> Result<(), Denial> {
        let Some(ticket) = self.tickets.get_mut(&id) else {
            self.record(id, AuditKind::Rejected);
            return Err(Denial::ExpiredOrMissingApproval);
        };
        if ticket.expires <= now {
            self.tickets.remove(&id);
            self.record(id, AuditKind::Rejected);
            return Err(Denial::ExpiredOrMissingApproval);
        }
        ticket.approved = true;
        self.record(id, AuditKind::Approved);
        Ok(())
    }

    pub fn deny(&mut self, id: Uuid) {
        if self.tickets.remove(&id).is_some() {
            self.record(id, AuditKind::Denied);
        }
    }

    /// A ticket is consumed on first attempt regardless of whether it matches,
    /// so wrong session/operation/server cannot be used for repeated probing.
    pub fn take(&mut self, id: Uuid, intent: &ToolIntent, now: Instant) -> Result<(), Denial> {
        let Some(ticket) = self.tickets.remove(&id) else {
            self.record(id, AuditKind::Rejected);
            return Err(Denial::ExpiredOrMissingApproval);
        };
        if !ticket.approved || ticket.expires <= now || ticket.intent != *intent {
            self.record(id, AuditKind::Rejected);
            return Err(Denial::ExpiredOrMissingApproval);
        }
        self.record(id, AuditKind::Consumed);
        Ok(())
    }

    /// A validated one-use ledger ticket yields an internal, non-serializable
    /// capability for the immediately scoped execution connection.
    pub(crate) fn take_operator_consent(
        &mut self, id: Uuid, intent: &ToolIntent, now: Instant,
    ) -> Result<VerifiedOperatorConsent, Denial> {
        self.take(id, intent, now)?;
        Ok(VerifiedOperatorConsent { _private: () })
    }

    pub fn revoke_session(&mut self, session_id: &str) {
        let mut revoked = Vec::new();
        self.tickets.retain(|id, ticket| {
            if ticket.intent.session_id == session_id {
                revoked.push(*id);
                false
            } else { true }
        });
        for id in revoked { self.record(id, AuditKind::Revoked); }
    }

    pub fn revoke_server(&mut self, server_id: &str) {
        let mut revoked = Vec::new();
        self.tickets.retain(|id, ticket| {
            if ticket.intent.server_id == server_id {
                revoked.push(*id);
                false
            } else { true }
        });
        for id in revoked { self.record(id, AuditKind::Revoked); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    fn address(ip: &str) -> IpAddr {
        ip.parse().unwrap()
    }
    fn consent() -> VerifiedOperatorConsent {
        VerifiedOperatorConsent { _private: () }
    }
    fn intent(session: &str, verb: Capability) -> ToolIntent {
        ToolIntent::from_config(session, "filesystem", b"stdio:npx @mcp/fs", verb).unwrap()
    }

    #[test]
    fn audit_is_bounded_and_contains_only_fixed_outcomes() {
        let mut ledger = McpPermissionLedger::default();
        let now = Instant::now();
        let intent = intent("session-a", Capability::ReadOnlyTool("safe/read".into()));
        let ticket = ledger.request(intent.clone(), now).unwrap();
        ledger.approve(ticket, consent(), now).unwrap();
        assert!(ledger.take(ticket, &intent, now).is_ok());
        assert!(ledger.take(ticket, &intent, now).is_err());
        assert_eq!(ledger.audit().iter().map(|entry| entry.kind).collect::<Vec<_>>(),
            [AuditKind::Requested,AuditKind::Approved,AuditKind::Consumed,AuditKind::Rejected]);
        let output = format!("{:?}", ledger.audit());
        assert!(!output.contains("session-a"));
        assert!(!output.contains("safe/read"));

        for i in 0..300 {
            let pending = ledger.request(intent.clone(), now).unwrap();
            if i % 2 == 0 { ledger.deny(pending); }
            else { ledger.revoke_session("session-a"); }
        }
        assert_eq!(ledger.audit().len(), McpPermissionLedger::MAX_AUDIT);
    }

    #[test]
    fn wrong_config_and_expired_ticket_consume_once_and_are_logged_as_rejected() {
        let mut ledger = McpPermissionLedger::default();
        let now = Instant::now();
        let first = intent("session-a", Capability::Connect);
        let other = intent("session-b", Capability::Connect);
        let id = ledger.request(first.clone(), now).unwrap();
        ledger.approve(id, consent(), now).unwrap();
        assert!(ledger.take(id, &other, now).is_err());
        assert!(ledger.take(id, &first, now).is_err());
        assert_eq!(ledger.audit().last().unwrap().kind, AuditKind::Rejected);
        let id = ledger.request(first.clone(), now).unwrap();
        assert!(ledger.approve(id, consent(), now + Duration::from_secs(121)).is_err());
        assert!(ledger.take(id, &first, now).is_err());
        assert_eq!(ledger.audit().last().unwrap().kind, AuditKind::Rejected);
    }

    #[test]
    fn invalid_identity_components_are_fail_closed_before_grant_creation() {
        for invalid in [
            "",
            "has space",
            "with\nnewline",
            "with\rreturn",
            "nul\0byte",
            "../hidden",
            "\u{202e}reversed",
            "caf\u{00e9}",
            "with\tseparator",
        ] {
            assert!(
                ToolIntent::from_config(invalid, "filesystem", b"valid", Capability::Connect)
                    .is_err(),
                "unsafe session id was accepted: {invalid:?}"
            );
            for capability in [
                Capability::ReadOnlyTool(invalid.to_owned()),
                Capability::MutatingTool(invalid.to_owned()),
            ] {
                assert!(
                    ToolIntent::from_config("session-1", "filesystem", b"valid", capability)
                        .is_err(),
                    "unsafe tool name was accepted: {invalid:?}"
                );
            }
        }
        assert!(ToolIntent::from_config(
            &"a".repeat(129),
            "filesystem",
            b"valid",
            Capability::Connect
        )
        .is_err());
        assert!(ToolIntent::from_config(
            "session-1",
            "filesystem",
            b"valid",
            Capability::ReadOnlyTool("a".repeat(257)),
        )
        .is_err());
    }

    #[test]
    fn structured_ascii_identifiers_preserve_grant_isolation() {
        let read = ToolIntent::from_config(
            "session:42/file.v1",
            "filesystem",
            b"stdio:validated",
            Capability::ReadOnlyTool("tools/read_file.v2".into()),
        )
        .unwrap();
        let mut book = McpPermissionLedger::default();
        let now = Instant::now();
        let id = book.request(read.clone(), now).unwrap();
        book.approve(id, consent(), now).unwrap();
        assert!(book.take(id, &read, now).is_ok());
        assert!(book.take(id, &read, now).is_err());
    }

    #[test]
    fn rejects_private_metadata_shared_benchmark_and_mixed_dns_answers() {
        let rejects = [
            "0.0.0.0",
            "10.0.0.1",
            "172.16.0.1",
            "192.168.1.1",
            "127.0.0.1",
            "169.254.169.254",
            "100.64.0.1",
            "198.18.0.1",
            "192.0.2.1",
            "198.51.100.2",
            "203.0.113.10",
            "224.0.0.1",
            "255.255.255.255",
            "fc00::1",
            "fe80::1",
            "::1",
            "2001:db8::1",
            "2002::1",
            "::ffff:127.0.0.1",
        ];
        for ip in rejects {
            assert_eq!(
                plan_http_connection("https://mcp.example.com/mcp", &[address(ip)]).err(),
                Some(Denial::ForbiddenTarget),
                "expected to deny {ip}"
            );
        }
        assert!(plan_http_connection(
            "https://mcp.example.com/mcp",
            &[address("8.8.8.8"), address("10.0.0.1")]
        )
        .is_err());
    }

    #[test]
    fn accepts_only_pinned_public_dns_and_enforces_connected_peer() {
        let plan = plan_http_connection(
            "https://mcp.example.com/mcp",
            &[address("8.8.8.8"), address("1.1.1.1")],
        )
        .unwrap();
        assert_eq!(plan.host(), "mcp.example.com");
        assert_eq!(plan.port(), 443);
        assert!(plan
            .verify_connected_peer("8.8.8.8:443".parse().unwrap())
            .is_ok());
        assert_eq!(
            plan.verify_connected_peer("1.1.1.1:8443".parse().unwrap()),
            Err(Denial::UnpinnedPeer)
        );
        assert_eq!(
            plan.accept_redirect("https://other.example.com/mcp"),
            Err(Denial::RedirectNotPermitted)
        );
    }

    #[test]
    fn loopback_cleartext_only_with_literal_local_hostname() {
        for url in [
            "http://localhost:8080/mcp",
            "http://127.0.0.1:8080/mcp",
            "http://[::1]:8080/mcp",
        ] {
            let ips = if url.contains("[::1]") {
                vec![IpAddr::V6(Ipv6Addr::LOCALHOST)]
            } else {
                vec![IpAddr::V4(Ipv4Addr::LOCALHOST)]
            };
            assert!(plan_http_connection(url, &ips).is_ok(), "{url}");
        }
        assert!(plan_http_connection("http://mcp.example.com/mcp", &[address("8.8.8.8")]).is_err());
        assert!(plan_http_connection(
            "https://localhost/mcp",
            &[address("127.0.0.1"), address("8.8.8.8")]
        )
        .is_err());
        assert!(plan_http_connection("https://mcp.example.com/mcp", &[]).is_err());
    }

    #[test]
    fn local_ip_literals_cannot_change_peer_and_port_zero_is_refused() {
        assert_eq!(
            plan_http_connection("http://127.0.0.1:8080/mcp", &[address("127.0.0.2")]).err(),
            Some(Denial::UntrustedDnsAnswer)
        );
        assert_eq!(
            plan_http_connection("http://localhost:0/mcp", &[address("127.0.0.1")]).err(),
            Some(Denial::InvalidEndpoint)
        );
    }

    #[test]
    fn rejects_inline_tokens_and_mismatched_literal_resolution() {
        for url in [
            "https://user:password@mcp.example.com/mcp",
            "https://mcp.example.com/mcp?token=x",
            "https://mcp.example.com/mcp#secret",
            "file:///etc/passwd",
        ] {
            assert!(
                plan_http_connection(url, &[address("8.8.8.8")]).is_err(),
                "{url}"
            );
        }
        assert_eq!(
            plan_http_connection("https://8.8.8.8/mcp", &[address("1.1.1.1")]).err(),
            Some(Denial::UntrustedDnsAnswer)
        );
    }

    #[test]
    fn permission_is_default_deny_one_shot_and_session_scoped() {
        let mut book = McpPermissionLedger::default();
        let now = Instant::now();
        let read = intent("session-a", Capability::ReadOnlyTool("read".into()));
        let other = intent("session-b", Capability::ReadOnlyTool("read".into()));
        let id = book.request(read.clone(), now).unwrap();
        assert_eq!(
            book.take(id, &read, now),
            Err(Denial::ExpiredOrMissingApproval)
        );
        let id = book.request(read.clone(), now).unwrap();
        book.approve(id, consent(), now).unwrap();
        assert_eq!(
            book.take(id, &other, now),
            Err(Denial::ExpiredOrMissingApproval)
        );
        assert_eq!(
            book.take(id, &read, now),
            Err(Denial::ExpiredOrMissingApproval)
        );
        let id = book.request(read.clone(), now).unwrap();
        book.approve(id, consent(), now).unwrap();
        book.take(id, &read, now).unwrap();
        assert!(book.take(id, &read, now).is_err());
    }

    #[test]
    fn mutating_and_discovery_permissions_never_inherit_read_only_grants() {
        let now = Instant::now();
        let read = intent("a", Capability::ReadOnlyTool("inspect".into()));
        let mutation = intent("a", Capability::MutatingTool("delete".into()));
        let mut book = McpPermissionLedger::default();
        let token = book.request(read.clone(), now).unwrap();
        book.approve(token, consent(), now).unwrap();
        assert!(book.take(token, &mutation, now).is_err());
    }

    #[test]
    fn expires_on_timeout_and_rejects_changed_server_configuration() {
        let now = Instant::now();
        let mut book = McpPermissionLedger::default();
        let read = intent("a", Capability::ReadOnlyTool("read".into()));
        let id = book.request(read.clone(), now).unwrap();
        let later = now + Duration::from_secs(121);
        assert_eq!(
            book.approve(id, consent(), later),
            Err(Denial::ExpiredOrMissingApproval)
        );
        let id = book.request(read.clone(), now).unwrap();
        book.approve(id, consent(), now).unwrap();
        let different = ToolIntent::from_config(
            "a",
            "filesystem",
            b"stdio:changed-bin",
            Capability::ReadOnlyTool("read".into()),
        )
        .unwrap();
        assert!(book.take(id, &different, now).is_err());
    }

    #[test]
    fn revoke_session_and_server_invalidates_outstanding_ticket() {
        let now = Instant::now();
        let mut book = McpPermissionLedger::default();
        let a = intent("a", Capability::Connect);
        let b = intent("b", Capability::Connect);
        let first = book.request(a.clone(), now).unwrap();
        let second = book.request(b.clone(), now).unwrap();
        book.approve(first, consent(), now).unwrap();
        book.approve(second, consent(), now).unwrap();
        book.revoke_session("a");
        assert!(book.take(first, &a, now).is_err());
        book.take(second, &b, now).unwrap();
        let third = book.request(b, now).unwrap();
        book.revoke_server("filesystem");
        assert!(book
            .take(third, &intent("b", Capability::Connect), now)
            .is_err());
    }
}
