# Task Checklist Planner — R1 Workspace directory and search safety

**Date:** 2026-10-23. **Request:** Continue R1 sequentially toward R8, no skipping failed phase acceptance.
**Team Lead:** TEAM_LEAD_SERIAL (no actual task-delegation API or independent reviewer available in this session).
**Base:** Integration source 093f397 following two safely merged provider lifecycle tracks. PR #26 remote head 2c33f95 at beginning of this task.
**Scope:** One step of R1 directory listing/search security using real filesystem tests; integrate previous R1 provider fixes and submit exact-head CI.
**Threat model:** Directory symlink swap after path validation or during enumeration may expose external filenames; cached search may return stale/outside paths. Enforce that listing never walks outside a project and avoid inventing native GUI acceptance.
**Constraints:** No writes to user home projects or installed app, no credential access, preserve legacy public IPC. No unsafe global security bypass. Windows CI compilation/smoke is not native Windows GUI E2E.

## B. Task assignments and acceptance

| ID | Acceptance / owned area | Actual worker | Depends | Status |
| --- | --- | --- | --- | --- |
| R1-DIR-01 | Audit list_files plus file_search symlink/race behavior and document precise scope | TEAM_LEAD_SERIAL | None | [ ] |
| R1-DIR-02 | Implement bounded safe directory listing for supported OS with no-follow scoped traversal | TEAM_LEAD_SERIAL | R1-DIR-01 | [ ] |
| R1-DIR-03 | Real filesystem symlink adversarial E2E, unit, smoke and Team Lead source review | TEAM_LEAD_SERIAL | R1-DIR-02 | [ ] |
| R1-DIR-04 | Integrate the already merged R1 provider patch, PR handoff and exact-head CI evidence | TEAM_LEAD_SERIAL | R1-DIR-03 | [ ] |
| R1-DIR-REVIEW | Independent approval before protected develop merge | INDEPENDENT_REVIEWER_PENDING | R1-DIR-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-DIR-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Scoping audit and prior tests inspection |
| R1-DIR-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Native directory-handle test and Windows compile check |
| R1-DIR-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Temp filesystem race tests, Node, Rust, smoke |
| R1-DIR-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | PR #26 exact-head 9 CI checks and signoff pending |
| R1-DIR-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No independent reviewer available |

## Acceptance checklist

- [x] Fresh planner prior to source edits; Team Lead serial fallback recorded
- [ ] Confirm threat and testable boundary before modifying list implementation
- [ ] Run native filesystem E2E/Smoke with actual symlink adversary and verify fail-closed behavior
- [ ] All new source changes reviewed by Team Lead and passing current-head tests
- [ ] Nine exact-head GitHub Actions checks PASS
- [ ] Independent review, native installed GUI/session acceptance, R1 exit GO

## Team Lead handoff

**Team Lead verdict:** GO for narrow R1 code/test work; protected branch merge, R1 phase exit and Production NO-GO until independent reviewer and native acceptance.
**Historic overlapping backlog:** engineering 21/24 packages pending, 7/7 branding signoff pending, 61 R0–R8 detailed acceptance boxes unaccepted. These are not additive.
