import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

const readConfig = async (path) => JSON.parse(await readFile(new URL(path, import.meta.url), 'utf8'));

const mergeConfig = (base, override) => {
  if (override === null || Array.isArray(override) || typeof override !== 'object') return override;

  const merged = { ...base };
  for (const [key, value] of Object.entries(override)) {
    merged[key] = value && typeof value === 'object' && !Array.isArray(value)
      ? mergeConfig(merged[key] ?? {}, value)
      : value;
  }
  return merged;
};

test('Beta overlay isolates identity and builds an app without a dev server', async () => {
  const [base, beta] = await Promise.all([
    readConfig('../../src-tauri/tauri.conf.json'),
    readConfig('../../src-tauri/tauri.beta.conf.json'),
  ]);
  const effective = mergeConfig(base, beta);

  assert.equal(effective.identifier, 'com.theerasakping.nexoracode.beta');
  assert.notEqual(effective.identifier, base.identifier);
  assert.equal(effective.productName, 'Nexora Code Beta');
  assert.equal(effective.app.windows[0].title, 'Nexora Code Beta');
  assert.equal(effective.app.windows[0].width, base.app.windows[0].width);
  assert.equal(effective.version, base.version);
  assert.equal(effective.build.devUrl, null);
  assert.equal(effective.build.beforeDevCommand, null);
  assert.equal(effective.build.beforeBuildCommand, base.build.beforeBuildCommand);
  assert.equal(effective.build.frontendDist, base.build.frontendDist);
  assert.equal(effective.bundle.active, true);
  assert.deepEqual(effective.bundle.targets, ['app']);
  assert.equal(effective.bundle.macOS.signingIdentity, '-');
});

test('isolated Mac build and runtime startup both enforce opt-in isolation', async () => {
  const {betaBuildArgs} = await import('./build-mac-beta.mjs');
  assert.ok(betaBuildArgs.includes('beta-isolation'), 'builder must enable beta feature');
  assert.ok(betaBuildArgs.includes('src-tauri/tauri.beta.conf.json'), 'builder must use separate bundle identity');
  assert.ok(betaBuildArgs.includes('--bundles') && betaBuildArgs.includes('app'), 'builder must make actual Mac app');
  const rust = await readFile(new URL('../../src-tauri/src/lib.rs', import.meta.url), 'utf8');
  assert.match(rust, /app\.config\(\)\.identifier\s*==\s*"com\.theerasakping\.nexoracode\.beta"/);
  assert.match(rust, /beta_identifier\s*!=\s*cfg!\(feature\s*=\s*"beta-isolation"\)/);
  const usage = await readFile(new URL('../../src-tauri/src/agent_usage.rs',import.meta.url),'utf8');
  assert.match(usage,/if cfg!\(feature\s*=\s*"beta-isolation"\)/);
  assert.match(usage,/Agent usage credential probes are disabled/);
});

test('isolated GUI launch never inherits provider credential variables', async () => {
  const shell=await readFile(new URL('./launch-mac-beta.sh',import.meta.url),'utf8');
  assert.match(shell,/com\.theerasakping\.nexoracode\.beta/);
  assert.match(shell,/\/usr\/bin\/env -i/);
  assert.match(shell,/\/private\/tmp\/nexora-code-beta-home/);
  assert.match(shell,/codesign --verify --deep --strict/);
  assert.doesNotMatch(shell,/security\s+find-generic-password|xattr\s+-d|spctl\s+--master-disable/);
});

test('early beta preparation does not use global CLI PATH, and legacy Keychain path is cfg-gated', async () => {
  const [runtime,provider]=await Promise.all([
    readFile(new URL('../../src-tauri/src/lib.rs',import.meta.url),'utf8'),
    readFile(new URL('../../src-tauri/src/provider_settings.rs',import.meta.url),'utf8')
  ]);
  assert.match(runtime,/pub fn prepare_environment\(\)\s*\{[\s\S]{0,190}#\[cfg\(not\(feature = "beta-isolation"\)\)\]\s*providers::widen_process_path\(\)/);
  assert.match(provider,/#\[cfg\(all\(target_os = "macos", not\(feature = "beta-isolation"\)\)\)\]\s*pub\(crate\) fn glm_key_for_usage/);
  assert.match(provider,/#\[cfg\(all\(feature = "beta-isolation", target_os = "macos"\)\)\]\s*pub\(crate\) fn glm_key_for_usage/);
});
