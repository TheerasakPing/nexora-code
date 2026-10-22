import test from 'node:test';
import assert from 'node:assert/strict';
import { parseMcpDraft, asDraft, transportSummary } from '../src/mcp-center-model.ts';

const stdio = { name: 'local_fs', description: 'Local tools', kind: 'stdio', command: 'npx',
  argsJson: '["-y","@example/mcp-server"]', url: '' };

test('stdio config round-trips without running or interpreting shell arguments', () => {
  const config = parseMcpDraft(stdio);
  assert.deepEqual(config.transport, { kind: 'stdio', command: 'npx', args: ['-y', '@example/mcp-server'] });
  assert.deepEqual(parseMcpDraft(asDraft(config)), config);
  assert.match(transportSummary(config), /stdio · npx/);
});

test('new inventory entries are off by default', () => {
  assert.equal(parseMcpDraft(stdio).enabled, false);
  assert.equal(parseMcpDraft(stdio, true).enabled, true);
});

test('invalid names, argument formats and secret flags are refused', () => {
  for (const name of ['../secret', 'BadCase', 'my server', '<img>', '', 'a'.repeat(65)]) {
    assert.throws(() => parseMcpDraft({ ...stdio, name }), /name/);
  }
  for (const argsJson of ['-y @example/mcp-server', '{}', '["x",4]', '["--api-key=secret"]',
                          '["Authorization: Bearer 123"]', '["--password"]']) {
    assert.throws(() => parseMcpDraft({ ...stdio, argsJson }), /Arguments|credentials/);
  }
});

test('stdio shells and traversal are refused before sending configuration to IPC', () => {
  for (const command of ['bash', '/bin/zsh', 'cmd.exe', '../runner', 'tools/../../cli']) {
    assert.throws(() => parseMcpDraft({ ...stdio, command }), /Shell|traverse/);
  }
});

test('remote URL rejects cleartext egress, URL credentials, query and fragments', () => {
  const remote = { ...stdio, kind: 'streamable_http', url: 'https://example.org/mcp' };
  assert.deepEqual(parseMcpDraft(remote).transport, { kind: 'streamable_http', url: 'https://example.org/mcp' });
  for (const url of ['http://169.254.169.254/latest', 'http://api.example.org/mcp',
                     'https://me:key@example.org/mcp', 'https://example.org/mcp?token=x',
                     'https://example.org/mcp#secret', 'file:///etc/passwd']) {
    assert.throws(() => parseMcpDraft({ ...remote, url }), /HTTPS|valid/);
  }
  assert.equal(parseMcpDraft({ ...remote, url: 'http://127.0.0.1:3456/mcp' }).transport.kind, 'streamable_http');
});

test('description control characters are rejected', () => {
  assert.throws(() => parseMcpDraft({ ...stdio, description: 'hello\nworld' }), /Description/);
});
