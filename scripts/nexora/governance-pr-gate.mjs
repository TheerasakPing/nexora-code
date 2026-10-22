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
  if (!/^\*\*Team Lead verdict:\*\*\s*(?:SOURCE REVIEW GO|GO)(?:\b|[ :])/mi.test(content)) {
    errors.push('Team Lead must record GO for PR submission before delivery.');
  }
  if ((content.match(/- \[[ x]\]/g) ?? []).length < 3) errors.push('Missing checkable task evidence.');
  return errors;
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
  for (const path of result.planners) process.stdout.write('Planner: ' + path + '\n');
  if (result.failures.length) {
    for (const failure of result.failures) process.stderr.write('ERROR: ' + failure + '\n');
    process.exitCode = 1;
    return;
  }
  process.stdout.write('NC-TEAM-001 structural PR checklist gate: PASS\n');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try { main(); }
  catch (error) {
    console.error('Governance check failed closed: ' + (error instanceof Error ? error.message : String(error)));
    process.exitCode = 1;
  }
}
