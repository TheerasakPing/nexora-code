# NC-TEAM-003 — Alpha packaging acceptance hardening (2026-10-24)

**Parent:** PR #38 integration branch. **Team Lead:** ChatGPT Integration Lead.
**Actual reviewers:** QA_REVIEW_AI_2 (read-only packaging audit); Lead is implementation owner. Not a human independent approval.
**Intent:** make unsigned test artifacts fail closed on source tests and ad-hoc signature integrity, and rerun on relevant source changes.
**Scope:** .github/workflows/nexora-r6-alpha.yml, scripts/nexora/*.sh and contract tests; no production app runtime changes, secrets, signing keys or release publication.

## Checklist planner (created & committed BEFORE patch)
- [x] PKG-01 / QA_REVIEW_AI_2 — review prior SHA a823ab0: source-only changes fail to trigger packaging; packaging lacks source-level CI gate; DMG validation allows other valid signatures. Remaining GUI/Windows11/FUSE gaps documented separately.
- [x] PKG-02 / TEAM_LEAD_SERIAL — source/capability/Cargo triggers plus per-OS same-SHA Node/Frontend/Rust/Clippy gate before artifact upload; hosted validation on new head still PENDING.
- [x] PKG-03 / TEAM_LEAD_SERIAL — staging always re-signs ad-hoc, staging+mounted app reject `Authority=`; prior GitHub DMG verified locally with strict new verifier: checksum, arm64, seal PASS, Gatekeeper REJECTED as expected.
- [x] PKG-04 / TEAM_LEAD_SERIAL — two new JS contract tests; targeted Mac preview tests 6/6 PASS, source triggers and signing metadata checks covered.
- [x] PKG-05 / TEAM_LEAD_SERIAL — local Mac M4 Rust 1120 PASS/0 FAIL/33 ignored, Clippy -D warnings PASS, Node 210/210, release contracts 33/33 and frontend build PASS, `bash -n`/diff PASS; source Lead GO for test-only alpha workflow patch.
- [ ] PKG-06 / TEAM_LEAD_SERIAL — push isolated branch to PR #38, dispatch unsigned exact-HEAD 3 OS installers, link CI and artifacts; do not merge protected develop/main.
- [ ] PKG-07 / HUMAN_REVIEW_PENDING — installed Mac GUI, actual Windows 11, Linux FUSE/Wayland and live provider session smoke; human security approval.

## Verification matrix — NC-TEAM-003
| Subtask | Worker | Unit/Contract | E2E | Smoke | Lead Review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| PKG-01 | QA_REVIEW_AI_2 | N/A (audit) | N/A (audit) | N/A (audit) | GO for diagnosis | Read-only AI audit 6 findings, not human review |
| PKG-02 | TEAM_LEAD_SERIAL | PASS | PENDING (new hosted installer) | PASS local contracts | GO source | CI workflow checks same source HEAD before upload on each target; hosted verification pending |
| PKG-03 | TEAM_LEAD_SERIAL | PASS | PASS Mac DMG integrity (old preview) | PASS | GO source | New verifier tested against real old GH DMG: SHA256, arm64 and ad-hoc PASS, Gatekeeper REJECTED expected; current build hosted PENDING |
| PKG-04 | TEAM_LEAD_SERIAL | PASS | N/A (static policy regression, no service) | PASS | GO source | 6 Mac contract tests PASS incl two new assertions; enforces source triggers/signature |
| PKG-05 | TEAM_LEAD_SERIAL | PASS | PASS native crash/PTY regression (Mac) | PASS | GO source | Mac Rust 1120 pass/33 ignored, Clippy PASS, Node/Release, frontend build PASS, shell syntax/diff PASS |
| PKG-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Hosted SHA and test-only artifacts |
| PKG-07 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | PENDING | No real Windows11/Gatekeeper/provider GUI acceptance |

**Release decision:** source-only PR may receive Lead GO; Protected merge and signed production release NO-GO pending independent human review and native installed acceptance. `windows-2022` runner is not Windows 11, while Linux Xvfb/AppImage is not FUSE/Wayland production acceptance.
