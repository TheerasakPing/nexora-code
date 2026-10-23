# Task Checklist Planner — R1 provider exit descendant cleanup

**Date/request:** 2026-10-23, user's new “ทำต่อเลย”. Created before any source edits.
**R1 sequential phase gate:** Prioritize provider session lifecycle; do not claim R1 complete or proceed to R2 until installed Mac native UI/provider/session recovery, security and independent review pass.
**Team Lead:** TEAM_LEAD_SERIAL. The connected remote workspace has no task delegation tool; no actual engineer workers or independent reviewer assigned.
**Base:** PR #26 in /Volumes/Data/workspace/git/nexora-code-integration-20261023, previous Mac process-group bounded helper fix at 0cba654 and tests verified.
**Scope:** Provider child lifecycle only; real macOS process trees, no secret/provider credential or external model requests. Must leave current Mac app, user sessions, Keychain, terminals untouched.

## B. Team Lead subtasks and assignments

| ID | Acceptance criteria | Real worker | File ownership | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-PD-01 | Reproduce provider group children surviving when leader exits, add deterministic cleanup regression | TEAM_LEAD_SERIAL | src-tauri/src/providers.rs | None | [ ] |
| R1-PD-02 | Correct provider group shutdown while preserving stop/cancel paths, make failures bounded | TEAM_LEAD_SERIAL | src-tauri/src/providers.rs | R1-PD-01 | [ ] |
| R1-PD-03 | Unit/contract + native process-tree E2E + smoke + review, run project suite and update docs | TEAM_LEAD_SERIAL | src-tauri/src/providers.rs, docs/nexora | R1-PD-02 | [ ] |
| R1-PD-04 | PR push and exact-head 3-OS CI evidence, R1 exit audit | TEAM_LEAD_SERIAL | PR #26, planner | R1-PD-03 | [ ] |
| R1-PD-REVIEW | Independent reviewer approval before protected develop merge | INDEPENDENT_REVIEWER_PENDING | GitHub PR | R1-PD-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-PD-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Real child process + orphan regression pending |
| R1-PD-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Provider finish_child native regression pending |
| R1-PD-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Node/Rust suites and targeted OS process test pending |
| R1-PD-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Current SHA CI/review pending |
| R1-PD-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No real independent reviewer available |

## Acceptance checklist

- [x] Planner created first; ownership/serialization and scope established
- [ ] Native provider-group E2E test demonstrates no background descendant after provider exit
- [ ] Existing stop/timeout and finder PATH tests regressions clear
- [ ] Unit/Contract, E2E, Smoke and per-subtask lead review documented
- [ ] CI at exact PR SHA passes all required Mac/Windows/Linux jobs
- [ ] Native installed GUI/real provider stop/resume, crash restoration and independent reviewer accepted

## Team Lead GO/NO-GO

**Team Lead verdict:** GO limited source/test work. Protected develop merge, R1 exit and Production are NO-GO until real reviewer and native GUI/provider acceptance.
**Roadmap remaining baseline:** R1 seven detailed checklist boxes unchecked in older snapshot; R0-R8 61 detailed unaccepted boxes, 21/24 engineering acceptance packages, 7/7 branding epics; overlapping measures not percentages.
