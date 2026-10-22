# R1 — Windows Turn Change Snapshot CI Stabilization

**Date:** 2026-10-22
**Integration base:** feat/integrate-governance-windows-gates (PR #7)
**Source attribution:** Nexora Code is a fork of PandaCode — https://github.com/aicodeking/pandacode

## Observed Windows regression

[CI Run 39733118915](https://github.com/TheerasakPing/nexora-code/actions/runs/39733118915)
failed the Windows Rust tests:

- snapshot_limits_are_partial_without_false_deletions_or_unchanged_dirty_files
  (expected 1 changed entry, observed 0)
- retained_summary_survives_patch_eviction_and_duplicate_ids_roll_back
  (expected a captured patch, received None)

These failures appeared in a heavily parallel test run; the log did not report
the capture's internal timeout/error. Their exact Windows root cause is
**not independently proven**.

## Bounded mitigation

The **test-only** workspace turn_snapshot_paths helper previously launched a
Git process even for temporary non-Git fixture directories. Git's Windows
process startup can consume much of the existing 500 ms fixture inventory
deadline, leading to incomplete change snapshots under CI load.

The helper now checks the directory **and every parent** for a .git
directory or worktree metadata file. If none exists, it returns Ok(None)
immediately so existing native directory inventory can enumerate files;
it does not launch Git merely to discover a non-repository. For actual Git
repositories, nested subdirectories, worktrees and tracked/ignored files, the
existing bounded read-only Git operations are unchanged.

This is a test/fixture inventory optimization, **not a production timeout
increase or a bypass of file containment checks**. No changes to snapshot
ownership, change persistence or authorized tool behavior.

## Regression evidence

- Non-Git temporary checkout detects a modified nested file using the native
  inventory, without a needless Git discovery subprocess.
- Git subdirectory under a parent repository still finds tracked/untracked
  files and respects ignored files.
- Mac Apple Silicon full turn_changes suite: 35 PASS / 0 FAIL.
- Full local Rust: 1008 PASS, 0 FAIL, 33 ignored + 1 CLI integration PASS.
- Rust all-target Clippy with denied warnings: PASS.
- **Windows CI still required** to verify whether the observed two failures
  disappear; no claim of Windows success until the matching commit's CI runs.

No GUI, installation, live MCP/Agent processes, protected branch merge or
production release was performed.
