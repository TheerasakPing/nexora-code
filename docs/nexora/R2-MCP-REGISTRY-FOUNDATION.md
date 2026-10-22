# R2 Advanced MCP — Registry Foundation

This is a fork of PandaCode: https://github.com/aicodeking/pandacode

This phase does not install, launch, invoke, or reconnect to any MCP server.
It introduces Nexora-specific persisted configuration and strictly typed native
IPC in the trusted main Tauri webview, separate from existing Codex read-only
integration discovery and the original per-run MCP host.

## Delivered

- Inventory storage at nexora-mcp-registry-v1.json in app-specific data directory.
- Schema v1, max 64 named servers, sorted by name.
- Stdio transport (executable and argv), Streamable HTTP transport (URL).
- Tauri IPC: mcp_registry_list, mcp_registry_upsert, mcp_registry_remove,
  mcp_registry_set_enabled.
- Enabled records intent only. It does NOT start a process or grant tool access.
- Strict JSON schema, bounded disk reads/writes, input validation.
- Atomic temporary-file replacement; Unix storage mode 0600.
- Malformed configuration fails safely without being overwritten.
- No OAuth, HTTP headers or environment variables in the stored schema.
- Shell wrappers and obvious inline secrets rejected.
- Synchronized configuration modifications within the application process.
- No IPC access from the launcher or untrusted webviews.

## Server configuration example

{
  "name": "filesystem",
  "description": "Workspace tools",
  "enabled": false,
  "transport": {
    "kind": "stdio",
    "command": "npx",
    "args": ["-y", "@modelcontextprotocol/server-filesystem"]
  }
}

For the configuration editor added in the next source increment, see\n[R2 MCP Center UI](R2-MCP-CENTER-UI.md). It does not start MCP servers.\n\n## Upcoming release blockers

1. MCP Center user interface and import review with explicit confirmation.
2. OS-native credential vault and non-exportable OAuth tokens.
3. stdio/Streamable HTTP protocol client with bounded lifecycle supervisor.
4. DNS/redirect/egress SSRF checks before any network connection.
5. MCP initialize and capability negotiation; Tools/Resources/Prompts discovery.
6. Per-agent permission grants and mutation approval, audit and UNKNOWN recovery.
7. Windows/macOS/Linux native integration and security tests.
8. Cross-process config locking and symlink-swap-resistant file descriptors.

MCP records are currently configuration only, not permission to spawn a process,
send credentials or execute tools. Never expose arbitrary host commands to an
agent without a separate operator approval check.

Test without installation:
cargo test --manifest-path src-tauri/Cargo.toml --locked mcp_registry
