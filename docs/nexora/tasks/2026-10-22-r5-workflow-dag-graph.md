# R5 Task Checklist — Workflow DAG Graph Preview

**Date:** 2026-10-22; created before source edits.
**Request:** Continue Nexora Code Roadmap by extending Workflow Studio into an understandable visual dependency preview.
**Team Lead:** acting assistant, TEAM_LEAD_SERIAL. No independently delegated agents or human reviewer available.
**Parent branch:** feat/r5-workflow-studio-validation, PR #20.
**Branch and worktree:** feat/r5-workflow-dag-graph / nexora-code-r5-dag-graph.
**Goal:** accessible read-only wave diagram showing job descriptions, dependency links, approval gates, effectful nodes and native validation limits.
**Safety:** no executor, run/approve button, Git mutation, tool or MCP call; never equate declared JSON approval with real operator consent; no main/develop protected merge or GUI deployment.
**Acceptance:** pure mapper tests for schema, wave ordering, missing nodes, unsafe strings, graph size limits, accessible DOM generated from text nodes, no loss of existing validation results. Full Node/build/release tests and CI before review.

| Task | Real worker | Scope | Acceptance | Status |
| --- | --- | --- | --- | --- |
| R5-GR-01 | TEAM_LEAD_SERIAL | src/workflow-graph.ts | Typed mapper and safe DOM graph view | [x] |
| R5-GR-02 | TEAM_LEAD_SERIAL | src/workflow-studio.ts and CSS | Mount graph after native validation; maintain keyboard accessibility | [x] |
| R5-GR-03 | TEAM_LEAD_SERIAL | tests/workflow-graph.test.mjs | Wave ordering, missing node, approval, hostile source validation | [x] |
| R5-GR-04 | TEAM_LEAD_SERIAL | docs/nexora/R5-WORKFLOW-GRAPH.md | Limitations, source evidence | [x] |
| R5-GR-QA | TEAM_LEAD_SERIAL | all diffs/tests | Build, release, R0, Team Lead GO/NO-GO | [x] |
| R5-GR-REVIEW | Independent reviewer PENDING | GitHub review | Required for protected merge | [ ] |

- [x] Planner prepared before source edit.
- [x] Read-only graph, malicious strings rendered as textContent only.
- [x] Focused mapper tests, existing Studio tests, full Node/Build/Release/R0 pass.
- [x] Team Lead checks source diff and PR handoff.
- [ ] Independent reviewer and six CI checks required before merge; no Production Release claim.

**Team Lead verdict:** GO for source-only stacked PR submission; NO-GO for protected merge/release before exact-head CI and independent human review.

## Team Lead source acceptance — 2026-10-22

- Worker mode: TEAM_LEAD_SERIAL. No real independently delegated Agent Worker or reviewer was available.
- Reviewed src/workflow-graph.ts, src/workflow-studio.ts, styling and mapper tests. Rust native IPC remains the read-only workflow_validate_plan validator. No mutating command was introduced.
- Missing/duplicate native DAG nodes are rejected. All graph data is rendered via textContent, never untrusted HTML.
- Focused graph tests 5 PASS; parent Workflow Studio tests 8 PASS.
- Full Node tests 156 PASS / 0 FAIL. Release tests 31 PASS / 0 FAIL. Production Vite/TypeScript build and R0 safety checks PASS.
- Existing Vite chunk-size advisory is not a test failure. This patch does not alter any Rust files.
- Native 3-OS GUI E2E, independent reviewer, GitHub CI and protected branch merge are PENDING.
