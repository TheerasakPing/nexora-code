# Task Checklist Planner — R1 Race-safe Workspace Preview File Handles

**Request/date:** 2026-10-23 / Continue remaining Nexora Code roadmap: close actionable high-priority security gap while prior PR #26 CI is green.
**Phase/Priority:** R1 / P0; scoped file read and image preview. This DOES NOT close all workspace/TOCTOU attack surfaces.
**Team Lead:** ChatGPT Integration Lead, acting TEAM_LEAD_SERIAL; Studio MCP does not expose engineer-task delegation API. No independent reviewer or autonomous agent assigned.
**Base:** PR #26 current head d5babeb40a9fbb9eb5f6a5ec023843bbd75d77db; separate worktree /Volumes/Data/workspace/git/nexora-code-integration-20261023.
**Security boundaries:** no user data/installed app/Keychain changes, no external MCP, no global Gatekeeper changes. Only safe temp-fixture tests.
**Accepted design:** Unix file/image preview must use dirfd-rooted openat no-follow walks so canonicalize-then-reopen is not exposed to path swap. Follow no symlinks in this operation, even internal aliases, rather than claiming a race-safe symlink follow; document compatible user impact. Windows targeted path has separate handle validation; do not claim full Windows-native smoke from Mac.
**Out of scope:** Git stage/diff/external git race, list_files directory enumeration and file_search cache, complete Windows reparse handling if not implemented, native WebKit GUI/provider tests, review, signing.

## B. Task decomposition and delegated ownership

| ID | Work & acceptance | Actual worker | Owned files | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-SAFE-01 | Add anchored, bounded OS-handle-based workspace preview opener; reject path symlink traversal | TEAM_LEAD_SERIAL | src-tauri/src/workspace.rs | none | [x] |
| R1-SAFE-02 | Race/adversarial symlink tests for file/image preview, including internal aliases and changed target | TEAM_LEAD_SERIAL | src-tauri/src/workspace.rs tests | R1-SAFE-01 | [x] |
| R1-SAFE-03 | Add source contract tests + scope/limitations doc; ensure no new user data or protocol changes | TEAM_LEAD_SERIAL | tests/, docs/nexora/ | R1-SAFE-01 | [x] |
| R1-SAFE-04 | Validate Unit, E2E, Smoke, source review and exact CI; push PR #26 | TEAM_LEAD_SERIAL | this plan and PR | R1-SAFE-01..03 | [ ] |
| R1-SAFE-REVIEW | Independent reviewer prior develop merge | INDEPENDENT_REVIEWER_PENDING | PR | R1-SAFE-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-SAFE-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | macOS M4: cargo test --lib workspace::tests:: 30/30, anchored OS handle read + image IPC underlying functions; cargo check --bin nexora-code PASS, team lead inspected source diff, 2026-10-23 |
| R1-SAFE-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | macOS M4: Rust 30/30 targeted, unit + actual in-process symlink swapping 400 times, FIFO nonblocking; linked file/image outside rejected; Lead verified no external data accessed |
| R1-SAFE-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | macOS M4: Node static 3/3, full npm test 178/178, release 33/33, Vite build PASS, docs security scope and compatibility explicitly listed; Lead review 2026-10-23 |
| R1-SAFE-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | CI exact-head and PR handoff pending |
| R1-SAFE-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No genuine independent reviewer yet |

## C. Checklist for lead acceptance

- [x] Fresh dated planner written before source edits; actual workers assessed, serial fallback recorded
- [x] Task decomposition, dependencies, file ownership and narrow threat model
- [x] Test triad passed for R1-SAFE-01/02/03 at local source/native function layer; exact-head CI PENDING
- [x] Team Lead examined diff and symlink compatibility; rejecting *internal* preview symlinks is intentional and documented
- [ ] Current SHA GitHub exact-head six required CI + 3 OS unsigned installers must pass (Mac-local source checks already passed)
- [ ] Final Team Lead source GO/NO-GO and next blockers documented
- [ ] Genuine independent reviewer and native OS acceptance completed before merge/release

## E. Team Lead initial decision

**Team Lead verdict:** SOURCE REVIEW GO for PR submission after local tests, not native acceptance.
**GO/NO-GO:** GO limited code/test/doc scope; NO-GO protected develop merge and production without independent human reviewer/native validation.
**Historical cross-cutting backlog:** 21/24 engineering packages unchecked, branding 7/7 signoff pending, 61 R0-R8 acceptance boxes unchecked; views overlap and are not a new percentage.

## Local verification ledger — Team Lead review 2026-10-23

- Environment: actual Mac M4, isolated temporary test folders; no user workspace or installed app changed.
- Focused Rust: `CARGO_TARGET_DIR=/Volumes/Data/workspace/git/nexora-code/src-tauri/target cargo test --locked --manifest-path src-tauri/Cargo.toml --lib workspace::tests:: --quiet`: **30 PASS / 0 FAIL**.
- E2E (module integration security path): test entrypoint `read_file` and `image_data_url` against a temporary real filesystem with outside symlinks, inside links, path traversal, case-spelled .git, 400 competing swap cycles, and a FIFO; **PASS**. This is not a Tauri GUI or cross-process multi-agent E2E.
- Source policy: `node --test tests/nexora-r1-workspace-preview-handles.test.mjs`: **3 PASS**; `npm test`: **178 PASS**; `npm run test:release`: **33 PASS**.
- Smoke: `cargo check --locked --bin nexora-code`: PASS on Mac M4; `npm run build`: PASS, some existing Vite chunk-size warnings; no application user project touched.
- Threat-model review: Unix anchored openat O_NOFOLLOW across all components; final O_NONBLOCK + fstat rejects FIFO/special files. Windows uses final FILE_FLAG_OPEN_REPARSE_POINT and kernel-resolved handle location against canonical root. Windows native acceptance still requires exact-head CI and a real device.
- Compatibility: previews of in-project symlink aliases now reject by design; opening target file directly works. `list_files`, Git operations and other mutable path APIs are not claimed fixed.
- Remaining tests: full Rust lib regression running locally; GitHub 3-OS CI after push still required, genuine independent reviewer and native installed GUI E2E PENDING.
- Team Lead verdict: SOURCE REVIEW GO for PR only, not protected merge. No workers delegated; TEAM_LEAD_SERIAL alone is not independent review.
