# Task Checklist Planner — R1 provider exit descendant cleanup

**Date/request:** 2026-10-23, user's new “ทำต่อเลย”. Created before any source edits.
**R1 sequential phase gate:** Prioritize provider session lifecycle; do not claim R1 complete or proceed to R2 until installed Mac native UI/provider/session recovery, security and independent review pass.
**Team Lead:** TEAM_LEAD_SERIAL. The connected remote workspace has no task delegation tool; no actual engineer workers or independent reviewer assigned.
**Base:** PR #26 in /Volumes/Data/workspace/git/nexora-code-integration-20261023, previous Mac process-group bounded helper fix at 0cba654 and tests verified.
**Scope:** Provider child lifecycle only; real macOS process trees, no secret/provider credential or external model requests. Must leave current Mac app, user sessions, Keychain, terminals untouched.

## B. Team Lead subtasks and assignments

| ID | Acceptance criteria | Real worker | File ownership | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-PD-01 | Reproduce provider group children surviving when leader exits, add deterministic cleanup regression | TEAM_LEAD_SERIAL | src-tauri/src/providers.rs | None | [x] |
| R1-PD-02 | Correct provider group shutdown while preserving stop/cancel paths, make failures bounded | TEAM_LEAD_SERIAL | src-tauri/src/providers.rs | R1-PD-01 | [x] |
| R1-PD-03 | Unit/contract + native process-tree E2E + smoke + review, run project suite and update docs | TEAM_LEAD_SERIAL | src-tauri/src/providers.rs, docs/nexora | R1-PD-02 | [x] |
| R1-PD-04 | PR push and exact-head 3-OS CI evidence, R1 exit audit | TEAM_LEAD_SERIAL | PR #26, planner | R1-PD-03 | [ ] |
| R1-PD-REVIEW | Independent reviewer approval before protected develop merge | INDEPENDENT_REVIEWER_PENDING | GitHub PR | R1-PD-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-PD-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 Rust native test 1/1 PASS: actual sh parent exits with sleep 30 holding stdout, watchdog returns in 1.22s, child reaped; Lead reviewed source 2026-10-23 |
| R1-PD-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 tokio child status poll 200ms + 1s drain + existing finish_child cleanup; rust full suite 1069 pass, 33 ignored; child kill E2E PASS; Lead reviewed stop-path safety |
| R1-PD-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 npm test 180/180, release 33/33, Vite PASS, Rust 1069/1069 (33 ignored), cargo check --bin nexora-code PASS; native mock-process integration, not installed GUI |
| R1-PD-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Current SHA CI/review pending |
| R1-PD-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No real independent reviewer available |

## Acceptance checklist

- [x] Planner created first; ownership/serialization and scope established
- [x] Native provider-group E2E test demonstrates no background descendant after provider exit
- [x] Existing stop/timeout and finder PATH tests regressions clear (Rust full lib 1069 PASS and 33 ignored; installed Finder live test remains PENDING)
- [x] Unit/Contract, scoped native process E2E, Smoke and source Lead Review for R1-PD-01/02/03 documented
- [ ] CI at exact PR SHA passes all required Mac/Windows/Linux jobs
- [ ] Native installed GUI/real provider stop/resume, crash restoration and independent reviewer accepted

## Team Lead GO/NO-GO

**Team Lead verdict:** GO limited source/test work. Protected develop merge, R1 exit and Production are NO-GO until real reviewer and native GUI/provider acceptance.
**Roadmap remaining baseline:** R1 seven detailed checklist boxes unchecked in older snapshot; R0-R8 61 detailed unaccepted boxes, 21/24 engineering acceptance packages, 7/7 branding epics; overlapping measures not percentages.

## Team Lead verification evidence — 2026-10-23

- Mac M4 actual child test command: `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib exited_provider_with_inherited_stdout_does_not_hang_and_reaps_descendant -- --nocapture`: **1 PASS / 0 FAIL**, finished in **1.22s**. Creates throwaway `sh` that backgrounds `sleep 30`; actual child inherits stdout and stderr, parent exits. No model login, network call or user workspace.
- Native process E2E: watchdog breaks the fake provider session before five seconds with actionable diagnostic, and `finish_child` removes the remaining owned child; test polls `ps` and rejects active leaked descendants. This is NOT installed native GUI/provider-login/session-restore acceptance.
- Node static/contract: `node --test tests/nexora-r1-provider-exit-watch.test.mjs`: **2 PASS**. Full `npm test`: **180 PASS**. `npm run test:release`: **33 PASS**.
- Mac Rust: `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib --quiet`: **1069 PASS, 0 FAIL, 33 ignored**, `cargo check --locked --bin nexora-code`: **PASS**. Vite `npm run build`: **PASS** with existing chunk-size warnings.
- Team Lead review: normal stdout result handling preserved while child remains running; finished leader receives a one-second final-frame grace and then bounded group cleanup; stop path returns Ok; no user data or credentials touched.
- Current-head CI/Windows/Linux native job result still **PENDING until pushed and checked**; no assumption from prior CI. Genuine independent reviewer **not assigned**.
- **GO:** source-only R1 provider exit watchdog PR handoff. **NO-GO:** R1 overall phase exit, develop merge, 3-OS native GUI and Production Release.
