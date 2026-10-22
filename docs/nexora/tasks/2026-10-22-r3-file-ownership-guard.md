# Task Checklist — R3 Shared-Workspace File Ownership Guard

**Date:** 2026-10-22 (created before implementation)
**Request:** Continue R0-R8 roadmap while protected PRs await reviewer approval; next priority R3 worktree isolation/file ownership.
**Team Lead:** acting assistant, TEAM_LEAD_SERIAL (no available independent worker/reviewer delegation tool).
**Branch:** feat/r3-file-ownership-guard (isolated worktree, based on origin/develop)
**Priority:** P1 runtime reliability/security
**User scope:** Build/test/push source and PR. No GUI, MCP server launch, secret access, protected merge or release.

## Design and acceptance

- Existing native architect scheduler serializes overlapping writers in the exact same checkout, but compares roots for equality only. Extend admission to deny conflicting scoped writes when two registered project checkouts are nested or aliases.
- Preserve independent work in separate real Git worktrees; never serialize unrelated repositories.
- Compare paths by components, not string prefix (src/a vs src/ab).
- Fail closed for malformed scopes or existing symlinked scoped paths that cannot safely claim ownership.
- Honor read-only workers and zero/unlimited parallel semantics.
- Native scheduler is the admission source of truth, not merely renderer advice.
- File claims are scheduler-level coordination only: shell/provider tools are NOT sandboxed to file scope; true filesystem enforcement and automatic worktree creation/cleanup require a later reviewed design.
- Verify full Rust test suite, Clippy, source quality and native CI for Windows/Linux after PR.
- Never overwrite a concurrent branch or claim independent review without one.

| ID | Actual worker | Owned files | Acceptance | Status |
| --- | --- | --- | --- | --- |
| R3-OWN-01 | TEAM_LEAD_SERIAL | src-tauri/src/architect/ownership.rs | Cross-root collision safety and unit tests | [x] |
| R3-OWN-02 | TEAM_LEAD_SERIAL | src-tauri/src/architect.rs | Wire native cross-team scheduling predicate | [x] |
| R3-OWN-03 | TEAM_LEAD_SERIAL | docs/nexora/R3-FILE-OWNERSHIP.md | Document security limits and follow-ons | [x] |
| R3-OWN-QA | TEAM_LEAD_SERIAL | Read-only full diff | Tests/compatibility/PR handoff | [x] |
| R3-REVIEW | Independent reviewer PENDING | GitHub review | No protected merge before approval | [ ] |

- [x] New task checklist saved before source editing.
- [x] Path nesting, siblings, symlinks, read-only and malformed inputs tested.
- [x] Full Rust tests and all-target Clippy pass.
- [x] Team Lead reviewed actual native admission logic and risk.
- [ ] GitHub PR submitted with evidence and limitations.
- [ ] CI and real native E2E accepted before protected merge.

**Team Lead verdict:** GO for tested source PR submission; NO-GO for protected-branch merge/release without real independent reviewer and GitHub CI.

## Actual QA and handoff

- Team Lead: acting assistant, TEAM_LEAD_SERIAL; no actual independent Agent Worker or reviewer was available.
- Native integration test: nested workspaces + two real simulated team schedules prevent conflicting scoped writes; 1 PASS.
- Ownership unit tests: 7 PASS, includes edge cases for nonexistent files, symlink scope, nested/sibling checkout and invalid scope fail-closed.
- Mac Rust full suite: 1014 PASS / 0 FAIL / 33 Ignored; Rust CLI integration 1 PASS.
- All-target Rust Clippy -D warnings: PASS.
- Frontend: TypeScript/Vite build PASS, npm test PASS, npm run test:release PASS, R0 safety verification PASS.
- No external tools/agents were launched; no user file writes or MCP traffic from the tested feature.
- Remaining separate requirements: real per-agent Git worktree lifecycle, OS-enforced write scope, durable crash recovery, 3-OS native GUI validation and authorized reviewer signoff.
