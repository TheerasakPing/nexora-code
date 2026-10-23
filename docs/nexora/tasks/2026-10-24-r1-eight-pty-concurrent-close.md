# Task Checklist Planner — R1 eight native PTY sessions concurrent shutdown

**Created:** 2026-10-24 Asia/Bangkok, before source modifications. **Requested:** Continue R1→R8 in order, with actual code/test delivery.
**Branch/worktree:** test/r1-eight-pty-concurrent-close-20261024 in /Volumes/Data/workspace/git/nexora-code-r1-eight-pty-20261024, stacked on PR #34 to avoid touching its active worktree or concurrent PR #36 session storage.
**Lead:** ChatGPT TEAM_LEAD_SERIAL. Current tools cannot establish a genuinely independent engineer agent task assignment/review. No fake agents/approval.
**Goal:** Establish real Mac M4 PTY process integration E2E for 8 simultaneous sessions in *one* shared terminal registry, interleaving blocked reader and long-running descendants. Concurrent close requests must complete, terminate/reap owned child shells, leave registry empty, and reject further input. This is not a full installed WKWebView/provider E2E.
**Scope:** Add test-only native Rust PTY fixture in src-tauri/src/terminal.rs. Only disposable system /bin/sh/sleep children; no real provider credentials, user projects, settings, network, production data, source binaries or installer mutation.
**Security:** test must kill only fixture-owned process groups; allow no unbounded waits (> seconds for startup/close), use shared registry; never kill other OS PIDs.
**Acceptance:** 8 PTYs spawn in one process, distinct IDs, mix input waiting and long-running exec, shared registry, concurrent close, each original child reaped, no registry entries, writing closed terminal fails; no hangs. Repeat focused test and full Rust, Node, Release, Clippy. Hosted macOS/Linux/Windows CI exact SHA. Linux-only test guarded cfg unix.
**Non-acceptance:** no approval for native installed GUI, external providers, 8 genuine agents × 2h soak, 50 crash/resume each OS, signing/notarization. Does not close entire R1 phase.
**Dependency:** PR #34 -> this PR; independent protected reviewer pending.

## B. Task decomposition and actual owner

| ID | Action | Worker | Owned path | Dependency | Status |
| --- | --- | --- | --- | --- | --- |
| R1-PTY8-01 | Add 8-PTY shared registry with real /bin/sh and PTY child ownership | TEAM_LEAD_SERIAL | src-tauri/src/terminal.rs tests | none | [x] |
| R1-PTY8-02 | Concurrent close and reaping, post-close rejects writes, leak guard | TEAM_LEAD_SERIAL | src-tauri/src/terminal.rs tests | 01 | [x] |
| R1-PTY8-03 | Unit/Contract + native PTY E2E + Smoke and Team Lead review | TEAM_LEAD_SERIAL | docs/nexora and tests | 02 | [x] |
| R1-PTY8-04 | CI exact-head stacked PR, required macOS/Linux/Windows, independent review gate | TEAM_LEAD_SERIAL | GitHub PR | 03 | [ ] |
| R1-PTY8-REVIEW | Genuine independent reviewer/installed native GUI signoff | INDEPENDENT_REVIEWER_PENDING | protected merge | 04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-PTY8-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 actual eight /bin/sh PTYs in one shared registry, Rust target PASS; 20 repeats 160 OS children; full Rust 1116 PASS, Lead inspected fixture |
| R1-PTY8-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 8 callers synchronized Barrier, all close returned, all native PTY OS children reaped, registry empty, writes rejected; 20/20 repeated, Lead reviewed PID reuse guard |
| R1-PTY8-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 Rust 1116 PASS, 33 ignored, Clippy -D warnings PASS, Node 208 PASS, Release 33 PASS, Vite build PASS, documented narrow E2E; Lead source GO |
| R1-PTY8-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | exact GitHub platform CI and native engineer installer pending |
| R1-PTY8-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | independent reviewer / installed GUI evidence missing |

## C. Checklist / exit gates

- [x] Fresh dated planner committed before source changes, isolated branch, no collision with active PRs
- [x] Task owners, dependency and Test Triad matrix established
- [x] 8 concurrent real PTYs under one registry pass bounded close and OS wait
- [x] Unit / E2E / Smoke evidence captured, diff reviewed by Team Lead on Mac M4 (not native installed GUI)
- [ ] PR exact SHA hosted CI all required systems and independent reviewer approval
- [ ] R1 native installed GUI/provider/PTY/soak acceptance and phase exit GO

## E. Team Lead GO/NO-GO

**Team Lead verdict:** GO to add and test isolated Unix PTY integration. **GO/NO-GO:** NO-GO protected merge, full R1 phase exit, and production until independent human reviewer and real installed native tests.

## Team Lead evidence ledger — 2026-10-24

- Unit/Contract: focused 1/1 Rust test PASS; 8 sessions have unique IDs/PIDs.
- Native OS PTY E2E (bounded): actual /bin/sh + portable_pty native handles, 8 concurrent close() under one shared registry, 8 child.wait() completed and all closed terminal writes rejected. Repeat 20 cycles, 160 native test-owned shells total, PASS.
- Smoke: cargo test --lib 1116 PASS, 33 ignored, 0 failed; cargo clippy --all-targets -- -D warnings PASS; npm run build PASS; npm test 208 PASS; npm run test:release 33 PASS.
- Review: Team Lead inspected test-only diff; fixture now skips destructor kill for children already reaped and limits waiting to 5s; no production source changed. Genuine independent reviewer PENDING.
- NO-GO: real installed Mac app/WKWebView/Provider E2E, 8 AI agents 2h soak, Windows native ConPTY and R1 phase exit.
