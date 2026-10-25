# R2 MCP Native Inspector — source delivery, security review and acceptance

**Date:** 23 October 2026. **Branch/PR:** PR #32 to `develop`.
**Lead:** ChatGPT Integration Lead, `TEAM_LEAD_SERIAL` (no real delegated
engineering agents or independent security reviewer assigned).
**Decision:** Source/test implementation **GO for PR review** only.
**Full R2 phase exit:** **NO-GO**, see blockers below.

## Implemented in this R2 package

- A native **one-time confirmation dialog** ("Approve Once" / "Deny") for each
  catalog-inspection attempt. Approval originates in Rust's OS dialog helper;
  the JavaScript caller cannot provide, deserialize or reuse a consent token.
  Dialog states server name, transport, configured argument count and SHA-256
  of persisted server configuration. The command accepts only persisted server
  name and supported protocol version; checks trusted main window, enabled
  state, *no credential-backed server*, and exactly one simultaneous dialog.
- Before any spawn/network request, the backend reloads the MCP Registry and
  checks that the **full server configuration equals what was approved**.
  Changed transport/port, disabled server or removed server fails closed.
- An ephemeral, **read-only** inspector runs MCP discovery plus capability-
  scoped and bounded `tools/list`, `resources/list`, `prompts/list`, then
  disconnects. It never executes `tools/call`, reads resources, resolves vault
  secrets or stores a background MCP session.
- Supports modern MCP `2026-07-28` stateless `server/discover` over pinned-
  peer Streamable HTTP and stdio. Stdio also supports legacy `2025-11-25`
  `initialize` and `notifications/initialized`. Legacy stateful HTTP is
  **NOT** enabled pending a separately audited session transport.
- HTTP accepts bounded JSON or SSE `event: message` containing an exact
  matching JSON-RPC response, with LF/CRLF handling and nonlossy UTF-8.
  Unexpected events, IDs, oversized input and malformed JSON fail closed.
  Client denies proxy and redirect behavior, validates DNS/IP targets and
  connected peer, and uses TLS hostname verification.
- MCP Center adds **Inspect catalogs** and (stdio-only) **Inspect legacy**
  actions. Disabled and credential-backed connections cannot be inspected.
  Remote-provided labels are escaped, rendered as inert text and cannot
  generate tool buttons or HTML/JavaScript execution.
- Registry `enabled` is still **only a saved config flag**. Existing generic
  MCP policy preflight denies all executable actions.

## Local Mac M4 test triad — evidence

| Evidence | Command / behavior | Result |
| --- | --- | --- |
| Rust unit/contract | `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib --quiet` | **1100 PASS / 0 failed / 33 ignored** |
| Rust scoped E2E | `cargo test --lib mcp_inspector::tests::`: temporary executable fixture answers actual stdio initialize + catalogs; local pinned HTTP test answers three catalogs; changed config and concurrent dialog gate checked | **6 PASS** |
| HTTP SSE | `cargo test --lib mcp_http::tests::` local loopback JSON/SSE, malformed and CRLF cases | **5 PASS** |
| Browser mock E2E | `npx playwright test tests/mcp-inspector.spec.ts --workers=1`: inspector UI, legacy option, disabled/vault blocks, malicious HTML inert | **2 PASS** |
| Source security guards | `node --test tests/nexora-r2-native-inspector-guard.test.mjs` | **3 PASS** |
| Full Node unit | `npm test` | **201 PASS** |
| Release contract | `npm run test:release` | **33 PASS** |
| Frontend and macOS Rust smoke | `npm run build` and `cargo check --locked --bin nexora-code` | **PASS** |
| GitHub exact-head CI | PR #32 checks Mac/Windows/Linux on final source SHA | **PENDING** |

All local HTTP tests use loopback `127.0.0.1`, and stdio tests use disposable
executable fixtures in a temporary directory. No real OS Keychain secret,
external MCP Server, subscription model, user's installed app or personal
workspace was accessed. Playwright injects a **simulated** Tauri IPC surface:
this is **browser-mocked UI E2E, not native desktop UI acceptance**.

## Operator workflow (preview, after installing reviewed build)

1. In Nexora Code → Integrations → Nexora MCP Registry, add a server.
   Modern Streamable HTTP requires HTTPS for public hosts; stdio inspection
   requires a trusted **absolute executable path**. Generic `npx` resolved
   from PATH will be rejected at runtime for security.
2. Enable the saved registry entry (does **not** connect or run anything).
3. Select **Inspect catalogs**. Check the native dialog's exact server,
   transport, argument count and SHA-256 before selecting **Approve Once**.
   Cancel/Deny should leave the server untouched and show a failure status.
4. After the scoped connection ends, inspect the read-only Tools, Resources
   and Prompts lists. There are no tool-execution buttons.
5. For an older trusted stdio server use **Inspect legacy**.
   Credentials stored as `credentialRef` remain unsupported for live use;
   those entries cannot connect until the native Keychain workflow is reviewed.

## Remaining P0/P1 R2 phase-exit blockers (must not be waived)

1. **Native manual acceptance:** Real installed app testing of the native
   dialog, cancellation, spoofing/accessibility, window focus and permission
   denial on macOS, Windows 11 and Linux. The Rust fixture never clicks
   a real OS dialog. No independent security reviewer yet.
2. **Tool/action executor:** Per-call operator approval with typed tool
   argument preview and intent-bound, expiring **single-use authorization at
   the actual `tools/call` / resource-read boundary**. Full tool dispatch is
   intentionally still disabled; read-only Inspector approval cannot grant it.
3. **Credential lifecycle:** Native secure-store set/get/delete/rotation,
   permission-denied and secret-leak tests across Keychain, Credential Manager
   and Secret Service, trusted UI integration and request header injection
   controls. Existing code only uses mocks for these tests.
4. **stdio hardening:** Trusted executable identity across symlink/rename
   swaps, native OS sandbox/Job/Object ownership and cancellation under real
   adversarial third-party MCP servers. An absolute path plus one dialog
   does not prove a stable executable binary identity.
5. **Streamable HTTP statefulness:** Real HTTPS/TLS CA/hostname, DNS rebinding
   and peer-pinning testbed, adversarial SSE, server sessions/reconnect/
   cancellation, reliable bounded retry policy, authenticated multi-server
   connection/disconnection, and legacy HTTP session negotiation.
6. **Acceptance governance:** 3-OS installed native MCP E2E/smoke, dedicated
   independent review, protected branch merge, R1 earlier release prerequisite
   and Production gate if applicable. CI compile alone cannot satisfy this.

**Security policy:** Do not enable automatic MCP tool execution or claim
"R2 completed" while any listed production security gate is unaccepted.
