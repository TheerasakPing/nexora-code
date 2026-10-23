# Task Checklist Planner — R1 Process Group Lifecycle

Date: 2026-10-23. New user task: continue sequential roadmap R1 to R8.
Lead: TEAM_LEAD_SERIAL. Independent reviewer pending. No real worker delegation API available.
Scope: Add lifecycle test coverage and fix validated R1 process-group problems.
Safety: Preserve pre-existing uncommitted workspace.rs and node_modules, user data and installed app.

## B. Team Lead task breakdown

| ID | Deliverable | Real Worker | Owned files | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-PG-01 | Validate successful parent exit with a background descendant | TEAM_LEAD_SERIAL | providers.rs tests | none | [x] |
| R1-PG-02 | Validate cancellation and forced-shutdown descendant cleanup | TEAM_LEAD_SERIAL | providers.rs tests | R1-PG-01 | [x] |
| R1-PG-03 | Unit, native process E2E, smoke, source review and CI | TEAM_LEAD_SERIAL | tests and planner | R1-PG-02 | [x] |
| R1-PG-04 | PR handoff and R1 status accounting | TEAM_LEAD_SERIAL | PR #26 | R1-PG-03 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-PG-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | macOS M4; real POSIX shell exits 0 with live background child; guard cleans it; 3/3 iterations PASS; cargo test --lib providers::tests::successful_provider_parent_exit_cleans_up_owned_background_descendant; Lead reviewed 2026-10-23 |
| R1-PG-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | macOS M4; cancellation subprocess cleanup PASS, forced-shutdown Rust regression PASS via full library; boundary guard test PASS; Lead reviewed source security boundary 2026-10-23 |
| R1-PG-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | macOS M4: full Rust lib 1071 PASS 33 ignored; Node 181 PASS; release 33 PASS; Vite build PASS; cargo check --bin nexora-code PASS; test real OS subprocess E2E; Lead diff reviewed |
| R1-PG-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Exact-head CI and reviewer needed |

## C. Checklist

- [x] New planner before source edits
- [x] Actual assignment is TEAM_LEAD_SERIAL
- [x] Focused process E2E plus smoke PASS at real macOS subprocess boundary (not native GUI)
- [x] Team Lead inspected code and local process tests; GitHub exact-head CI / PR record still PENDING
- [ ] Genuine independent review before protected merge

## E. GO/NO-GO

**Team Lead verdict:** GO for PR submission of narrow R1 source work. NO-GO R1 phase exit and R2 advancement until real native and review gates pass.


## Local test ledger and handoff — macOS Apple M4

- 2026-10-23, isolated worktree feat/r1-lifecycle-20261023; modified only src-tauri/src/providers.rs, this planner, source contract test and docs.
- **Unit/Contract:** Group ID guard rejects signed-32-bit overflow and special IDs 0/1 before Unix signal delivery; Rust invalid-id regression PASS; Node 3 source assertions PASS.
- **Native process E2E:** Rust tests start real test-owned shell/process groups, verify background child alive after parent exits successfully, then guard cleans it; 3/3 repeats PASS. Real cancellation task cleanup PASS, no registry leaks. Existing forced shutdown regression also passed under full library suite.
- **Full library:** cargo test --locked --lib: 1071 passed, 0 failed, 33 ignored (1104 total).
- **Smoke:** cargo check --locked --bin nexora-code PASS; npm run build PASS (existing chunk-size warnings only), npm test 181/181 PASS, npm run test:release 33/33 PASS.
- **Known initial test setup:** Node tests initially failed 2 bundle fixtures because this new isolated worktree had no dist directory; built frontend first, reran same tests and obtained 181/181 PASS. Rust first compile also had a fixture quotation typo, corrected before PASS. These were tests/build setup, not runtime behavior.
- **Lead review:** verified Linux/macOS test portability, same owned process group only, no handling of unrelated processes, registry cleanup, no changes to user files/PI credentials, and no modification of dirty older worktree. Source review GO for narrow process package.
- **R1 outstanding:** session checkpoint/recovery, UI/PTY/provider E2E, out-of-group detached children, native Finder acceptance, workspace full TOCTOU, independent reviewer, exact-head CI.
- **R2–R8:** queued sequentially; R1 exit still NO-GO.
- **Exact head after submission:** PENDING until pushed and verified; do not count CI from an earlier SHA.
