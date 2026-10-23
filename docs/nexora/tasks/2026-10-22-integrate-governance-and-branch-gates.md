# Task Checklist — 2026-10-22-integrate-governance-and-branch-gates

- **Request:** Finish governance integration: repair CI, review and merge PR #4 then #5 into develop, and enforce required GitHub checks/branch protection.
- **Team Lead:** acting assistant / Integration Lead, TEAM_LEAD_SERIAL (no independently delegated worker or reviewer).
- **Created before code edits:** 2026-10-22
- **Priority:** P0 CI/review integrity; P1 governance.
- **Repository:** TheerasakPing/nexora-code; target develop; main remains untouched.
- **Working branch:** feat/governance-task-gate; worktree nexora-code-governance-ci.
- **Scope:** real CI triage, minimal patch, merge only when documented gates met, branch rule configuration and evidence.
- **Safety boundaries:** no app install, GUI, live MCP, credentials, force push, release, or merge to main.
- **Dependencies:** PR #2 cross-OS fix; PR #4 team rule; PR #5 verifier; concurrent PR #6 is separately owned and MUST NOT be overwritten.
- **DoD:** develop includes reviewed rule + safe single PR gate; CI for Mac/Windows/Linux and frontend passes; branch protection on develop demonstrably enforces checks/review. If real independent review is unavailable, leave merge blocked with explicit NO-GO instead of bypass.

| Task | Real worker | Scope and acceptance | Status |
| --- | --- | --- | --- |
| GOV-INT-01 | TEAM_LEAD_SERIAL | Diagnose exact Windows and PR #5 failures and document cause | [x] |
| GOV-INT-02 | TEAM_LEAD_SERIAL | Review PR #2 fix and verify CI before merging to develop | [ ] |
| GOV-INT-03 | TEAM_LEAD_SERIAL | Correct governance PR #4 with real tests/PR handoff; verify all CI and reviewer gate | [ ] |
| GOV-INT-04 | TEAM_LEAD_SERIAL | Integrate PR #5, single safe trusted-base validator and tests, verify CI | [ ] |
| GOV-INT-05 | TEAM_LEAD_SERIAL | Configure develop branch protection and required checks without bypass | [ ] |
| GOV-INT-QA | TEAM_LEAD_SERIAL | Review every final diff, SHA, CI result, branch settings; GO/NO-GO report | [ ] |

## Evidence and review

- [ ] Real cross-platform CI results verified with run links / SHA / failure logs.
- [ ] Shared contracts/PR #6 checked for collision and left unchanged.
- [ ] No skipped required check, forged agent/reviewer or false GO.
- [ ] Target branch update and branch protection reviewed.
- [ ] Final tests, merge state and residual risks recorded.

**Team Lead verdict:** PENDING. Independent reviewer: PENDING (not fabricated).

## CI triage evidence

- PR #2 GitHub six checks **all SUCCESS** on its branch including Windows Clippy; source patch preserves Unix mode 0700 and scopes a test helper to Unix.
- PR #4 and #5 Windows Clippy fail at existing providers/pi.rs:13 (unused mut), because develop has not incorporated PR #2.
- PR #5 frontend failure was test-fixture contamination from GitHub Actions global GITHUB_EVENT_PATH; isolated test environment now uses an empty synthetic value unless a test explicitly sets its own event. Production checker logic remains unchanged.
- Existing PR #6 is concurrently maintained by another branch. No edits to it.
