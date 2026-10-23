# R3 — Read-only Agent Git Worktree Isolation Audit

**Date:** 2026-10-22 · **Status:** Source-tested candidate, not native GUI E2E accepted
**Upstream:** Nexora Code fork of PandaCode (https://github.com/aicodeking/pandacode).

## Why this is needed

The Architect scheduler currently creates Agent Worker sessions using the Team
Lead's existing worktree_path. This is suitable for scheduler-coordinated
shared writes, but does NOT provide physical Git checkout isolation for each
Agent. The existing Git commands create_worktree and attach_worktree already
support separately created linked worktrees for tasks. A new automatic
provisioner cannot safely assume that each inherited path is already isolated.

The R3 Audit is the observable preflight step before introducing explicit
worker-specific provisioning and ownership/cleanup lifecycle.

## Architecture and behavior

- New read-only native IPC: architect_audit_isolation(sessionId).
- One bounded SQLite snapshot obtains the Team Lead checkout and DB-owned
  worker session references; the DB mutex is released BEFORE any Git calls.
- A native Git worktree verifier checks that a candidate is:
  1. A real directory, not a symlink alias for the checkout root.
  2. A directory with a linked-worktree .git metadata file.
  3. A separately registered, non-prunable, branch-bound checkout in the
     main repository's git worktree list, not merely a text path with .git.
  4. Not the lead's own checkout and not the same checkout as another worker.
- The audit classifies each worker:
  * shared_lead — worker uses the Team Lead checkout.
  * shared_workers — at least two workers share one non-lead checkout.
  * verified_linked_worktree — unique and registered Git linked checkout.
  * unverified — unavailable, unregistered, or suspicious checkout.
- Missing DB worker sessions and invalid/unavailable Git metadata fail to an
  unverified status. They do NOT cause silent positive "isolated" claims.
- The UI adds an opt-in Git worktree isolation details panel in Architect &
  Workers, with an explicit Verify worktrees button, loading/error status,
  per-worker result, and a stale revision warning. All native text and file
  paths are escaped before rendering; responses from closed/switched sessions
  are ignored.

## Threat model / missing roadmap steps

**IMPORTANT: registered Git worktrees are NOT OS security sandboxes.**
A provider terminal/tool may still write outside its declared files, and
another checkout can refer to shared data via mounts/symlink aliases or
external resources. This audit does not grant authorization or modify any
files, checkouts, tool access, user session status, processes or Team config.

The source in this PR does **not** automatically give a unique Git worktree
to each Agent. R3 is NOT complete. Next reviewed capabilities must include:

1. Opt-in per-worker provisioning of unique checkout/branch from an approved
   base, with a crash-consistent claim record and a recovery procedure.
2. Safe lifecycle/cleanup: do not remove dirty/unmerged worktrees; protect
   commits, coordinate process termination, and avoid orphaned branches.
3. Verified authorization for Git operations and OS-specific file tool
   restrictions; do not equate file ownership labels with filesystem ACLs.
4. Team Lead diff inspection, security approval and provenance before merging
   worker branches; prevent stale-generation effects.
5. macOS M4 / Windows 11 x64 / Linux native UI and 1/4/8/16-worker E2E,
   stress/crash recovery, CI, trusted independent reviewers and release gate.

## Source acceptance evidence

- 7 focused native audit/worktree tests PASS, including a real temporary Git
  worktree, a spoofed .git file, symlink alias, inherited checkout,
  unregistered checkout, multi-worker sharing, persisted Store read-only test.
- 5 focused UI renderer tests PASS, including hostile worker-provided HTML,
  missing values, changed revisions and unknown state.
- Full Mac Rust: 1044 PASS / 0 FAIL / 33 ignored; Rust CLI integration 1 PASS.
- Clippy all targets with denied warnings PASS.
- Node unit tests 139 PASS; release tests 31 PASS; TypeScript/Vite production
  build and R0 fork/update safety PASS.
- No actual desktop GUI was launched, no live Agents/MCP tools were executed,
  no OS sandbox or Native E2E acceptance claimed. GitHub CI and independent
  reviewer approval are separate prerequisites before protected merge.
