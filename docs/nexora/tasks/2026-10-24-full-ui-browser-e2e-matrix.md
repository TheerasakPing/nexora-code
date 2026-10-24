# NC-TEAM-003 — Full 744-case UI Browser E2E Matrix (2026-10-24)

**Lead/actual worker:** TEAM_LEAD_SERIAL. No invented subagent execution or human sign-off.
**Base:** PR #40 latest develop-synced head `ad0eb64b`. Target branch scoped to Playwright CI only; no application runtime changes.
**Motivation:** PR #42's four-suit UI E2E passed 33/33 twice on hosted Linux. Broader Playwright config enumerates **744 cases from 57 files**. Mac local full 4-worker run stopped after **12 failures** dominated by missing Preview selected task at cold boot. Isolated Agent Input 2 worker had 4/7 PASS; experimental Vite warmup 4 workers 3/7 PASS (rejected; no fix shipped). Need reliable hosted Linux baseline to distinguish platform flake from real functional bugs.

## Checklist planner (first commit before any new test source/workflow changes)
- [x] FULL-01 / TEAM_LEAD_SERIAL — audit selected full-suite test count, logs, environment and root-cause hypothesis; no unfounded PASS.
- [ ] FULL-02 / TEAM_LEAD_SERIAL — create disposable GitHub-hosted Chromium 4-shard workflow (all 744, zero retries, safe concurrency), no real user data, no secrets.
- [ ] FULL-03 / TEAM_LEAD_SERIAL — run all shard jobs; collect exact failures/traces and screenshot artifacts; no skipping tests to improve reported rate.
- [ ] FULL-04 / TEAM_LEAD_SERIAL — fix actual root causes with regressions after reproducible hosted trace; rerun until all shards green or explicitly enumerate remaining blockers.
- [ ] FULL-05 / TEAM_LEAD_SERIAL — check TypeScript/Node, Rust/Clippy cross-platform and shell smoke; Team Lead inspect actual diff and independent AI diagnostic review when available.
- [ ] FULL-06 / TEAM_LEAD_SERIAL — merge only after source checks and E2E pass; protected develop and production stay governed by independent human/release gates.
- [ ] FULL-07 / HUMAN_REVIEW_PENDING — installed Mac/Win11/Linux GUI/provider E2E, external security review and digital signing.

## Required per-task matrix
| Task | Worker | Unit/Contract | Browser E2E | Smoke | Lead Review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| FULL-01 | TEAM_LEAD_SERIAL | Audit | Mac 12 FAIL at cap / 744 total | Mac Chrome boot flaky | GO diagnostic | PR #40 audit comment |
| FULL-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | hosted matrix |
| FULL-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | shard reports |
| FULL-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | trace-backed fixes |
| FULL-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | CI |
| FULL-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | source review |
| FULL-07 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO production | genuine independent approval |

**Policy:** Browser UI tests never substitute a signed/notarized installed 3-OS native application acceptance.
