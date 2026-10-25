# Nexora Code R6 — PR45 macOS Shard 4 new-task bootstrap
Date 2026-10-25 Asia/Bangkok; product **Nexora Code**, NOT Studio. Branch fix/r6-full-ui-shard4-20261025; baseline d123eaf.
## New CI failure and why
- [x] Actual GitHub run 40202351377, macOS shard 4/4: **185 passed / 1 failed**; previous Board-history test tests/ui.spec.ts:326 now PASSED. New failure tests/ui.spec.ts:1798 'caps long live tool activity without dropping the output being read' at line1812: cannot find New task heading after clicking `.new-task-button`.
- [x] Retrieved real CI artifact via GitHub Actions, error-context snapshot shows existing conversation and status toast `Open a project to start a task.`. Source `src/main.ts:createTask()` refuses if `state.projects` empty. The test at line1809 `await page.reload()` and immediately pauses mocked clock/clicks; unlike the adjacent same-kind test around line1743, it does not wait for selected task/project initialization. This can race preview app bootstrap.
## Checklist Planner / agent responsibility
- [ ] W1 QA engineer: ONLY tests/ui.spec.ts around line1798. Align with already-proven neighbor by asserting selected preview session/project after reload, BEFORE pausing Playwright clock/clicking. Preserve all 40 tool events, focus, output, cap, text and late error assertions. No sleep/retries/skips/prod change.
- [ ] W2 independent diagnosis reviewer: inspect downloaded GH error-context, snapshot trace, createTask early-return, compare adjacent tests. Verify failure isn't a true runtime regression or missing project; report ACCEPT / REQUEST CHANGES with evidence; read only.
- [ ] Lead: run macOS Host Playwright focused test with --retries=0 plus repeat-each, Node tests, TS/Vite/release contracts; do not concurrently run costly build and browser. Independent final read-only reviewer checks actual patch. Commit/push only reviewed PR branch; GitHub full 744-case matrix must run green before merging.
- [ ] Confirm Native Rust windows-x64 check and all hosted jobs GREEN. Keep Draft PR if any red.
## Evidence
| Owner | Commit | Unit/Browser | Reviewer |
|---|---|---|---|
| W1 | pending | pending | pending |
| W2 | read-only | pending | pending |
| Lead | pending | pending | pending |
## Release / Merge gates
Browser E2E is not native Windows or macOS WKWebView acceptance, signing or human release approval. Do not merge protected develop/main until required status + independent review + user approval. Preserve previous PR45 CI results and screenshots.
