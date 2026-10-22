import test from 'node:test';
import assert from 'node:assert/strict';
import { isolationAuditMarkup } from '../src/isolation-audit.ts';

const sample = overrides => ({
  revision: 10, isolatedCount: 1, sharedCount: 1, unverifiedCount: 0,
  workers: [
    { workerId: 'worker-one', name: 'Writer', classification: 'shared_lead',
      checkoutPath: '/workspace/team', reason: 'Shared Team Lead checkout.' },
    { workerId: 'worker-two', name: 'Agent', classification: 'verified_linked_worktree',
      checkoutPath: '/workspace/agent', reason: 'Registered worktree, not a sandbox.' },
  ],
  note: 'No workers were moved or started.', ...overrides,
});

test('shows actual verified vs shared classification with safe labels', () => {
  const html = isolationAuditMarkup(sample(), 10);
  assert.match(html, /1 verified · 1 shared · 0 unverified/);
  assert.match(html, /Shared with Team Lead/);
  assert.match(html, /Registered Git worktree/);
  assert.match(html, /not a sandbox/);
  assert.doesNotMatch(html, /Team changed/);
});
test('stale revisions must trigger a visible audit refresh warning', () => {
  assert.match(isolationAuditMarkup(sample(), 11), /Team changed since this audit/);
});
test('all worker strings and paths are escaped against HTML injection', () => {
  const bad = '<img src=x onerror=alert(1)> " & \'/';
  const html = isolationAuditMarkup(sample({
    workers: [{ workerId: bad, name: bad, classification: bad, reason: bad, checkoutPath: bad }],
    note: bad,
  }), 10);
  assert.doesNotMatch(html, /<img/);
  assert.match(html, /&lt;img/);
  assert.match(html, /&quot;/);
  assert.match(html, /&#39;/);
  assert.match(html, /&amp;/);
  assert.doesNotMatch(html, /Unknown classification.*<img/);
});
test('empty result and unknown status are honestly displayed', () => {
  const empty = isolationAuditMarkup(sample({ workers: [], isolatedCount: 0, sharedCount: 0 }), 10);
  assert.match(empty, /No workers to inspect/);
  const unknown = isolationAuditMarkup(sample({
    workers: [{ workerId: 'a', name: 'A', checkoutPath: null, classification: 'unverified', reason: 'Missing' }],
  }), 10);
  assert.match(unknown, /Not verified/);
  assert.match(unknown, /Missing/);
});
test('malformed counters are never rendered as numeric HTML content', () => {
  const html = isolationAuditMarkup(sample({ isolatedCount: NaN, sharedCount: -5 }), 10);
  assert.match(html, /\? verified · \? shared/);
});
