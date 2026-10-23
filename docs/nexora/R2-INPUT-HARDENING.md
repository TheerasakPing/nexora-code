# R2 — MCP Approval Intent Identifier Validation (Source-Only)

**Date:** 2026-10-23. This change strengthens the input boundary for the existing
ephemeral, one-use `ToolIntent` and `McpPermissionLedger`. It does not enable an
MCP runtime, operator approval UI, transport, credential vault, or tool execution.

## Scope

- Session IDs (128-byte maximum) and read-only/mutating MCP tool names
  (256-byte maximum) are validated as nonempty ASCII identifiers.
- Accepted identity characters: `A–Z a–z 0–9 _ - . / :`. No whitespace, control
  bytes, terminal escape sequences, invalid UTF-8 or invisible Unicode markers.
- Server IDs retain the stricter existing lowercase/number/underscore/hyphen
  validator. Server configuration remains bounded to 16 KiB and hashed.
- Tests cover malformed input, length limits, an ordinary structured identifier,
  one-use grant semantics and replay refusal.
- No privileged MCP connection is made. No secrets, process launch, network
  socket or elevated approval is introduced.

## Tradeoff and follow-up

Tool names outside the safe ASCII identifier alphabet now fail closed at the
approval-intent layer. Before enabling real third-party MCP servers, verify the
approved MCP tool-name compatibility contract against actual clients and retain
a per-server configuration fingerprint to prevent mismatched grants.

**Still pending R2:** trusted native operator consent, vault-backed credentials,
stdio/Streamable HTTP connection lifecycle, peer-pinned TLS/SSRF protections at
the actual socket, tools/resources/prompts inspector, cancellation and 3-OS E2E.

**Release assessment:** Source hardening only. R2 native acceptance: NO-GO.
