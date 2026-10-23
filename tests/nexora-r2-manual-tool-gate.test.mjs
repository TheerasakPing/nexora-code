import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';

const read=(f)=>readFileSync(new URL('../'+f,import.meta.url),'utf8');

test('only manual per-action and exact-config native approval creates tool authorization',()=>{
  const inspector=read('src-tauri/src/mcp_inspector.rs');
  const security=read('src-tauri/src/mcp_security.rs');
  const core=read('src-tauri/src/lib.rs');
  for(const marker of [
    'window.label() != "main"',
    'validate_manual_tool_input',
    'mcp_security::confirm_tool_execution',
    'approved_config_unchanged',
    'McpPermissionLedger::default',
    'tickets.request(',
    'tickets.approve(',
    'tickets.take_operator_consent(',
    'perform_approved_tool',
    'Requested MCP tool was not advertised'
  ]) assert.ok(inspector.includes(marker),marker);
  assert.ok(security.includes('Run Once'));
  assert.ok(security.includes('Arguments (complete)'));
  assert.ok(core.includes('mcp_inspector::mcp_execute_tool_once'));
  assert.ok(!core.includes('mcp_vault::resolve,'));
});

test('MCP transports cannot dispatch generic arbitrary method or an unbounded result',()=>{
  const stdio=read('src-tauri/src/mcp_stdio.rs');
  const http=read('src-tauri/src/mcp_http.rs');
  for (const source of [stdio,http]) {
    assert.ok(source.includes('invoke_approved_tool'));
    assert.ok(source.includes('"tools/call"'));
    assert.ok(source.includes('MAX_RPC_FRAME'));
    assert.ok(source.includes('9471'));
    assert.ok(source.includes('result'));
  }
  assert.ok(stdio.includes('read_bounded_frame'));
  assert.ok(http.includes('verify_connected_peer(peer)'));
  assert.ok(http.includes('.no_proxy()'));
  assert.ok(http.includes('Policy::none()'));
});

test('MCP UI keeps disabled and vault-backed servers uncallable, renders returned text inert',()=>{
  const ui=read('src/mcp-center.ts');
  for (const marker of [
    "action === 'run-tool'",
    'data-nexora-tool-args',
    'mcp_execute_tool_once',
    'native approval',
    'JSON.parse(json)',
    'output.textContent',
    'inspected.tools.has(toolName)',
    'target.credentialRef'
  ]) assert.ok(ui.includes(marker),marker);
});
