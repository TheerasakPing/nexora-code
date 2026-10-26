import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readdir, readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { resolve, dirname } from 'node:path';

const repo = resolve(dirname(fileURLToPath(import.meta.url)), '../..');

test('browser E2E never writes screenshots into a host-specific /private/tmp directory', async () => {
  const files = (await readdir(resolve(repo, 'tests'))).filter(file => file.endsWith('.spec.ts'));
  assert.ok(files.length >= 50, 'full browser spec inventory changed unexpectedly');
  let portable = 0;
  for (const file of files) {
    const source = await readFile(resolve(repo, 'tests', file), 'utf8');
    assert.doesNotMatch(source, /\/private\/tmp\//, `${file} depends on the Mac-only private tmp directory`);
    portable += (source.match(/test\.info\(\)\.outputPath\(/g) ?? []).length;
  }
  assert.ok(portable >= 64, `expected portable screenshot coverage, found ${portable}`);
});
