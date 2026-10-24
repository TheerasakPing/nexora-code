# NC-TEAM-003 — Expanded UI Functional E2E (2026-10-24)

**Lead / real worker:** TEAM_LEAD_SERIAL. No fictitious delegated agent or human approval.
**Base:** PR #40, security fixes retained; stacked PR #42, tests only. Rust/Tauri v2/React original UI and features unchanged.
**Request:** 33 expanded Chromium UI tests across Terminal, Settings, Agent Teams, MCP Inspector must PASS without retries or skipped tests.
**Original issue (Mac):** Previous 33-case Mac browser run showed an incomplete Preview bootstrap; at least 4 errors captured under concurrent CPU contention. This was not an exhaustive final failure count.
**Hosted Linux reproduction:** GitHub run [40109772312](https://github.com/TheerasakPing/nexora-code/actions/runs/40109772312) provided reliable disposable Chromium + traces: **31 PASS, 2 FAIL**.
**Root cause (actual evidence):** Exactly two Terminal tests wrote screenshots to macOS-specific `/private/tmp`; Linux failed with `ENOENT` before later assertions. The Preview seeds and functional UI passed other 31 cases.
**Fix:** Both screenshots now write to `test.info().outputPath(...)`, isolated per Playwright test, cross-platform and no assertion removal.
**Verified E2E:** [Run 40109957999](https://github.com/TheerasakPing/nexora-code/actions/runs/40109957999), commit `bc1754d08ca2006aed1acb22f43d53304f62b93b`, **33/33 PASS, 0 FAIL, 0 SKIP, retries=0, 50.0s**, Linux Chromium two workers. Hosted frontend TypeScript/Vite build PASS.

## Checklist Planner — actual execution
- [x] UI-E2E-01 / TEAM_LEAD_SERIAL — audit Mac failures and fixture/boot path; determine root cause only after hosted reproduction.
- [x] UI-E2E-02 / TEAM_LEAD_SERIAL — add hosted Linux Chromium 4-suite workflow, no credentials, screenshots/traces on failure.
- [x] UI-E2E-03 / TEAM_LEAD_SERIAL — reproduce 31/33, identify two hardcoded Mac screenshot paths, patch only path to Playwright's per-test output directory.
- [x] UI-E2E-04 / TEAM_LEAD_SERIAL — full 33/33 successful re-run, no retries or skips.
- [ ] UI-E2E-05 / TEAM_LEAD_SERIAL — exact final-head CI and R0 Rust baseline remain PENDING at documentation cut (the preceding code head's UI E2E passed).
- [x] UI-E2E-06 / TEAM_LEAD_SERIAL — create stacked PR #42 against PR #40 branch and perform Team Lead source-only review: no runtime changes, no test skipped, screenshots still captured.
- [ ] UI-E2E-07 / HUMAN_REVIEW_PENDING — independent human review and installed native GUI multi-OS Provider/PTY E2E and production signing remain NO-GO.

## G. Per-task quality gate matrix
| Task | Worker | Unit/Contract | Functional E2E | Smoke | Lead review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| UI-E2E-01 | TEAM_LEAD_SERIAL | N/A diagnosis | 31/33 baseline | PASS hosted harness | GO diagnosis | 2 ENOENT failures in Linux, not Preview startup |
| UI-E2E-02 | TEAM_LEAD_SERIAL | PASS TS build | PASS hosted Playwright start | PASS Vite/Chromium boot | GO source | Hosted workflow with trace retention |
| UI-E2E-03 | TEAM_LEAD_SERIAL | PASS screenshot path API | PASS 33/33 after patch | PASS output path | GO source | test.info().outputPath twice |
| UI-E2E-04 | TEAM_LEAD_SERIAL | PASS | **33/33 PASS** | PASS | GO browser E2E | 40109957999, retries=0, no skip |
| UI-E2E-05 | TEAM_LEAD_SERIAL | PENDING other CI | PENDING exact final-head run | PENDING | PENDING | Hosted baseline workflows |
| UI-E2E-06 | TEAM_LEAD_SERIAL | PASS diff review | PASS tested commit | PASS PR setup | GO for browser-only change | PR #42, independent approval absent |
| UI-E2E-07 | HUMAN_REVIEW_PENDING | PENDING | PENDING native installed | PENDING | NO-GO production | Human signoff and 3-OS installed |

**Scope constraint:** 33/33 passing is **browser preview functional E2E** on Linux Chromium, not evidence of installed Tauri GUI/provider user journey on all OS. Never equate it to production release acceptance or human approval.
