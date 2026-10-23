import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const root = new URL('../', import.meta.url);
const read = file => readFileSync(new URL(file, root), 'utf8');

test('every Nexora agent entrypoint requires a per-request task planner', () => {
  for (const path of ['AGENTS.md', 'CONTRIBUTING.md', 'docs/nexora/SOP-DEVELOPMENT-RELEASE.md']) {
    const content = read(path);
    assert.match(content, /NC-TEAM-001/, path);
    assert.match(content, /docs\/nexora\/tasks\/YYYY-MM-DD-task-slug\.md/, path);
    assert.match(content, /Team Lead|หัวหน้าทีม/, path);
  }
});

test('task checklist template includes ownership, worker evidence and GO/NO-GO', () => {
  const template = read('docs/nexora/templates/TASK-CHECKLIST.md');
  for (const required of ['Team Lead', 'TEAM_LEAD_SERIAL', 'Worker handoff', 'Owned files', 'GO / NO-GO']) {
    assert.ok(template.toLowerCase().includes(required.toLowerCase()), 'missing: ' + required);
  }
  assert.ok((template.match(/- \[ \]/g) ?? []).length >= 10);
});

test('PR template requires Team Lead review and planner link', () => {
  const template = read('.github/pull_request_template.md');
  assert.match(template, /NC-TEAM-001/);
  assert.match(template, /Team Lead GO/);
  assert.match(template, /docs\/nexora\/tasks\//);
  assert.match(template, /not ready for merge/);
});

test('example governance planner exists for this rule change', () => {
  const plan = read('docs/nexora/tasks/2026-10-22-team-lead-planner-gate.md');
  assert.match(plan, /GOV-01/);
  assert.match(plan, /GOV-05/);
  assert.match(plan, /Team Lead verdict/);
});

test('NC-TEAM-003 requires task-by-task E2E, Smoke and Lead review across entrypoints', () => {
  for (const file of ['AGENTS.md', 'docs/nexora/SOP-DEVELOPMENT-RELEASE.md', 'docs/nexora/templates/TASK-CHECKLIST.md']) {
    const value = read(file);
    for (const requirement of ['NC-TEAM-003', 'E2E', 'Smoke', 'Team Lead']) {
      assert.ok(value.includes(requirement), file + ': missing ' + requirement);
    }
  }
  assert.match(read('scripts/nexora/governance-pr-gate.mjs'), /validateSubtaskMatrix/);
  assert.match(read('docs/nexora/templates/TASK-CHECKLIST.md'), /\| Subtask ID \| Worker \| Unit\/Contract \| E2E \| Smoke \| Team Lead Review \|/);
});
