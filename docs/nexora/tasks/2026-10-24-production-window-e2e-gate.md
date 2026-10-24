# NC-TEAM-003 — Production-grade GUI readiness smoke (2026-10-24)

**Worktree:** feat/production-gui-acceptance-20261024. **Lead/worker:** TEAM_LEAD_SERIAL. No false external worker claims.
**Input:** PR #38 merged to develop SHA 6dd348b; acceptance docs PR #39 merged SHA 6541b. Source already Rust/Tauri v2/React.
**Aim:** strengthen disposable hosted native installer smoke to assert a real app window (Windows/Linux) in addition to process survival; fail closed on missing GUI. Never use signing keys, inspect user data or publish release.
**Limits:** Windows Server 2022 != Windows 11; Xvfb X11 != Wayland/FUSE desktop acceptance; macOS existing local window smoke != all controls/provider E2E.

## Checklist planner (commit before implementation)
- [x] GUI-01 / TEAM_LEAD_SERIAL — audit CI startup smoke, host runner limitations, package install and cleanup.
- [ ] GUI-02 / TEAM_LEAD_SERIAL — Windows installed app: detect owned native window with bounded check (not just process survival).
- [ ] GUI-03 / TEAM_LEAD_SERIAL — Linux AppImage: ensure isolated X display and require a visible named X11 app window, bounded cleanup.
- [ ] GUI-04 / TEAM_LEAD_SERIAL — add tests for ownership/timeouts/non-GUI fixtures; keep existing installer-survival contract.
- [ ] GUI-05 / TEAM_LEAD_SERIAL — Mac local Node/Release/Frontend, Rust/Clippy regression, diff/shell check. E2E/smoke on hosted Win/Linux exact PR SHA.
- [ ] GUI-06 / TEAM_LEAD_SERIAL — PR against develop, hosted CI and three unsigned installer jobs; Team Lead compare + review evidence.
- [ ] GUI-07 / HUMAN_SECURITY_REVIEW_PENDING — independent human code review, Mac rendered GUI + provider E2E, Windows 11 and Linux Wayland/FUSE acceptance, publisher signing/notarization; never waive.

## Mandatory test matrix
| Subtask | Worker | Unit/Contract | E2E | Smoke | Lead Review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| GUI-01 | TEAM_LEAD_SERIAL | N/A diagnostic | N/A diagnostic | Existing 3-OS alpha CI PASS | GO diagnosis | CI current window check missing |
| GUI-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | native Win window owner/geometry |
| GUI-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Xvfb visible named window |
| GUI-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | injected test fixtures |
| GUI-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | local tests |
| GUI-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | exact hosted PR SHA |
| GUI-07 | HUMAN_SECURITY_REVIEW_PENDING | PENDING | PENDING | PENDING | PENDING | independent approval/keys |

**Release verdict at start: NO-GO**, until independent review, production signing and native full functional acceptance.

## Source implementation evidence (before hosted CI)
- Windows hosted-window probe: installed app PID, nonzero native handle, expected window title, two-second recheck, root/taskkill cleanup; wrapper will fail if no window.
- Linux hosted-window probe: private Xvfb/DBus display, named visible window >= 860×580, geometry stability after two seconds, isolated setsid app group bounded TERM/KILL cleanup.
- Local contract test 14 cases: 13 PASS, 1 Linux behavioral fixture SKIPPED on Mac (must run on Linux hosted).
- Read-only Codex QA flagged a Linux P1 cleanup issue; isolated POSIX process group and bounded shutdown added. Windows descendants on root crash remain runner-contained, not equivalent to production leak acceptance.
- Full native GUI feature interactions on Windows11/Linux Wayland/real provider remain PENDING, even after hosted Xvfb/Windows Server window smoke.
