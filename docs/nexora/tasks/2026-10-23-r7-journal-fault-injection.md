# Checklist Planner — R7 Journal restart and failure-injection tests

**Date:** 2026-10-23, Asia/Bangkok. Created before source edits.
**Request:** Continue Nexora Code roadmap after R0–R6 source consolidation, reduce gaps in R7 evidence.
**Team Lead/worker:** assistant / TEAM_LEAD_SERIAL. No independent Agent Worker or approving human reviewer.
**Workspace:** /Volumes/Data/workspace/git/nexora-code-r7-journal-fault-tests, branch test/r7-journal-50-restarts.
**Base/target:** canonical integration/r0-r5-consolidated PR #21; merge the verified result onto that single develop-facing branch, do not create additional long-lived PRs.
**Priority:** P0 data integrity; R4 internal SQLite intent journal, R7 fault testing.
**Scope:** Rust isolated on-disk SQLite regression tests for 50 restart/recover cycles and deliberate event insert failures, audit atomicity/UNKNOWN no-replay. Docs, source CI evidence.
**No-go:** no external Agent/MCP effect, no live workflow executor, no actual app GUI install, no user's production data, no privileged actions, no fake OS kill or 8-worker soak claims.
**Safety:** SQLITE mock crash by closing/reopening DB is not an OS power-loss test. Triggered transactional insert failures are not physical disk faults. Do not pretend these satisfy the entire 50-cycle per-OS release gate.

| Task | Real owner | Ownership | Acceptance | State |
| --- | --- | --- | --- | --- |
| R7-J-01 | TEAM_LEAD_SERIAL | workflow_journal.rs tests | 50 on-disk SQLite reconnect cycles, zero replays | [x] |
| R7-J-02 | TEAM_LEAD_SERIAL | workflow_journal.rs tests | injected commit/event errors roll back intent/outcome/recovery | [x] |
| R7-J-03 | TEAM_LEAD_SERIAL | docs/nexora/R7-JOURNAL-FAULT-INJECTION.md | Evidence and limitations | [x] |
| R7-J-QA | TEAM_LEAD_SERIAL | complete diff/tests | focused/full CI/Clippy and canonical PR update | [x] |
| R7-J-REVIEW | Independent reviewer PENDING | GitHub review | required before develop merge | [ ] |

- [x] Planner prepared before source modifications.
- [x] Only temp SQLite files are touched; no real effects/permissions or auto replay.
- [x] Focused tests prove successful and failed durable state transitions.
- [ ] Exact HEAD six required OS/Frontend CI passes after integration.
- [ ] External independent approval needed before protected develop merge.

**Team Lead GO/NO-GO:** GO for source-level testing, NO-GO for protected merge or R8 release.

## Team Lead source inspection — 2026-10-23

- Verified source scope: new tests inside workflow_journal.rs; no changes to
  production state machine, migrations, IPC, UI, Agent, MCP or permissions.
- Tests: fifty closed/reopened SQLite cycles each fence exactly one unresolved
  intent, preserve successful dependency, reject replay/stale results, keep
  four audit records each. Final asserted counts: 50 UNKNOWN and 200 events.
- Injected TEMP trigger aborts audit INSERT, and verifies atomic rollback for
  initial intent, outcome and recovery; success after removing trigger.
- All data is in unique temporary SQLite databases; no user project files are
  modified.
- Mac Rust toolchain began recompiling full test executable (shared target);
  observed extended compilation. Actual PASS and Clippy are not claimed
  until process and GitHub CI return completed results.
- This is not power-loss/real process crash testing and does not satisfy the
  full per-OS 50-crash/cancel/resume or eight-worker two-hour R7 exit gate.
- Source GO for CI validation; NO-GO for R7/R8 and for develop merge without
  actual six checks and independent human approval.

## Exact local Mac source test evidence — 2026-10-23

- Focused: cargo test --locked workflow_journal --quiet: 6 PASS / 0 FAIL (1088 other tests filtered out), including two new 50-cycle and injected-failure tests.
- Full Rust unit tests: 1061 PASS / 0 FAIL / 33 IGNORED; Rust CLI integration: 1 PASS.
- All-target Clippy with --locked -D warnings: PASS.
- npm run build: PASS.
- npm test: 156 PASS / 0 FAIL.
- npm run test:release: 33 PASS / 0 FAIL.
- npm run test:nexora:r0: PASS.
- git diff --check: PASS.
- No GUI was installed/launched, no actual external effects were executed, and no physical OS kill/failure was simulated.
- This suite must still be accepted by GitHub 6 required checks for the canonical integration HEAD and independent reviewer before protected develop merge.


## G. NC-TEAM-003 — Subtask Verification Matrix (retrospective audit, not invented tests)

This planner existed **before NC-TEAM-003**. Historic checked [x] boxes
record earlier **narrow source-only** milestones and do NOT assert NC-TEAM-003
E2E/Smoke acceptance. Every historic subtask below is explicitly PENDING
until the Team Lead rechecks actual test command/OS/SHA/log and signs each.
Missing or unverified native evidence remains an acceptance NO-GO. The
retrospective entries are not fabricated executions or agent delegations.

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R7-J-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| R7-J-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| R7-J-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| R7-J-QA | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| R7-J-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |


**Team Lead GO/NO-GO (retrospective):** SOURCE PR GO for further review only.
Individual NC-TEAM-003 gates remain **PENDING** (not accepted); the prior
task's source-level status is not revoked or silently upgraded. Protected
merge and production NO-GO until genuine independent review / native gates.
