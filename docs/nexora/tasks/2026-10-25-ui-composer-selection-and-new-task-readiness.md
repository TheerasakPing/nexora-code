# NC-TEAM-003 — Mac Browser UI cursor selection + New Task readiness
**Date:** 2026-10-25 Asia/Bangkok. **Lead/actual worker:** TEAM_LEAD_SERIAL. **Independent human reviewer:** PENDING.
**Source baseline:** PR #44 head bc6dab3 stacked on PR #43. Previous head 64a1b35 hosted 744 Browser E2E run 40160629470 completed 742 PASS / 2 FAIL, no retries: shard1/2/3 each 186/186 PASS, shard4 184/186. Failures: tests/ui.spec.ts:795 composer focus & selection after async reasoning save; tests/ui.spec.ts:2302 rejected first prompt New Task after reload (heading missing). Current bc6dab3 CI running independently; results must not be conflated.
**Scope:** Preserve actual user selection and drafts; no weakened selection asserts or disabled tests. Reproduce, inspect and correct evidence-based root cause.

## Checklist Planner — committed BEFORE edits
- [x] CURSOR-01 / TEAM_LEAD_SERIAL — audit hosted selection diff (expected [8,17], actual [41,41]), New Task heading trace and composer rerender source.
- [x] CURSOR-02 / TEAM_LEAD_SERIAL — reproduce reasoning save cursor test, address genuine focus/selection reset without relaxing assertion.
- [x] TASK-03 / TEAM_LEAD_SERIAL — wait for preview task fixture after reload before creating new draft; retain rejection rollback assertions.
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

## Execution and focused Mac E2E evidence
- The controlled reasoning test was establishing [8,17] cursor range immediately after switching back to session-1, while its old history request could still replace the composer independently of the still-held reasoning save. Test now checks actual selected session-1, completed history text and enabled composer **before** typing, then verifies both the immediate precondition [8,17] and after released-save range [8,17], exact text and focus.
- Rejected first prompt after reload now waits for known session-1 seeded sidebar task before clicking New task. Existing rejected send, task must vanish, draft must restore assertions remain.
- Local Mac Chrome prior isolated repeat 2/2 PASS without fix (flaky host-only); after repair **2/2 PASS**, workers=1, retries=0. These are fixture readiness improvements, not proven 744-hosted acceptance. Tests still 744 and no assertion removal.
- Next: exact-head hosted full macOS 4-shard 744, 33 UI, Node/Release/Build and Rust/installer CI. Independent human signoff/signing remain NO-GO.
