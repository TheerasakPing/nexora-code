# NC-TEAM-003 — Choice menu eager-loading regression acceptance
**Date:** 2026-10-25 Asia/Bangkok. **Lead/actual worker:** TEAM_LEAD_SERIAL. Independent human review absent.
**Evidence:** Source commit 93f10d7 moved `choice-menu` from async dynamic import to static boot import, preventing a first-click asynchronous chunk race. Read-only Codex AI review found one source-vs-test P2: `tests/ui.spec.ts` runs three "dismiss pending choice-menu import" cases that attach a network route AFTER boot, await a request that will never occur, then hang. These must now assert the *real synchronous menu* dismisses on Escape, outside pointer and same-trigger click. Keep the model-picker/project-picker delayed-import cases unchanged. Never skip the 3 cases or weaken focus/next-open assertions.
## Mandatory planner (committed before test migration)
- [x] MENU-01 / TEAM_LEAD_SERIAL: Source and independent AI QA review shows three stale post-boot import interception cases.
- [ ] MENU-02 / TEAM_LEAD_SERIAL: Replace the 3 irrelevant pending choice-menu download tests with 3 eager menu activation/dismissal/reopen tests, retain 6 other delayed picker tests.
- [ ] MENU-03 / TEAM_LEAD_SERIAL: Run focused 3+9 cases, 33/33 expanded, full 744/744 zero-retry Mac E2E, TypeScript/Node/Release and hosted 3-OS Rust/installer.
- [ ] MENU-04 / TEAM_LEAD_SERIAL: Lead source acceptance and protected merge only when full exact-head CI green.
- [ ] MENU-05 / HUMAN_REVIEW_PENDING: signed production, independent security and native installed provider E2E separate NO-GO.
| Item | Worker | Unit/Contract | E2E | Smoke | Lead |
| --- | --- | --- | --- | --- | --- |
| MENU-01 | TEAM_LEAD_SERIAL | P2 root cause from AI code review | old 3 cases obsolete | Source audit | GO |
| MENU-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| MENU-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| MENU-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| MENU-05 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO |
