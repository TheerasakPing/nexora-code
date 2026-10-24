# NC-TEAM-003 — Portable screenshots for all full UI browser specs
**Date:** 2026-10-24 Asia/Bangkok. **Lead/worker:** TEAM_LEAD_SERIAL. Independent human reviewer pending.
**Parent:** PR #43 full 744-case E2E at SHA 5a80b8f. Four GitHub-hosted Linux Chromium shards ran 744 cases and all failed: 38,24,14,40 failures respectively (116 total); a large class of failures were ENOENT from macOS-only `/private/tmp` hardcoded screenshot paths. Others include genuine UI click/load timeouts, to be investigated separately.
**Scope:** change only screenshot *destinations* inside Playwright browser specs to `test.info().outputPath`, preserving screenshot calls, animations, every assertion and all 744 cases. Special nested folder names flattened within a per-test folder. No app runtime mutation, skipped test or retry.
## Mandatory checklist planner (committed before editing source)
- [x] SHOT-01 / TEAM_LEAD_SERIAL — gather hosted run 40113014356 4-shard logs, classify ENOENT paths vs UI functional failures.
- [x] SHOT-02 / TEAM_LEAD_SERIAL — rewrite all static and templated `/private/tmp` screenshot paths to portable per-test Playwright paths, no skipped tests.
- [ ] SHOT-03 / TEAM_LEAD_SERIAL — test contract: no `/private/tmp` screenshots left; TypeScript build + browser screenshot samples, 33-case targeted E2E 33/33; run hosted full shard matrix.
- [ ] SHOT-04 / TEAM_LEAD_SERIAL — inspect residual failures from hosted logs and fix separately, document honest PASS/FAIL results; preserve artifacts.
- [ ] SHOT-05 / TEAM_LEAD_SERIAL — branch/PR review, exact SHA hosted checks and Team Lead acceptance before protected merge.
- [ ] SHOT-06 / HUMAN_REVIEW_PENDING — independent human security approval and signed 3-OS installed GUI/provider E2E remain production NO-GO.
| Subtask | Actual worker | Unit/Contract | E2E | Smoke | Team Lead | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| SHOT-01 | TEAM_LEAD_SERIAL | Audit | 116 failing across 4 shards | N/A | GO diagnostics | hosted 40113014356 |
| SHOT-02 | TEAM_LEAD_SERIAL | PASS static count | PENDING hosted full E2E | PASS local path assertions | GO source | 64 paths across 16 spec files, no skips |
| SHOT-03 | TEAM_LEAD_SERIAL | PASS portable-path Node contract | PENDING hosted E2E | PASS Vite/TypeScript build, 744 spec discovered | GO for local checks | 1/1 Node regression, npm audit 0, Vite build PASS |
| SHOT-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | residual failure logs |
| SHOT-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | PR |
| SHOT-06 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO | independent review |

## Local source evidence before push
- Portable screenshot destination substitutions: **64**, exactly **16 spec files**, no `/private/tmp/` references in any Playwright browser spec. Nested screenshot paths flattened to filenames while retaining every screenshot, animation option and assertion.
- `node --test scripts/release/ui-artifact-paths.test.mjs`: 1/1 PASS.
- `npm ci --ignore-scripts`: PASS and 0 vulnerabilities.
- `npm run build`: TS and Vite PASS (large chunk warning is nonfatal).
- `npx playwright test --list`: 744 tests / 57 files discovered, no test removed.
- `git diff --check`: PASS. Hosted 4-shard retest remains required before merge.
