# NC-TEAM-003 — Five remaining hosted macOS UI E2E failures
**Date:** 2026-10-25 Asia/Bangkok. **Actual worker/Team Lead:** TEAM_LEAD_SERIAL. **Human independent reviewer:** PENDING.
**Baseline:** HEAD 526b858 on PR #44; GitHub full UI run 40151762269: **739/744 PASS, 5 FAIL**, 0 retries, Shard 1 and Shard 3 186/186 PASS, Shard 2 185/186, Shard 4 183/186. CI, expanded UI 33, hosted 3-OS Rust and unsigned installer smoke all PASS at this SHA.
**Verified failures:**
- Shard1: project Board archived-after-week fixture click on Filter projects immediately following page.reload, no menu item.
- Shard2: Jev session change, active run in new task not watched in 5s after old stopped, need verify async browser teardown/list runs and ownership isolation.
- Shard4: Permissions selection rejected by native fixture leaves error on *open menu*, but test re-click expecting it to have closed; two Usage Limit tests intermittently lose limit banner/control after limited/error/done simulation.
**Quality rules:** No disabling/flaking tests, no retry, no removing accessibility checks, no bypass of bounded session ownership or usage limits.
## Checklist Planner committed before edit
- [x] FINAL5-01 / TEAM_LEAD_SERIAL: collect exact 5 error stacks and separate fixture timing from application logic.
- [ ] FINAL5-02 / TEAM_LEAD_SERIAL: adjust Board reload readiness and Permissions failed-save expected UX, browser E2E smoke.
- [ ] FINAL5-03 / TEAM_LEAD_SERIAL: determine root cause of Jev lease after task switch, preserve task lease isolation; targeted UI E2E.
- [ ] FINAL5-04 / TEAM_LEAD_SERIAL: reproduce Usage Limit limited/error/done race, fix actual bug or fixture timing with failing tests first.
- [ ] FINAL5-05 / TEAM_LEAD_SERIAL: run all 744 macOS zero-retry and 33 expanded tests, Node/Release, 3-OS Rust and Installer; code review and security smoke.
- [ ] FINAL5-06 / HUMAN_REVIEW_PENDING: human security approval, installed native provider E2E, Apple signing/notarization, Windows signing and Linux distribution production gates are NO-GO.
| Task | Real worker | Unit/Contract | E2E | Smoke | Team Lead |
| --- | --- | --- | --- | --- | --- |
| FINAL5-01 | TEAM_LEAD_SERIAL | PASS triage | 739/744 hosted baseline | 4 shards completed | GO diagnosis |
| FINAL5-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| FINAL5-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| FINAL5-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| FINAL5-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| FINAL5-06 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO |
