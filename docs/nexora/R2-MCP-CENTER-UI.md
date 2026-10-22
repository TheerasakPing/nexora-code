# R2 — MCP Center UI (configuration-only development)

Nexora Code is a fork of PandaCode — https://github.com/aicodeking/pandacode.

This change adds a Nexora-specific configuration management area to
Settings > Tools & integrations while keeping existing Codex MCP discovery
and hosted tools untouched. All operations use typed Tauri commands already
restricted to the main application webview.

## Implemented

- List registry records, transport summary and saved enabled flag.
- Add and edit server entries, remove with explicit confirmation.
- Enable/disable registry configuration only; never launch or stop MCP servers.
- Stdio command + JSON argv form (not free-form shell), Streamable HTTP URL form.
- Fail-closed browser preview (configuration editor disabled without Tauri).
- Validation for server names, bounded arguments, inline secret markers and
  HTTPS versus localhost HTTP; native Rust validation remains authoritative.
- Escaped server names, descriptions and paths; no user content is inserted
  into executable JavaScript/HTML without escaping.
- Explicit live-region status, error feedback and a pending-write guard.
- Existing Codex MCP catalogs/skills/apps continue to display as before.

## Non-goals and blockers

This UI does NOT execute or connect to MCP, test live servers, handle OAuth,
manage provider-specific MCP settings, store secrets, grant tool permissions,
start processes, or update the original PandaCode app. An enabled record
is not equivalent to an active MCP connection or a tool authorization.

Before connection support is considered production-safe, implement the
OS credential vault, operator-approved server launch, network SSRF protections
including DNS and redirects, resource budgets, isolation/permissions,
atomic durable journal, cross-process configuration locking, and
Windows/Linux native acceptance.

## Verification commands

- node --test tests/mcp-center-model.test.mjs
- npm run build
- npm test
- npm run test:nexora:r0
- cargo test --manifest-path src-tauri/Cargo.toml --locked
- cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings

These are source/build checks only. No GUI installation or execution occurred.

## Local source acceptance — 22 October 2026

- MCP center model validation: 6/6 PASS in Node 26 on Apple M4.
- Full Node unit suite: 110/110 PASS.
- Release-script suite: 31/31 PASS.
- Rust test suite: 993 PASS, 0 FAIL, 33 ignored; CLI integration 1 PASS.
- Rust all-target Clippy with -D warnings: PASS.
- TypeScript type-check and Vite production build: PASS.
- R0 trust/identity/attribution verifier: PASS.
- No install, app launch, remote MCP connection or GitHub push.

UI acceptance on real native desktops and Windows/Linux validation remain pending.
