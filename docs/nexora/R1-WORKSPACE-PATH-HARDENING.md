# R1: Workspace path hardening — initial patch

Status: local engineering implementation. Not yet cross-platform or production certified.

## Finding

The old scoped-path guard rejected the exact path component .git but did not
reject .GIT, .Git or .gIt. Filesystems on the target macOS and Windows
machines may treat these names as the same Git metadata directory. The listing
code also excluded only the exact lowercase spelling.

## Fix

- Reject any ASCII case variation of .git components in native scoped paths,
  including nested segments and linked-worktree Git metadata pointers.
- Exclude case variants of Git, Mercurial and SVN metadata from file search.
- Exclude case variants of .git from native workspace directory listings.
- Preserve ordinary source files, .github and .gitignore.
- Do not change provider command dispatch or Git history.

## Verification (Apple M4 Mac)

- Scoped path and file-preview regression: PASS.
- Case-variant VCS exclusion regression: PASS.
- Full Rust tests: 984 passed, 0 failed, 33 ignored.
- CLI integration target additionally passed 1/1.
- R0 identity and update isolation: PASS.

## Open engineering risk

A scoped-path check followed by a separate file open can race a concurrent
rename or symlink swap (TOCTOU). This patch does not solve that class of flaw.
Complete descriptor-relative no-follow operations for each OS and adversarial
swap tests before calling R1 security hardened.
Windows/Linux native acceptance and desktop GUI remain pending.
