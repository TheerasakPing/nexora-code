# Task Checklist Planner — R1 successful helper exit process-tree verification

**Date:** 2026-10-23; new user instruction "ต่อ".
**Phase/priority:** R1/P0. Do not advance to R2 until R1 exit accepted.
**Baseline:** PR #26 at 0cba654 (already contains prior successful-helper process cleanup patch).
**Team Lead:** TEAM_LEAD_SERIAL; external multi-agent delegation and independent reviewer not available. No worker or reviewer fabricated.
**Task:** Independently verify newly added group cleanup with actual macOS process tests, code review, R1 backlog reconciliation, smoke, and exact-head CI. Repair only reproducible regressions.

## B. Task decomposition

| ID | Acceptance | Actual worker | Files | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-PG-01 | Verify helper-grandchild group cleanup after success/failure and stale process count | TEAM_LEAD_SERIAL | src-tauri/src/workspace.rs, tests | baseline | [x] |
| R1-PG-02 | Run full workspace/native Rust + Node/release contract and smoke on M4 | TEAM_LEAD_SERIAL | QA artifacts, planner | R1-PG-01 | [x] |
| R1-PG-03 | Reconcile R1 exit against real Mac GUI/provider/session requirements and exact CI | TEAM_LEAD_SERIAL | phase audit / PR | R1-PG-02 | [ ] |
| R1-PG-REVIEW | Independent review before protected merge | INDEPENDENT_REVIEWER_PENDING | PR | R1-PG-03 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-PG-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4: targeted Rust test successful_bounded_process_reaps_background_process_group 1/1 PASS; actual sh background sleep process is reaped after parent success; Team Lead reviewed process group cleanup diff, commit 0cba654 |
| R1-PG-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4: cargo test --locked --lib 1068 passed, 33 ignored, 0 failed; npm test 178/178 passed; full CLI and native Rust test smoke; E2E scoped to native helper process, not installed GUI |
| R1-PG-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | R1 gate reconciliation, CI exact SHA and native missing gates |
| R1-PG-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No real independent reviewer |

## C. Checklist

- [x] New planner committed before source changes
- [x] Real macOS helper process lifecycle tests completed with proof
- [x] Whole Rust+Node regressions and smoke pass
- [ ] Team Lead reviews complete subtask evidence
- [ ] Current-head 3-OS CI pass and independent review completed
- [ ] R1 native session/provider recovery acceptance passed

## E. Team Lead GO/NO-GO

**Team Lead verdict:** GO for R1 verification code/test work; NO-GO R1 exit, develop merge and production until native E2E and independent review.

## Team Lead evidence and remaining R1 exit blockers

- **Mac M4 focused integration E2E:** `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib successful_bounded_process_reaps_background_process_group -- --nocapture` — **1 PASS**. Shell exits 0 after backgrounding `sleep 30`; process group cleanup is checked with `ps` and must leave no active descendant.
- **Full native-library smoke:** `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib --quiet` — **1068 passed, 0 failed, 33 ignored**.
- **Frontend unit/contract smoke:** `npm test` — **178 passed, 0 failed**.
- **Source code Lead review:** existing 0cba654 patch calls `stop_process_group` on successful Unix output capture, after pipes are closed. Focused native test independently exercised the patch on M4, not an invented multi-agent assignment.
- **Still unverified:** installed Native GUI/session/provider/terminal E2E, process crash/resume under load, Windows-installed child process cleanup, *actual independent reviewer*, and exact-head 3-OS CI for latest planner commit.
- **Verdict:** Team Lead GO for narrow source/test milestone. R1 EXIT **NO-GO**; do not mark R2 started or R1 complete.
