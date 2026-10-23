# R1 P0 — No-follow Workspace Directory Listing

**Date:** 2026-10-23 | **Subscope:** `workspace::list_files` for macOS and Linux.

## Risk and implementation

The former Workspace directory preview called `scoped_path` to check a
relative folder then reopened it by string path with `fs::read_dir`.
Replacing the checked folder with an external symlink before the second call
could list filenames outside the checkout.

The Unix implementation now anchors a file descriptor to the checkout and
walks nested paths with `openat(O_DIRECTORY | O_NOFOLLOW)` at every component,
then enumerates from the already-opened descriptor using `fdopendir` and
`readdir`. Each entry's type is checked using `fstatat` relative to the
opened directory with `AT_SYMLINK_NOFOLLOW`. The bounded and sorted 500-item
response keeps existing IPC shape. Directory entries that are symlinks,
including internal symlink aliases, are **omitted** intentionally. The user
can browse the real folder directly.

## Tests and limits

On actual Mac M4:
- Unit/Contract: `cargo test --lib workspace::tests::` 33/33 PASS.
- Scoped native filesystem E2E: adversarial 250-cycle directory rename/symlink
  swap while `list_files` runs; any result must have only the internal
  `inside.txt` name. Results containing `outside_secret.txt` fail.
- Smoke: regular nested directory listing, .git restriction, 500-item cap
  and filename edge cases are in that Rust test suite. CI must compile and
  run on macOS ARM64 and Linux; Windows installer smoke must also pass.

**Not yet covered:** Windows `list_files` still uses the older
`scoped_path` then `fs::read_dir` approach. A Windows-native handle-based
reparse-point/junction fix and adversarial Windows E2E is **pending**. The
Git staging/diff and `file_search` traversal paths still have independent
TOCTOU risk and must be handled in later R1 subtasks. No claim of full
filesystem race security or native GUI acceptance. No user files touched.

**Team Lead:** SOURCE review GO for narrow Unix listing implementation, only
after exact-head CI verifies. Protected branch merge/Production NO-GO until
independent reviewer and all native R1 acceptance gates.
