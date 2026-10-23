# R2 MCP Discovery Inspector — source implementation and acceptance boundary

**Date:** 2026-10-23 · **Phase:** R2/P0 · **Status:** source-only, no live MCP connections.

## Implemented and checked

The new internal `mcp_protocol.rs` defines an offline JSON-RPC discovery
state machine for tools/resources/prompts. It keeps modern (2026-07-28)
discovery and legacy (2025-11-25) initialization wire shapes explicit,
and rejects unexpected responses, unsupported versions, unadvertised
capabilities, malformed catalogs, duplicate entries, pagination cursor cycles,
oversized frames, pages or total entries, and remote error payloads without
logging the untrusted remote text. No network, stdio subprocess, token,
credentials or executor authority exists in this module.

**Safety gate:** `mcp_permissions::evaluate()` STILL denies all MCP actions,
even for enabled entries. The existence of this parser MUST NOT be interpreted
as authorization, network access or active tool execution.

## Local verified tests (Mac M4, 2026-10-23)

- `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib mcp_protocol::tests:: --quiet`: 5/5 PASS.
- `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib --quiet`: **1080 PASS, 0 FAIL, 33 ignored**.
- `node --test tests/nexora-r2-protocol-gate.test.mjs`: 2/2 PASS.
- `npm test`: 189/189 PASS.
- `npm run test:release`: 33/33 PASS.
- `npm run build`: PASS (preexisting Vite large-chunk advisory).
- Exact-head GitHub CI on three OS: pending upload/re-run; NOT claimed pass.
- Native MCP live/operator trust boundary E2E: **PENDING / NO-GO**.

## What remains before R2 phase exit

1. OS keychain/Secret Service vault-backed credential references and safe
   migration with no plaintext JSON/argv/renderer/log leakage.
2. Trusted operator consent UI whose secret/review result cannot be forged by
   renderer/agent, with an expiring, one-use authorization token strictly
   checked at the executor boundary.
3. Real supervised stdio MCP client with bounded frames, nonblocking IO,
   cancellation, child-tree cleanup, and trust-scoped environment.
4. Streamable HTTP client with pinned peers, verified original TLS hostname,
   redirects disabled and SSRF/DNS rebinding/timeout tests.
5. Inspector integrated with the authorized transport, diagnostics, safe
   pagination, capability status, per-session scopes and revoke behavior.
6. Installed native macOS M4, Windows 11 x64 and Linux x64 operator approval,
   lifecycle, tool/resource/prompt tests, adversarial security and genuine
   independent review.

**Team Lead decision:** GO for source PR review only, NO-GO for enabling MCP,
protected merge without independent approval, R2 phase acceptance or Production.
