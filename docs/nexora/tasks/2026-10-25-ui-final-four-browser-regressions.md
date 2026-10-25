# NC-TEAM-003 — Last four full Browser UI E2E failures (2026-10-25)
**Lead + executing worker:** TEAM_LEAD_SERIAL; independent human sign-off PENDING.
**Head:** c2886bb PR #44; 4 Mac shards all finished run 40153652731: 740/744 PASS, **4 FAIL** (Shard1 184/186, Shard2 186/186, Shard3 186/186, Shard4 184/186). Existing CI, 33/33, Windows/macOS/Linux Rust and unsigned installer smoke all SUCCESS.
**Trace review:** Board reloaded sample shows task rows (including session-1), but no selected row, and a prompt "This task is complete"; selected task is not a valid "app boot ready" sentinel when mocked archival fixture appends completed tasks. UI status-approval case pauses Playwright fake clock 1s before New task, producing no new-task heading (clock stalled pending creation). Inspector case blindly toggles inspector after reload; screenshot shows active toggle and visible conversation but diff button hidden in a collapsed/hidden panel. Concurrent approval expiry case has final saved content plus a transient empty streaming-content element, making generic multi-match selector fail strict Playwright locator mode; require final persisted answer without ignoring another distinct nonempty reply.
## Checklist Planner — commit BEFORE source/test edits
- [x] FOUR-01 TEAM_LEAD_SERIAL — confirm four exact errors and inspect screenshot/context from GitHub hosted run 40153652731.
- [x] FOUR-02 TEAM_LEAD_SERIAL — replace Board false selected-row sentinel with actual visible known task from fixture and assert no false selection requirement; keep archive assertions.
- [x] FOUR-03 TEAM_LEAD_SERIAL — delay fake clock pause until New task UI is fully created; do not change app functionality.
- [x] FOUR-04 TEAM_LEAD_SERIAL — assert inspector actual hidden/visible state before toggling and use visible actual changed-file button before click, preserving loading isolation checks.
- [x] FOUR-05 TEAM_LEAD_SERIAL — assert persisted final reply in concurrent stream while excluding transient streaming placeholders; check no duplicate saved answers.
- [ ] FOUR-06 TEAM_LEAD_SERIAL — Unit/Contract+33 E2E+full 744 no retry+smoke/Rust 3-OS, independent AI source review and Team Lead gate.
- [ ] FOUR-07 HUMAN_REVIEW_PENDING — real human approval and signed/notarized native installed multi-OS feature E2E before Production.
| Item | Worker | Unit/Contract | Browser E2E | Smoke | Lead |
| --- | --- | --- | --- | --- | --- |
| FOUR-01 | TEAM_LEAD_SERIAL | Audit PASS | Baseline 740/744 | Trace inspected | GO diagnosis |
| FOUR-02..05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| FOUR-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| FOUR-07 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO |

## Source implementation & local Lead QC before CI
- Board reload screenshot clearly showed sidebar tasks with no selected row because completed conversation is current. Wait for actual visible `session-1` row, not a fabricated selected class; preserve all archive/counter assertions.
- Controlled status test's `page.clock.pauseAt` occurred before async New Task bootstrap; pause only after new task heading is visible, and await fixture task row after reload.
- Inspector test clicked a hidden transcript `[data-action=open-diff]` because its unscoped first locator matched before the inspector row. Scope click to `#inspector-content .change-row [data-action=open-diff]` and retain modal/Stage button and project handoff checks.
- Concurrent completion test matched persisted final assistant message **and** transient empty streaming placeholder in strict mode. Target non-streaming saved response; retain exact final response text and subsequent approval ownership checks.
- `npm run build`: PASS after installing cached local dependencies in isolated worktree (TypeScript + Vite); `npx playwright test --list`: **744 tests / 57 files unchanged**; portable screenshot contract 1/1 PASS; `git diff --check`: PASS.
- Mac-focused four-case E2E, full 744 hosted CI and cross OS acceptance PENDING at source edit; do not mark 744 PASS before exact SHA check. Local worktree dependency symlink is excluded from commit.

## FOLLOWUP FOUR-04 root cause before modification
- Real isolated Mac Chrome E2E of the four corrected cases: **3 PASS / 1 FAIL** (Inspector only). The snapshot shows the inspector toggle may be already active after reload, yet the test blindly clicks it; the tested `#inspector-content .change-row` exists in DOM but is not visible because the toggle closed its parent. The test must check `#inspector.hidden` and click toggle **only if hidden**, assert inspector visible and changes tab displayed, then click exact visible file.
- [ ] FOUR-04B TEAM_LEAD_SERIAL: guard inspector toggle against already-open state, check meaningful visible panel; run focused 1-case and 4-case E2E, then hosted 744 regression. Keep task-scoped change loading and stale action disappearance assertions.
