# NC-TEAM-003 — Last Jev task-switch Browser lifecycle E2E
**Date:** 2026-10-25 Asia/Bangkok. **Actual executing worker/Team Lead:** TEAM_LEAD_SERIAL. **Human independent approval:** pending.

**Exact failing hosted evidence:** PR #44 SHA b3ee1ca, [Mac 744 suite 40156688673](https://github.com/TheerasakPing/nexora-code/actions/runs/40156688673), **743 PASS / 1 FAIL**, zero retries. All three other 186-case shards PASS, shard 2 185/186. Last failing test `tests/jev-live.spec.ts:127` task switch: `openBrowser(page,false)` expects a visible Browser URL textbox after clicking Open browser; snapshot shows that the Browser panel and *correct task's* Jev live viewer are actually visible and active, so browser URL is hidden by design during live playback. Correct acceptance must assert run ownership, old lease release, new viewer visible, and that Return to browser restores URL controls. Do not weaken lease isolation or disable any test.
**Other exact-head gates:** CI, 33/33 UI, Native Rust Windows/macOS/Linux, unsigned installers all PASS. Trusted production remains blocked independently by signing and human review.

## Checklist planner — commit BEFORE modifying test/runtime
- [x] JEV-LAST-01 / TEAM_LEAD_SERIAL — download hosted failure screenshot/context, compare source BrowserPanel and JevLiveView state machine.
- [x] JEV-LAST-02 / TEAM_LEAD_SERIAL — update only Jev switch test: after Open browser accept the correct *live Jev view*, verify lease and report are for the new task, use Return to browser to prove URL view is available and lease closes, maintain old-task isolation assertions.
- [ ] JEV-LAST-03 / TEAM_LEAD_SERIAL — run focused Jev and whole 6+ Jev Browser tests Mac Chrome, zero retries; Node/Release/Build/Smoke as feasible.
- [ ] JEV-LAST-04 / TEAM_LEAD_SERIAL — push PR #44 and confirm 4 hosted 186-case shards 744/744, all same-head gate checks; inspect Team Lead QA before merge.
- [ ] JEV-LAST-05 / HUMAN_REVIEW_PENDING — do not assert independent human approval, signed/notarized production or installed Win11/Ubuntu/Wayland native provider E2E without real proof.
| Item | Worker | Unit/Contract | E2E | Smoke | Lead |
| --- | --- | --- | --- | --- | --- |
| JEV-LAST-01 | TEAM_LEAD_SERIAL | PASS source/trace | 743/744 hosted baseline | Screenshot verified | GO diagnosis |
| JEV-LAST-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| JEV-LAST-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| JEV-LAST-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| JEV-LAST-05 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO |

## Task-scoped Jev UI implementation and verification
- When opening task-4 Browser, `mountJevLive` legitimately auto-selects its existing active report and hides the ordinary URL input. Trace showed visible Jev Live Region with "Other task only", not missing Browser panel. Updated one test to assert task-owned Jev viewer, old-task unwatch, new-task watch and report ownership, then click Return to browser and assert restored URL control/new-task unwatch; added guard against rewatching old task.
- On Mac M4 Chrome focused original failure 1/1 PASS (7.7s, 0 retries). Under parallel local heavy JS build + Node/Release, full file Jev E2E 8/10, with two **different cases failing before Jev-specific assertions** because initial .task-item.selected missing. Do not claim 10/10 PASS. Hosted exact SHA run is required to establish full 744 regression.
- Same worktree `npm run build` PASS, `npm test` 210/210 PASS, `npm run test:release` 36/37 PASS (1 existing platform SKIP), full unchanged 744 tests / 57 files.
- No changes to production Browser/Jev runtime, security controls, IPC, test selection or retry settings. Human reviewer and production signing still PENDING.

## QA reviewer P2 follow-up — before patch
- Read-only AI reviewer found two P2 source assertions in our last Jev task-switch test:
  1. Expecting exactly one historical session-1 `watching=true` conflicts with native 5-second lease renewals when UI/load is slow; instead checkpoint the count **after receiving old-task unwatch** and require no *subsequent* old-task renewal.
  2. Seeing one `other-run watching=false` when Return to browser is clicked is not sufficient if a leaked timer sends `watching=true` again. After Return, observe beyond the 5-second renewal interval and require no new watch for either run.
- [ ] JEV-LAST-02B / TEAM_LEAD_SERIAL — test old-session leases and new-session renewal quiescence with bounded fixture observation, preserve all task-ownership assertions, re-run one-case Mac E2E and exact-head hosted 744. No production runtime changes.
- AI reviewer is not an independent human reviewer. No signing or native provider production acceptance established.
