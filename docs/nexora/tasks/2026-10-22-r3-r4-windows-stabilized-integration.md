# Task Checklist — R3/R4 Consolidated Windows-Stabilized Integration

**Date:** 2026-10-22 · **Task:** Integrate source-complete governance, R3 and R4 PR branches with independently confirmed Windows CI fixes.
**Team Lead:** acting assistant / TEAM_LEAD_SERIAL; no real delegated Agent Workers/independent reviewer available.
**Base:** origin/fix/ci-windows-turn-snapshot (PR #9), already includes PR #7 governance and Windows build fixes.
**Branch:** integration/r3-r4-windows-stabilized; isolated Nexora Code integration worktree.
**Priority:** P0 cross-platform CI, P1 multi-agent/workflow integration.
**Scope:** Merge preserving original commit history of R3 Preview/UI, R3 scoped file ownership, and R4 DAG/SQLite intent journal.
**Safety:** No protected develop/main merge, no actual GUI, agent, MCP, OS installers, secrets or production release.
**Dependencies:** PR #9 successful six checks; PRs #10, #11, #13 source verified on Mac. Genuine external approving reviewer remains a separate blocker.
**Acceptance:** Conflict-free integrated source, no accidentally dropped safety modules, full Rust/Clippy, Node build/tests, release tests; six required CI checks on exact integrated SHA; no P0 regression.
**Assurance scope:** Local pure/native unit+integration test only, NOT real Mac/Windows/Linux GUI E2E or OS sandbox approval.

| Task | Worker | Result expected | Status |
| --- | --- | --- | --- |
| INT-01 | TEAM_LEAD_SERIAL | Merge R3 Preview/UI history, keep native IPC intact | [x] |
| INT-02 | TEAM_LEAD_SERIAL | Merge R3 Ownership and resolve native architect conflicts | [x] |
| INT-03 | TEAM_LEAD_SERIAL | Merge R4 DAG/Journal and retain full schema/guard tests | [x] |
| INT-04 | TEAM_LEAD_SERIAL | Verify full Node/Rust/Clippy with SHA evidence and diff review | [x] |
| INT-05 | TEAM_LEAD_SERIAL | Push reviewable integration PR and track 3-OS CI | [ ] |
| INT-06 | Independent Reviewer pending | Approve after cross-platform checks | [ ] |

- [x] Planner committed and every source-branch history retained.
- [x] Actual diffs and conflict resolutions inspected by Team Lead.
- [x] Mac full regression suite passes; ignored tests disclosed.
- [ ] GitHub six required checks on integrated head pass.
- [ ] Real independent reviewer and project owner grant release/merge permissions.

**Team Lead verdict:** GO for integration PR submission only; NO-GO for protected develop merge or release without a real second approving reviewer and GitHub 3-OS CI.

## Integration evidence (2026-10-22, Mac M4)

- All three source branch histories integrated without resetting originals: PR #10 (R3 Scheduler Preview/UI), PR #11 (R3 scoped writer arbitration), PR #13 (R4 DAG validation and internal SQLite intent journal). PR #9 base already includes PR #7 (governance/Windows build), plus Windows turn_changes test repair.
- Merge conflicts were limited to architect module declaration and native lib.rs IPC registration. Team Lead kept both abilities, and inspected resolutions.
- Full Rust: **1038 PASS / 0 FAIL / 33 ignored**, CLI integration **1 PASS**. All-target Clippy -D warnings PASS.
- Frontend: npm test **134 PASS / 0 FAIL**; release tests **31 PASS**, production TypeScript/Vite build PASS; R0 identity/updater safety PASS.
- Source code was not installed/run as an app. No actual Agent Worker/MCP connection, secrets or production release.
- GitHub required checks on exact integrated SHA: PENDING at this preparation step.
- No real independent reviewer is listed as a repository collaborator besides TheerasakPing (the PR author); protected merge remains NO-GO.
- Native GUI tests, signed installers, secure scope enforcement and true workflow runner remain outside this integration acceptance.
