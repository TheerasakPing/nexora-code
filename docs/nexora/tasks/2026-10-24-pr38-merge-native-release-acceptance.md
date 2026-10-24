# NC-TEAM-003 — Post-merge native GUI + production release acceptance

**Date:** 2026-10-24 Asia/Bangkok
**Lead:** ChatGPT Integration Lead | **Actual source worker:** TEAM_LEAD_SERIAL
**Human independent reviewer:** NOT ASSIGNED, NOT APPROVED.
**Requested tasks:** merge PR #38 into develop; install and E2E smoke Native GUI 3 OS; human independent review; signed production release.
**Source reference:** user-authorized merge [PR #38](https://github.com/TheerasakPing/nexora-code/pull/38) at `6dd348b82b7343d8ccee372912ef18e647fbd676`; tested head `472158c63b7955d96220f3286b6f17e8ec6a1fa5`.
**Source checks:** CI, R0 cross-platform and strict same-SHA alpha installers all SUCCESS, [run 40052688950](https://github.com/TheerasakPing/nexora-code/actions/runs/40052688950).
**Merge governance:** GitHub develop protection required six green checks, strict branch checks, admin enforcement and conversations resolved; required approving review count was **0**. User explicitly instructed merge. This does **not** equal independent human audit approval.

## B. Checklist Planner
- [x] NATIVE-01 / TEAM_LEAD_SERIAL — inspect exact head, branch protection, required CI and human review state.
- [x] NATIVE-02 / TEAM_LEAD_SERIAL — merge exact SHA 472158c PR #38 via guarded GitHub merge to develop, resulting SHA 6dd348b. No bypass or force push.
- [x] NATIVE-03 / TEAM_LEAD_SERIAL — install unsigned Mac ARM64 DMG into disposable /tmp path with isolated HOME (no existing user app/data overwritten). Check checksum and codesign, launch actual native binary, WindowServer confirms 1440×940 window owned by process PID. Mac installed *startup/window* smoke only.
- [ ] NATIVE-03B / TEAM_LEAD_SERIAL — full macOS native user interactions (WKWebView contents, UI controls, Agent/PTY/MCP/provider E2E); sandbox-extension warning remains to investigate. Screenshot attempt returned `could not create image from window`, not a verified rendered screenshot.
- [x] NATIVE-04A / TEAM_LEAD_SERIAL — Windows Server 2022 x64 hosted unsigned installer + survival smoke PASS; Ubuntu 22.04 Xvfb x64 AppImage build/startup PASS.
- [ ] NATIVE-04B / NATIVE_QA_PENDING — actual interactive Windows **11** x64 WebView2/ConPTY/agent + Linux desktop FUSE/Wayland/X11 GUI and provider E2E; hosted process survival does not prove UI.
- [ ] NATIVE-05 / HUMAN_REVIEW_PENDING — obtain genuinely independent human code/security review; repo collaborators check returned only owner TheerasakPing. Prior AI reviewers are not human approvals.
- [ ] NATIVE-06 / RELEASE_OWNER_PENDING — production signing/notarization, trusted updater, rollback, migration, full security/soak/8-real-agents acceptance, signed Release.
- [x] NATIVE-07 / TEAM_LEAD_SERIAL — publish audit record into merged PR #38 as comment (issue tracker disabled: GitHub Issues create returned 410).
- [ ] NATIVE-08 / TEAM_LEAD_SERIAL — final production GO only after all missing evidence and real reviewer signatures.

## G. NC-TEAM-003 Verification Matrix
| ID | Worker | Unit/Contract | Native E2E | Smoke | Team Lead | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| NATIVE-01 | TEAM_LEAD_SERIAL | PASS | N/A read-only | PASS source CI | GO diagnostic | Branch protection 6 checks, reviewer count 0 |
| NATIVE-02 | TEAM_LEAD_SERIAL | PASS | PASS GitHub merge | PASS GitHub merge | GO | PR #38 merged SHA 6dd348b |
| NATIVE-03 | TEAM_LEAD_SERIAL | PASS DMG | PARTIAL window only | PASS Mac installed startup | GO for startup only | macOS M4 27.2 ARM64, PID 89635 alive 8s, PID 91112 visible 1440x940 window |
| NATIVE-03B | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | NO-GO | native rendered screenshot/UI user journey absent |
| NATIVE-04A | TEAM_LEAD_SERIAL | PASS hosted contracts | PARTIAL process survival only | PASS hosted | GO for package only | run 40052688950 |
| NATIVE-04B | NATIVE_QA_PENDING | PENDING | PENDING | PENDING | NO-GO | Windows11/desktop Linux unavailable |
| NATIVE-05 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO | only repo owner listed; independent reviewer absent |
| NATIVE-06 | RELEASE_OWNER_PENDING | PENDING | PENDING | PENDING | NO-GO | zero Mac signing identities, no GitHub repo secret names returned |
| NATIVE-07 | TEAM_LEAD_SERIAL | PASS | N/A audit | PASS | GO | https://github.com/TheerasakPing/nexora-code/pull/38#issuecomment-6385269391 |
| NATIVE-08 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | NO-GO | Cannot sign/release until acceptance |

**Production status: NO-GO.** Installer artifacts are **unsigned engineering Alpha**, NOT a signed, notarized stable release; cannot claim Windows 11 installed GUI success, human review or production signing without evidence.

**Process note:** Local planner was initially committed in an isolated Mac worktree before merge; the Mac tool tunnel disconnected following test execution. This GitHub-backed copy preserves verifiable acceptance and follow-up obligations without overwriting the source commit.
