import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { createTaskPlan } from '../scripts/nexora/new-task-plan.mjs';

function fixture() {
  const directory = mkdtempSync(join(tmpdir(), 'nexora-planner-'));
  const template = join(directory, 'template.md');
  writeFileSync(template, '# Task Checklist Planner — YYYY-MM-DD-task-slug\n\n**Team Lead:** TBD\n\n**GO / NO-GO:** PENDING\n');
  return { directory, template };
}

test('creates unique planner from approved task template', () => {
  const options = fixture();
  const target = createTaskPlan({ date: '2026-10-22', slug: 'fix-warm-agent' }, options);
  assert.equal(target, join(options.directory, '2026-10-22-fix-warm-agent.md'));
  assert.match(readFileSync(target, 'utf8'), /2026-10-22-fix-warm-agent/);
  assert.match(readFileSync(target, 'utf8'), /Team Lead/);
});

test('never overwrites existing planner on repeat creation', () => {
  const options = fixture();
  const name = { date: '2026-10-22', slug: 'safe-task' };
  const target = createTaskPlan(name, options);
  writeFileSync(target, 'WORKER EVIDENCE MUST SURVIVE');
  assert.throws(() => createTaskPlan(name, options), /EEXIST/);
  assert.equal(readFileSync(target, 'utf8'), 'WORKER EVIDENCE MUST SURVIVE');
});

test('rejects traversal, invalid slugs, dates and oversized names', () => {
  const options = fixture();
  for (const slug of ['../bad', 'UPPER', 'spaces here', 'a--b', '-bad', 'bad-', 'a'.repeat(71)]) {
    assert.throws(() => createTaskPlan({ date: '2026-10-22', slug }, options), /Slug/);
  }
  for (const date of ['2026-13-01', '2026-02-30', 'tomorrow', '2026-1-2']) {
    assert.throws(() => createTaskPlan({ date, slug: 'good' }, options), /Date/);
  }
});

test('refuses malformed template without creating task file', () => {
  const options = fixture();
  writeFileSync(options.template, 'not a checklist');
  assert.throws(() => createTaskPlan({ date: '2026-10-22', slug: 'example' }, options), /Invalid/);
  assert.equal(existsSync(join(options.directory, '2026-10-22-example.md')), false);
});
