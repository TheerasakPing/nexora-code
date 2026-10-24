# NC-TEAM-003 — Correct macOS CI accessibility baseline without touching production
**Date:** 2026-10-25 Asia/Bangkok. Lead/actual worker: TEAM_LEAD_SERIAL. Human independent reviewer pending.
**Verified evidence:** GitHub-hosted macOS Chrome v152 (run 40116407912, all four shards) reports `matchMedia('(prefers-reduced-transparency: reduce)').matches === true` even with contrast and motion disabled. All four headless Chrome wallpaper smoke jobs failed waiting for `.workspace[data-wallpaper="mon-morning"]`. Local connected Mac M4 Chrome reports the same preference FALSE and the exact wallpaper smoke passes 1/1. The app is behaving correctly: Respect system preference, hide wallpaper when Reduce Transparency is enabled. Neither test source nor production CSS should override the user's actual Accessibility preference.
**Scope:** On disposable GitHub-hosted macOS runner only, explicitly set a normal-transparency baseline in macOS preferences before launching Chrome, verify effective media query via real Chrome, then run isolated wallpaper smoke and all 744 browser cases with 0 retries. Fail closed if the environment cannot be normalized.

## Checklist planner (committed before source edits)
- [x] TRANS-01 / TEAM_LEAD_SERIAL — compare local vs hosted media diagnostics and UI smoke; inspect accessibility APIs and macOS defaults; root cause confirmed hosted reducedTransparency=TRUE.
- [x] TRANS-02 / TEAM_LEAD_SERIAL — scope CI-only preference normalization to disposable hosted macOS runner, not production source or user Mac; verify same browser sees reducedTransparency=FALSE or fail.
- [ ] TRANS-03 / TEAM_LEAD_SERIAL — rerun pinned wallpaper smoke + 4 shards, record actual passed/failed counts.
- [ ] TRANS-04 / TEAM_LEAD_SERIAL — inspect residual functional failures, TypeScript/Node/Release and smoke; lead code review and QA matrix.
- [ ] TRANS-05 / HUMAN_REVIEW_PENDING — independent human review, installed Windows11/Linux/macOS feature E2E and production publisher signing remain separate NO-GO.

| Task | Worker | Unit/Contract | E2E | Smoke | Lead QA |
| --- | --- | --- | --- | --- | --- |
| TRANS-01 | TEAM_LEAD_SERIAL | PASS diagnostics | Hosted wall smoke 0/4 / local 1/1 | PASS evidence | GO diagnostic |
| TRANS-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| TRANS-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| TRANS-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| TRANS-05 | HUMAN_REVIEW_PENDING | PENDING | PENDING native | PENDING | NO-GO |

## Lead source-review checkpoint
- Disposable GH-hosted macOS runner preference keys `com.apple.universalaccess reduceTransparency` and `com.apple.Accessibility ReduceTransparencyEnabled` written FALSE; cfprefsd refreshed before Chrome launch.
- The exact Chrome page diagnostic now **fails closed** if `matchMedia(prefers-reduced-transparency: reduce)` remains true. This distinguishes a CI machine prerequisite failure from product behavior and preserves real user accessibility.
- YAML parse + 4-shard/retries=0 contract PASS, `git diff --check` PASS. No runtime source changes. Full hosted Mac E2E and smoke remain PENDING until same-HEAD result.
