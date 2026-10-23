import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const provider = readFileSync(new URL('../src-tauri/src/providers.rs', import.meta.url), 'utf8');

test('provider supervisor starts child in its own process group', () => {
  assert.match(provider, /command\.process_group\(0\)/);
  assert.match(provider, /ProcessGroup::register\(child\.id\(\)\)/);
  assert.match(provider, /\.kill_on_drop\(true\)/);
  assert.match(provider, /impl Drop for ProcessGroup/);
  assert.match(provider, /fn shutdown_all\(\)/);
});

test('Unix supervisor never signals special process groups 0, 1 or overflowed IDs', () => {
  assert.ok(provider.includes('i32::try_from(pid)'));
  assert.ok(provider.includes('if group <= 1 { return; }'));
  assert.ok(provider.includes('libc::kill(-group, libc::SIGKILL)'));
});

test('native process lifecycle regressions belong to the Rust test suite', () => {
  for (const name of [
    'successful_provider_parent_exit_cleans_up_owned_background_descendant',
    'cancelled_provider_task_cleans_up_its_owned_process_group',
    'forced_shutdown_terminates_owned_provider_group',
    'group_cleanup_rejects_invalid_or_dangerous_unix_process_group_ids',
  ]) assert.ok(provider.includes('fn ' + name + '('), name);
});
