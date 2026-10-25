# Planner — iPhone Duo pose frame sequence, 2026-10-26

- [x] Team lead: reproduce failed macOS E2E on PR #43 head, 3/3 failures
- [x] QA: identify expected pending-frame behavior from CI shard 3 log
- [x] Runtime owner: identify stale frame cursor after simulator pose changes
- [ ] Runtime owner: reset frame cursor on successful pose before restarting pump
- [ ] QA: repeat targeted E2E, run Node tests and production build
- [ ] Team lead: review diff and publish stacked PR; do not merge with red full suite

Cause: native simulator pose changes frame generation; existing `seq` cursor can match the first post-pose frame sequence, so `simulator_frame(after: seq)` returns no image while spinner remains busy. Failed-pose recovery already resets `seq`; successful pose must do the same. Preserve fail-closed behavior and no arbitrary timers/skips/retries.
