import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const source=readFileSync(new URL('../src-tauri/src/providers.rs',import.meta.url),'utf8');

test('provider IPC actively checks for exited process with inherited stdout pipes',()=>{
  assert.match(source,/let mut exit_probe = tokio::time::interval\(Duration::from_millis\(200\)\)/);
  assert.match(source,/let mut leader_exited_at: Option<tokio::time::Instant> = None/);
  assert.match(source,/child\.try_wait\(\)/);
  assert.match(source,/since\.elapsed\(\) >= Duration::from_secs\(1\)/);
  assert.match(source,/background child kept its output stream open/);
});
test('provider early-exit branch still runs the existing owned-group final cleanup',()=>{
  const run=source.slice(source.indexOf('async fn run_process_with_tools('),source.indexOf('#[cfg(test)]\nmod tests'));
  assert.match(run,/finish_child\(&mut child, stdin, &mut group\)\.await/);
  assert.match(source,/fn exited_provider_with_inherited_stdout_does_not_hang_and_reaps_descendant/);
  assert.match(run,/if stopped \{ break Ok\(\(\)\); \}/);
});
