# R3 — Per-Worker Git Worktree Provisioning Foundation

**Date:** 2026-10-22. **State:** internal source-only primitive; production execution disabled.

## Delivered

Nexora Code already had a protected Git Worktree creator for individual task
sessions. This R3 change stages an **internal-only** Agent Worker provisioner
built on that creator, following the R3 Isolation Audit. It is not registered
as a Tauri IPC action and is not callable by the renderer, agent tools or MCP.

The internal function requires:
- A verified Architect Team and worker membership in the SQLite mapping.
- A paused Team with no active workers or pending setup.
- A queued, writable, non-abandoned Worker with an idle child Session.
- Matching parent/worker project and inherited checkout, not an existing
  separately provisioned Worktree.
- A clean Git repository at the Team Lead checkout and a precise committed
  HEAD as immutable branch base (uncommitted changes are not copied).
- A unique derived branch for the intended Worker; existing branches/claims
  cannot be silently reused.
- Before any Git side effect, a durable SQLite claim is recorded as PREPARING.
  The Git creator makes a new physical linked worktree; verification checks
  native Git registration rather than trusting the path string.
- Child Session linked-worktree path and branch, claim ATTACHED and Team
  revision are committed in one SQLite transaction.

## Failure and crash model

A Git failure or process crash may leave a PREPARING claim and either an
uncreated or partially created Git worktree. These outcomes are **UNKNOWN** to
the caller. They are not silently retried or cleaned up. An internal
unresolved_claims query exposes the durable entries for a later human-driven
reconciler, including the Worker Session, branch and base SHA.

This staged primitive makes no automatic merge, force-push, branch deletion
or worktree removal. It does not reuse approvals between Agent generations.

## Security limitations and next gates

**The method is deliberately not callable from the application UI or agent
harness.** A paused status alone is not an operator authorization or proof
of process isolation. Before exposing a user-controlled button:

1. Add real native operator consent and permission records, not a boolean
   submitted from the renderer. Recheck team generation and process ledger.
2. Add a crash-safe reconciliation UI to inspect PREPARING vs Git reality,
   retain unmerged/dirty changes, and require explicit operator decisions.
3. Run Git operations in an owned dedicated blocking executor with cancellation
   and an integration contract for the process/session scheduler.
4. Scope external provider file operations with actual OS sandbox/ACL measures.
   A separate Git checkout does not restrict access to other folders.
5. Verify real worker startup, 1/4/8/16 concurrency, fault injection,
   50 restart cycles and native Mac M4/Windows 11 x64/Linux E2E.
6. Obtain independent reviewer approval and enforce protected-branch CI.

No actual user project worktrees were provisioned, and no production Agent
operations, GUI, MCP or installer were started during this change.

## Tests

The focused Rust suite provisions a real checkout in a temporary Git
repository, verifies linked Git registration and persisted worker ownership,
rejects unpaused/active/dirty teams, blocks unsafe duplicate or interrupted
claims, and exercises invalid worker membership. See the companion Checklist
Planner for exact final test counts.
