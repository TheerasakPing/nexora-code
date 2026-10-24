import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const read = path => readFileSync(new URL('../'+path,import.meta.url),'utf8');
test('R2 MCP protocol parser is bounded and never spawns or uses network/credentials',()=>{
  const code=read('src-tauri/src/mcp_protocol.rs');
  assert.match(code,/pub const MAX_RPC_FRAME: usize = 128 \* 1024/);
  assert.match(code,/const MAX_ENTRIES_TOTAL: usize = 256/);
  assert.match(code,/UnexpectedResponse/);
  assert.match(code,/pub fn accept_discovery/);
  assert.match(code,/pub fn accept_page/);
  assert.doesNotMatch(code,/std::process::Command|TcpStream|reqwest::|tokio::process::|std::env::var/);
});
test('live registry policy still explicitly denies all MCP execution',()=>{
  const policy=read('src-tauri/src/mcp_permissions.rs');
  assert.match(policy,/approval_system_pending/);
  assert.match(policy,/allowed: false/);
  assert.match(read('src-tauri/src/lib.rs'),/mod mcp_protocol;/);
});
