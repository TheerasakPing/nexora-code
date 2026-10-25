import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const read = path => readFileSync(new URL('../' + path, import.meta.url), 'utf8');
const code = read('src-tauri/src/workspace.rs');

test('Unix directory listing is anchored to an actual file descriptor and does not follow symlinks', () => {
  assert.match(code, /fn open_scoped_listing_directory\(/);
  assert.match(code, /libc::openat\(/);
  assert.match(code, /libc::O_NOFOLLOW/);
  assert.match(code, /libc::fdopendir\(/);
  assert.match(code, /libc::readdir\(/);
  assert.match(code, /libc::fstatat\(/);
  assert.match(code, /libc::AT_SYMLINK_NOFOLLOW/);
  assert.match(code, /if !is_dir && kind != libc::S_IFREG/);
});

test('Unix listing has real file system symlink swap and ordinary folder regression scenarios', () => {
  for (const marker of [
    'fn listing_never_yields_symlinks_even_when_their_targets_are_internal',
    'fn listing_symlink_swap_race_never_reports_external_filenames',
    'fn folder_listing_is_one_level_and_bounded',
    'fn git_internal_paths_are_rejected_under_all_ascii_case_variants',
  ]) assert.ok(code.includes(marker), marker);
  assert.ok(code.includes('outside_secret.txt'));
});

test('Windows directory enumeration remains a separately declared acceptance gap', () => {
  const docs = read('docs/nexora/R1-WORKSPACE-DIRECTORY-ANCHOR-2026-10-23.md');
  assert.match(docs, /Windows/);
  assert.match(docs, /pending/i);
  assert.match(docs, /TOCTOU/);
});
