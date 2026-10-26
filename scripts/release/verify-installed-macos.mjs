/** Non-mutating installed Nexora Code local smoke and release gates.
 * Does not launch, kill, replace or alter the app, settings or database. */
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const expected = { bundleId: 'com.theerasakping.nexoracode', executable: 'nexora-code', appName: 'Nexora Code.app' };

export function classifyAcceptance(evidence) {
  const failed = [];
  if (!evidence.bundleExists) failed.push('installed bundle missing');
  if (evidence.bundleId !== expected.bundleId) failed.push('bundle identifier mismatch');
  if (!evidence.architecture?.includes('arm64')) failed.push('arm64 binary not confirmed');
  if (!evidence.processRunning) failed.push('installed native process not running');
  if (evidence.sqliteIntegrity !== 'ok') failed.push('local database integrity not confirmed');
  const signedByDeveloperId = evidence.signature === 'developer-id';
  const gates = {
    installedRuntime: failed.length === 0 ? 'PASS' : 'FAIL',
    developerIdSignature: signedByDeveloperId ? 'PASS' : 'BLOCKED',
    notarization: evidence.notarized ? 'PASS' : 'BLOCKED',
    interactiveNativeGui: 'PENDING_MANUAL_ACCEPTANCE',
    independentHumanReview: 'PENDING_HUMAN_REVIEW',
  };
  return { localSmokePass: failed.length === 0, productionReady: false, failures: failed, gates };
}

function run(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', timeout: 15000 });
  return { ok: !result.error && result.status === 0, output: `${result.stdout ?? ''}\n${result.stderr ?? ''}`.trim() };
}

export function inspectInstalledMac({ appPath = '/Applications/Nexora Code.app', home = os.homedir() } = {}) {
  const binary = path.join(appPath, 'Contents', 'MacOS', expected.executable);
  const plist = path.join(appPath, 'Contents', 'Info.plist');
  const db = path.join(home, 'Library', 'Application Support', expected.bundleId, 'kingcode.sqlite3');
  const bundleExists = existsSync(binary) && existsSync(plist);
  const plistBundle = bundleExists ? run('/usr/libexec/PlistBuddy', ['-c', 'Print CFBundleIdentifier', plist]) : { ok: false };
  const arch = bundleExists ? run('/usr/bin/file', ['-b', binary]) : { ok: false };
  const pgrep = bundleExists ? run('/usr/bin/pgrep', ['-fl', `^${binary.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}$`]) : { ok: false };
  const sqlite = existsSync(db) ? run('/usr/bin/sqlite3', ['-readonly', db, 'PRAGMA integrity_check;']) : { ok: false };
  const code = bundleExists ? run('/usr/bin/codesign', ['-dv', appPath]) : { ok: false, output: '' };
  const stapler = bundleExists ? run('/usr/bin/xcrun', ['stapler', 'validate', appPath]) : { ok: false };
  const signature = code.ok && /Authority=Developer ID Application/.test(code.output) ? 'developer-id' : code.ok && /Signature=adhoc/.test(code.output) ? 'adhoc' : 'unknown';
  return {
    bundleExists,
    bundleId: plistBundle.ok ? plistBundle.output : null,
    architecture: arch.ok ? arch.output : null,
    processRunning: pgrep.ok,
    sqliteIntegrity: sqlite.ok ? sqlite.output.split(/\s+/)[0] : null,
    signature,
    notarized: stapler.ok,
  };
}

export function runMacAcceptance({ appPath, home } = {}) {
  const data = inspectInstalledMac({ appPath, home });
  return { ...data, ...classifyAcceptance(data) };
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.platform !== 'darwin') {
    process.stderr.write('macOS-only acceptance check\n');
    process.exitCode = 2;
  } else {
    const result = runMacAcceptance();
    process.stdout.write(JSON.stringify(result, null, 2) + '\n');
    if (!result.localSmokePass) process.exitCode = 1;
  }
}
