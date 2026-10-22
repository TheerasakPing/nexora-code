import test from 'node:test';
import assert from 'node:assert/strict';
import { validatePlanner, validateChangedFiles } from '../scripts/nexora/governance-pr-gate.mjs';

const path = 'docs/nexora/tasks/2026-10-22-demo-request.md';
const good = [
  '# Task Checklist — demo',
  '- **Team Lead:** Acting Team Lead',
  '- **Worker:** TEAM_LEAD_SERIAL (no external worker)',
  '- **Acceptance:** Diff and tests reviewed',
  '- [x] Created a new task plan',
  '- [x] Checked code and tests',
  '- [x] Reported genuine evidence',
  '',
  '## Evidence and review gate',
  '**GO / NO-GO** review completed.',
  '**Team Lead verdict:** GO for PR submission; protected main merge pending.',
  '',
].join('\n');

test('accepts a newly added completed per-request checklist', () => {
  assert.deepEqual(validatePlanner(path, good), []);
  assert.deepEqual(validateChangedFiles([{ status: 'A', path }], () => good).failures, []);
});

test('rejects PRs with no newly added task planner', () => {
  const result = validateChangedFiles([
    { status: 'M', path: 'src/app.ts' },
    { status: 'M', path },
    { status: 'A', path: 'docs/nexora/templates/other.md' },
  ]);
  assert.equal(result.planners.length, 0);
  assert.equal(result.failures.length, 1);
});

test('rejects incomplete and placeholder-only checklist', () => {
  const result = validateChangedFiles([{ status: 'A', path }], () =>
    good.replace('**Team Lead verdict:** GO', '**Team Lead verdict:** PENDING'));
  assert.match(result.failures.join('\n'), /record GO/);
  assert.ok(validatePlanner(path, '# Task Checklist\n').length >= 4);
});

test('enforces safe task path, worker assignment and task evidence', () => {
  assert.match(validatePlanner('../tasks/escape.md', good).join('\n'), /path/);
  assert.match(validatePlanner(path, good.replace('TEAM_LEAD_SERIAL', 'nobody')).join('\n'), /worker assignment/i);
  assert.match(validatePlanner(path, good.replaceAll('- [x]', 'not checked')).join('\n'), /checkable/);
});

test('rejects unexpectedly large task planner files', () => {
  const oversized = good + 'a'.repeat(260 * 1024);
  assert.match(validatePlanner(path, oversized).join('\n'), /256 KiB/);
});

test('fails closed if a new task plan cannot be read', () => {
  const result = validateChangedFiles([{ status: 'A', path }], () => { throw new Error('ENOENT'); });
  assert.match(result.failures.join('\n'), /unreadable/);
});
