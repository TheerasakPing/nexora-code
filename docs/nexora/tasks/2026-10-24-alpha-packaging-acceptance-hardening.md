# NC-TEAM-003 — Alpha packaging acceptance hardening (2026-10-24)

**Parent:** PR #38 integration branch. **Team Lead:** ChatGPT Integration Lead.
**Actual reviewers:** QA_REVIEW_AI_2 (read-only packaging audit); Lead is implementation owner. Not a human independent approval.
**Intent:** make unsigned test artifacts fail closed on source tests and ad-hoc signature integrity, and rerun on relevant source changes.
**Scope:** .github/workflows/nexora-r6-alpha.yml, scripts/nexora/*.sh and contract tests; no production app runtime changes, secrets, signing keys or release publication.

## Checklist planner (created & committed BEFORE patch)
- [x] PKG-01 / QA_REVIEW_AI_2 — review prior SHA a823ab0: source-only changes fail to trigger packaging; packaging lacks source-level CI gate; DMG validation allows other valid signatures. Remaining GUI/Windows11/FUSE gaps documented separately.
- [ ] PKG-02 / TEAM_LEAD_SERIAL — add source triggers and exact-run Node/Rust Unit/Contract, strict Clippy to each native packaging matrix before artifact publication.
- [ ] PKG-03 / TEAM_LEAD_SERIAL — make macOS preview enforce `Signature=adhoc`, reject `Authority=`; verify both staged and mounted app; preserve Gatekeeper warning.
- [ ] PKG-04 / TEAM_LEAD_SERIAL — expand contract tests to fail closed if any new source/security/signature gate is deleted.
- [ ] PKG-05 / TEAM_LEAD_SERIAL — run Node/Release contracts, Mac syntax & signature local smoke, Rust/native regression, Build, Clippy; inspect diff and record evidence before push.
- [ ] PKG-06 / TEAM_LEAD_SERIAL — push isolated branch to PR #38, dispatch unsigned exact-HEAD 3 OS installers, link CI and artifacts; do not merge protected develop/main.
- [ ] PKG-07 / HUMAN_REVIEW_PENDING — installed Mac GUI, actual Windows 11, Linux FUSE/Wayland and live provider session smoke; human security approval.

## Verification matrix — NC-TEAM-003
| Subtask | Worker | Unit/Contract | E2E | Smoke | Lead Review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| PKG-01 | QA_REVIEW_AI_2 | N/A (audit) | N/A (audit) | N/A (audit) | GO for diagnosis | Read-only AI audit 6 findings, not human review |
| PKG-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Workflow source checks + hosted matrix |
| PKG-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Mac codesign metadata test + DMG |
| PKG-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Node regression tests |
| PKG-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Mac source and test evidence |
| PKG-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Hosted SHA and test-only artifacts |
| PKG-07 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | PENDING | No real Windows11/Gatekeeper/provider GUI acceptance |

**Release decision:** source-only PR may receive Lead GO; Protected merge and signed production release NO-GO pending independent human review and native installed acceptance. `windows-2022` runner is not Windows 11, while Linux Xvfb/AppImage is not FUSE/Wayland production acceptance.
