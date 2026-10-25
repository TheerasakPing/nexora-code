import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const source=readFileSync(new URL('../src-tauri/src/workspace/file_search.rs',import.meta.url),'utf8');
test('Unix cached file search results use fd-anchored openers',()=>{
  assert.match(source,/super::open_scoped_preview\(root, &file\.path\)/);
  assert.match(source,/super::open_scoped_listing_directory\(root, Some\(&file\.path\)\)/);
  assert.match(source,/fn cached_search_rejects_swapped_internal_symlink_aliases/);
  assert.match(source,/#\[cfg\(not\(unix\)\)\]\s*let live/);
});
