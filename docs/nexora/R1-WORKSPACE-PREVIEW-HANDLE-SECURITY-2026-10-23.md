# R1 P0 — Workspace file/image preview: no-follow, anchored handles

**Date:** 2026-10-23 · **Scope:** `src-tauri/src/workspace.rs` for `read_file` and `image_data_url` only. Not all workspace API/TOCTOU work.

## Threat and previous behavior

Workspace preview previously called `scoped_path()`, then reopened the
same relative filename with `File::open`. A local attacker/another worker
could swap an ancestor or final path to a symlink between canonicalization
and reopening, so a preview could read bytes outside the authorized workspace.
An unbounded FIFO could also block a worker thread during read.

## Implementation

- On macOS/Linux: retain existing path lexical/.git restrictions, then
  open the **workspace root directory** using `O_DIRECTORY|O_NOFOLLOW`.
  Traverse each filename component through `openat` on the preceding
  directory FD, with `O_NOFOLLOW` on **every component**. For the final
  component use `O_NONBLOCK` to prevent FIFO hangs, then require a regular
  file based on the opened FD (not a string-path stat). The already-open file
  handle supplies both metadata and bounded bytes.
- On Windows: use `FILE_FLAG_OPEN_REPARSE_POINT` and reject a reparse-point
  final handle; `GetFinalPathNameByHandleW` checks the **opened handle's**
  resolved location against the canonical workspace root to reject ancestor
  junction/symlink swaps before reading. Build and native regression required
  on Windows before claiming native test acceptance.
- Both text and image previews share the secure opener. Images reject files
  growing beyond the 8 MiB byte cap while being read; text remains capped
  at 128 KiB with binary/UTF-8 checks.

## Compatibility and boundaries

File previews intentionally **reject even in-project symlinks**; users may
open the real target directly. This is a stricter, secure-by-default change
from the earlier local-symlink preview behavior. Git's actual version
control operations, directory listing, file search cache and other preview
entrypoints still need separate TOCTOU risk assessment, not implicitly fixed
by this package. No user project or existing settings are rewritten.
The Rust functions are exposed to the frontend via existing Tauri commands;
no IPC names or payload contracts were changed.

## Verification gates

- Unit/contract: Rust text/image preview fixtures, canonicalization,
  .git variants, binary/size limits, local/external symlink and FIFO tests.
- E2E/security integration: targeted native Rust entrypoint reads while
  adversarial thread repeatedly swaps file/symlink; every returned byte
  string must equal in-workspace content, never outside secret; also image
  data path denies external symlink.
- Smoke: native Rust `workspace::tests::` focused run, Node static guard,
  project Node suite and exact-head CI on macOS, Windows and Linux.
- Native **installed GUI** and real multi-agent race testing: pending;
  the fixture cannot prove these.
- Independent source reviewer approval and R1 whole-workspace acceptance:
  pending; do not merge to protected develop solely because CI turns green.

## Accepted risk/next security tasks

Review and fix `list_files`, `file_search`, `git_stage` and
`git_diff` path operation races separately, including Windows reparse
and checkout replacement. Consider a common capability-dirfd abstraction
for all workspace file operations rather than reintroducing
canonicalize-then-reopen when adding new features.

**Lead acceptance:** source-only GO after tests; no native production claim.
