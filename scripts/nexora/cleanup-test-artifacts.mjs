/**
 * Safe, repository-local cleanup for generated test output.
 * Dry-run by default. Never delete source tests, tracked fixtures or user data.
 */
import { execFileSync } from 'node:child_process';
import { existsSync, lstatSync, realpathSync, rmSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const SCRIPT = fileURLToPath(import.meta.url);
const REPO = resolve(dirname(SCRIPT), '../..');
export const GENERATED_TEST_DIRS = Object.freeze([
  'test-results',
  'playwright-report',
  'coverage',
]);

export function cleanupTestArtifacts(root, { apply = false } = {}) {
  const resolved = realpathSync(root);
  const gitRoot = realpathSync(execFileSync('git', ['rev-parse', '--show-toplevel'], {
    cwd: resolved, encoding: 'utf8',
  }).trim());
  if (resolved !== gitRoot) throw new Error('Cleanup requires the exact Git worktree root.');
  const report = { dryRun: !apply, candidates: [], removed: [], skipped: [] };
  for (const name of GENERATED_TEST_DIRS) {
    const target = join(resolved, name);
    if (!existsSync(target)) continue;
    const info = lstatSync(target);
    if (info.isSymbolicLink() || !info.isDirectory()) {
      report.skipped.push({ name, reason: 'not a regular generated directory' });
      continue;
    }
    const tracked = execFileSync('git', ['ls-files', '--', name], {
      cwd: resolved, encoding: 'utf8',
    }).trim();
    if (tracked) {
      report.skipped.push({ name, reason: 'contains Git-tracked files' });
      continue;
    }
    report.candidates.push(name);
    if (apply) {
      rmSync(target, { recursive: true, force: true, maxRetries: 3 });
      report.removed.push(name);
    }
  }
  return report;
}

if (process.argv[1] && resolve(process.argv[1]) === SCRIPT) {
  const args = process.argv.slice(2);
  if (args.some(arg => arg !== '--apply')) {
    process.stderr.write('Usage: node scripts/nexora/cleanup-test-artifacts.mjs [--apply]\n');
    process.exitCode = 2;
  } else {
    try {
      const result = cleanupTestArtifacts(REPO, { apply: args.includes('--apply') });
      process.stdout.write(JSON.stringify(result, null, 2) + '\n');
    } catch (error) {
      process.stderr.write(String(error) + '\n');
      process.exitCode = 1;
    }
  }
}
