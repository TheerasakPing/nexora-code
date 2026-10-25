# NC-TEAM-003 — Isolated-browser E2E 4-shard stabilization (2026-10-25)
**Team Lead/real worker:** TEAM_LEAD_SERIAL. Independent human reviewer PENDING.
**Previous evidence:** [full hosted run 40145856789](https://github.com/TheerasakPing/nexora-code/actions/runs/40145856789) on source SHA 03b3098 had 732 PASS / 12 FAIL (Shard1 186/186, Shard2 181/186, Shard3 185/186, Shard4 180/186). Browser check and 1/1 pinned Wallpaper passed each shard. Some residual test names changed from previous 16, consistent with shared Vite/Chrome scheduling pressure and native timing race; do NOT claim a proven root cause.
**Change hypothesis:** One Chromium worker per *isolated GitHub-hosted Mac* avoids simultaneous Vite module-interception and UI state fixture contention inside each runner. Preserve 4 shards (186 cases each), all 744 tests, max runtime 35min/job, zero retries, no test skips, screenshot trace failure artifacts.
## Checklist planner committed before workflow source change
- [x] ISOLATE-01 / TEAM_LEAD_SERIAL — audit honest full 744 outcomes and nonrepeatable failing case names.
- [ ] ISOLATE-02 / TEAM_LEAD_SERIAL — lower per-runner E2E workers 2→1 with same 4-shard, 0-retry selection; no source/app behavior change.
- [ ] ISOLATE-03 / TEAM_LEAD_SERIAL — run 4 hosted shards, assess timing, true pass/fail, investigate stable failures without false pass.
- [ ] ISOLATE-04 / TEAM_LEAD_SERIAL — Node/Release/TS/Vite + 33-case E2E and cross-OS smoke, read-only QA reviewer, Team Lead merge gate.
- [ ] ISOLATE-05 / HUMAN_REVIEW_PENDING — actual installed Mac/Win11/Linux GUI/provider and signed production approval separately blocked.
| Item | Worker | Unit/Contract | E2E | Smoke | Lead QA |
| --- | --- | --- | --- | --- | --- |
| ISOLATE-01 | TEAM_LEAD_SERIAL | PASS diagnostics | 732/744 baseline | Mac 4/4 wallpaper | GO diagnostic |
| ISOLATE-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| ISOLATE-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| ISOLATE-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| ISOLATE-05 | HUMAN_REVIEW_PENDING | PENDING | PENDING native | PENDING | NO-GO |
