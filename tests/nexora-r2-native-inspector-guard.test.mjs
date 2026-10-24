import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const read = file => readFileSync(new URL('../'+file,import.meta.url),'utf8');

test('MCP native inspection cannot be auto-approved by renderer input', () => {
  const source = read('src-tauri/src/mcp_inspector.rs');
  const permissions = read('src-tauri/src/mcp_security.rs');
  for(const requirement of [
    'window.label() != "main"',
    'confirm_catalog_inspection',
    'try_lock_owned()',
    'approved_config_unchanged',
    'server.credential_ref.is_some()',
    'server.enabled',
    'ProtocolFailure::CapabilityUnavailable'
  ]) assert.ok(source.includes(requirement), requirement);
  assert.match(permissions,/MessageDialogButtons::OkCancelCustom/);
  assert.match(permissions,/\.blocking_show\(\)/);
  assert.match(permissions,/if approved \{/);
  assert.doesNotMatch(source,/pub (?:fn|async fn).*tools_call/);
});

test('Inspector UI escapes remote catalog data and leaves operations disabled',()=>{
  const ui=read('src/mcp-center.ts');
  assert.match(ui,/escape\(item\.label\)/);
  assert.match(ui,/escape\(item\.name\)/);
  assert.match(ui,/item\.kind === kind/);
  assert.match(ui,/mcp_inspect_catalog/);
  assert.match(ui,/inspected_disconnect/);
  assert.doesNotMatch(ui,/data-nexora-mcp="invoke-tool"|call\('mcp_tool_call'/);
});

test('Tauri inspector command registered and scoped to main; vault remains internal',()=>{
  const core=read('src-tauri/src/lib.rs');
  assert.match(core,/mcp_inspector::mcp_inspect_catalog/);
  assert.match(core,/mcp_inspector::InspectionGate::default/);
  assert.match(core,/mcp_permissions::mcp_policy_preflight/);
  assert.doesNotMatch(core,/mcp_vault::create|mcp_vault::resolve/);
});
