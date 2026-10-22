# R2 — MCP Permission Preflight

Nexora Code is a fork of PandaCode — https://github.com/aicodeking/pandacode.

## Delivered

- New Rust MCP permission policy module.
- Typed request actions: connect, list tools/resources/prompts, invoke tool,
  read resource and get prompt.
- Native Tauri preflight API available only to the trusted main webview.
- Three failure states: not_found, disabled, approval_system_pending.
- Every policy decision denies execution and explicitly requires approval.
- Registry's enabled flag means saved preference only; it cannot permit
  connections, execute tool calls, or bypass approvals.
- Security regression tests check every action and reject unknown action names.

## Deliberate safeguards

This is a source-only fail-closed policy foundation. It does not add networking,
server spawning, OAuth, executable permissions or a tool executor. No server
was installed or connected during development.

The existing upstream Codex-managed MCP feature remains separate and retains
its own provider-specific policy. The new Nexora registry and preflight API do
not claim to intercept or authorize Codex's existing MCP tool calls.

## Next engineering gates

1. Operator-approved capabilities scoped to project, session, server and tool.
2. Expiring grants stored securely without dumping secrets into app settings.
3. Immutable audit log and reviewable launch/tool-use requests.
4. No-replay handling for any uncertain mutating tool response.
5. Deny-by-default native executor that accepts only validated, scoped grants.
6. DNS/rebinding/redirect SSRF protections before enabling HTTP transport.
7. Full three-platform contract and native approval UI tests.

No production security claim is justified until the above capabilities exist.
