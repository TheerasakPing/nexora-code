import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const read = path => readFileSync(new URL('../' + path, import.meta.url), 'utf8');

test('read-only MCP HTTP pins IPs, disables proxies/redirects and denies tool invocation', () => {
  const source=read('src-tauri/src/mcp_http.rs');
  for (const requirement of ['.no_proxy()', 'Policy::none()', 'resolve_to_addrs(', 'verify_connected_peer(peer)', 'MAX_RPC_FRAME', 'VerifiedOperatorConsent', 'Mcp-Method']) {
    assert.ok(source.includes(requirement), 'HTTP missing ' + requirement);
  }
  assert.match(source, /if !matches!\(method,/);
  assert.doesNotMatch(source, /#\[tauri::command\]|client\.get\(.*secret/);
});

test('read-only MCP stdio refuses shells, secret environment and unbounded streams', () => {
  const source=read('src-tauri/src/mcp_stdio.rs');
  for (const requirement of ['VerifiedOperatorConsent', '.env_clear()', '.kill_on_drop(true)', '.process_group(0)', 'MAX_RPC_FRAME', 'Duration::from_secs(5)', 'ProcessGroup::register', 'mcp_registry::validate(server)']) {
    assert.ok(source.includes(requirement), 'stdio missing ' + requirement);
  }
  assert.match(source, /if !path\.is_absolute\(\)/);
  assert.doesNotMatch(source, /#\[tauri::command\]/);
});

test('MCP permission audit is bounded and stores only random ticket IDs plus fixed outcomes', () => {
  const source=read('src-tauri/src/mcp_security.rs');
  assert.match(source, /const MAX_AUDIT: usize = 256/);
  assert.match(source, /VecDeque<AuditEvent>/);
  assert.match(source, /self\.record\(id, AuditKind::Rejected\)/);
  assert.match(source, /self\.record\(id, AuditKind::Revoked\)/);
  assert.doesNotMatch(source, /audit\.push_back\(.*intent/);
});
