# Task Checklist Planner — R1 npmrc cross-platform CI repair

**Date:** 2026-10-23. **User work request:** “ต่อเลย”; continue sequential R1→R8 via real evidence and do not skip failing acceptance.
**Branch/worktree:** fix/r1-finder-npmrc-bounded-20261023 /Volumes/Data/workspace/git/nexora-code-r1-npmrc-bounded-20261023 (PR #34 stacked on #33).
**Team Lead/worker:** ChatGPT Integration Lead, actual TEAM_LEAD_SERIAL; no independent reviewer or multi-agent delegation available.
**Observed blockers (before source change):** PR #34 commit 2bf5d1b had Windows Clippy -D warnings failure at providers/launch.rs:334 needless_return; Mac arm64 runner was not acquired under GitHub-hosted capacity pressure (no test executed), Linux and 3 unsigned installers passed. Source and hosted availability are distinct failure types.
**Scope:** fix Windows-only lint without changing file open behavior, test exact 64 KiB bound, re-run full local Rust/Node/Release and exact-head CI. Do not modify unrelated user projects, system config, GitHub protected branch, or Apple certificates.
**DoD:** 0 Windows Clippy needless_return, unit test exact boundary and oversized rejection, file type/FIFO failures still bounded, GitHub CI all required passing for same HEAD before acceptance. A hosted runner queue failure does not count as product regression or success.
**Phase gate:** R1 full exit, protected merge, installed app native E2E and production R2–R8 are NOT accepted by this repair alone.

## B. Task breakdown and ownership

| Task ID | Action | Real worker | File ownership | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-NPMCI-01 | Repair Windows-only Clippy needless_return while retaining fallback logic | TEAM_LEAD_SERIAL | src-tauri/src/providers/launch.rs | none | [x] |
| R1-NPMCI-02 | Add exact 64 KiB boundary contract test and preserve filesystem failure test | TEAM_LEAD_SERIAL | src-tauri/src/providers/launch.rs tests | R1-NPMCI-01 | [x] |
| R1-NPMCI-03 | Validate Unit/Contract + E2E + Smoke, team lead diff audit, document Mac runner capacity failure | TEAM_LEAD_SERIAL | docs/nexora, tests logs | R1-NPMCI-02 | [x] |
| R1-NPMCI-04 | Push PR #34, exact HEAD CI (macOS, Windows, Linux) + native unsigned engineering bundles | TEAM_LEAD_SERIAL | PR #34 | R1-NPMCI-03 | [ ] |
| R1-NPMCI-REVIEW | Independent reviewer and installed real GUI acceptance | INDEPENDENT_REVIEWER_PENDING | protected develop | R1-NPMCI-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-NPMCI-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | GitHub Windows Clippy needless_return root cause confirmed in failed job 120460476576; minimal source correction locally Mac Clippy all-targets -D warnings PASS, native module tests included in 1115 PASS; Lead reviewed semantics; Windows hosted rerun PENDING |
| R1-NPMCI-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 Rust full 1115 PASS covers exact 65536-byte valid npmrc + over-bound, Unix real FIFO no-writer, symlink and regular file paths; native filesystem module E2E (not GUI); Lead checked file/test diff |
| R1-NPMCI-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 full Rust 1115 PASS/0 FAIL/33 ignored, Clippy all-targets -D warnings PASS, frontend production build PASS, Node 208 PASS, Release 33 PASS; Mac hosted prior run was cancelled due runner capacity, not product test; Team Lead GO source |
| R1-NPMCI-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | same-SHA CI Mac/Win/Linux pending; runner availability separate |
| R1-NPMCI-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | no genuine outside review or installed GUI E2E yet |

## C. Exit checklist

- [x] Fresh planner committed before any source modifications; actual worker and task scope recorded
- [x] GitHub Windows and Mac failure modes classified using job evidence
- [x] Windows Clippy repair and bounded reader exact boundary checks (Mac source and test evidence; hosted Windows CI pending)
- [x] Unit/Contract + native module E2E + Smoke + Lead source GO for subtasks 01/02/03; full installed GUI still PENDING
- [ ] Exact SHA GitHub checks and installer smoke passing on all three systems
- [ ] Independent reviewer and R1 full phase native acceptance before protected merge

## E. Lead GO/NO-GO

**Team Lead verdict:** GO for source-level repair and PR submission after local tests. **GO/NO-GO:** NO-GO R1 phase exit or production, no claim that macOS runner unavailable was a passing native test.

## Repaired CI and Team Lead evidence (23 Oct 2026)

- Baseline PR #34 HEAD 2bf5d1b: Windows clippy needless_return with -D warnings; Mac runner not allocated by GitHub (no executed macOS tests); Linux checks green and unsigned engineering installers all green.
- Repair at source commit 4a215f4: remove Windows-only needless return and add 64KiB exact-maximum config contract test with retained >64KiB, FIFO, symlink and PATH precedence guards.
- **Mac M4 Unit/Contract:** full Rust lib 1115 PASS, 0 FAIL, 33 ignored; all-targets Clippy -D warnings PASS; new fixture is included in full suite.
- **Mac native filesystem E2E:** real mkfifo no writer returns without blocking, symlink/non-file skips, exact bound parsed; no real installed app session, provider nor shell-startup executed. E2E is scoped to the Rust path-discovery module.
- **Smoke:** Vite production build PASS, Node 208/208 PASS, Release Contract 33/33 PASS.
- **Team Lead source review GO for PR** narrow syntax/fixture correction and honest gate accounting; NO-GO protected merge, installed Finder GUI/native agent acceptance, and full R1 phase exit. R1-NPMCI-04 exact-head CI and R1-NPMCI-REVIEW remain PENDING.
