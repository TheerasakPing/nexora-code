# R3 — Cross-Team File Ownership Admission Guard

**Date:** 2026-10-22 · **Status:** source hardening, native reviewer/CI pending
**Heritage:** Nexora Code is a fork of PandaCode (https://github.com/aicodeking/pandacode).

## Current architectural fact

Existing Architect Teams support multiple agents and use file declarations to
schedule non-overlapping writers. Existing native logic prevents two teams in
the **exact same checkout root** from scheduling overlapping file scopes.
However, comparing roots by equality can miss a parent workspace nested inside
another registered workspace, even when two agents write the same physical file.

## Changes and behavior

- Add a conservative, bounded native **file-ownership conflict predicate**
  operating on canonical workspace roots and normalized relative file scopes.
- Integrate it into the existing native cross-team scheduler admission path,
  not just an untrusted frontend visualization.
- Block two concurrent writer assignments if their effective file scopes
  overlap through a common file/directory or nested workspace root.
- Preserve concurrency for independent Git worktrees/physical checkouts,
  disjoint paths, and read-only workers. Existing within-team policy remains.
- Treat no scope as an entire-workspace write claim. Compare components so
  src/a does not collide with src/ab; case fold conservatively across OSes.
- Reject untrustworthy or legacy malformed scopes: path traversal, absolute
  path, excessive lengths, backslashes, wildcard, colon/alternate stream or
  existing symlink components cannot demonstrate safe independent ownership.
- If the other team's workspace cannot be resolved, conservatively block
  concurrent writers rather than treating failure as no conflict.

## Threat model and limitations

**This is scheduler arbitration, not filesystem enforcement.** File scopes
are declared by workers and not necessarily enforced by external CLI tools
or provider runtimes. It does not prevent a malicious/buggy external agent
from editing a path outside its declared scope. Symlinks created after
preflight (TOCTOU) are not prevented by the scheduler; true path enforcement
requires handle-based filesystem guards / a sandbox and secure process
isolation on each supported OS.

**It does not create per-worker Git worktrees.** Automatic worktree creation,
branch ownership, cleanup, merge/review gates and isolation across filesystem
mount aliases remain separate R3 deliverables. A complete production version
requires: write lease persistence/crash cleanup, scoped tool host enforcement,
worktree lifecycle on 3 platforms, rollback and native E2E.

## Source validation

- Focused Rust tests cover physical nested workspaces, disjoint worktrees,
  directories vs prefixes, case variants, malformed legacy scopes, symlinks,
  read-only workers, empty scopes and length/capacity bounds.
- Existing Architect tests cover within- and cross-team shared-root scheduler
  serialization and read-only parallelism.
- Run full Rust test suite, Clippy (denied warnings), Git diff checks and
  GitHub 3-OS CI before review and merging.

No GUI was launched, no agents spawned, no MCP servers connected, no secrets
were accessed and no protected branch or production release was changed.

## Verified source results (Mac M4; 2026-10-22)

- New pure ownership unit suite: 7 PASS.
- Native shared-scheduler nested-workspace integration regression: 1 PASS.
- Full Rust tests: 1014 PASS / 0 FAIL / 33 ignored; CLI integration: 1 PASS.
- Rust Clippy all-targets -D warnings: PASS.
- Vite/TypeScript build, Node unit suite, release tests and R0 safety: PASS.
- Windows/Linux runner evidence for this exact feature commit: pending.
