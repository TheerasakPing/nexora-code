# Nexora Code R6 — Full Browser UI E2E Mac shard 4 remediation
Date: 2026-10-25 Asia/Bangkok. Base 669e9bc, PR #44 fix/full-ui-artifacts-cross-platform-20261024. Product **Nexora Code**, fork PandaCode, never Nexora Studio.
## Why / bounded scope
- [x] PR #43 tests 744 UI cases; PR #44 fixes macOS screenshots. GitHub run 40161607545: macOS UI E2E shard 1/4, 2/4, 3/4 PASS, shard 4/4 FAIL. Other native Rust/installer checks green. Read actual failing log before edits.
- [ ] W1 trace and reproduce exact failing tests on Mac. Own narrowly scoped failing tests or actual code defect. Preserve all original PandaCode functions, no hiding failures by skip/only/force.
- [ ] W2 independent QA audit of shard CI setup, Mac screenshot portability, existing E2E assertions and fixture security. Separate worktree; may own separate contract tests after concrete issue.
- [ ] Independent read-only source review of W1/W2 and Lead integration QA: actual focused browser E2E, npm run build, npm test, release contract.
- [ ] Push reviewed dev PR branch only; CI rerun; no merging protected main/develop without green gates/human approval.
## Agent assignments
- W1: assigned failing test/features only with smoke/E2E evidence and log.
- W2: independent CI/tests audit and separate bounded test if real issue.
- R: independent final reviewer; Lead responsible integration/approval.
## Evidence ledger
| Lane | Commit | QA/E2E | Review |
|---|---|---|---|
| W1 | pending | pending | pending |
| W2 | pending | pending | pending |
| Lead | pending | pending | pending |
## Production gate
744 browser tests cannot substitute real native Mac/Windows/Linux GUI, signing, updater and human signoff.

## Implemented evidence — 2026-10-25
- **Actual CI incident:** GitHub run `40161607545`, macOS shard 4/4 job `121074320319`: **185/186 pass**, `tests/ui.spec.ts:326` Board History fetch expected 151 messages after manual top scroll, remained 100 for 5 s. Shards 1/2/3 passed. The rendered Board panel is selected asynchronously, so a scroll before the refreshed message page has resolved can be dropped by `loadOlderMessages` (requires `hasOlder` and `messagesLoading=false`).
- **W1:** commit `8619f92` modifies only `tests/ui.spec.ts`; before scrolling, asserts exact selected Board card, 100 initial messages, and NEW get_messages initial request since before click. After scroll, asserts exactly one older-history IPC with `beforeId=653`, 151 messages and original streaming/anchor preservation. No skip, sleeps, retries or disabled production code. W1 sandbox browser server bind failed `EPERM` and sandbox node tests failed `listen EPERM`; transparently BLOCKED, not claimed pass.
- **W2 independent QA auditor:** read-only traced the route, async selection, workflow `--shard=4/4 --workers=1 --retries=0`; suggested hydration readiness and exact historical IPC assertions. Discovered two collateral portable-screenshot paths in `tests/message-actions.spec.ts` — log for separate future scope; do not widen this fix.
- **Lead macOS Host (not Agent sandbox):** original focused test 6/6 PASS before patch (flaky race); W1 patch focused 4/4 PASS after build `/tmp/nxcode-shard4-lead-browser-idle.log`; strengthened Lead diff 4/4 PASS `/tmp/nxcode-shard4-integration-host-e2e.log`; `npm test` **210/210 PASS** after build `/tmp/nxcode-shard4-lead-unit-after-build.log`; `npm run build` PASS `/tmp/nxcode-shard4-lead-build.log`. An overloaded concurrent 10-test run was **8/10** with two conversation test 20-s timeouts while building; explicitly retained as environmental stress signal, NOT a green 744-case suite. `npm run test:release` reported **35 passed, 1 failed** startup observation under local Mac host; independent Release-gate blocker and not waived by this PR (`/tmp/nxcode-shard4-lead-release-contracts.log`).
- [x] Planned source fix implemented with real Mac focused browser and Node smoke evidence. [ ] Full 744-case matrix CI rerun GREEN on GitHub; [ ] independent final source review verdict; [ ] native macOS/Windows/Linux operator GUI and Production approvals. No `develop`/`main` merge and no signed release.

## Reviewer acceptance and final Mac Host outcome
- Independent read-only final reviewer `/tmp/nxcode-shard4-final-review.log` gives **ACCEPT DEVELOPMENT** for exact `tests/ui.spec.ts` delta: waits selected task, 100 first messages, a genuinely new initial get_messages invocation, then requires exactly one history-page IPC `beforeId=653`, 151 messages and preserved streaming/anchor position. This is **not** native GUI signoff.
- Lead final isolated Mac Host browser command `npx playwright test tests/ui.spec.ts -g 'keeps (conversation|board) streaming and the reading position' --repeat-each=2 --workers=1` returns **4/4 PASS** without retries (`/tmp/nxcode-shard4-integration-host-e2e.log`). Node 210/210 PASS after build; TypeScript/Vite release build PASS. `npm run test:release` still one unrelated startup fixture failure: not waived. Earlier stress E2E 8/10 remains recorded, full 744 on GitHub PENDING.
- [x] Independent source review **ACCEPT DEVELOPMENT**; [x] Lead focused Mac browser E2E PASS without retries; [ ] PR-head/full UI 744-case GitHub CI GREEN and human review; [ ] protected develop/main merge and Production Release.
