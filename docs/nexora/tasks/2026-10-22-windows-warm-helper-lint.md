# Task Checklist — 2026-10-22-windows-warm-helper-lint

- **Request:** Continue R1/CI development while governance PR #4 is under review.
- **Task:** CI-WIN-WARM-001; Windows Clippy warning blocks PR #2.
- **Team Lead:** Acting assistant, TEAM_LEAD_SERIAL. No independent worker delegated.
- **Branch:** fix/ci-windows-pi-dirbuilder; separate Git worktree nexora-code-windows-ci.
- **Root cause:** Windows lib-test compile includes warm::waiting with cfg(test), but only Unix tests use it. Rust -D warnings rejects dead_code.
- **Scope:** src-tauri/src/providers/warm.rs: cfg on test-only helper; this task planner, PR #2 description.
- **Acceptance:** Unix focused tests + full Mac Rust tests and Clippy pass, Windows CI rerun; preserve Unix runtime behavior.
- **Risk:** Platform conditional changes; don't silence warnings globally or disable Windows checks.
- **Permissions:** No install, app launch, secrets, protected merge or release.

| Task | Worker | Deliverable | Evidence | State |
| --- | --- | --- | --- | --- |
| WIN-01 | TEAM_LEAD_SERIAL | Validate Windows CI root cause | GitHub failed job log | [x] |
| WIN-02 | TEAM_LEAD_SERIAL | Restrict helper to Unix tests | Rust test suite PASS | [x] |
| WIN-03 | TEAM_LEAD_SERIAL | Verify full Rust/Clippy and diff | 1006 Rust PASS / 33 ignored / Clippy PASS | [x] |
| WIN-04 | TEAM_LEAD_SERIAL | Push PR #2 update | Commit + GitHub SHA | [x] |

## GO/NO-GO Review

- [x] Diff checked for correctness, unexpected changes and platform behaviors.
- [x] Actual tests and ignored counts recorded; Windows CI not inferred.
- [x] PR #2 updated; remote SHA documented in Git branch history.
- [x] Acting Team Lead reviewed actual diff; independent PR reviewer remains pending.

**Team Lead verdict:** GO for PR submission; NO-GO for merging until Windows CI and independent review pass.

### Source evidence (Mac Apple Silicon, 2026-10-22)
- cargo test --manifest-path src-tauri/Cargo.toml --locked --quiet: 1006 PASS, 0 FAIL, 33 ignored, plus 1 CLI integration PASS.
- cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings: PASS.
- git diff --check: PASS.
- Fix scopes an existing helper to Unix-only tests; native Win11 GUI validation remains PENDING.
- Team Lead: acting assistant, TEAM_LEAD_SERIAL. No actual separate worker or independent reviewer was used.
- PR: https://github.com/TheerasakPing/nexora-code/pull/2 (commit in PR history).
- No app installation, GUI, MCP execution, production release or protected-branch merge.
