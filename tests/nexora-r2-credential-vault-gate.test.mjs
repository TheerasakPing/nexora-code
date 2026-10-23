import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const read=path=>readFileSync(new URL('../'+path,import.meta.url),'utf8');

test('MCP OS vault backend is internal and only reachable through a native consent gate',()=>{
  const source=read('src-tauri/src/mcp_vault.rs');
  assert.match(source,/VerifiedOperatorConsent/);
  assert.match(source,/secret_store::set\(SERVICE/);
  assert.match(source,/secret_store::get\(SERVICE/);
  assert.match(source,/secret_store::remove_checked\(SERVICE/);
  assert.doesNotMatch(source,/#\[tauri::command\]|\.arg\(secret\)|\.env\(".*SECRET"/);
  assert.match(read('src-tauri/src/mcp_security.rs'),/struct VerifiedOperatorConsent/);
  assert.match(read('src-tauri/src/mcp_vault_commands.rs'),/confirm_vault_change/);
});
test('registry persists only server-scoped vault references, never secret bytes',()=>{
  const registry=read('src-tauri/src/mcp_registry.rs');
  assert.match(registry,/pub credential_ref: Option<String>/);
  assert.match(registry,/fn valid_credential_ref/);
  assert.match(registry,/Uuid::parse_str/);
  assert.match(read('src-tauri/src/mcp_permissions.rs'),/approval_system_pending/);
});
