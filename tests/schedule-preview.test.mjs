import test from 'node:test';
import assert from 'node:assert/strict';
import { schedulePreviewMarkup } from '../src/schedule-preview.ts';

const snapshot = overrides => ({
  revision: 7, maxParallel: 2, activeWorkerIds: ['active'],
  waves: [{ workerIds: ['one', 'two'] }, { workerIds: ['three'] }],
  blocked: [{ workerId: 'blocked', reason: 'Unknown dependency' }],
  note: 'Hypothetical only; no workers started.', ...overrides,
});
const roster = ['active', 'one', 'two', 'three', 'blocked'].map(id => ({ id, name: id.toUpperCase() }));

test('renders active agents, ordered future waves and blocked dependencies', () => {
  const html = schedulePreviewMarkup(snapshot(), roster, 7);
  assert.match(html, /Currently active:<\/strong> ACTIVE/);
  assert.match(html, /Wave 1/);
  assert.match(html, /ONE, TWO/);
  assert.match(html, /Wave 2/);
  assert.match(html, /THREE/);
  assert.match(html, /Unknown dependency/);
  assert.match(html, /Read-only estimate/);
  assert.doesNotMatch(html, /Team changed since/);
});

test('stale snapshot is called out without launching or changing state', () => {
  const data = snapshot();
  const original = structuredClone(data);
  const html = schedulePreviewMarkup(data, roster, 8);
  assert.match(html, /Team changed since this snapshot/);
  assert.deepEqual(data, original);
});

test('escapes untrusted worker names, IDs, reasons and notes', () => {
  const bad = '<img src=x onerror=alert(1)> & \'quoted\'';
  const html = schedulePreviewMarkup(snapshot({
    activeWorkerIds: ['missing<danger>'], waves: [{ workerIds: ['one'] }],
    blocked: [{ workerId: 'blocked', reason: bad }], note: bad,
  }), [{ id: 'one', name: bad }, { id: 'blocked', name: bad }], 7);
  assert.doesNotMatch(html, /<img/);
  assert.doesNotMatch(html, /<danger>/);
  assert.match(html, /&lt;img/);
  assert.match(html, /&lt;danger&gt;/);
  assert.match(html, /&amp;/);
  assert.match(html, /&#39;/);
});

test('empty schedule renders clear zero-state and bounded numerical caption', () => {
  const html = schedulePreviewMarkup(snapshot({
    maxParallel: Number.POSITIVE_INFINITY, activeWorkerIds: [], waves: [], blocked: [],
  }), [], 7);
  assert.match(html, /No active worker/);
  assert.match(html, /No queued tasks are ready/);
  assert.match(html, /No unresolved dependencies/);
  assert.match(html, /up to \? concurrent/);
});

test('unknown worker IDs have safe fallback and no fabricated assignment', () => {
  const html = schedulePreviewMarkup(snapshot({ waves: [{ workerIds: ['unlisted'] }], blocked: [] }), [], 7);
  assert.match(html, /unlisted/);
  assert.doesNotMatch(html, /Unknown dependency/);
});
