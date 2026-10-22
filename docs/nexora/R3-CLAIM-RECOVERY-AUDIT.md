# R3 — Interrupted Worktree Claim Recovery Audit

**2026-10-22 · Source-only recovery diagnostics · NO automatic cleanup**

Nexora Code is a fork of PandaCode. This phase follows the internal
worker worktree provisioner in PR #16, included in consolidated PR #18.

## Why recovery needs a separate gate

The R3 provisioner creates a durable PREPARING claim **before** it
attempts a Git branch/worktree operation. A process crash may leave
the claim with no branch, just a branch, or a registered checkout.
None of these observations proves an external effect was completely
reconciled or that a retry or deletion is safe.

The new internal-only recovery module reads existing claims and verifies
branch/checkout registration without changing SQLite, Git, or Agent state.

| Evidence | Meaning | Authorized action |
| --- | --- | --- |
| branch_missing | Exact local reserved branch ref is absent | Human review required; no automatic retry |
| branch_only | Branch exists, no linked checkout is registered | Human review; no automatic cleanup |
| linked_checkout_present | Branch is bound to a verified Git linked checkout | Inspect user files and running processes manually |
| unverified | Invalid Git metadata, unavailable project, prunable checkout, unsafe paths or errors | Treat as UNKNOWN, manual investigation required |

All PREPARING claims retain requiresOperatorReview=true regardless of
evidence. The audit is not an authorization mechanism and does **not**
modify the claim status. An operator cannot infer from this report
alone that a branch is safe to force-reset or remove.

## Implementation and safety limits

- Only SELECT queries on the existing journal table. If the table
  does not exist, the audit reports zero claims **without creating it**.
- Bounds inspection to 512 rows and uses the native Git subprocess
  runner with timeouts, capped output, disabled hooks and optional locks.
- Validates reserved local branch naming; inspects exact refs and
  bounded git worktree list --porcelain evidence.
- A missing worktree folder with prunable registration, a symlink,
  a forged .git metadata file, or Git command errors becomes unverified.
- No git worktree prune, remove, add, branch deletion, checkout, fetch,
  filesystem deletion, retries, or database writes occur in the audit.
- The module is **internal only**, not exposed through Tauri IPC or
  Agent tools. It cannot be used to launch an Agent.

## Operator runbook required before a future UI/host integration

1. Stop new task dispatch for the affected Team and verify every
   existing Agent process has exited.
2. Independently inspect the Git registration, checkout contents,
   dirty files/commits, session identity and durable claim generation.
3. Preserve unexpected files/commits in a reviewable location before
   considering any branch/worktree cleanup.
4. Require a verified owner approval for a specific planned mutation,
   logging exact project, worker, branch, SHA and desired resolution.
5. Implement a separate crash-consistent reconciler with rollback,
   stale-generation rejection and native macOS/Windows/Linux fault injection.
6. Re-run independent code/security review and CI before enabling live use.

## Tests

Focused Rust tests cover absence of a claims table, true branch-only
vs missing-branch evidence, a registered linked worktree, an orphaned
prunable checkout, invalid metadata/project IDs and unsafe branch strings.
Full Rust/Clippy/Node and all three GitHub OS runners must pass separately.

**Not production-complete:** automatic per-worker recovery, OS sandboxing,
actual provider idempotency, native app E2E, installer signing and authorized
R8 release remain separate outstanding Roadmap items.
