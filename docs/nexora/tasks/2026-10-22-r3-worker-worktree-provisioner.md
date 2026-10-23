# Task Checklist — R3 Worker Worktree Provisioning Foundation

**Date:** 2026-10-22 · **Team Lead:** acting assistant (TEAM_LEAD_SERIAL, no independently dispatched workers/reviewers available).
**User request:** Continue Nexora Code R0–R8 Roadmap with next R3 Git worktree per Worker.
**Branch:** feat/r3-worker-worktree-provisioner; isolated worktree; based on feat/r3-worktree-isolation-audit (PR #15).
**Scope:** internal-only paused-queued Worker provisioning primitive with persisted pending intent, safe Git commit base, unique deterministic branch, verified linked worktree and DB attachment. Test temporary repositories only.
**Out of scope:** Tauri public IPC, UI buttons, automatic dispatch/provision, existing user repository Git mutations, real Worker launches, MCP, secret or credential changes, installer/release, develop/main merge.
**Safety:** no live Agent can call this primitive; integration requires explicit native user consent, state.agents run guard, async host, crash/UNKNOWN reconciler, OS sandbox before production.

| Task | Actual assignee | File ownership | Acceptance | State |
| --- | --- | --- | --- | --- |
| R3-P-01 | TEAM_LEAD_SERIAL | workspace.rs | Read-only trusted HEAD commit helper | [x] |
| R3-P-02 | TEAM_LEAD_SERIAL | architect/provision.rs | Internal-only durable claim + paused preflight + safe provisioning; test Git repos | [x] |
| R3-P-03 | TEAM_LEAD_SERIAL | architect.rs | Internal module registration; no public command or launch | [x] |
| R3-P-04 | TEAM_LEAD_SERIAL | docs/nexora/R3-WORKER-PROVISIONING.md | Explain release blockers and manual orphan reconciliation | [x] |
| R3-P-QA | TEAM_LEAD_SERIAL | Entire diff | Focused and full Rust/Clippy/Node tests; PR handoff | [x] |
| R3-P-REVIEW | INDEPENDENT_REVIEWER_PENDING | PR review | Genuine approval needed before protected merge | [ ] |

- [x] Source work only in this Worktree and checklist created before edits.
- [x] Test real Git linked-worktree creation in temporary fixtures, state invariants, duplicate claim and crash fence.
- [x] Check full tests, Clippy, and source-level behavior with no live app/user Git changes.
- [x] Record Team Lead GO/NO-GO, constraints, CI and reviewer gates honestly.
- [x] Open stacked PR and verify pushed SHA.
- [ ] No protected branch merge without authorized second reviewer and CI.

**Team Lead decision:** GO for an internal-only source PR. NO-GO for live Agent/Renderer integration, protected merge or release until real operator-consent boundary, process fencing, CI and independent reviewer approval.

## Actual Team Lead review and test evidence — Mac M4, 2026-10-22

- Actual assignment model: TEAM_LEAD_SERIAL; no separate coding Agents or independent reviewers were invoked.
- Reviewed source changes: immutable HEAD helper, internal module registration, paused-queued worker preflight, SQLite preparing/attached reservation, registered worktree proof and atomic DB session attachment.
- Focused provisioning and persistent recovery tests **5 PASS / 0 FAIL**. Physical Git creation was in temporary test repositories, NOT the user's active source workspace.
- Full Rust suite **1049 PASS / 0 FAIL / 33 Ignored**; native CLI integration **1 PASS**.
- Rust all-target Clippy -D warnings **PASS**.
- Frontend Node **139 PASS / 0 FAIL**; release-script tests **31 PASS / 0 FAIL**; TypeScript/Vite production build and R0 fork/updater identity safety **PASS**.
- Important remaining: durable claim storage migration, native user consent proof, async process/Agent ledger fencing, operator-guided UNKNOWN reconciliation, OS file sandbox and Mac/Windows/Linux real worker E2E.
- GitHub required checks for this new SHA and independent Reviewer: PENDING at submission. No production feature was activated.

## GitHub handoff evidence

- **PR #16:** https://github.com/TheerasakPing/nexora-code/pull/16
- **Verified original source commit:** 35faf448857b3a54acc2f7b2629ebf279823175b
- **Base:** feat/r3-worktree-isolation-audit (PR #15); no develop/main merge.
- **GitHub CI:** PENDING on this PR at initial publication; passing Mac source
  tests are not native Windows/Linux GUI/Agent acceptance.
- **Independent Reviewer:** PENDING, not inferred or fabricated.
- **Scope exclusions remain:** live IPC, actual Agent processes, arbitrary
  user's checkout mutations, installer/signing and production release.
