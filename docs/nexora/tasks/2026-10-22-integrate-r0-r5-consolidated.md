# Task Checklist — Consolidate R0–R5 (R3 Crash Audit + R5 Studio)

**Date:** 2026-10-22, authored before source modification.
**User goal:** Continue roadmap while keeping working features integrated and reviewed.
**Team Lead:** acting assistant, TEAM_LEAD_SERIAL. No real separately dispatched Agent Workers or independent Reviewer.
**Base:** origin/integration/r0-r4-consolidated (PR #18, six original checks success).
**Branch:** integration/r0-r5-consolidated; isolated nexora-code-r0-r5-integration worktree.
**Goal:** Combine original PR #19 R3 orphan-claim crash audit and PR #20 R5 read-only Workflow Studio with PR #18 for one reviewable PR, without changing their branches.
**Scope:** source merge, minimal conflict resolution, tests, GitHub CI. No native installer, live MCP/Agent, filesystem sandbox, privileged workflow or protected-branch override.
**Acceptance:** preserve full source ancestry, combine all tests, app build, required CI on final SHA, true independent Reviewer before protected develop merge.
**Priority:** P1 R3 reliability/R5 usability, P0 safe governance.
**DoD:** source merge is conflict-free or reviewed, Mac tests clean, six required cross-platform CI gates, actual reviewer approval, no regression. Production release requires separate R6–R8 and native acceptance.

| Task | Actual worker | Scope | Status |
| --- | --- | --- | --- |
| R05-INT-01 | TEAM_LEAD_SERIAL | Include R3 crash-claim audit PR #19 | [x] |
| R05-INT-02 | TEAM_LEAD_SERIAL | Include R5 Workflow Studio PR #20 | [x] |
| R05-INT-03 | TEAM_LEAD_SERIAL | Review merged diff, run full frontend/Rust tests | [x] |
| R05-INT-04 | TEAM_LEAD_SERIAL | Push integration PR with SHA and CI evidence | [ ] |
| R05-INT-05 | INDEPENDENT_REVIEWER_PENDING | Actual GitHub review before protected merge | [ ] |

- [x] Confirm ancestry and any conflict resolution.
- [x] Validate Node tests, release tests, R0 and TypeScript/Vite Build.
- [x] Validate locked full Rust and Clippy denied warnings on integrated SHA.
- [ ] Check GitHub 3-OS CI on integrated SHA and reviewer gate.
- [ ] Do not claim native GUI app, live Executor or R8 production readiness.

**Team Lead verdict:** GO for source integration PR submission; NO-GO for protected develop merge and Production Release without genuine independent approval and native OS acceptance.

## Team Lead acceptance evidence, Mac M4, 2026-10-22

- Actual worker mode: TEAM_LEAD_SERIAL; no separately delegated Agent Coder or independent GitHub Reviewer was invoked.
- Base was PR #18 (R0-R4 consolidation). PR #19 orphan-claim audit and PR #20 Workflow Studio source histories both merged with no conflicts; ancestry checked with git merge-base.
- Full Rust tests: 1056 PASS, 0 FAIL, 33 ignored. Rust CLI integration: 1 PASS.
- All-target Rust Clippy with -D warnings: PASS.
- Full Node tests: 151 PASS, 0 FAIL; release tests 31 PASS.
- TypeScript/Vite production build and R0 fork/updater identity verification: PASS.
- Team Lead inspected orphan claim read-only code, strict text-only preview UI, no actual Executor/Agent/MCP/Git mutation, no native UI acceptance claim.
- The existing Vite chunk-size advisory is nonfatal and remains an optimization task.
- Exact-head GitHub Windows/macOS/Linux CI and genuine approving Reviewer: required, pending at PR submission.
- R3 per-agent OS sandbox, R4 authorized Executor, Mac/Windows/Linux native GUI tests, installers/signing/rollback and R6-R8 production gates remain incomplete. No release approval.
