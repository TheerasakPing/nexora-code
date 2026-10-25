# NC-TEAM-003 — Mac Browser UI cursor selection + New Task readiness
**Date:** 2026-10-25 Asia/Bangkok. **Lead/actual worker:** TEAM_LEAD_SERIAL. **Independent human reviewer:** PENDING.
**Source baseline:** PR #44 head bc6dab3 stacked on PR #43. Previous head 64a1b35 hosted 744 Browser E2E run 40160629470 completed 742 PASS / 2 FAIL, no retries: shard1/2/3 each 186/186 PASS, shard4 184/186. Failures: tests/ui.spec.ts:795 composer focus & selection after async reasoning save; tests/ui.spec.ts:2302 rejected first prompt New Task after reload (heading missing). Current bc6dab3 CI running independently; results must not be conflated.
**Scope:** Preserve actual user selection and drafts; no weakened selection asserts or disabled tests. Reproduce, inspect and correct evidence-based root cause.

## Checklist Planner — committed BEFORE edits
- [x] CURSOR-01 / TEAM_LEAD_SERIAL — audit hosted selection diff (expected [8,17], actual [41,41]), New Task heading trace and composer rerender source.
- [ ] CURSOR-02 / TEAM_LEAD_SERIAL — reproduce reasoning save cursor test, address genuine focus/selection reset without relaxing assertion.
- [ ] TASK-03 / TEAM_LEAD_SERIAL — wait for preview task fixture after reload before creating new draft; retain rejection rollback assertions.
- [ ] CHECK-04 / TEAM_LEAD_SERIAL — focused E2E, relevant spec, Node/Release/Build, 33 UI, full 744 zero-retry host, native 3OS and installers.
- [ ] LEAD-05 / TEAM_LEAD_SERIAL — source review, protected merge only after same-head CI green.
- [ ] HUMAN-06 / HUMAN_REVIEW_PENDING — independent human review and signed/notarized installed 3OS production NO-GO.
| Task | Worker | Unit/Contract | Browser E2E | Smoke | Lead |
| --- | --- | --- | --- | --- | --- |
| CURSOR-01 | TEAM_LEAD_SERIAL | PASS audit | old HEAD 742/744 | Trace seen | GO |
| CURSOR-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| TASK-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| CHECK-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| LEAD-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| HUMAN-06 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO |
