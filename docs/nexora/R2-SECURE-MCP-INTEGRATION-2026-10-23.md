# R2 MCP — Source implementation and native acceptance boundary

**Date:** 2026-10-23. **Lead:** TEAM_LEAD_SERIAL. **PR:** new follow-up integration PR to develop (PR #26 was already merged at 8f8bd40).
**State:** partial R2 source deliverables implemented, **R2 PHASE EXIT NO-GO**.
No actual network connections, MCP servers, Keychain writes or model API calls
were made outside temporary in-process test fixtures.

## Implemented and source-tested

1. **MCP protocol**, supports legacy `2025-11-25` initialize and current
   `2026-07-28` stateless `server/discover` and per-request `_meta`.
   JSON-RPC id checking, capability-checked `tools/list`, `resources/list`,
   `prompts/list`, output size 128 KiB, at most 64 entries per page,
   256 entries per catalog and 16 pages, cursor cycle/duplicate rejection.
   Untrusted remote error bodies are not echoed.
2. **Vault references**: v1 MCP Registry retains only server-bound
   `vault:v1:<server>:<UUIDv4>` references. No direct passwords/API keys in
   Registry JSON or the browser. The internal `mcp_vault` wrapper uses existing
   OS Keychain/Secret Service/Credential Manager backend and requires an
   unforgeable `VerifiedOperatorConsent` Rust value for create/read/delete.
   This value currently has no production constructor; only unit-test fixtures
   can generate it. Test secret values remained in an in-memory fake store.
3. **HTTP read-only discovery**: `mcp_http` only permits discover/init/list
   calls, pins all DNS answers approved by R2 SSRF policy into reqwest via
   `resolve_to_addrs`, disables proxies and redirects, verifies actual
   connected socket peer, retains host for TLS certificate verification and
   caps response size/time. Denies mutating tools and unexpected SSE content
   until a separate reviewed transport is implemented.
4. **stdio read-only discovery**: `mcp_stdio` requires an approved **absolute**
   executable, rejects shell wrappers via Registry policy, clears inherited
   environment variables, runs in a process group/Windows Job containment,
   bounds single JSON-line output to 128 KiB, uses five-second I/O timeouts
   and reaps child tree on termination. Test processes were short-lived local
   fixtures only; no registered third-party server was started.
5. **Approvals and audit**: existing short-lived, one-use,
   session/server/config/action scoped ledger stays fail-closed. New 256-item
   ring records fixed outcomes with random ticket IDs only (no session IDs,
   credentials, tool arguments, arbitrary agent text or remote errors).
   Expiry, wrong-session, repeated use and revocation tests pass.

## Local test evidence (Mac M4)

- Rust `cargo test --locked --lib --quiet`: **1092 PASS, 0 FAIL,
  33 ignored**. This includes real local loopback HTTP fixture,
  local spawned stdio subprocess, modern/legacy protocol fixtures,
  secure-store test double and consent-audit tests.
- `npm test`: **198 PASS**, including UI model roundtrip, R2 native source
  contracts and no-execution guards.
- `npm run test:release`: **33 PASS**.
- `npm run build`: **PASS** (existing large-chunk warnings only).
- `cargo check --locked --bin nexora-code`: **PASS** on Mac M4.
- Read-only R2 catalog/mock E2E is a backend test. It is **not** a genuine
  external MCP Server, OS keychain, WebView approval or 3-OS GUI E2E.

## P0 blockers to marking R2 complete

- **Trusted operator consent UI**: No reviewed native gate that binds
  real user intent to each session/server/config/action. Preflight
  still denies every live MCP action. `enabled=true` still means ONLY saved
  config state; do not expose read-only test clients to the renderer.
- **Full Credential lifecycle**: Real Mac Keychain + Linux Secret Service +
  Windows Credential Manager permission prompts, rotation, delete, isolation,
  migration and secret-leak tests on installed apps remain to be accepted.
- **Real MCP transport lifecycle**: stdio executable identity/swap protection
  and cancellation supervision under actual servers; Streamable HTTP
  redirect/TLS/DNS-rebinding tests on real networks, SSE handling,
  authenticated requests, initialization/capability revalidation and
  multi-server lifecycle still missing.
- **Tool/resource/prompt Inspector GUI and action executor**: currently only
  internal read-only parsing + saved configuration UI. Tools, resource reads,
  and prompts need explicit per-action native approval and audit. There is no
  production tool invocation exposed.
- **Native platform & security QA**: Real Windows 11/macOS/Linux end-to-end,
  stress, malformed streams, real user approvals, adversarial symlink tests,
  network egress testbed, independent security reviewer, protected develop
  merge and production publisher approval remain outstanding.

## Team Lead decision

**GO** source/test review and GitHub PR submission with exact SHA cross-platform
CI. **NO-GO** R2 phase exit, live MCP execution, protected develop merge,
notarized Production release or any claim that MCP is fully secure.
The intentionally private `VerifiedOperatorConsent` constructor is an
**enforced product limitation** until an independent reviewer approves
the native interactive grant path.
