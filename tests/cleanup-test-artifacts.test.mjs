import { test } from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdtempSync, mkdirSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { cleanupTestArtifacts } from '../scripts/nexora/cleanup-test-artifacts.mjs';

function fixture(run) {
  const root = mkdtempSync(join(tmpdir(), 'nexora-cleanup-test-'));
  try {
    execFileSync('git', ['init', '--quiet'], { cwd: root });
    mkdirSync(join(root, 'tests'));
    writeFileSync(join(root, 'tests', 'keep.test.mjs'), 'keep');
    execFileSync('git', ['add', 'tests/keep.test.mjs'], { cwd: root });
    return run(root);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

test('dry-run lists generated directories without touching regression tests', () => fixture(root => {
  mkdirSync(join(root, 'test-results'));
  writeFileSync(join(root, 'test-results', 'trace.zip'), 'old log');
  assert.deepEqual(cleanupTestArtifacts(root).candidates, ['test-results']);
  assert.equal(existsSync(join(root, 'test-results', 'trace.zip')), true);
  const result = cleanupTestArtifacts(root, { apply: true });
  assert.deepEqual(result.removed, ['test-results']);
  assert.equal(existsSync(join(root, 'test-results')), false);
  assert.equal(existsSync(join(root, 'tests', 'keep.test.mjs')), true);
}));

test('refuses to delete tracked fixture files or symlinked paths', () => fixture(root => {
  mkdirSync(join(root, 'coverage'));
  writeFileSync(join(root, 'coverage', 'keep.txt'), 'tracked fixture');
  execFileSync('git', ['add', 'coverage/keep.txt'], { cwd: root });
  const outside = join(root, 'tests');
  symlinkSync(outside, join(root, 'test-results'), process.platform === 'win32' ? 'junction' : 'dir');
  const result = cleanupTestArtifacts(root, { apply: true });
  assert.deepEqual(result.removed, []);
  assert.equal(result.skipped.length, 2);
  assert.equal(existsSync(join(root, 'coverage', 'keep.txt')), true);
  assert.equal(existsSync(join(root, 'tests', 'keep.test.mjs')), true);
}));

test('requires an exact git worktree root, not an arbitrary subfolder', () => fixture(root => {
  assert.throws(() => cleanupTestArtifacts(join(root, 'tests'), { apply: true }), /exact Git worktree root/);
}));
