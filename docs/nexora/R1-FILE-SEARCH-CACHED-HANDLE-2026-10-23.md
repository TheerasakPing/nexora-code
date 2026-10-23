# R1 cached file-search result revalidation — Unix no-follow handles

**Date:** 2026-10-23. **Scope:** `src-tauri/src/workspace/file_search.rs` `ranked()` only.

When `search_files` serves a cached filename, a canonicalize-then-file-type
check has a race if a directory is renamed or replaced by a symlink. On Unix,
cached file results now call `open_scoped_preview(root, path)`, which anchors
each component via `openat(O_NOFOLLOW)` and checks the opened regular-file
handle. Cached directory results use
`open_scoped_listing_directory(root, Some(path))` with anchored directory
handles, also excluding aliases to internal workspace paths. No user data is
read into memory by this validation beyond metadata; returned IPC shape and
ranking are unchanged.

**Verification — real Mac M4:** existing symlink escape and cache fixture,
plus a new test replacing a directory with an internal symlink alias, run
against actual temporary disk objects. Cache may expire between checks:
results for legitimately rescanned `detached/report.txt` are allowed, but
results for the swapped alias `folder/report.txt` are not. Tests:
`cargo test --locked --lib workspace::file_search::tests::` **12/12 PASS**;
whole Rust lib **1075 PASS, 0 FAIL, 33 ignored**;
`node --test tests/nexora-r1-file-search-handle.test.mjs` **1/1 PASS**;
`npm test` **187/187 PASS**; `npm run test:release` **33/33 PASS**.
These are OS-file-handle integration tests, not installed GUI E2E.

**Not closed:** `scan_with_budget` still uses `std::fs::read_dir` by a
path during directory descent, and may be raced. The cached-result guard
prevents serving an alias at validation time, but does **not** prove the
scanner cannot enumerate outside workspace under adversarial concurrent
renames. Windows `ranked` retains legacy path validation and requires a
separate reparse-safe implementation and real Windows GUI tests. Git stage,
diff, sessions and provider crash recovery remain R1 blockers.

**Review:** TEAM_LEAD_SERIAL reviewed the narrow code and tests; no
independent reviewer. GO for source PR/CI, NO-GO R1 phase exit or Production.
