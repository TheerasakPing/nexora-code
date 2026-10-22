# Task Checklist — R4 Typed Workflow DAG Validation

**Created first:** 2026-10-22
**User request:** Continue toward complete Nexora Code R0-R8 roadmap while dependent protected-branch PRs await independent reviewers.
**Phase:** R4 durable Workflow Engine, foundation 1/3 (schema and planning)
**Lead:** acting assistant, TEAM_LEAD_SERIAL, no independent worker agent/reviewer.
**Branch:** feat/r4-workflow-dag-validator on dedicated isolated worktree, based on origin/develop.
**Priority:** P1 and P0 security restrictions (no automatic mutation).
**Scope:** pure Rust typed DAG schema+validator, version gate, strict IDs/edges, approval barrier, deterministic preview waves, bounded inputs, native read-only validation IPC, tests, docs.
**No side effects:** no workflow execution, tool invocation, Git/file writes to user workspace, network, real Agent spawn, live MCP, installers, protected merges or release.
**Acceptance:** reject future schema versions, duplicate IDs, cycles, missing/self refs, illegal fields, excessive sizes. Enforce explicit approval dependencies for mutating nodes. Produce stable waves and blocked errors; no automatic effects.
**Dependencies:** R3 team scheduler is separate; R4 durable journal/executor remains pending after validation.

| ID | Worker | Files | Done when | Status |
| --- | --- | --- | --- | --- |
| R4-01 | TEAM_LEAD_SERIAL | src-tauri/src/workflow_plan.rs | Strict pure typed DAG + tests | [x] |
| R4-02 | TEAM_LEAD_SERIAL | src-tauri/src/lib.rs | Read-only native validation command registered | [x] |
| R4-03 | TEAM_LEAD_SERIAL | docs/nexora/R4-DAG-FOUNDATION.md | Honest safe limitations, dependencies, next gates | [x] |
| R4-QA | TEAM_LEAD_SERIAL | all diffs + tests | Full Rust/Clippy, frontend build/tests, PR | [x] |
| R4-REVIEW | Independent reviewer pending | GitHub approval | protected merge cannot pass without it | [ ] |

- [x] Checklist created before coding and no other worktree modified.
- [x] Threat review: no mutating commands invoked, execution forbidden.
- [x] Cycle/duplicate/schema/input/approval edge tests pass.
- [x] Rust test suite and denied-warning Clippy pass.
- [ ] PR submitted with actual tests and realistic release blocker.
- [ ] 3-OS native CI and independent code review before merging.

**Team Lead decision:** GO for scope-limited source PR; NO-GO for protected merge/release without CI and independent reviewer.

## Source and reviewer evidence (Mac M4, 2026-10-22)

- Actual worker: TEAM_LEAD_SERIAL; no independent delegate or reviewer available.
- Pure R4 tests: 8 PASS (schema, DAG, approval ancestry, cycles, input bounds, serialization).
- Full Rust: 1014 PASS, 0 FAIL, 33 Ignored; CLI integration 1 PASS.
- All-target Clippy -D warnings: PASS.
- Node frontend tests: 110 PASS, 0 FAIL; release script tests 31 PASS.
- TypeScript/Vite Build PASS and R0 identity/security verification PASS.
- Diff review: only workflow_plan module + pure read-only command registration; no user workspace writes, process launches or MCP connection.
- Important: approval node in submitted JSON is not a real operator authorization, and no execution or durable journal exists yet. Real R4 runner is blocked until effect-safe journal/permissions/fault injection acceptance.
- GitHub cross-OS checks and independent Reviewer: PENDING before develop merge.
