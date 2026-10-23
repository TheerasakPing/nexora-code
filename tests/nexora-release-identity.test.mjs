import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const read = path => readFileSync(new URL('../' + path, import.meta.url), 'utf8');
test('shipped executable is Nexora Code but existing Rust library and package IDs are stable', () => {
  const cargo = read('src-tauri/Cargo.toml');
  assert.match(cargo, /\[package\]\s*\nname = "pandacode"/);
  assert.match(cargo, /\[\[bin\]\]\s*\nname = "nexora-code"\s*\npath = "src\/main.rs"/);
  assert.match(cargo, /\[lib\]\s*\nname = "pandacode_lib"/);
  assert.match(read('src-tauri/src/main.rs'), /pandacode_lib::run\(\)/);
});
test('native release and Windows smoke checks expect branded executable', () => {
  assert.match(read('scripts/release/build.mjs'), /Contents\/MacOS\/nexora-code/);
  assert.doesNotMatch(read('scripts/release/build.mjs'), /Contents\/MacOS\/pandacode/);
  assert.match(read('scripts/release/ci-startup-smoke.mjs'), /nexora-code\.exe/);
  assert.match(read('scripts/measure.mjs'), /release\/nexora-code/);
  assert.match(read('src-tauri/tauri.conf.json'), /"productName": "Nexora Code"/);
});
test('agent client metadata is branded but stable tool commands remain compatible', () => {
  assert.match(read('src-tauri/src/providers.rs'), /"name":"nexora-code","title":"Nexora Code"/);
  assert.match(read('src-tauri/src/integrations.rs'), /"name":"nexora-code","title":"Nexora Code"/);
  assert.match(read('src-tauri/src/architect.rs'), /pandacode_finish_team/);
});

test('fork website never routes downloads to upstream or invents signed release URLs', () => {
  const source = read('website/src/lib/distribution.ts');
  assert.match(source, /github\.com\/TheerasakPing\/nexora-code/);
  assert.match(source, /actions\/workflows\/nexora-r6-alpha\.yml/);
  assert.doesNotMatch(source, /aicodeking\/pandacode|latest\/download/);
  assert.match(read('website/src/components/SiteFooter.astro'), /This is a fork of/);
  assert.match(read('website/src/pages/license.astro'), /https:\/\/github\.com\/aicodeking\/pandacode/);
});
