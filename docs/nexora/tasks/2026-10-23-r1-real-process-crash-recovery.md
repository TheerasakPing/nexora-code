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
| R1-CRASH-01 | Isolated test-subprocess fixture persists running session/tool; signals ready after durable write | TEAM_LEAD_SERIAL | src-tauri/src/storage.rs tests | none | [x] |
| R1-CRASH-02 | Kill child at OS level; reopen SQLite, assert interruption and no replay repeatedly | TEAM_LEAD_SERIAL | src-tauri/src/storage.rs tests | R1-CRASH-01 | [x] |
| R1-CRASH-03 | Verify Unit/Contract + E2E + Smoke, capture run/OS and source review | TEAM_LEAD_SERIAL | docs/nexora/ and planner | R1-CRASH-02 | [x] |
| R1-CRASH-04 | PR/CI exact-head handoff and protected merge gating | TEAM_LEAD_SERIAL | PR | R1-CRASH-03 | [ ] |
| R1-CRASH-REVIEW | Genuine reviewer (not worker) before protected merge | INDEPENDENT_REVIEWER_PENDING | protected PR | R1-CRASH-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-CRASH-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 native Rust test fixture spawned separate test binary; 10/10 disposable WAL databases, ready marker and real process child; focused E2E 1 PASS + full Rust 1113 PASS; Team Lead reviewed child scope and bounded startup |
| R1-CRASH-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 OS SIGKILL 10 cycles; 2 reopens each: interrupted session, failed tool once, preserved completed tool, no duplicate prompt/tool; Rust full 1113 PASS; Team Lead verified unclean OS exit semantics |
| R1-CRASH-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 full Rust 1113 PASS/33 ignored; Node 208/208 after fresh npm run build; Release 33/33; docs/nexora/R1-REAL-PROCESS-CRASH-RECOVERY-2026-10-23.md reviewed for truthful OS scope; no user data |
| R1-CRASH-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | exact-head CI and PR review pending |
| R1-CRASH-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | no independent reviewer assigned |

## C. Checklist

- [x] Fresh dated planner committed before source modifications
- [x] Task ownership and honest serial fallback recorded, no imaginary agents
- [x] OS-killed child tested against real persisted SQLite; no user data touched
- [x] Unit/E2E/Smoke and Team Lead Review GO for R1-CRASH-01/02/03 only; PR/independent review PENDING
- [ ] All CI/checks exact SHA and independent reviewer accept
- [ ] R1 native GUI/provider, complete TOCTOU, multi-session load and phase exit all accepted

## E. Team Lead initial decision

**Team Lead verdict:** GO for PR source submission after Mac M4 unit/integration E2E and smoke tests. **GO/NO-GO:** GO for source review; NO-GO for protected merge, native/production release or R1 phase exit without outstanding OS/manual gates. Original 21/24 packages, 61 detailed checks and 7/7 brand signoffs pending are overlapping views, not a percent.

## Local verification ledger / 2026-10-23

- Mac M4 native module E2E actual OS kill: 1 focused test PASS, 10 distinct child SIGKILL cycles, 20 restart openings.
- Cargo full library: **1113 PASS, 0 FAIL, 33 ignored**. Node source/frontend: **208/208 PASS** after npm run build; Release: **33/33 PASS**. Existing Node bundle guard requires built dist, initial no-dist run failed its prerequisite, then full suite passed.
- Team Lead inspection: 117-line additive test-only change to storage.rs; no application code changed, no secrets/network/user storage accessed.
- Exactly what it does not prove: real external tool side-effect idempotency, installed GUI/PTY/provider crash/resume or Windows 11 native test. R1 gate NO-GO until these are demonstrated.
- R1-CRASH-04 and independent human reviewer remain PENDING. Source GO for PR submission only.
