# Task Checklist Planner — R1 multi-session crash recovery batch

**Date:** 2026-10-23. **User request:** keep implementing sequential roadmap R1–R8.
**Team Lead:** TEAM_LEAD_SERIAL; no actual independent engineering agent workers or reviewer present. Separate R1 test worktree and branch; do not touch concurrent PR #34.
**Base:** test/r1-sigkill-store-recovery-20261023 (PR #33); stacked PR for this test-only increment.
**Security scope:** disposable SQLite file and spawned test binary; no real user session, network, secrets, or external agent invocation.
**DoD:** 8 concurrent *persisted* session rows and their tool activities in one child process, force OS child kill after durable marker, reopen and assert all 8 interrupted, no replay or duplicate tools on two fresh openings; 5 rounds per test. This is process-level persistence regression, NOT 8 actual live provider agents × 2h, installed GUI, real Windows device, or R7 acceptance.
**Cleanup:** explicitly-owned disposable child PID and own unique temporary databases, kill child even on early timeout; no global kill by name.

## B. Tasks and ownership

| ID | Work | Worker | Files | Dependency | State |
| --- | --- | --- | --- | --- | --- |
| R1-MULTI-01 | Add multi-session fixture writer with real SQLite 8 active sessions | TEAM_LEAD_SERIAL | src-tauri/src/storage.rs tests | PR #33 fixture | [x] |
| R1-MULTI-02 | OS child kill, re-open, assert 8 interruption/idempotence and tool state | TEAM_LEAD_SERIAL | src-tauri/src/storage.rs tests | R1-MULTI-01 | [x] |
| R1-MULTI-03 | Unit/Contract, real process E2E, Smoke and Team Lead audit | TEAM_LEAD_SERIAL | tests/docs | R1-MULTI-02 | [x] |
| R1-MULTI-04 | Push stacked PR and cross-OS CI handoff | TEAM_LEAD_SERIAL | PR | R1-MULTI-03 | [ ] |
| R1-MULTI-REVIEW | independent reviewer before protected merge/native R1 acceptance | INDEPENDENT_REVIEWER_PENDING | PR | R1-MULTI-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-MULTI-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 disposable child, 8 persisted active sessions with tool and ready marker; focused test 1/1; full Rust 1115/1115; Lead inspected separate process |
| R1-MULTI-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 real child kill 5 times, 8 sessions per DB, two restarts per cycle; 80 session state checks, no tool replay; Lead reviewed all assertions |
| R1-MULTI-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac Rust 1115 pass/0 fail/33 ignored, Node 208 PASS, Release 33 PASS and frontend build PASS; docs/nexora/R1-MULTI-SESSION-CRASH-EVIDENCE-2026-10-23.md. Team Lead GO narrow source only |
| R1-MULTI-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | hosted CI and PR pending |
| R1-MULTI-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | no outside review |

## C. Checklist

- [x] Planner created and committed before source modification; real worker scope
- [x] 8-session persisted process fixture, kill and repeated restart checked
- [x] Unit/Contract + E2E + Smoke per implementation task and Lead source review
- [ ] Exact-head CI and independent reviewer
- [ ] Installed Native GUI/Provider soak, full R1 exit

## E. Team Lead GO/NO-GO

**Team Lead verdict:** GO for narrow regression PR submission after tests. **GO/NO-GO:** protected merge and R1 phase acceptance NO-GO pending real native E2E and independent review.

## Local verification ledger

**Actual Mac M4:** targeted cargo test --lib r1_eight_persisted_sessions_survive_real_child_process_kill 1 PASS, five separate process kills and 8 persisted sessions verified after two restarts; full cargo test --lib 1115 PASS / 0 fail / 33 ignored; npm run build PASS; npm test 208 PASS; npm run test:release 33 PASS.

**Team Lead source GO for PR** source and fixture are disposable, no production API changed. No independent reviewer or actual installed GUI/PTY/provider acceptance. **GO/NO-GO:** PR source GO; merge, whole R1 exit and subsequent R2–R8 production acceptance NO-GO.
