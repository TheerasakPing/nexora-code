# NC-TEAM-003 — Last Jev task-switch Browser lifecycle E2E
**Date:** 2026-10-25 Asia/Bangkok. **Actual executing worker/Team Lead:** TEAM_LEAD_SERIAL. **Human independent approval:** pending.

**Exact failing hosted evidence:** PR #44 SHA b3ee1ca, [Mac 744 suite 40156688673](https://github.com/TheerasakPing/nexora-code/actions/runs/40156688673), **743 PASS / 1 FAIL**, zero retries. All three other 186-case shards PASS, shard 2 185/186. Last failing test `tests/jev-live.spec.ts:127` task switch: `openBrowser(page,false)` expects a visible Browser URL textbox after clicking Open browser; snapshot shows that the Browser panel and *correct task's* Jev live viewer are actually visible and active, so browser URL is hidden by design during live playback. Correct acceptance must assert run ownership, old lease release, new viewer visible, and that Return to browser restores URL controls. Do not weaken lease isolation or disable any test.
**Other exact-head gates:** CI, 33/33 UI, Native Rust Windows/macOS/Linux, unsigned installers all PASS. Trusted production remains blocked independently by signing and human review.

## Checklist planner — commit BEFORE modifying test/runtime
- [x] JEV-LAST-01 / TEAM_LEAD_SERIAL — download hosted failure screenshot/context, compare source BrowserPanel and JevLiveView state machine.
- [ ] JEV-LAST-02 / TEAM_LEAD_SERIAL — update only Jev switch test: after Open browser accept the correct *live Jev view*, verify lease and report are for the new task, use Return to browser to prove URL view is available and lease closes, maintain old-task isolation assertions.
- [ ] JEV-LAST-03 / TEAM_LEAD_SERIAL — run focused Jev and whole 6+ Jev Browser tests Mac Chrome, zero retries; Node/Release/Build/Smoke as feasible.
- [ ] JEV-LAST-04 / TEAM_LEAD_SERIAL — push PR #44 and confirm 4 hosted 186-case shards 744/744, all same-head gate checks; inspect Team Lead QA before merge.
- [ ] JEV-LAST-05 / HUMAN_REVIEW_PENDING — do not assert independent human approval, signed/notarized production or installed Win11/Ubuntu/Wayland native provider E2E without real proof.
| Item | Worker | Unit/Contract | E2E | Smoke | Lead |
| --- | --- | --- | --- | --- | --- |
| JEV-LAST-01 | TEAM_LEAD_SERIAL | PASS source/trace | 743/744 hosted baseline | Screenshot verified | GO diagnosis |
| JEV-LAST-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| JEV-LAST-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| JEV-LAST-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| JEV-LAST-05 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO |
