# NC-TEAM-003 — macOS Chrome parity for full UI E2E
**Date:** 2026-10-25 Asia/Bangkok. Lead/actual worker TEAM_LEAD_SERIAL. Independent human review absent.
**Evidence before patch:** PR #44 hosted macos-15 using Playwright-managed Chromium, run 40115492393: **684 PASS / 60 FAIL** (10/6/9/35 failures on shards 1/2/3/4). Many failures in wallpaper-related tests report no `data-wallpaper` even though test fixture enables/pins a scene. The **same `tests/wallpapers.spec.ts:64` passed 1/1 on connected Mac M4 using the repository's default installed Google Chrome** in 3.7s. Therefore browser/host rendering parity is a plausible cause; not yet proven.
**Strict scope:** Switch disposable hosted macOS testing to a verified installed Google Chrome binary if available, with explicit `test -x`, and run isolated pinned-wallpaper smoke first. Do NOT change production UX, CSS or obscure real reduced-transparency preferences. Keep all 744 full browser tests, 4 shards, no retries.

## Checklist Planner (commit before workflow edits)
- [x] CHROME-01 / TEAM_LEAD_SERIAL — investigate hosted shard failures; verify local Mac Chrome pinned wallpaper smoke 1/1 PASS.
- [x] CHROME-02 / TEAM_LEAD_SERIAL — align hosted macOS browser selection with local Chrome and add explicit wallpaper smoke; fail closed if missing.
- [ ] CHROME-03 / TEAM_LEAD_SERIAL — hosted 4-shard CI; report Chrome smoke and 744-case true PASS/FAIL without skipping assertions.
- [ ] CHROME-04 / TEAM_LEAD_SERIAL — inspect residual errors, run release/Node/TypeScript smoke; Team Lead review real diff.
- [ ] CHROME-05 / HUMAN_REVIEW_PENDING — independent security review and native 3-OS signed production acceptance remain blocked.
| Task | Worker | Unit/Contract | Browser E2E | Smoke | Lead | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| CHROME-01 | TEAM_LEAD_SERIAL | Diagnostic | Mac local 1/1 | PASS local | GO diagnostics | hosted 684/744 vs local wallpaper 1/1 |
| CHROME-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | browser selection |
| CHROME-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | hosted run |
| CHROME-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | test logs |
| CHROME-05 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO | human/3-OS |

## Implementation evidence before new hosted run
- Workflow now requires executable installed Google Chrome at official macOS application path and logs its version. No silent Chromium fallback; missing browser fails the job.
- CI logs only anonymous browser rendering preferences (reduced transparency, contrast, motion, visibility state, platform) to diagnose wallpaper; no tokens/private content.
- Isolated pinned wallpaper smoke must PASS before running full 744 cases. Four independent shards, two workers each, retries=0 unchanged.
- Local Mac 1-case wallpaper smoke PASS. YAML parser validation PASS. Hosted same-head outcome **PENDING**.
