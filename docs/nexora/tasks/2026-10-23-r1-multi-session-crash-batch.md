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
| R1-MULTI-01 | Add multi-session fixture writer with real SQLite 8 active sessions | TEAM_LEAD_SERIAL | src-tauri/src/storage.rs tests | PR #33 fixture | [ ] |
| R1-MULTI-02 | OS child kill, re-open, assert 8 interruption/idempotence and tool state | TEAM_LEAD_SERIAL | src-tauri/src/storage.rs tests | R1-MULTI-01 | [ ] |
| R1-MULTI-03 | Unit/Contract, real process E2E, Smoke and Team Lead audit | TEAM_LEAD_SERIAL | tests/docs | R1-MULTI-02 | [ ] |
| R1-MULTI-04 | Push stacked PR and cross-OS CI handoff | TEAM_LEAD_SERIAL | PR | R1-MULTI-03 | [ ] |
| R1-MULTI-REVIEW | independent reviewer before protected merge/native R1 acceptance | INDEPENDENT_REVIEWER_PENDING | PR | R1-MULTI-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-MULTI-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | actual child fixture, 8 records pending |
| R1-MULTI-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | kill + restart evidence pending |
| R1-MULTI-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Mac Rust/Node/Release + source audit pending |
| R1-MULTI-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | hosted CI and PR pending |
| R1-MULTI-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | no outside review |

## C. Checklist

- [x] Planner created and committed before source modification; real worker scope
- [ ] 8-session persisted process fixture, kill and repeated restart checked
- [ ] Unit/Contract + E2E + Smoke per task and Lead source review
- [ ] Exact-head CI and independent reviewer
- [ ] Installed Native GUI/Provider soak, full R1 exit

## E. Team Lead GO/NO-GO

**Team Lead verdict:** GO for narrow regression PR submission after tests. **GO/NO-GO:** protected merge and R1 phase acceptance NO-GO pending real native E2E and independent review.
