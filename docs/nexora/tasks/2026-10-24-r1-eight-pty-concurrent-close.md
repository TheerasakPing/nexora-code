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
| R1-PTY8-01 | Add 8-PTY shared registry with real /bin/sh and PTY child ownership | TEAM_LEAD_SERIAL | src-tauri/src/terminal.rs tests | none | [ ] |
| R1-PTY8-02 | Concurrent close and reaping, post-close rejects writes, leak guard | TEAM_LEAD_SERIAL | src-tauri/src/terminal.rs tests | 01 | [ ] |
| R1-PTY8-03 | Unit/Contract + native PTY E2E + Smoke and Team Lead review | TEAM_LEAD_SERIAL | docs/nexora and tests | 02 | [ ] |
| R1-PTY8-04 | CI exact-head stacked PR, required macOS/Linux/Windows, independent review gate | TEAM_LEAD_SERIAL | GitHub PR | 03 | [ ] |
| R1-PTY8-REVIEW | Genuine independent reviewer/installed native GUI signoff | INDEPENDENT_REVIEWER_PENDING | protected merge | 04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-PTY8-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | actual Mac 8 PTY fixture not run yet |
| R1-PTY8-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | concurrent shared close and OS child wait pending |
| R1-PTY8-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | full native regression and scope review pending |
| R1-PTY8-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | exact GitHub platform CI and native engineer installer pending |
| R1-PTY8-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | independent reviewer / installed GUI evidence missing |

## C. Checklist / exit gates

- [x] Fresh dated planner committed before source changes, isolated branch, no collision with active PRs
- [x] Task owners, dependency and Test Triad matrix established
- [ ] 8 concurrent real PTYs under one registry pass bounded close and OS wait
- [ ] Unit / E2E / Smoke evidence captured, diff reviewed by Team Lead
- [ ] PR exact SHA hosted CI all required systems and independent reviewer approval
- [ ] R1 native installed GUI/provider/PTY/soak acceptance and phase exit GO

## E. Team Lead GO/NO-GO

**Team Lead verdict:** GO to add and test isolated Unix PTY integration. **GO/NO-GO:** NO-GO protected merge, full R1 phase exit, and production until independent human reviewer and real installed native tests.
