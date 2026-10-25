# NC-TEAM-003 — Triage residual 16/744 hosted macOS UI E2E failures
**Date:** 2026-10-25 Asia/Bangkok. **Team Lead & executing worker:** TEAM_LEAD_SERIAL. **Human independent approval:** PENDING.
**Base:** fix/full-ui-artifacts-cross-platform-20261024 commit bb274b7, PR #44 stacked on PR #43. Original suite count remains 744 tests in 57 files.
**Evidence:** [GitHub run 40116912265](https://github.com/TheerasakPing/nexora-code/actions/runs/40116912265) on hosted macOS, normal transparency verified via real Chrome, 4 wallpaper smokes PASS. 4 zero-retry shards: PASS 186/186, 183/186, 182/186, 177/186 = **728 PASS, 16 FAIL**. Honest residual grouping:
- UI spec: 3 rename menu focus expectations, 2 new-task heading not found, 1 project filter item, 1 provider logo, 1 permission menu focus (total 7? actually 9 UI spec errors); plus 1 VM screen and 1 wallpaper CSS.
- macOS Simulator: 2 visible-tab failures.
- Support: 2 stale upstream GitHub URLs, while product now correctly links nexora-code.
- Jev assist/live: 2 asynchronous switch/review failures.
- Layout: 1 narrow width 3px overrun.
**Rule:** Do not skip assertions, lengthen timeouts indiscriminately, hide retries or disable functionality. Fix source UX defects vs stale test expectations only with evidence, isolate each in validated subtask.

## Checklist planner (committed before changes)
- [x] LAST16-01 / TEAM_LEAD_SERIAL — collect all hosted errors, true 728/744 baseline and classify target areas.
- [ ] LAST16-02 / TEAM_LEAD_SERIAL — update stale Support GitHub link test expectations to released Nexora destination; preserve privacy/new-window assertions. Test isolated.
- [ ] LAST16-03 / TEAM_LEAD_SERIAL — reproduce and correctly resolve menu focus and new-task selected/blank state UI cases.
- [ ] LAST16-04 / TEAM_LEAD_SERIAL — reproduce Simulator/Jev/VM state failures and fix only proven cause.
- [ ] LAST16-05 / TEAM_LEAD_SERIAL — inspect 3px layout clipping and wallpaper theme assertion, preserve actual UX accessibility.
- [ ] LAST16-06 / TEAM_LEAD_SERIAL — rerun 33 functional E2E, full 744/744 4 shards, Node 210+, Release 36+, TS/Vite, cross-OS native CI, smoke; zero retries/skips beyond existing contractual skip.
- [ ] LAST16-07 / TEAM_LEAD_SERIAL — source diff review and PR review, guarded merge only once all mandatory checks PASS; otherwise report unresolved failures accurately.
- [ ] LAST16-08 / HUMAN_REVIEW_PENDING — production release independent reviewer and signed installed macOS/Win11/Linux remain blocked.

| Subtask | Real worker | Unit/Contract | E2E | Smoke | Team Lead QA | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| LAST16-01 | TEAM_LEAD_SERIAL | PASS audit | 728/744 hosted baseline | 4/4 wallpaper smokes PASS | GO diagnostic | 40116912265 |
| LAST16-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | test support |
| LAST16-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | menu/new task |
| LAST16-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | simulator/Jev/VM |
| LAST16-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | clipping/theme |
| LAST16-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | 744-run |
| LAST16-07 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | CI/PR |
| LAST16-08 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO | human+signing |

## Executed stage 1 evidence
- Two Support E2E cases had stale `aicodeking/pandacode/issues` and `releases` expectations; production About/Support intentionally links `TheerasakPing/nexora-code` plus upstream attribution link. Updated assertions to check correct new repo, license, releases, issues AND preserve upstream attribution, without disabling browser privacy assertions. Local `npx playwright test tests/support.spec.ts --workers=2 --retries=0`: **7/7 PASS**.
- Task Actions popover source `src/main.ts` now lists `Pin task` as first keyboard item and `Rename task` second. Updated three E2E cases to assert Pin receives initial focus, ArrowDown focuses Rename, Enter activates Rename; preserve keyboard access assertions. Local rerun 1/3 PASS; two other test setups failed with missing seeded selected task (boot flake) before reaching menu, so **not yet accepted**; hosted full-suite rerun required.
- Two custom simulator probe specs navigate without waiting for initial selected task unlike the shared `open()` fixture, causing startup races; now wait for visible selected task before tab interaction. Change to helper also checks semantic inspector.hidden, not stale CSS visibility. Local run still in progress at documentation cut; **do not declare PASS**.
- Remaining new-task state, Jev, VM, browser wallpaper CSS and narrow layout still pending; do not skip. No production source mutations in this stage.

## Stage 2 — follow-up evidence
- Hosted commit `03b3098` on [run 40145856789](https://github.com/TheerasakPing/nexora-code/actions/runs/40145856789): **732 PASS / 12 FAIL**. Shards [186/186,181/186,185/186,180/186]. The remaining failures vary with runner scheduling.
- Isolated `tests/wallpaper-settings.spec.ts` Mac Chrome: pinned full→solid transition PASS 1/1 after updating hardcoded legacy palette assertion to compare live computed `--bg` color; retains opaque fallback and persistence checks. Source `src/styles.css` compact header gap reduced 1px→0 without shrinking 24px controls for 760px header 3px overflow; full layout test local retried but failed at **earlier** inspector geometry/readiness, so exact later-stage layout acceptance pending hosted run.
- Hosted simulator fold test failed asserting optimistic Closed pose within 200ms while injected native error rejected immediately (before browser could observe). Test now parks the native failure promise to assert visible optimistic Closed and aria-busy, releases it, then asserts rollback Open and error toast. Local follow-up 1-case failed at an earlier asynchronous frame-release assertion, so this change **is not yet accepted**; requires hosted isolated runner verification.
- A read-only Codex AI QA reviewer reported no confirmed P0/P1/P2 on 5 changed source/test files; no human approval, no native 3-OS feature E2E.
- Same worktree Node 210/210 PASS, Release 36/37 PASS (1 pre-existing SKIP), `npm audit` 0, TS/Vite production build PASS after compact header and computed solid-theme test. Existing 33/33 hosted Chromium and native baseline separate.
