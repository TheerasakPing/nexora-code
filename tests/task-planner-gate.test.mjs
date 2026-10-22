import test from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, writeFileSync, rmSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

const checker = resolve('scripts/nexora/check-task-planner.mjs');
const git = (root, ...args) =>
  execFileSync('git', args, { cwd: root, encoding: 'utf8' }).trim();
const fullPlan = [
  '# Task Checklist — sample',
  '- Request: implement a scoped fix and document evidence.',
  '- Team Lead: TEAM_LEAD_SERIAL (no real workers delegated)',
  '- Acceptance: focused Node tests pass.',
  '**Team Lead verdict:** GO for PR submission. GO/NO-GO for release requires review.',
  '- Status: PENDING, do not claim native release.',
  '- [ ] Make change',
  '- [ ] Team Lead inspect diff',
  '- [ ] Capture SHA, tests and reviewer.',
  ''
].join('\n');
function repository(fn) {
  const root = mkdtempSync(join(tmpdir(), 'nexora-plan-'));
  try {
    git(root, 'init', '-q');
    git(root, 'config', 'user.email', 'tests@example.invalid');
    git(root, 'config', 'user.name', 'Test Runner');
    writeFileSync(join(root, 'baseline.txt'), 'base\n');
    git(root, 'add', '-A');
    git(root, 'commit', '-qm', 'base');
    const base = git(root, 'rev-parse', 'HEAD');
    return fn(root, base);
  } finally { rmSync(root, { recursive: true, force: true }); }
}
function run(root, base, number = '5', env = {}) {
  return spawnSync(process.execPath, [checker], {
    cwd: root,
    encoding: 'utf8',
    env: { ...process.env, BASE_SHA: base, HEAD_SHA: git(root, 'rev-parse', 'HEAD'),
      PR_NUMBER: number, ...env },
  });
}
function commit(root) { git(root, 'add', '-A'); git(root, 'commit', '-qm', 'change'); }
function makePlan(root, name, content = fullPlan) {
  const folder = join(root, 'docs', 'nexora', 'tasks');
  mkdirSync(folder, { recursive: true });
  writeFileSync(join(folder, name), content);
}
test('new valid per-request task planner passes', () => repository((root, base) => {
  makePlan(root, '2026-10-22-good-task.md');
  commit(root);
  const result = run(root, base);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /NC-TEAM-001 PASS/);
}));
test('code-only PR without new planner is rejected', () => repository((root, base) => {
  writeFileSync(join(root, 'baseline.txt'), 'changed\n');
  commit(root);
  const result = run(root, base);
  assert.equal(result.status, 1);
  assert.match(result.stderr, /NEW/);
}));
test('modified old task planner does not substitute for new task', () => repository((root) => {
  makePlan(root, '2026-10-21-old.md');
  commit(root);
  const base = git(root, 'rev-parse', 'HEAD');
  makePlan(root, '2026-10-21-old.md', fullPlan + '\nupdate\n');
  commit(root);
  assert.equal(run(root, base).status, 1);
}));
test('malformed planner is rejected', () => repository((root, base) => {
  makePlan(root, '2026-10-22-bad.md', '# brief\n');
  commit(root);
  const result = run(root, base);
  assert.equal(result.status, 1);
  assert.match(result.stderr, /incomplete/);
}));
test('legacy PR 1-3 exemptions are explicit; PR 4 must comply', () => repository((root, base) => {
  writeFileSync(join(root, 'baseline.txt'), 'changed\n');
  commit(root);
  for (const number of [1, 2, 3]) {
    const result = run(root, base, String(number));
    assert.equal(result.status, 0);
    assert.match(result.stdout, /LEGACY/);
  }
  assert.equal(run(root, base, '4').status, 1);
}));
test('invalid environment inputs are rejected without shell evaluation', () => repository((root, base) => {
  makePlan(root, '2026-10-22-good-task.md');
  commit(root);
  for (const env of [
    { BASE_SHA: '$(touch /tmp/should-not-exist)' },
    { HEAD_SHA: base + '; echo nope' },
    { PR_NUMBER: 'nan' },
  ]) {
    const result = run(root, base, '5', env);
    assert.equal(result.status, 1);
    assert.match(result.stderr, /NC-TEAM-001 FAIL/);
  }
}));


test('governance CI uses trusted base code and never executes PR source', () => {
  const workflow = readFileSync(resolve('.github/workflows/nexora-task-governance.yml'), 'utf8');
  assert.match(workflow, /pull_request_target:/);
  assert.match(workflow, /persist-credentials: false/);
  assert.match(workflow, /permissions:\s+contents: read/);
  assert.match(workflow, /pull_request\.base\.sha/);
  assert.match(workflow, /refs\/pull\//);
  assert.match(workflow, /node scripts\/nexora\/check-task-planner\.mjs/);
});

test('PR handoff must cite every new planner and Team Lead GO', () => repository((root, base) => {
  const planner = 'docs/nexora/tasks/2026-10-22-good-task.md';
  makePlan(root, '2026-10-22-good-task.md');
  commit(root);
  const event = join(root, 'event.json');
  writeFileSync(event, JSON.stringify({ pull_request: { body: 'No handoff submitted.' } }));
  let result = run(root, base, '5', { GITHUB_EVENT_PATH: event });
  assert.equal(result.status, 1);
  assert.match(result.stderr, /Pull request body/);
  writeFileSync(event, JSON.stringify({ pull_request: { body:
    'Task Planner: ' + planner + ' — Team Lead: acting assistant; TEAM_LEAD_SERIAL. GO for PR submission.' } }));
  result = run(root, base, '5', { GITHUB_EVENT_PATH: event });
  assert.equal(result.status, 0, result.stderr);
}));
