# Task Checklist Planner — R1 Process Group Lifecycle

Date: 2026-10-23. New user task: continue sequential roadmap R1 to R8.
Lead: TEAM_LEAD_SERIAL. Independent reviewer pending. No real worker delegation API available.
Scope: Add lifecycle test coverage and fix validated R1 process-group problems.
Safety: Preserve pre-existing uncommitted workspace.rs and node_modules, user data and installed app.

## B. Team Lead task breakdown

| ID | Deliverable | Real Worker | Owned files | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-PG-01 | Validate successful parent exit with a background descendant | TEAM_LEAD_SERIAL | providers.rs tests | none | [ ] |
| R1-PG-02 | Validate cancellation and forced-shutdown descendant cleanup | TEAM_LEAD_SERIAL | providers.rs tests | R1-PG-01 | [ ] |
| R1-PG-03 | Unit, native process E2E, smoke, source review and CI | TEAM_LEAD_SERIAL | tests and planner | R1-PG-02 | [ ] |
| R1-PG-04 | PR handoff and R1 status accounting | TEAM_LEAD_SERIAL | PR #26 | R1-PG-03 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-PG-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Mac native process fixture |
| R1-PG-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Controlled native process lifecycle fixture |
| R1-PG-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Node and Rust suite reports |
| R1-PG-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Exact-head CI and reviewer needed |

## C. Checklist

- [x] New planner before source edits
- [x] Actual assignment is TEAM_LEAD_SERIAL
- [ ] Focused process E2E plus smoke PASS
- [ ] Team Lead source review and PR tests recorded
- [ ] Genuine independent review before protected merge

## E. GO/NO-GO

Team Lead verdict: GO for R1 source work. NO-GO R1 phase exit and R2 advancement until real native and review gates pass.
