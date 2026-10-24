# R2 — Manual MCP Tool Execution, Native Approval and Release Limits

**Date:** 2026-10-23 · **Status:** Source feature/test GO, **R2 phase exit NO-GO** until live installed GUI, credentials and independent security review.
**Worktree/PR:** Nexora Code PR #32, follow-up to read-only Inspector.
**Operator authorization:** User explicitly requested enabling tools for R2 acceptance tests.
**Lead:** ChatGPT Integration Lead, TEAM_LEAD_SERIAL; no independent reviewer or engineer worker has been delegated.

## User workflow in the newly built Nexora Code

1. Open **Integrations → Nexora MCP Registry**. Add an MCP Server with an absolute, reviewed executable path (stdio) or an HTTPS URL / local loopback HTTP. It must be enabled and must NOT have a vault-backed `credentialRef` yet.
2. Select **Inspect catalogs** (or **Inspect legacy** for an older stdio server). Confirm the separate **native OS read-only catalog approval**; the UI lists tools/resources/prompts as *inert text*, with no automatic tool use.
3. Under a listed **Tool**, enter an explicit bounded JSON object in the **JSON arguments** field (default `{}`) and click **Run Once — native approval required**.
4. Read the *native OS tool approval* before clicking **Run Once**. It displays the exact server, transport/command, tool and **all** arguments; it warns that tools can modify files, execute commands or send data. Deny/close → nothing is called.
5. After approval, native Rust rechecks the saved Registry configuration and issues+consumes a single-use intent ticket bound to that config and the hash of the exact tool name+JSON arguments. The runtime reconnects ephemerally, performs protocol discovery, validates this tool is advertised by the current connected server, sends only the exact `tools/call` with bounded 128 KiB response and I/O deadline, then disconnects.
6. The UI prints only a truncated, escaped/textContent representation of the returned JSON and indicates when the Tool reports `isError`. Each subsequent call repeats the native approval; **Enable server is NOT a blanket grant**.

**What is NOT enabled:** Automatic Agent tool calling, credential-backed live connections, unsigned/unreviewed executable trust, arbitrary MCP method execution, generic resource reads and background MCP sessions. This is an operator-driven testing feature, not production-ready unattended execution. Main app and user's installed app were not changed by source development.

## Security boundaries and evidence

- Renderer cannot mint, deserialize or reuse a `VerifiedOperatorConsent`; only a trusted native Tauri dialog returns it.
- The executable Rust call holds the same single-concurrent-operation gate used by the Inspector, compares persisted configuration after dialog approval, then issues+consumes a `McpPermissionLedger` ticket. Ticket ID never leaves Rust; approval is one-shot even on rejection/failure.
- Tool name must be within 128 UTF-8 bytes with no control code; argument input is a JSON object <=2,048 bytes so the native OS dialog can display it without truncation; remote results are capped at 128 KiB.
- Mac M4 local check: `cargo test --locked --lib --quiet` **1,104 PASS / 0 fail / 33 ignored**, `npm test` **204 PASS**, release contracts **33 PASS**, mocked browser Playwright **4 PASS**, frontend Build PASS, `cargo check --bin nexora-code` PASS.
- Real local mock stdio test forks a disposable MCP script, verifies a `tools/call` changes ONLY a temporary test marker and rejects unadvertised tools. Real loopback HTTP test verifies the matching `server/discover`, `tools/list`, `tools/call` sequence and response ID. Repeated/use-wrong-ticket tests reject.
- Existing HTTP security: peer-pinned destination, no proxies/redirects, TLS hostname preserved, bounded JSON/SSE parsing, timeout and no credentials. Existing stdio policy: clean env, absolute executable, process group cleanup. These source controls still need hostile native/executable identity review.
- **Frontend mocked Tauri browser E2E is not native OS dialog E2E**. Unit/contract/functional tests cannot substitute for a real human clicking Allow/Deny on an installed app.

## Required final R2 acceptance still pending

- Real native UI approve/deny under macOS ARM64, Windows 11 x64 and Linux x64; verify denial leaves no process/socket, recheck re-approval, timeout/cancel and accessiblity/window spoofing.
- Real OS Keychain/Credential Manager/Secret Service user-approved credential create/resolve/rotate/delete, authenticated request isolation and leak testing (new live tool feature intentionally denies credential-backed configs).
- Peer review of executable identity under symlink/rename swaps, hostile servers, tool schemas, SSE session/reconnect, HTTP TLS/DNS changes, unexpected tool effects and cancellation.
- Per-tool integration with autonomous **Agent** sessions only after a real action-scoped consent/revocation model, not implicit trust from server Enabled state.
- Current-head GitHub 3-OS CI and unsigned engineering installer artifacts. This is not a notarized/signed production release.
- Actual independent reviewer and QA sign-off before protected `develop` merge and R2 exit GO.

## Linux CI regression correction in same PR

The older native provider subprocess regression occasionally returned the valid
`Broken pipe` terminal status before the background-descendant exit watchdog
message. Both paths perform owned-process-group cleanup. The test now accepts
either correctly bounded early-termination diagnostic **but continues to check
that the background child no longer runs**. Linux exact-head CI still required.

**GO:** Source feature tested and opt-in testing capability available in the new code.
**NO-GO:** R2 final acceptance/production/automatic MCP agent use pending independent reviewer and installed OS validation.
