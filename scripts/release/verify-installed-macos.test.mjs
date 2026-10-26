import test from 'node:test';
import assert from 'node:assert/strict';
import { classifyAcceptance } from './verify-installed-macos.mjs';

const base = { bundleExists: true, bundleId: 'com.theerasakping.nexoracode', architecture: 'Mach-O 64-bit executable arm64', processRunning: true, sqliteIntegrity: 'ok', signature: 'adhoc', notarized: false, accessibilityAutomation: 'denied' };

test('local runtime can pass while unsigned distribution and human acceptance are blocked', () => {
  const state = classifyAcceptance(base);
  assert.equal(state.localSmokePass, true);
  assert.equal(state.productionReady, false);
  assert.equal(state.gates.installedRuntime, 'PASS');
  assert.equal(state.gates.developerIdSignature, 'BLOCKED');
  assert.equal(state.gates.notarization, 'BLOCKED');
  assert.equal(state.gates.interactiveNativeGui, 'PENDING_MANUAL_ACCEPTANCE');
  assert.equal(state.gates.independentHumanReview, 'PENDING_HUMAN_REVIEW');
});

test('wrong app identifier cannot pass a local smoke', () => {
  const state = classifyAcceptance({ ...base, bundleId: 'com.example.bad' });
  assert.equal(state.localSmokePass, false);
  assert.match(state.failures.join(';'), /identifier/);
});

test('database corrupt, wrong architecture or dead process rejects local runtime', () => {
  for (const change of [{ sqliteIntegrity: 'corrupt' }, { architecture: 'x86_64' }, { processRunning: false }, { bundleExists: false }]) {
    const state = classifyAcceptance({ ...base, ...change });
    assert.equal(state.localSmokePass, false);
  }
});

test('signing/notarization alone never implies human or interactive release acceptance', () => {
  const state = classifyAcceptance({ ...base, signature: 'developer-id', notarized: true });
  assert.equal(state.gates.developerIdSignature, 'PASS');
  assert.equal(state.gates.notarization, 'PASS');
  assert.equal(state.productionReady, false);
});


test('denied accessibility is separate from passing installed-runtime smoke', () => {
  const state = classifyAcceptance(base);
  assert.equal(state.localSmokePass, true);
  assert.equal(state.gates.runnerAccessibility, 'BLOCKED_ACCESSIBILITY');
  assert.equal(state.gates.interactiveNativeGui, 'PENDING_MANUAL_ACCEPTANCE');
  assert.equal(state.productionReady, false);
});

test('granted accessibility is not proof of native GUI interaction or production release', () => {
  const state = classifyAcceptance({ ...base, accessibilityAutomation: 'granted' });
  assert.equal(state.gates.runnerAccessibility, 'AVAILABLE');
  assert.equal(state.gates.interactiveNativeGui, 'PENDING_MANUAL_ACCEPTANCE');
  assert.equal(state.productionReady, false);
});

test('unknown accessibility is not misreported as granted', () => {
  const state = classifyAcceptance({ ...base, accessibilityAutomation: 'unknown' });
  assert.equal(state.gates.runnerAccessibility, 'UNVERIFIED');
  assert.equal(state.productionReady, false);
});
