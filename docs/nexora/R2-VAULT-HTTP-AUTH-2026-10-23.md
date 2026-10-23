# R2 — OS Vault, Credential Rotation and Authenticated HTTP MCP

**Date:** 2026-10-23
**Scope:** Source engineering implementation and local acceptance only. Not production signoff.
**Team Lead:** ChatGPT Integration Lead — TEAM_LEAD_SERIAL. No independent reviewer assigned.

## Available in the new source

- MCP Center provides Store token, Rotate token, Revoke token for Streamable HTTP servers only. Tokens enter a password field, pass once through local trusted Tauri IPC, and are stored in OS secure storage after native approval. The password DOM input is cleared. Tokens are not returned to the webview, stored in Registry JSON, injected into stdio argv, or logged. A token temporarily exists in renderer memory during entry and IPC; this is not a native OS password input.
- Every change has an Approve Once / Deny OS-native confirmation showing server, endpoint and the hash of its full saved configuration. A concurrent MCP operation lock prevents overlapping approvals. Main-window requests only.
- After confirmation, the native worker rechecks saved Registry configuration, creates an opaque vault:v1 reference, and uses a full configuration compare-and-swap under the Registry lock to prevent URL swaps. HTTP servers with attached tokens cannot change endpoint or be removed without revocation. Rotation unlinks the old token and reports OS cleanup failures. Revoke checks secure-store deletion error rather than suppressing it.
- Authenticated read-only catalog and manual tools/call resolve a server-bound OS vault reference only after one-use native operator approval. Bearer header is marked sensitive; no proxies, no redirects, DNS/peer restrictions and verified TLS hostname still apply. No token is passed to stdio.
- Autonomous Agent MCP Tool Execution remains disabled. Enabled config is not authorization.

## Local Mac M4 test evidence

| Test | Result |
| --- | --- |
| Rust library | 1111 PASS, 0 FAIL, 33 ignored |
| Registry validation and URL-swap CAS | 14/14 PASS |
| Vault lifecycle (in-memory OS secure-store double) | 2/2 PASS |
| HTTP token/peer redaction fixtures | 6/6 PASS |
| Complete vault ref to authenticated localhost Inspector | PASS |
| Browser-mocked MCP Center vault rotation/revoke and no token display | 5/5 PASS |
| Node full suite | 208 PASS |
| Release contracts | 33 PASS |
| Vite and Mac native Cargo check | PASS |

All secrets were synthetic test fixtures; no real user OS vault or external MCP server was accessed. Browser-mocked tests do not test clicking actual OS-native dialogs.

## Remaining R2 final acceptance blockers

- Real installed macOS/Windows/Linux Native Approval allow/deny/cancel E2E.
- OS Keychain, Windows Credential Manager and Linux Secret Service permission denied/locked-store/recovery/rotation tests using actual OS accounts.
- Independent security review and adversarial tests: executable/symlink swap, DNS rebinding, TLS hostname/peer pinning, malformed SSE/session lifecycle, prompt/tool/secret leaks, window spoofing.
- Actual native multi-server GUI tests, signed Windows Authenticode and notarized Mac release, user/owner acceptance.
- Autonomous Agent MCP execution requires separately approved scoped grants; remains disabled intentionally.

**Team Lead GO:** Source PR submission.
**R2 Phase Exit:** NO-GO until genuine independent security review and installed native platform acceptance. Source completion is separate from release acceptance; CI can finish without blocking development.
