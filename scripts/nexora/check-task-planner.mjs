#!/usr/bin/env node
// NC-TEAM-001: evidence-based PR checklist enforcement.
// Never executes user-authored content or touches application credentials.
import { execFileSync } from 'node:child_process';

const die = message => {
  process.stderr.write('NC-TEAM-001 FAIL: ' + message + '\n');
  process.exitCode = 1;
};
const sha = value => /^[a-f0-9]{40}$/i.test(value ?? '');
const base = process.env.BASE_SHA;
const head = process.env.HEAD_SHA;
const number = Number(process.env.PR_NUMBER ?? '0');
if (!sha(base) || !sha(head)) {
  die('BASE_SHA and HEAD_SHA must be full, valid Git commit SHAs.');
} else if (!Number.isSafeInteger(number) || number < 1) {
  die('PR_NUMBER must be a positive integer from the GitHub pull_request event.');
} else if (number <= 3) {
  // Exact pre-policy PRs, not a general bypass for new branches.
  console.log('NC-TEAM-001 LEGACY PR #' + number + ': waived (created before adoption); review remains required.');
} else {
  try {
    const raw = execFileSync('git', [
      'diff', '--name-status', '-z', '--no-renames', '--diff-filter=AM', base + '...' + head,
    ], { encoding: 'utf8', maxBuffer: 1024 * 1024 });
    const fields = raw.split('\0').filter(Boolean);
    if (fields.length % 2 !== 0) throw new Error('Git change list was malformed.');
    const added = [];
    for (let i = 0; i < fields.length; i += 2) {
      if (fields[i] === 'A' &&
          /^docs\/nexora\/tasks\/[0-9]{4}-[0-9]{2}-[0-9]{2}-[a-z0-9][a-z0-9-]*\.md$/.test(fields[i + 1])) {
        added.push(fields[i + 1]);
      }
    }
    if (!added.length) {
      die('Every new PR must add a NEW docs/nexora/tasks/YYYY-MM-DD-task-slug.md. ' +
        'A modified old planner does not count.');
    } else {
      for (const path of added) {
        const body = execFileSync('git', ['show', head + ':' + path], {
          encoding: 'utf8', maxBuffer: 128 * 1024,
        });
        const missing = [];
        const checks = {
          'request/scope': /\bRequest\b|คำขอ|คำสั่ง/,
          'actual Team Lead': /Team Lead|หัวหน้าทีม/,
          'assignment/decomposition': /TEAM_LEAD_SERIAL|worker|agent|Assigned worker|เจ้าของงาน/i,
          'acceptance criteria': /Acceptance|DoD|เกณฑ์ผ่าน|Test/i,
          'checklist': /- \[[ xX]\] /,
          'review decision': /GO.{0,5}NO-GO|Team Lead (?:decision|verdict)|ผลตรวจ/i,
          'pending status / evidence': /PENDING|GO|NO-GO|PASS|FAIL|blocked/i,
        };
        for (const [name, pattern] of Object.entries(checks)) {
          if (!pattern.test(body)) missing.push(name);
        }
        if (body.length < 250) missing.push('substantive planner content');
        if (missing.length) {
          die(path + ' is incomplete: ' + missing.join(', '));
        } else {
          console.log('NC-TEAM-001 CHECKED: ' + path);
        }
      }
      if (!process.exitCode) {
        console.log('NC-TEAM-001 PASS: added task planner(s) with ownership, checkboxes and review fields.');
      }
    }
  } catch (error) {
    die('Could not verify PR Git changes: ' + (error instanceof Error ? error.message : String(error)));
  }
}
