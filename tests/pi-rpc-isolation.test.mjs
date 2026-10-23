import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const pi = readFileSync(new URL('../src-tauri/src/providers/pi.rs', import.meta.url), 'utf8');
const bridge = readFileSync(new URL('../src-tauri/src/providers/pi-extension.mjs', import.meta.url), 'utf8');
const provider = readFileSync(new URL('../src-tauri/src/providers.rs', import.meta.url), 'utf8');

test('Nexora embedded Pi opts out of user extensions and native MCP, but keeps its explicit bridge', () => {
  assert.match(pi, /command\.args\(\["--mode", "rpc", "--no-extensions", "--no-mcp"\]\)/);
  assert.match(pi, /\.arg\("--extension"\)\s*\.arg\(path\)/);
  assert.match(pi, /PANDACODE_PI_BRIDGE/);
  assert.match(bridge, /pandacode-bridge/);
  assert.match(bridge, /pi\.on\('tool_call'/);
  // The CLI in Terminal remains unchanged; no package installs or modification.
  assert.doesNotMatch(pi, /\.pi\/agent\/settings\.json|npm\s+(remove|uninstall)/);
});

test('Nexora supplies a specific Pi timeout diagnostic without altering other providers', () => {
  assert.match(provider, /fn initialization_timeout_message\(provider: &str\)/);
  assert.match(provider, /"pi"\s*\{[^}]*Pi RPC initialization timed out/);
  assert.match(provider, /"Agent initialization timed out\. Check the CLI/);
  assert.match(provider, /initialization_timeout_message\(&protocol\.config\.provider\)/);
});
