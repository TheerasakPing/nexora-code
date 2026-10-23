#!/usr/bin/env node
/**
 * Structural PR governance gate. Automated text checks cannot verify real
 * approval, so branch protection and human review remain separate gates.
 * For the synthetic pull_request merge checkout, compare HEAD^1 and HEAD^2.
 */
import { execFileSync } from 'node:child_process';
import { lstatSync, readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

const PLANNER_PATH = /^docs\/nexora\/tasks\/\d{4}-\d{2}-\d{2}-[a-z0-9][a-z0-9-]*\.md$/;
const MAX_PLAN_BYTES = 256 * 1024;

export function validatePlanner(path, content) {
  const errors = [];
  if (!PLANNER_PATH.test(path)) errors.push('Task planner path must contain a date and slug.');
  if (Buffer.byteLength(content, 'utf8') > MAX_PLAN_BYTES) errors.push('Task planner exceeds 256 KiB.');
  if (!/^# Task Checklist\b/m.test(content)) errors.push('Missing Task Checklist heading.');
  if (!/Team Lead/i.test(content)) errors.push('Missing actual Team Lead ownership.');
  if (!/TEAM_LEAD_SERIAL|worker\s*(?:id|agent)|assigned worker/i.test(content)) {
    errors.push('Missing real worker assignment or documented serial fallback.');
  }
  if (!/Acceptance|DoD|Definition of Done/i.test(content)) {
    errors.push('Missing acceptance criteria.');
  }
  if (!/Test|Evidence/i.test(content)) errors.push('Missing test/evidence section.');
  if (!/GO\s*\/\s*NO-GO|GO\/NO-GO/.test(content)) errors.push('Missing GO/NO-GO gate.');
  errors.push(...validateSubtaskMatrix(content, path));
  if (!/^\*\*Team Lead verdict:\*\*\s*(?:SOURCE REVIEW GO|GO)(?:\b|[ :])/mi.test(content)) {
    errors.push('Team Lead must record GO for PR submission before delivery.');
  }
  if ((content.match(/- \[[ x]\]/g) ?? []).length < 3) errors.push('Missing checkable task evidence.');
  return errors;
}

/**
 * NC-TEAM-003 structural subtask handoff gate.
 * This only validates the report shape and consistency; human review must
 * verify that a worker and the test evidence are real.
 */
export function validateSubtaskMatrix(content, plannerPath = '') {
  const errors = [];
  const heading = content.match(/^## G\. NC-TEAM-003[^\n]*$/m);
  if (!heading) return ['Missing NC-TEAM-003 per-subtask verification matrix.'];
  const section = content.slice(heading.index + heading[0].length).split(/^## /m)[0];
  const lines = section.split('\n').map(line => line.trim()).filter(line => line.startsWith('|'));
  const fields = line => line.split('|').slice(1, -1).map(cell => cell.trim());
  const expected = ['Subtask ID', 'Worker', 'Unit/Contract', 'E2E', 'Smoke', 'Team Lead Review', 'Evidence / N/A reason'];
  if (!lines.length || expected.some((name, i) => fields(lines[0])[i] !== name)) {
    return ['NC-TEAM-003 matrix must have columns: ' + expected.join(' | ')];
  }
  const rows = lines.slice(1).filter(line => !/^\|[\s|:-]+\|$/.test(line)).map(fields);
  if (!rows.length) return ['NC-TEAM-003 requires at least one real subtask row.'];
  const seen = new Set();
  const rowById = new Map();
  for (const row of rows) {
    const [id, worker, unit, e2e, smoke, lead, evidence] = row;
    if (row.length !== 7) { errors.push('NC-TEAM-003 matrix row has missing columns.'); continue; }
    if (!/^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+$/.test(id)) errors.push('Invalid subtask ID in matrix: ' + id);
    if (seen.has(id)) errors.push('Duplicate subtask ID in matrix: ' + id);
    seen.add(id);
    rowById.set(id, row);
    if (!worker || /^(TBD|NONE|UNASSIGNED)$/i.test(worker)) errors.push(id + ': real worker ID or TEAM_LEAD_SERIAL required.');
    for (const [gate, result] of [['Unit/Contract',unit], ['E2E',e2e], ['Smoke',smoke]]) {
      if (!/^(?:PASS|FAIL|PENDING|BLOCKED(?:: .+)?|N\/A: .+)$/i.test(result)) {
        errors.push(id + ': ' + gate + ' must be PASS, FAIL, PENDING, BLOCKED or reasoned N/A.');
      }
      if (/^N\/A:/i.test(result) && !/Team Lead|Lead approved|Lead approval/i.test(result + ' ' + evidence)) {
        errors.push(id + ': N/A requires documented Team Lead approval.');
      }
    }
    if (!/^(?:GO|NO-GO|PENDING)$/i.test(lead)) errors.push(id + ': Team Lead Review must be GO, NO-GO or PENDING.');
    if (lead === 'GO' && [unit, e2e, smoke].some(value => /^(PENDING|BLOCKED|FAIL)/i.test(value))) {
      errors.push(id + ': Team Lead cannot approve a subtask with missing or failed tests.');
    }
    if (!evidence || /^(TBD|PLACEHOLDER|NONE)$/i.test(evidence)) errors.push(id + ': actual evidence or explicit pending blocker required.');
  }
  // Match declared implementation subtasks, not final independent approval
  // rows. Earlier plans can have varied B formatting; always require the
  // matrix and at least one row even if a B header is unavailable.
  const decomposition = content.match(/^## B\.[^\n]*\n([\s\S]*?)(?=^## [A-Z]\.|$(?![\s\S]))/m);
  if (decomposition) {
    // Existing planners merged before NC-TEAM-003 retain their historical
    // source-only [x] checkmarks; they MUST include retrospective matrix rows,
    // but cannot be rewritten to claim previously unrecorded native E2E.
    // New plans are strictly checked: an [x] needs triad + Team Lead GO.
    const historicalPlanners = new Set([
      '2026-10-23-nc-team-002-progress-accountability.md',
    ]);
    const isLegacy = historicalPlanners.has(plannerPath.split('/').at(-1)) &&
      /NC-TEAM-003[^\n]*retrospective audit/.test(content);
    for (const line of decomposition[1].split('\n').filter(line => line.startsWith('|'))) {
      const [id, ...cells] = line.split('|').slice(1, -1).map(v => v.trim());
      if (!/^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+$/.test(id)) continue;
      if (!seen.has(id) && !/(?:-REVIEW|-QA)$/.test(id)) {
        errors.push(id + ': missing NC-TEAM-003 test/review matrix row.');
      }
      if (!isLegacy && cells.some(v => /^\[x\]$/i.test(v))) {
        const row = rowById.get(id);
        if (!row || row[5] !== 'GO' || row.slice(2, 5).some(v => !/^(PASS|N\/A: .+)$/i.test(v))) {
          errors.push(id + ': [x] accepted subtask requires verified Unit/Contract, E2E, Smoke and Team Lead GO.');
        }
      }
    }
  }
  return errors;
}

/** Structural PR handoff check; not proof of real reviewer approval. */
export function validatePullRequestBody(body, planners) {
  if (typeof body !== 'string' || !body.trim() || body.length > 128 * 1024) {
    return ['Pull request body must contain a bounded nonempty task handoff.'];
  }
  const failures = [];
  for (const path of planners) {
    if (!body.includes(path)) failures.push('Pull request body must link added task planner: ' + path);
  }
  if (!/Team Lead/i.test(body)) failures.push('Pull request body must name the Team Lead.');
  if (!/TEAM_LEAD_SERIAL|Worker(?:s| IDs)?/i.test(body)) {
    failures.push('Pull request body must describe real worker assignments or TEAM_LEAD_SERIAL.');
  }
  if (!/\bGO for PR\b/i.test(body)) {
    failures.push('Pull request body must record Team Lead GO for PR submission.');
  }
  if (!/NC-TEAM-003/i.test(body) || !/\bE2E\b/i.test(body) || !/\bSmoke\b/i.test(body)) {
    failures.push('PR handoff must cover NC-TEAM-003 per-subtask E2E and Smoke verification.');
  }
  if (!/Team Lead (?:Review|GO|NO-GO)/i.test(body)) {
    failures.push('PR handoff must describe Team Lead review of every subtask.');
  }
  return failures;
}

export function validateChangedFiles(changes, read = readFileSync) {
  const planners = changes.filter(({ status, path }) => status === 'A' && PLANNER_PATH.test(path));
  const failures = [];
  if (planners.length === 0) failures.push('Every PR must add a new per-request docs/nexora/tasks/YYYY-MM-DD-slug.md planner.');
  for (const { path } of planners) {
    try {
      if (read === readFileSync) {
        const meta = lstatSync(path);
        if (!meta.isFile() || meta.isSymbolicLink() || meta.size > MAX_PLAN_BYTES) {
          throw new Error('Planner must be a bounded regular file, not a symlink.');
        }
      }
      const text = read(path, 'utf8');
      failures.push(...validatePlanner(path, String(text)).map(error => path + ': ' + error));
    } catch {
      failures.push(path + ': new planner file is missing or unreadable.');
    }
  }
  return { planners: planners.map(({ path }) => path), failures };
}

function resolvedSha(rev) {
  if (!/^HEAD\^[12]$/.test(rev) && !/^[a-f0-9]{40}$/.test(rev)) {
    throw new Error('Only commit SHAs or HEAD^1 / HEAD^2 are supported.');
  }
  const sha = execFileSync('git', ['rev-parse', '--verify', rev + '^{commit}'], {
    encoding: 'utf8', maxBuffer: 1024 * 1024, timeout: 10000,
  }).trim();
  if (!/^[a-f0-9]{40}$/.test(sha)) throw new Error('Invalid resolved SHA.');
  return sha;
}

function main() {
  const [from, to] = process.argv.slice(2);
  if (!from || !to) throw new Error('Usage: governance-pr-gate.mjs <base-sha> <head-sha>');
  const base = resolvedSha(from);
  const head = resolvedSha(to);
  const output = execFileSync('git', ['diff', '--name-status', '--diff-filter=A', base, head, '--'], {
    encoding: 'utf8', maxBuffer: 1024 * 1024, timeout: 10000,
  });
  const changes = output.trim().split('\n').filter(Boolean).map(line => {
    const [status, ...parts] = line.split('\t');
    return { status, path: parts.join('\t') };
  });
  const result = validateChangedFiles(changes);
  if (!process.env.GITHUB_EVENT_PATH) {
    throw new Error('GITHUB_EVENT_PATH is required for PR checklist verification.');
  }
  const event = JSON.parse(readFileSync(process.env.GITHUB_EVENT_PATH, 'utf8'));
  const failures = [...result.failures,
    ...validatePullRequestBody(event?.pull_request?.body, result.planners)];
  for (const path of result.planners) process.stdout.write('Planner: ' + path + '\n');
  if (failures.length) {
    for (const failure of failures) process.stderr.write('ERROR: ' + failure + '\n');
    process.exitCode = 1;
    return;
  }
  process.stdout.write('NC-TEAM-001/002/003 structural PR checklist gate: PASS\n');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try { main(); }
  catch (error) {
    console.error('Governance check failed closed: ' + (error instanceof Error ? error.message : String(error)));
    process.exitCode = 1;
  }
}
