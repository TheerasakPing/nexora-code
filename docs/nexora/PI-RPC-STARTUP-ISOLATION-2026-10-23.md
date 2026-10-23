# Pi RPC startup timeout on Mac M4 — user extension isolation

**Date:** 2026-10-23. **Scope:** Nexora Code's embedded Pi provider; does not modify the standalone Pi CLI or the user's `~/.pi/agent` package settings.

## Actual reproduction

The user's Mac mini M4/macOS 27.2 has Pi CLI 1.4.0 and dozens of installed user extensions.
Two of them declare Pi-host-provided modules in ordinary `dependencies`
rather than `peerDependencies: {"package":"*"}`:

- `pi-memory-md` 0.1.38 declares `typebox` as a dependency.
- `pi-manage-todo-list` 0.2.0 declares `@mariozechner/pi-ai`,
  `@mariozechner/pi-coding-agent`, `@mariozechner/pi-tui` and
  `@sinclair/typebox` as dependencies.

These warnings concern duplicate runtime-module hazards; they are **warnings,
not definitive proof of a login failure**. They originate in third-party
extension manifests, not in Nexora's own code.

Probes (no user prompt sent, offline, no session, no model inference):

| Launch mode | Pi RPC `get_state` | Outcome |
| --- | --- | --- |
| Normal configured user extensions | no response after 18s, many `extension_ui_request` events; warnings reproduced | TIMEOUT / bounded probe terminated |
| `--no-extensions --no-mcp` (no configured extension discovery) | ~2.5s | PASS |
| `--no-extensions --no-mcp --extension <Nexora bridge>` | `pandacode-bridge: ready` and successful `get_state` in ~7.66s | PASS |

## Fix and security boundary

Nexora launches embedded Pi with `--mode rpc --no-extensions --no-mcp`
and injects its own permission bridge as an **explicit `--extension` path**.
Pi's CLI intentionally continues to load explicitly named extensions when
automatic extension discovery is disabled. Native MCP tool discovery is
disabled in Pi; the app-controlled and approval-protected MCP host still uses
the explicit bridge. User skills and context files remain available.

This prevents unrelated user extensions from blocking RPC initialization or
executing startup behavior outside Nexora's approval boundary, **without
uninstalling, editing or changing user-owned packages**. Running `pi` in
Terminal retains the user's full existing extension configuration and may
still show the extension manifest warnings.

The tradeoff is intentional: **third-party Pi extensions are not automatically
available to agents run inside Nexora Code**. Do not silently fall back to
loading them if the embedded bridge fails. A future UI may provide a
separately reviewed explicit extension allowlist with individual consent.

## Verification and remaining gates

- Exact CLI reproduction on Mac M4 without external API/model calls: PASS.
- New Rust unit coverage asserts explicit bridge, flags and Pi-specific
  timeout diagnostics; CI results are recorded in the PR.
- Pi session/model response requires an authenticated model for a real turn;
  these startup probes deliberately did not prompt models or send user data.
- Mac `Nexora Code.app` must be rebuilt from the fix and tested with a real
  GUI and provider credentials **without exposing them in logs**.
- Existing unsigned Preview is still Gatekeeper-rejected without manual
  owner-specific approval. Production notarization/Developer ID and genuine
  independent review remain outstanding.

**GO:** source-only safe RPC launch and new unsigned engineering preview.
**NO-GO:** fully native/production acceptance or protected develop merge
without genuine reviewer and exact-head CI.

**Remaining master backlog baseline:** 21/24 engineering packages unchecked,
7/7 branding epics unchecked, and 61 detailed R0–R8 checks unchecked.
These views overlap and must not be added.
