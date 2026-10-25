# Nexora Code R6 — Full Browser UI E2E Mac shard 4 remediation
Date: 2026-10-25 Asia/Bangkok. Base 669e9bc, PR #44 fix/full-ui-artifacts-cross-platform-20261024. Product **Nexora Code**, fork PandaCode, never Nexora Studio.
## Why / bounded scope
- [x] PR #43 tests 744 UI cases; PR #44 fixes macOS screenshots. GitHub run 40161607545: macOS UI E2E shard 1/4, 2/4, 3/4 PASS, shard 4/4 FAIL. Other native Rust/installer checks green. Read actual failing log before edits.
- [ ] W1 trace and reproduce exact failing tests on Mac. Own narrowly scoped failing tests or actual code defect. Preserve all original PandaCode functions, no hiding failures by skip/only/force.
- [ ] W2 independent QA audit of shard CI setup, Mac screenshot portability, existing E2E assertions and fixture security. Separate worktree; may own separate contract tests after concrete issue.
- [ ] Independent read-only source review of W1/W2 and Lead integration QA: actual focused browser E2E, npm run build, npm test, release contract.
- [ ] Push reviewed dev PR branch only; CI rerun; no merging protected main/develop without green gates/human approval.
## Agent assignments
- W1: assigned failing test/features only with smoke/E2E evidence and log.
- W2: independent CI/tests audit and separate bounded test if real issue.
- R: independent final reviewer; Lead responsible integration/approval.
## Evidence ledger
| Lane | Commit | QA/E2E | Review |
|---|---|---|---|
| W1 | pending | pending | pending |
| W2 | pending | pending | pending |
| Lead | pending | pending | pending |
## Production gate
744 browser tests cannot substitute real native Mac/Windows/Linux GUI, signing, updater and human signoff.
