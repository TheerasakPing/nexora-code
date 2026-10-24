# Task Checklist Planner — R1 npmrc Windows absolute-path test fixture

**Date / request:** 2026-10-23 / continue R1 strict acceptance.
**Lead / actual worker:** TEAM_LEAD_SERIAL. Independent reviewer pending; no invented delegated agent.
**Repository / worktree:** TheerasakPing/nexora-code, PR #34, fix/r1-finder-npmrc-bounded-20261023, /Volumes/Data/workspace/git/nexora-code-r1-npmrc-bounded-20261023.
**Baseline:** PR #34 8d73b3c, hosted 6 checks 5 PASS / 1 FAIL Windows. Windows npmrc_discovery_is_bounded_and_preserves_prefix_priority test failed expected Some("/safe/bin") vs actual None: /safe is not absolute on Windows. macOS/Linux checks PASS and unsigned installer 3/3 PASS. Windows job 120481588419, run 39973298519.
**Acceptance:** Use true OS-absolute fixture prefix derived from canonical temp path. Verify 64KiB file accepted and >64KiB rejected without changing runtime logic, preserve env override precedence and symlink/FIFO guards; actual Mac Clippy, Rust all, Node and Release; Windows CI exact-head required. No production users or system files modified.
**Out of scope:** real Windows 11 installed GUI smoke, external provider side effects, independent review, whole R1 exit or R2–R8.
**Safety:** only isolated temporary fixture, only test source / docs changed; no real npmrc or user data.

## B. Task decomposition / file owners

| Subtask ID | Goal | Actual worker | Files | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-NPMWIN-01 | Repair fixture to use portable absolute root for 64KiB npmrc and assert expected prefix | TEAM_LEAD_SERIAL | src-tauri/src/providers/launch.rs (tests) | none | [x] |
| R1-NPMWIN-02 | Retest focused + full Rust/Node/Release and static planner; lead diff review | TEAM_LEAD_SERIAL | tests/evidence and this planner | R1-NPMWIN-01 | [x] |
| R1-NPMWIN-03 | Push PR #34 exact HEAD and request 3-OS CI, track Windows native test outcome | TEAM_LEAD_SERIAL | PR #34 | R1-NPMWIN-02 | [ ] |
| R1-NPMWIN-REVIEW | Outside review, GUI acceptance, R1 exit decisions | INDEPENDENT_REVIEWER_PENDING | GitHub gate | R1-NPMWIN-03 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-NPMWIN-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 targeted Rust PASS and all 1115 PASS; path derived from canonical fixture root (portable Windows + Unix); Team Lead reviewed only test diff; Windows hosted CI still pending |
| R1-NPMWIN-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 Rust lib 1115 PASS / 0 FAIL / 33 ignored, Clippy -D warnings PASS, Node 208/208, Release 33/33, frontend Vite production build PASS; native real temp-file module E2E preserved from parent; Lead diff review GO |
| R1-NPMWIN-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | exact-head Windows/macOS/Linux CI pending |
| R1-NPMWIN-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | independent reviewer not available |

## C. Checklist

- [x] New dated planner committed before changes
- [x] Exact hosted failed assertion and OS path root cause captured
- [x] Fix portable fixture, no runtime behavior modifications
- [x] Test triad plus Team Lead review documented for implementation subtasks 01/02 (not OS-installed GUI)
- [ ] All exact-head checks pass (including Windows), review approved
- [ ] R1 real native GUI and whole-phase acceptance completed

## E. Team Lead GO/NO-GO

**Team Lead verdict:** GO for PR source/test correction once local regression passes. **GO/NO-GO:** Source-only GO; NO-GO protected branch merge and entire R1 exit pending exact-head hosted CI, independent review and real installed-native E2E.

## Lead source acceptance ledger, 2026-10-23

- Root cause / Windows hosted job: failure line 554 at commit 8d73b3c: expected Some("/safe/bin") but actual None, because POSIX /safe is not an absolute drive-rooted Path on Windows.
- Change only to tests: derive prefix with f.0.join("safe-npm-prefix") from canonical OS temp root, assert Path::is_absolute, generate exactly 65536 bytes and compare parsed prefix with safe_prefix.join("bin") cross-platform. No app runtime or config reader behavior altered.
- Unit/Contract: cargo test --lib providers::launch::tests::npmrc_discovery_is_bounded_and_preserves_prefix_priority PASS on Mac M4; full cargo test --lib 1115 PASS / 0 FAIL / 33 ignored.
- Native module-level E2E: pre-existing tempfile + special-file/FIFO and symlink guards, verified by full Rust suite. No actual GUI/Provider process launched.
- Smoke: Mac cargo clippy --all-targets -D warnings PASS, frontend npm run build PASS (existing chunk advisory), npm test 208/208 PASS, npm run test:release 33/33 PASS.
- Team Lead review: inspected minimal test-only diff, no production files or user dotfiles touched. Team Lead GO for PR source/test submission; NO-GO full R1, protected merge pending exact-head 3-OS CI and independent review.
- Worker: TEAM_LEAD_SERIAL only; no independent engineer evidence.
