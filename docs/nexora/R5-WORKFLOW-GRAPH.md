# R5 Workflow Studio — Dependency Wave Diagram (Read-only)

**Date:** 2026-10-22.
**Delivery:** Source-only UI enhancement stacked on Workflow Studio PR #20.
**Product:** Nexora Code, fork of PandaCode: https://github.com/aicodeking/pandacode

## What this adds

After choosing Workflow Studio → Validate plan, the existing Rust
workflow_validate_plan command provides the validated dependency waves.
The UI displays a horizontally scrollable graph-like arrangement of wave
columns. Each wave contains cards with node title, kind, ID, predecessors,
and explicit labels for Approval nodes and potentially effectful nodes.

The graph uses semantic ordered/unordered lists. No layout engine, external
graph service, network resource or new native permission is introduced.

## Safety and failure rules

- The trusted native Rust validator remains responsible for DAG ordering,
  schema rules, effect types and approval ancestry.
- The graph mapper verifies every declared node appears exactly once in
  the native preview. A mismatched, stale or incomplete result fails closed.
- Rendered graph node names, dependencies and notes use DOM textContent
  nodes only. User strings are never treated as HTML.
- Existing Workflow Studio async generation guards ignore old native replies
  after editing, resetting or closing.
- No approval grant, Git mutation, Agent/MCP call or workflow executor exists
  in this feature. A declared Approval node is NOT genuine user consent.
- The graph gives information only, never permission to start an operation.

## Source-level acceptance on local Mac

- 5 focused graph tests and 8 existing Workflow Studio tests PASS.
- 156 total Node tests PASS / 0 FAIL.
- 31 release script tests PASS / 0 FAIL.
- TypeScript/Vite production build PASS (existing chunk-size advisory).
- R0 identity and updater-safety checks PASS; Git diff checks PASS.

Still pending: native macOS/Windows/Linux GUI E2E, screen-reader testing in
installed applications, exact-head GitHub CI, genuine independent reviewer,
signing, deployment and Production Release acceptance.
