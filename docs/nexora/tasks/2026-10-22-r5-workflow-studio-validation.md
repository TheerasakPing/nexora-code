# Task Checklist — R5 Workflow Studio: Read-only DAG Validation UI

**Created before source edits:** 2026-10-22.
**User request:** Continue the road map and consolidate Nexora Code toward a usable product, without bypassing CI/reviewer protections.
**Team Lead:** acting assistant, TEAM_LEAD_SERIAL. No real independently delegated agents or reviewer.
**Branch/worktree:** feat/r5-workflow-studio-validation / nexora-code-r5-workflow-studio.
**Base/dependency:** PR #18 integration/r0-r4-consolidated provides R4 workflow_validate_plan native IPC; this is a stacked PR.
**Scope:** accessible opt-in Workflow Studio dialog, JSON edit/validate only, typed validation result waves, approvals and risk note, input limit, no unsafe HTML, async stale-response guards, tests, docs.
**Excluded:** persistent workflow edits, automatic task execution, native operator approval/consent, MCP, Git mutations, Worker dispatch, live GUI acceptance, installers, main/develop merges.
**Failure behavior:** browser preview or native backend unavailable -> explicit error, no fake success. Reject malformed JSON/oversized input before invoking native; never claim a JSON Approval node is real approval.
**DoD:** UI wired to a discoverable main toolbar action, keyboard/escape support, no backend writes, local frontend tests/build/release/R0, exact SHA CI, human reviewer gate.

| Task | Actual owner | Files | Done when | State |
| --- | --- | --- | --- | --- |
| R5-ST-01 | TEAM_LEAD_SERIAL | src/workflow-studio.ts + tests | Safe editor, validation response, states and stale isolation | [x] |
| R5-ST-02 | TEAM_LEAD_SERIAL | src/workflow-studio.css + src/main.ts | Main UI entrypoint with accessibility | [x] |
| R5-ST-03 | TEAM_LEAD_SERIAL | docs/nexora/R5-WORKFLOW-STUDIO.md | End-user behavior and limitations | [x] |
| R5-ST-QA | TEAM_LEAD_SERIAL | all changed diffs | Build, tests, review, PR | [x] |
| R5-ST-REVIEW | independent reviewer PENDING | GitHub approval | Required for protected merge | [ ] |

- [x] Planner written before source edits, isolated worktree; concurrent R3 Claim Audit untouched.
- [x] Preview only; no Agent, MCP, approval grant or Git side effects.
- [x] Malformed input, size limit, XSS strings, stale asynchronous response tests.
- [x] Frontend tests, build, release and R0 checks verified.
- [x] Actual Team Lead source diff inspected; protected merge requires independent reviewer and exact-head CI.

**Team Lead verdict:** GO for tested source PR submission; NO-GO for protected merge, live workflow execution and release.

## Mac local acceptance — 2026-10-22

- Real worker: TEAM_LEAD_SERIAL. Independent coding Agent/Reviewer not available.
- Files: src/workflow-studio.ts, src/workflow-studio.css, src/main.ts and tests/workflow-studio.test.mjs. R3 Claim Audit worktree unchanged.
- Safety: read-only workflow_validate_plan IPC, no Executor/Agent/MCP/Git/consent calls, 128 KiB parse bound and trusted native schema checks, DOM text rendering, async stale-generation fencing.
- Focused Node 8 PASS / 0 FAIL; entire Node suite 151 PASS / 0 FAIL; release scripts 31 PASS / 0 FAIL; production build PASS; R0 identity/updater safety PASS.
- Existing Vite large-chunk advisory not interpreted as build failure.
- Exact-head GitHub cross-OS CI, actual installed GUI testing, independent reviewer approval and live Workflow execution gates remain PENDING.
