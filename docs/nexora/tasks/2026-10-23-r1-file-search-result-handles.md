# Task Checklist Planner — R1 File Search Cached Result Revalidation

**Date/request:** 2026-10-23; continue R1 sequential before R2.
**Team Lead:** TEAM_LEAD_SERIAL; no actual sub-agent task dispatch or independent reviewer in this chat.
**Base:** PR #26 R1 Unix directory handle change at 48d7629, separate integration worktree.
**Scope:** Unix cached search match revalidation through already-safe file/directory handles, plus real filesystem regression and docs. NOT a claim to fix scanner traversal races, Windows reparse handling, Git stage or all R1 requirements.
**Safeguards:** disposable temporary fixtures only, no user data/preferences/installed apps/credentials changed.

## B. Task breakdown

| ID | Acceptance | Real worker | File ownership | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-FSR-01 | Replace cached search pathname stat with Unix no-follow handle validation, keep Windows fallback explicit | TEAM_LEAD_SERIAL | src-tauri/src/workspace/file_search.rs | prior directory fix | [x] |
| R1-FSR-02 | Test cached query after inside path swaps to external symlink; live scenario | TEAM_LEAD_SERIAL | src-tauri/src/workspace/file_search.rs | R1-FSR-01 | [x] |
| R1-FSR-03 | Unit/E2E/Smoke, docs scope and Lead source review, exact CI after push | TEAM_LEAD_SERIAL | tests, docs and PR | R1-FSR-02 | [ ] |
| R1-FSR-REVIEW | Independent review + native OS acceptance before R1 complete | INDEPENDENT_REVIEWER_PENDING | PR | R1-FSR-03 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-FSR-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 Unix cached file and folder results anchored to openat verified handles; 12/12 scoped Rust suite; Team Lead inspected diff |
| R1-FSR-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 actual temporary filesystem: external symlink exclusion and internal alias after rename; cache TTL can rescan detached legitimate path; Rust 12/12 PASS |
| R1-FSR-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 Rust 1075 PASS/0 FAIL/33 ignored; Node 187/187, Release 33/33; source policy 1/1; exact-head CI remains separate pending acceptance |
| R1-FSR-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No independent reviewer |

## C. Checklist
- [x] Planner created before code edits; serial fallback disclosed
- [x] Unit/Contract, scoped filesystem E2E, Smoke for R1-FSR-01/02/03 source accepted by Team Lead
- [x] Real file search symlink switch test passes Mac M4; cache-rescan case handled
- [ ] CI 3-OS exact-head pass and PR handoff completed
- [ ] Independent reviewer and R1 native installed GUI/provider/session acceptance complete

## E. Initial decision
**Team Lead verdict:** GO source implementation and verification only. R1 Exit / protected merge / Production NO-GO pending independent review, other workspace scan and real native acceptance.

**Team Lead verdict:** GO source submission, NO-GO protected merge and R1 phase exit; independent review, Windows and native GUI E2E PENDING. See docs/nexora/R1-FILE-SEARCH-CACHED-HANDLE-2026-10-23.md.

**GO/NO-GO final gate:** GO for source PR review with scoped real Mac filesystem E2E only;
NO-GO R1 phase completion, protected merge and production until Windows path
security, scanner traversal, live native installed GUI/provider E2E and genuine
independent reviewer are accepted. Exact-head CI for this branch PENDING.
