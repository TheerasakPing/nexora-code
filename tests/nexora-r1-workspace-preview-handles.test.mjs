import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const read = (path) => readFileSync(new URL('../' + path, import.meta.url), 'utf8');
const source = read('src-tauri/src/workspace.rs');

test('workspace preview text and image use same scoped handle, not pathname File::open', () => {
  const image = source.split('pub fn image_data_url(')[1].split('pub fn read_file(')[0];
  const text = source.split('pub fn read_file(')[1].split('#[cfg(test)]')[0];
  assert.ok(image.includes('open_scoped_preview(root, path)'));
  assert.ok(text.includes('open_scoped_preview(root, &path)'));
  for (const implementation of [image, text]) {
    assert.doesNotMatch(implementation, /File::open\(/);
    assert.match(implementation, /metadata()/);
  }
  assert.ok(image.includes('bytes.len() as u64 > IMAGE_LIMIT'));
});

test('both OS paths check file handles; unix links and FIFOs are never followed', () => {
  assert.ok(source.includes('libc::openat(directory.as_raw_fd()'));
  assert.match(source, /libc::O_NOFOLLOW | libc::O_NONBLOCK/);
  assert.match(source, /libc::O_DIRECTORY | libc::O_NOFOLLOW/);
  assert.match(source, /FILE_FLAG_OPEN_REPARSE_POINT/);
  assert.match(source, /GetFinalPathNameByHandleW/);
  assert.ok(source.includes('!actual.starts_with(&root)'));
  assert.match(source, /fn preview_handles_reject_file_and_directory_symlinks_and_special_files/);
  assert.match(source, /fn preview_race_never_returns_bytes_from_swapped_symlink/);
});

test('preview security docs state exact remaining TOCTOU scope and independent review', () => {
  const docs = read('docs/nexora/R1-WORKSPACE-PREVIEW-HANDLE-SECURITY-2026-10-23.md');
  for (const phrase of ['symlinks', 'openat', 'GetFinalPathNameByHandleW', 'list_files', 'git_stage', 'Independent', 'pending']) {
    assert.match(docs, new RegExp(phrase));
  }
});
