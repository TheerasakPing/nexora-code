# R2 MCP Security Preflight — IP Literal and Port Validation

**This is a fork of PandaCode** — https://github.com/aicodeking/pandacode.

## Change and rationale

A prior pure network-policy check required DNS/IP-literal agreement for remote
public IP hosts but treated every loopback literal as equivalent. This could
cause an HTTP target configured as 127.0.0.1 to be accepted with a different
loopback address in its proposed resolved peer set. The hardened policy rejects
a different resolved IP for **all** literal hosts, including loopback addresses.
It also rejects port 0 as a non-connectable configured destination.

## Scope and tests

- Adds a focused test rejecting 127.0.0.1 -> 127.0.0.2 substitution.
- Adds a focused test rejecting localhost:0 as a destination.
- Maintains support for localhost resolved to a loopback address, without
  requiring a caller to launch a daemon or connect to a port.
- No sockets, MCP process launches, user-installed software or app GUI.
- No changes to upstream PandaCode provider transport paths.
- This does **not** prove complete SSRF protection. A future client must use
  one of the pinned peer addresses as its socket destination with the original
  TLS hostname, refuse redirects and re-validate when an endpoint changes.
- Secure operator consent is not yet connected to the UI or tool executor;
  tickets are deliberately non-issuable through public IPC.

## Source-only verification

    cargo test --manifest-path src-tauri/Cargo.toml --locked mcp_security
    cargo test --manifest-path src-tauri/Cargo.toml --locked
    cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings

Native Windows and Linux verification remains pending.

## Verification evidence — Mac M4, 22 October 2026

- MCP network/policy focused tests: 9 PASS / 0 FAIL.
- Full Rust unit suite: 1006 PASS / 0 FAIL / 33 ignored.
- Additional CLI integration test: 1 PASS.
- Rust all-target Clippy with denied warnings: PASS.
- No installation, application launch, MCP server launch, HTTP connection,
  GitHub push, deployment or release took place.

The results are for source tests only. UI and 3-platform E2E remain pending.
