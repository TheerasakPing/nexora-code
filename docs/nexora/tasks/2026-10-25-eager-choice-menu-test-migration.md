# NC-TEAM-003 — Choice menu eager-loading regression acceptance
**Date:** 2026-10-25 Asia/Bangkok. **Lead/actual worker:** TEAM_LEAD_SERIAL. Independent human review absent.
**Evidence:** Source commit 93f10d7 moved `choice-menu` from async dynamic import to static boot import, preventing a first-click asynchronous chunk race. Read-only Codex AI review found one source-vs-test P2: `tests/ui.spec.ts` runs three "dismiss pending choice-menu import" cases that attach a network route AFTER boot, await a request that will never occur, then hang. These must now assert the *real synchronous menu* dismisses on Escape, outside pointer and same-trigger click. Keep the model-picker/project-picker delayed-import cases unchanged. Never skip the 3 cases or weaken focus/next-open assertions.
## Mandatory planner (committed before test migration)
- [x] MENU-01 / TEAM_LEAD_SERIAL: Source and independent AI QA review shows three stale post-boot import interception cases.
- [x] MENU-02 / TEAM_LEAD_SERIAL: Replace the 3 irrelevant pending choice-menu download tests with 3 eager menu activation/dismissal/reopen tests, retain 6 other delayed picker tests.
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

## Follow-up BEFORE menu-runtime fix (same-trigger behavior)
- Local Mac real Chrome focused 3-case eager-menu E2E: Escape PASS, outside pointer PASS, same-trigger FAIL. Repro: second click on already-open Filter projects button re-opens a popover instead of closing it.
- Root cause audited in `src/choice-menu.ts`: native focusout listener closes panel immediately when focus returns to the anchor, before the button's click event can execute the app-level toggle. The pointerdown outside guard already excludes the anchor; keyboard Tab and Escape separately handle closure.
- [x] MENU-03A TEAM_LEAD_SERIAL — keep popover open on focusout *to its own anchor* so following click runs the single toggle close path. Other focusout targets still close as before.
- [ ] MENU-03B TEAM_LEAD_SERIAL — rerun all 3 eager-menu E2E, 6 async picker E2E, 33 target and full Mac 744; no skipped/fake results.


## Verified source/UX checkpoint
- Replaced exactly 3 obsolete pending-`choice-menu` download tests by 3 eager menu open/close/reopen checks. Retained the 6 asynchronous model/project picker cancellation tests and all 744 Browser UI test selections.
- Real Mac Chrome first pass: Escape and outside pointer PASS, same-trigger FAIL (menu closed on focusout before click, then clicked button reopened the menu).
- Applied `src/choice-menu.ts` guard: focusout returning to the original anchor defers closure to the following button-click toggle, while outside focus, outside pointer, Tab and Escape retain their old behavior.
- Focused real Chrome `npx playwright test tests/ui.spec.ts --grep 'dismisses an eagerly loaded choice menu' --workers=1 --retries=0`: **3/3 PASS, 0 retries/skips** after source fix.
- Full Mac 744 regression is **not yet accepted**. Previous head `93f10d7` hosted [run 40150763398](https://github.com/TheerasakPing/nexora-code/actions/runs/40150763398) finished 735 PASS / 9 FAIL (including these 3 known obsolete tests). Recheck exact new source head via CI before any protected merge.
