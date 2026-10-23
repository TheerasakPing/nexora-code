# Task Checklist Planner — R1 real process SIGKILL and session recovery

**Date / user request:** 2026-10-23 / “ทำให้เสร็จเลย” — deliver actual R1→R8 progress with no fabricated phase acceptance.
**Team Lead:** ChatGPT Integration Lead, acting TEAM_LEAD_SERIAL (tool API does not expose engineering task delegation). No genuinely independent reviewer.
**Repository/worktree:** TheerasakPing/nexora-code / /Volumes/Data/workspace/git/nexora-code-r1-sigkill-20261023; branch test/r1-sigkill-store-recovery-20261023, base existing PR #32, separate from concurrent PR worktrees.
**Scope:** R1 filesystem-backed SQLite Store recovery when actual disposable OS subprocess is SIGKILLed with a running task and in-flight tool. Focus crash-consistency and preventing silent automatic replay. Repeated cycles on Mac M4 plus hosted platform CI.
**Out of scope:** 8-real-agent soak, signed app/installed GUI provider behavior, live tool execution, user data or secrets, R2-R8 acceptance/signing, merge without human independent review. Does NOT complete entire R1 exit.
**Test safety:** unique temp project/SQLite, disposable test binary child, SIGKILL only observed test-owned child PID, sentinel and bounded cleanup, never signal arbitrary PIDs.
**Acceptance:** Child persists running Session/Tool; actual SIGKILL; re-open same WAL database, interrupted status and failed in-flight tool with explicit notice, completed tool unchanged; no auto tool rerun, second reopen idempotent. 10 crash/restart iterations on Mac. Cross-platform independently gated, not claimed locally.

## B. Team Lead task decomposition and assignments

| Subtask ID | Deliverable | Real worker | File ownership | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-CRASH-01 | Isolated test-subprocess fixture persists running session/tool; signals ready after durable write | TEAM_LEAD_SERIAL | src-tauri/src/storage.rs tests | none | [ ] |
| R1-CRASH-02 | Kill child at OS level; reopen SQLite, assert interruption and no replay repeatedly | TEAM_LEAD_SERIAL | src-tauri/src/storage.rs tests | R1-CRASH-01 | [ ] |
| R1-CRASH-03 | Verify Unit/Contract + E2E + Smoke, capture run/OS and source review | TEAM_LEAD_SERIAL | docs/nexora/ and planner | R1-CRASH-02 | [ ] |
| R1-CRASH-04 | PR/CI exact-head handoff and protected merge gating | TEAM_LEAD_SERIAL | PR | R1-CRASH-03 | [ ] |
| R1-CRASH-REVIEW | Genuine reviewer (not worker) before protected merge | INDEPENDENT_REVIEWER_PENDING | protected PR | R1-CRASH-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-CRASH-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | unique disposable child test executable + SQLite fixture pending |
| R1-CRASH-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | actual OS SIGKILL and restart fixture pending |
| R1-CRASH-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | full Rust/Node regression and docs review pending |
| R1-CRASH-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | exact-head CI and PR review pending |
| R1-CRASH-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | no independent reviewer assigned |

## C. Checklist

- [x] Fresh dated planner committed before source modifications
- [x] Task ownership and honest serial fallback recorded, no imaginary agents
- [ ] OS-killed child tested against real persisted SQLite; no user data touched
- [ ] Unit/E2E/Smoke and Lead Review evidence signed for each subtask
- [ ] All CI/checks exact SHA and independent reviewer accept
- [ ] R1 native GUI/provider, complete TOCTOU, multi-session load and phase exit all accepted

## E. Team Lead initial decision

**GO:** implement/test source-level real crash recovery; **NO-GO** native/production release or R1 phase exit without outstanding OS/manual gates. Original 21/24 packages, 61 detailed checks and 7/7 brand signoffs pending are overlapping views, not a percent.
