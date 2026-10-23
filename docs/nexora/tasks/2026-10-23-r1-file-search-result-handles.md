# Task Checklist Planner — R1 File Search Cached Result Revalidation

**Date/request:** 2026-10-23; continue R1 sequential before R2.
**Team Lead:** TEAM_LEAD_SERIAL; no actual sub-agent task dispatch or independent reviewer in this chat.
**Base:** PR #26 R1 Unix directory handle change at 48d7629, separate integration worktree.
**Scope:** Unix cached search match revalidation through already-safe file/directory handles, plus real filesystem regression and docs. NOT a claim to fix scanner traversal races, Windows reparse handling, Git stage or all R1 requirements.
**Safeguards:** disposable temporary fixtures only, no user data/preferences/installed apps/credentials changed.

## B. Task breakdown

| ID | Acceptance | Real worker | File ownership | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-FSR-01 | Replace cached search pathname stat with Unix no-follow handle validation, keep Windows fallback explicit | TEAM_LEAD_SERIAL | src-tauri/src/workspace/file_search.rs | prior directory fix | [ ] |
| R1-FSR-02 | Test cached query after inside path swaps to external symlink; live scenario | TEAM_LEAD_SERIAL | src-tauri/src/workspace/file_search.rs | R1-FSR-01 | [ ] |
| R1-FSR-03 | Unit/E2E/Smoke, docs scope and Lead source review, exact CI after push | TEAM_LEAD_SERIAL | tests, docs and PR | R1-FSR-02 | [ ] |
| R1-FSR-REVIEW | Independent review + native OS acceptance before R1 complete | INDEPENDENT_REVIEWER_PENDING | PR | R1-FSR-03 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-FSR-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Rust matching + no-follow Unix dirfd checks pending |
| R1-FSR-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Real cached query symlink swap test pending |
| R1-FSR-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Rust, Node, exact PR CI pending |
| R1-FSR-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No independent reviewer |

## C. Checklist
- [x] Planner created before code edits; serial fallback disclosed
- [ ] Unit/Contract, E2E, Smoke each subtask accepted by Team Lead
- [ ] Real file search symlink switch test passes Mac M4
- [ ] CI 3-OS exact-head pass and PR handoff completed
- [ ] Independent reviewer and R1 native installed GUI/provider/session acceptance complete

## E. Initial decision
**Team Lead verdict:** GO source implementation and verification only. R1 Exit / protected merge / Production NO-GO pending independent review, other workspace scan and real native acceptance.
