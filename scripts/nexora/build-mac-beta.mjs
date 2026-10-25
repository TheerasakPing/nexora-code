#!/usr/bin/env node
// Nexora Code ONLY. An opt-in local test build; never a signed production release.
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, resolve, join } from 'node:path';

export const betaBuildArgs = Object.freeze([
  'build', '--debug', '--config', 'src-tauri/tauri.beta.conf.json',
  '--features', 'beta-isolation', '--bundles', 'app', '--ci',
]);

const repo = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.length > 2 && process.argv[2] !== '--print-args') {
    console.error('Expected no arguments or --print-args');
    process.exitCode = 2;
  } else if (process.argv[2] === '--print-args') {
    console.log(JSON.stringify(betaBuildArgs));
  } else if (process.platform !== 'darwin' || process.arch !== 'arm64') {
    console.error('BLOCKED: macOS arm64 required.');
    process.exitCode = 3;
  } else {
    // This is the ONLY supported Beta build recipe. The Rust setup() gate also
    // rejects a beta identifier that lacks the isolation feature.
    const cli = join(repo, 'node_modules/.bin/tauri');
    const child = spawnSync(cli, betaBuildArgs, {cwd: repo, stdio: 'inherit'});
    process.exitCode = child.status ?? 1;
  }
}
