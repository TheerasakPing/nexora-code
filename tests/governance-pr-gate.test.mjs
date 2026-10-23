import test from 'node:test';
import assert from 'node:assert/strict';
import { validatePlanner, validateSubtaskMatrix, validateChangedFiles, validatePullRequestBody } from '../scripts/nexora/governance-pr-gate.mjs';

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
  '## G. NC-TEAM-003 — Subtask Verification Matrix',
  '| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |',
  '| --- | --- | --- | --- | --- | --- | --- |',
  '| T-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | 2026-10-23 macOS; node --test tests/example.test.mjs; smoke command; source SHA abcdef1 |',
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
  assert.match(validatePlanner(path, good.replaceAll('TEAM_LEAD_SERIAL', 'UNASSIGNED')).join('\n'), /worker assignment/i);
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

test('PR handoff requires each planner path, Team Lead and assigned workers', () => {
  const other = 'docs/nexora/tasks/2026-10-22-second-task.md';
  const valid = [
    '## Task Planner', path, other,
    'Team Lead: acting assistant',
    'Workers: TEAM_LEAD_SERIAL (no separate agent)',
    'Team Lead GO for PR submission. Independent review pending.',
    'NC-TEAM-003: E2E and Smoke evidence checked per subtask; Team Lead Review completed.',
  ].join('\\n');
  assert.deepEqual(validatePullRequestBody(valid, [path, other]), []);
  assert.match(validatePullRequestBody(valid.replace(other, ''), [path, other]).join(' '), /must link/);
  assert.match(validatePullRequestBody(valid.replaceAll('Team Lead', 'Coordinator'), [path]).join(' '), /Team Lead/);
  assert.match(validatePullRequestBody(valid.replace('Workers: TEAM_LEAD_SERIAL', 'Owner: unassigned'), [path]).join(' '), /worker assignments/);
  assert.match(validatePullRequestBody('', [path]).join(' '), /nonempty/);
  assert.match(validatePullRequestBody(null, [path]).join(' '), /nonempty/);
});

test('NC-TEAM-003 rejects missing E2E, Smoke, unreasoned N/A and fake Lead GO', () => {
  const errors = matrix => validateSubtaskMatrix(matrix).join('\n');
  assert.match(errors('# Task Checklist - incomplete'), /Missing NC-TEAM-003/);
  assert.match(errors(good.replace(' | PASS | PASS | PASS | GO |', ' | PASS |  | PASS | GO |')), /E2E/);
  assert.match(errors(good.replace(' | PASS | PASS | PASS | GO |', ' | PASS | PASS |  | GO |')), /Smoke/);
  assert.match(errors(good.replace(' | PASS | PASS | PASS | GO |', ' | PASS | N\/A: docs only | PASS | GO |')
    .replace('TEAM_LEAD_SERIAL | PASS', 'TEAM_LEAD_SERIAL | PASS')
    .replace('source SHA abcdef1', 'source SHA abcdef1')), /N\/A requires/);
  assert.match(errors(good.replace(' | PASS | PASS | PASS | GO |', ' | PASS | BLOCKED | PASS | GO |')), /cannot approve/);
  assert.match(errors(good.replace(' | PASS | PASS | PASS | GO |', ' | PASS | PASS | FAIL | GO |')), /cannot approve/);
  assert.deepEqual(validateSubtaskMatrix(good.replace(' | PASS | PASS | PASS | GO |', ' | PASS | PASS | PASS | PENDING |')), []);
  assert.deepEqual(validateSubtaskMatrix(good), []);
});

test('NC-TEAM-003 requires one matrix row per declared implementation subtask', () => {
  const plan = good.replace('## G. NC-TEAM-003', '## B. Team Lead tasks\n| ID | Worker | Scope |\n| --- | --- | --- |\n| T-01 | TEAM_LEAD_SERIAL | first |\n| T-02 | TEAM_LEAD_SERIAL | second |\n## G. NC-TEAM-003');
  assert.match(validateSubtaskMatrix(plan).join('\n'), /T-02: missing/);
  const extra = '| T-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Blocked pending test lab + Team Lead review |\n';
  assert.deepEqual(validateSubtaskMatrix(plan.replace('| T-01 | TEAM_LEAD_SERIAL | PASS', extra + '| T-01 | TEAM_LEAD_SERIAL | PASS')), []);
});

test('NC-TEAM-003 prevents marking subtask complete before Lead-approved triad', () => {
  const plan = good.replace('## G. NC-TEAM-003',
    '## B. Team Lead tasks\n| ID | Worker | Scope | State |\n| --- | --- | --- | --- |\n| T-01 | TEAM_LEAD_SERIAL | change | [x] |\n## G. NC-TEAM-003');
  assert.deepEqual(validateSubtaskMatrix(plan), []);
  const pending = plan.replace(' | PASS | PASS | PASS | GO |', ' | PASS | PASS | PENDING | PENDING |');
  assert.match(validateSubtaskMatrix(pending).join('\n'), /accepted subtask requires/);
});
