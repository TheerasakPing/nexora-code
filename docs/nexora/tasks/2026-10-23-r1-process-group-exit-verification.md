# Task Checklist Planner — R1 successful helper exit process-tree verification

**Date:** 2026-10-23; new user instruction "ต่อ".
**Phase/priority:** R1/P0. Do not advance to R2 until R1 exit accepted.
**Baseline:** PR #26 at 0cba654 (already contains prior successful-helper process cleanup patch).
**Team Lead:** TEAM_LEAD_SERIAL; external multi-agent delegation and independent reviewer not available. No worker or reviewer fabricated.
**Task:** Independently verify newly added group cleanup with actual macOS process tests, code review, R1 backlog reconciliation, smoke, and exact-head CI. Repair only reproducible regressions.

## B. Task decomposition

| ID | Acceptance | Actual worker | Files | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-PG-01 | Verify helper-granchild group cleanup after success/failure and stale process count | TEAM_LEAD_SERIAL | src-tauri/src/workspace.rs, tests | baseline | [ ] |
| R1-PG-02 | Run full workspace/native Rust + Node/release contract and smoke on M4 | TEAM_LEAD_SERIAL | QA artifacts, planner | R1-PG-01 | [ ] |
| R1-PG-03 | Reconcile R1 exit against real Mac GUI/provider/session requirements and exact CI | TEAM_LEAD_SERIAL | phase audit / PR | R1-PG-02 | [ ] |
| R1-PG-REVIEW | Independent review before protected merge | INDEPENDENT_REVIEWER_PENDING | PR | R1-PG-03 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-PG-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Real Mac process-tree regression and owned-child cleanup evidence |
| R1-PG-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Full test commands and exit statuses |
| R1-PG-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | R1 gate reconciliation, CI exact SHA and native missing gates |
| R1-PG-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No real independent reviewer |

## C. Checklist

- [x] New planner committed before source changes
- [ ] Real macOS process lifecycle tests completed with proof
- [ ] Whole Rust+Node regressions and smoke pass
- [ ] Team Lead reviews complete subtask evidence
- [ ] Current-head 3-OS CI pass and independent review completed
- [ ] R1 native session/provider recovery acceptance passed

## E. Team Lead GO/NO-GO

**Team Lead verdict:** GO for R1 verification code/test work; NO-GO R1 exit, develop merge and production until native E2E and independent review.
