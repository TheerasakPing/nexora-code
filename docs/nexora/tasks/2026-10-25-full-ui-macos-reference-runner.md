# NC-TEAM-003 — Run Mac-specific full Browser UI contracts on the correct OS
**Date:** 2026-10-25 Asia/Bangkok. **Lead/real worker:** TEAM_LEAD_SERIAL. **Independent human:** PENDING.

**Evidence before source change:** PR #43 full 744 Linux Chromium shards: 628 pass / 116 fail, including 50+ `ENOENT` screenshot paths. PR #44 fix 64 screenshot destinations led to **680 PASS / 64 FAIL**; Shard1 186/186 PASS. Remaining failures include Linux missing macOS-specific labels `New task ⌘ N`, `Toggle terminal (⌘ J)`, `Toggle inspector (⌘ ⇧ B)`, and macOS-only simulator/VM tabs hidden on Linux, as verified from run 40114518021 logs and screenshot contexts. Browser preview tests were originally authored around macOS UX, not universally Linux platform-neutral behavior.

## Checklist planner (committed before workflow edits)
- [x] MACUI-01 / TEAM_LEAD_SERIAL — triage exact hosted residual errors and correlate with platform-specific UX. No false production bug claims.
- [ ] MACUI-02 / TEAM_LEAD_SERIAL — run full 744-case hosted macOS Chromium across four independent shards, no hidden skips/retries; maintain Linux-specific hosted 33/33 contract and 3-OS native startup checks on existing CI.
- [ ] MACUI-03 / TEAM_LEAD_SERIAL — collect all Mac shard tests and screenshots/traces. Do not claim 744 PASS until all 4 succeed.
- [ ] MACUI-04 / TEAM_LEAD_SERIAL — investigate actual Mac remaining functional failures, implement minimal fixes with regression and run again.
- [ ] MACUI-05 / TEAM_LEAD_SERIAL — Unit/Contract, full Browser E2E, Smoke, Team Lead source review and GitHub exact-head CI before merge.
- [ ] MACUI-06 / HUMAN_REVIEW_PENDING — independent security reviewer and installed Win11/Linux/Mac native provider UX are separate required production gates.

| Task | Worker | Unit/Contract | E2E | Smoke | Lead review |
| --- | --- | --- | --- | --- | --- |
| MACUI-01 | TEAM_LEAD_SERIAL | Diagnostic evidence | 64 residual on Linux | 4 sharded jobs complete | GO diagnosis |
| MACUI-02 | TEAM_LEAD_SERIAL | PENDING | PENDING 744 Mac | PENDING | PENDING |
| MACUI-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| MACUI-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| MACUI-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| MACUI-06 | HUMAN_REVIEW_PENDING | PENDING | PENDING native | PENDING | NO-GO |

**Safety:** Do not mask failing assertions, silently retry, change product runtime to appease browser runner, or claim Mac browser preview equals trusted signed native release.
