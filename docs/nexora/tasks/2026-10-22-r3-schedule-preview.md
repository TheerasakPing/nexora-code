# Task Checklist — R3 Schedule Preview — 2026-10-22

- **Request:** While protected develop PR awaits CI/review, develop another independent feature.
- **Lead / reviewer:** current acting assistant, TEAM_LEAD_SERIAL. No invented agents or independent reviewers.
- **Phase / priority:** R3 Multi-Agent; P1.
- **Branch:** feat/r3-schedule-preview; separate nexora-code-r3-schedule-preview worktree, based on origin/develop.
- **Goal:** Read-only, deterministic proposed Agent execution waves from existing TeamSnapshot, showing blocked dependencies/file conflicts and concurrency cap; no actual worker launch.
- **Scope:** isolated scheduler preview module, typed native IPC read-only command, tests, documentation. Shared architect scheduler remains authoritative and unchanged.
- **Dependencies:** preexisting architect queues and file scope conflict policy. Protected-branch PR #7 is separate.
- **Out of scope:** real Agent spawn, scheduler runtime replacement, worktree creation, UI app launches, credentials, MCP, main/develop merge, production release.
- **Risks:** Planned waves are *simulations*, not guaranteed future runtime execution. Data is session-scoped; error on unknown team. File scope overlap semantics must match existing conflicts.
- **Acceptance:** no writes during preview; cap and dependencies obeyed; conflicting writes never in same wave; blocked/missing/failed dependency reported; deterministic/order stable; unit + full Rust + Clippy pass; PR opened with CI pending.

| Task ID | Worker | Owned scope | Depends | Status |
| --- | --- | --- | --- | --- |
| R3-01 | TEAM_LEAD_SERIAL | src-tauri/src/architect/preview.rs and architect/tests.rs, pure simulation & tests | none | [x] |
| R3-02 | TEAM_LEAD_SERIAL | src-tauri/src/architect.rs + lib.rs, read-only command | R3-01 | [x] |
| R3-03 | TEAM_LEAD_SERIAL | docs/nexora/R3-SCHEDULE-PREVIEW.md | R3-02 | [x] |
| R3-QA | TEAM_LEAD_SERIAL | all changed diffs + source test evidence | all | [x] |

- [x] Team Lead inspected actual diff for side effects and ACL issues.
- [x] Focused tests pass (all cases); no fake worker delegation.
- [x] Full Rust unit and Clippy pass; skipped tests disclosed.
- [x] Git diff --check and no shared-branch overwrite.
- [ ] PR with Task Planner, risk/status, GO/NO-GO, test evidence.
- [ ] Real independent review and GitHub native OS CI before protected merge.

**Team Lead verdict:** GO for scoped source-only PR submission; NO-GO for protected branch merge/release until required CI and real independent review.

## Team Lead handoff evidence (2026-10-22, Mac M4)

- R3 simulation: 9 focused scenarios PASS, and one persisted TeamSnapshot state-nonmutation regression PASS.
- Full Rust tests: 1016 PASS, 0 FAIL, 33 Ignored; CLI integration 1 PASS.
- Rust all-target Clippy with denied warnings: PASS.
- Threat review: reads existing per-session TeamSnapshot; no process/file writes, permission grants, network, or agent execution.
- Limitations: preview waves are hypothetical; other team scheduling, mid-turn failures and OS process caps are excluded.
- Actual worker assignment: TEAM_LEAD_SERIAL; no separate agent or independent Reviewer was invoked.
- Pending: native Windows/Linux CI, GUI E2E, reviewer signoff, protected merge. No installations or production release.
