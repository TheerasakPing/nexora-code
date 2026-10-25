import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const read = file => readFileSync(new URL('../'+file,import.meta.url),'utf8');

test('Native vault provisioning has explicit consent, main window and config recheck',()=>{
  const source=read('src-tauri/src/mcp_vault_commands.rs');
  const security=read('src-tauri/src/mcp_security.rs');
  for (const required of [
    'window.label() != "main"', 'try_lock_owned()', 'confirm_vault_change',
    'approved_config_unchanged', 'mcp_vault::create', 'mcp_vault::delete',
    'registry.set_credential_ref', 'cleanup_warning'
  ]) assert.ok(source.includes(required),required);
  assert.match(security,/fn confirm_vault_change/);
  assert.match(security,/Approve Once/);
  assert.match(security,/Deny/);
  assert.doesNotMatch(source,/println!\([^)]*secret|eprintln!\([^)]*secret/);
});

test('Server endpoint cannot be swapped or removed until vault credential is revoked',()=>{
  const source=read('src-tauri/src/mcp_registry.rs');
  assert.match(source,/server != approved/);
  assert.match(source,/Revoke the MCP credential before changing its HTTP endpoint/);
  assert.match(source,/Revoke the MCP vault credential before removing this server/);
  assert.match(source,/only supported for HTTP MCP/);
});

test('HTTP Bearer credentials remain sensitive and are not passed to a renderer or stdio process',()=>{
  const source=read('src-tauri/src/mcp_http.rs');
  const inspector=read('src-tauri/src/mcp_inspector.rs');
  const vault=read('src-tauri/src/mcp_vault.rs');
  assert.match(source,/set_sensitive\(true\)/);
  assert.match(source,/header::AUTHORIZATION/);
  assert.match(source,/\.no_proxy\(\)/);
  assert.match(source,/Policy::none\(\)/);
  assert.match(inspector,/mcp_vault::resolve/);
  assert.match(vault,/secret_store::get/);
  assert.doesNotMatch(read('src-tauri/src/lib.rs'),/mcp_vault::resolve/);
});

test('Browser password value is cleared and never rendered as a credential ref',()=>{
  const ui=read('src/mcp-center.ts');
  for (const item of [
    'type="password"', 'autocomplete="new-password"', 'vaultSecret.value =',
    'mcp_vault_change', 'Store token', 'Rotate token', 'Revoke token'
  ]) assert.ok(ui.includes(item),item);
  assert.doesNotMatch(ui,/innerHTML\s*=\s*secret/);
});
