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
| R1-SAFE-01 | Add anchored, bounded OS-handle-based workspace preview opener; reject path symlink traversal | TEAM_LEAD_SERIAL | src-tauri/src/workspace.rs | none | [ ] |
| R1-SAFE-02 | Race/adversarial symlink tests for file/image preview, including internal aliases and changed target | TEAM_LEAD_SERIAL | src-tauri/src/workspace.rs tests | R1-SAFE-01 | [ ] |
| R1-SAFE-03 | Add source contract tests + scope/limitations doc; ensure no new user data or protocol changes | TEAM_LEAD_SERIAL | tests/, docs/nexora/ | R1-SAFE-01 | [ ] |
| R1-SAFE-04 | Validate Unit, E2E, Smoke, source review and exact CI; push PR #26 | TEAM_LEAD_SERIAL | this plan and PR | R1-SAFE-01..03 | [ ] |
| R1-SAFE-REVIEW | Independent reviewer prior develop merge | INDEPENDENT_REVIEWER_PENDING | PR | R1-SAFE-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-SAFE-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Rust tests, macOS root-anchored open + symlink scenarios; current SHA pending |
| R1-SAFE-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Bounded symlink race fixture; run logs pending |
| R1-SAFE-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Node static contract, docs smoke/link check pending |
| R1-SAFE-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | CI exact-head and PR handoff pending |
| R1-SAFE-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No genuine independent reviewer yet |

## C. Checklist for lead acceptance

- [x] Fresh dated planner written before source edits; actual workers assessed, serial fallback recorded
- [x] Task decomposition, dependencies, file ownership and narrow threat model
- [ ] Test triad passed for each implementation subtask, command/OS/SHA recorded
- [ ] Threat model and compatibility reviewer inspection performed, no silent symlink follow regression
- [ ] Exact source diff / unit/integration regression / 3-OS CI checked
- [ ] Final Team Lead source GO/NO-GO and next blockers documented
- [ ] Genuine independent reviewer and native OS acceptance completed before merge/release

## E. Team Lead initial decision

**Team Lead verdict:** SOURCE REVIEW GO for PR submission after local tests, not native acceptance.
**GO/NO-GO:** GO limited code/test/doc scope; NO-GO protected develop merge and production without independent human reviewer/native validation.
**Historical cross-cutting backlog:** 21/24 engineering packages unchecked, branding 7/7 signoff pending, 61 R0-R8 acceptance boxes unchecked; views overlap and are not a new percentage.
