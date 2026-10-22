# Checklist Planner — R4 Approval on Every Dependency Path

**Created before edits:** 2026-10-22.
**User goal:** Continue R0–R8 roadmap without regressions while R5/R3 PRs proceed separately.
**Team Lead:** acting assistant, TEAM_LEAD_SERIAL (no independent Agent Worker/reviewer available).
**Priority:** P0 policy enforcement; R4 typed DAG validation.
**Base/branch:** origin/integration/r0-r4-consolidated → fix/r4-approval-all-paths.
**Ownership:** src-tauri/src/workflow_plan.rs and docs/nexora/R4-DAG-FOUNDATION.md, task planner only.
**Problem:** The existing DAG verifier uses ANY incoming approval-tainted edge. A join of one approved and one unapproved chain could validate an effectful node despite an unapproved dependency path.
**Design:** Require all root-to-node dependency paths to cross an Approval node before any effectful node. A declared approval node is NOT an actual user permission grant. Nodes without predecessors are unapproved unless they themselves declare Approval.
**DoD:** Fail-open counterexample reproduced first, then gate fixed; reject mixed-approval joins, including transitive joins and all effectful kinds; allow truly approved graphs and preserve stable waves. Full Rust tests/Clippy, frontend/release tests, diff and CI. No agent/tool execution, approval granting, GUI or protected merge.
**Dependencies:** PR #18 includes read-only workflow_validate_plan. Concurrent R5 Workflow Studio PR #20 and R3 orphan audit PR #19 must remain untouched.

| Task | Actual assignee | Scope | Status |
| --- | --- | --- | --- |
| R4-APP-01 | TEAM_LEAD_SERIAL | Add exploit-shaped regression tests in workflow_plan.rs | [x] |
| R4-APP-02 | TEAM_LEAD_SERIAL | Enforce all incoming approved paths before effectful nodes | [x] |
| R4-APP-03 | TEAM_LEAD_SERIAL | Update security contract and limitations in documentation | [x] |
| R4-APP-QA | TEAM_LEAD_SERIAL | Run focused/full tests, check diffs and push reviewable PR | [x] |
| R4-APP-REVIEW | INDEPENDENT_REVIEWER_PENDING | Actual approval required before protected merge | [ ] |

- [x] Reproduction fails before fix and passes after fix.
- [x] No runtime executor or permission grant added.
- [x] Full available source CI / strict Clippy evidence recorded, skipped tests disclosed.
- [ ] PR is reviewed independently and cross-OS CI checks pass before merge.
- [ ] Team Lead records GO for PR submission only, NO-GO for production/release.

**Team Lead verdict:** GO for a scoped source PR; NO-GO for protected develop merge / Production Release until genuine independent review and exact-head CI pass.

## Team Lead source review (Mac M4, 2026-10-22)

- Actual worker: TEAM_LEAD_SERIAL. No independent delegated Agent Worker or reviewer ran.
- **RED proof:** Before the fix, new mixed-approval-path test **FAILED as expected**: native validator wrongly returned Ok for approval + independent unapproved input joined before an effectful MCP node.
- **GREEN proof:** After replacing existential ANY-parent approval reachability with ALL-parent propagation and nonempty-dependency guard, R4-focused tests **11 PASS / 0 FAIL** (includes 3 new tests and six effectful node types).
- **Full Rust:** 1053 PASS, 0 FAIL, 33 Ignored; CLI integration 1 PASS.
- **Rust Clippy:** all-targets, --locked, -D warnings PASS.
- **Frontend:** Node unit tests 143 PASS / 0 FAIL; Release-script tests 31 PASS; TypeScript/Vite build PASS, R0 identity/updater safety PASS.
- Reviewed scope: src-tauri/src/workflow_plan.rs only plus documentation, no new permission grant, database migration, API exposure, workflow executor or external side effects.
- Remaining: GitHub cross-OS CI for this SHA, real human Reviewer, live operator-consent boundary, R4 authorized executor E2E and Production Release safety gates.
