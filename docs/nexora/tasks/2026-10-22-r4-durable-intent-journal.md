# Task Checklist — R4 Durable Intent Journal

**Created before code edits:** 2026-10-22.
**User request:** Continue full Nexora Code roadmap, next R4 crash-consistent intent and UNKNOWN fencing work.
**Team Lead:** acting assistant / TEAM_LEAD_SERIAL, independent reviewer unavailable.
**Priority:** P0 prevent duplicate external mutations on crash/restart.
**Worktree and branch:** nexora-code-r4-journal, feat/r4-durable-intent-journal, based on PR #12 branch.
**Scope:** Internal Rust SQLite durable intent ledger with explicit generations/statuses, recovery UNKNOWN fencing, integrity tests and honest documentation.
**No production executor:** No native UI IPC for journal, no tool calls, MCP traffic, agent spawning, filesystem user modifications or grants. This module is not an execution authorization boundary.
**Constraints:** Internal journal cannot decide operator consent; no automatic retry of UNKNOWN or running mutations; reconciliation must remain unavailable until real authorization is implemented.

| ID | Actual worker | File ownership | Acceptance | Status |
| --- | --- | --- | --- | --- |
| R4-J-01 | TEAM_LEAD_SERIAL | src-tauri/src/workflow_journal.rs | Atomic SQLite durable intents + state machine and tests | [x] |
| R4-J-02 | TEAM_LEAD_SERIAL | src-tauri/src/lib.rs | Module registration (internal only) | [x] |
| R4-J-03 | TEAM_LEAD_SERIAL | docs/nexora/R4-INTENT-JOURNAL.md | State gates, limitations and recovery behavior | [x] |
| R4-J-QA | TEAM_LEAD_SERIAL | Read-only diff/CI review | Full Rust test/Clippy, security and PR evidence | [x] |
| R4-J-REVIEW | Independent reviewer PENDING | GitHub review | Needed for merging protected branch | [ ] |

- [x] Planner authored before implementation, working tree isolated.
- [x] Intent is committed before starting a mutating side effect.
- [x] Restart marks any in-flight intent UNKNOWN; dependent work cannot proceed.
- [x] Stale generation and duplicate effect IDs rejected, no replay.
- [ ] Source tests, 3-OS CI, reviewer signoff checked separately.
- [ ] Team Lead signs GO/NO-GO without claiming runtime production feature is complete.

**Team Lead verdict:** GO for staged internal source PR; NO-GO for linking a live executor, protected-branch merge or release.

## Real tests and review evidence — 2026-10-22

- Actual worker/reviewer: TEAM_LEAD_SERIAL only. No independent Agent Worker, human reviewer or OS device E2E ran.
- Four focused SQLite tests PASS, including on-disk reopen and UNKNOWN/no-replay.
- Rust full suite 1018 PASS / 0 FAIL / 33 ignored, CLI integration 1 PASS.
- Rust Clippy all-targets with denied warnings PASS (initial unused drop test warnings corrected, not suppressed).
- Frontend Node tests 110 PASS; release tests 31 PASS; Vite build and R0 identity verification PASS.
- Team Lead inspected transactions and state/generation/intent uniqueness, rollback and lack of Tauri API. No native execution or permission boundary exists for this module yet.
- Critical unresolved: secure trusted real-user approval integration; storage migrations; crash/power-loss fault injection; idempotent external provider boundary; reconcile UNKNOWN; three-platform native acceptance and independent review.
