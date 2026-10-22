# Checklist Planner — R0–R5 Security & Workflow Graph Integration

**Created before source merges:** 2026-10-22 (Thailand time).
**User request:** Continue Nexora Code now that the Mac machine is back, consolidate safe finished work and advance toward native cross-OS validation.
**Team Lead:** acting assistant, TEAM_LEAD_SERIAL; no independently delegated Agent Worker or approving reviewer.
**Branch/worktree:** integration/r0-r5-security-graph / nexora-code-r5-security-integration.
**Base:** origin/integration/r0-r5-consolidated, PR #21.
**Inputs:** PR #22 (read-only Workflow DAG Graph), PR #23 (R4 approval-all-paths security + Linux turn-change retention test correction).
**Risk:** Merge resolution, graph UI/integration consistency, regression in native storage ordering, lack of real operator consent for executable workflows.
**Scope:** integrate existing reviewed-source candidate commits, avoid rewriting feature branches, run available fast tests, push exact SHA for GitHub six required checks.
**No-go:** no protected main/develop merge, no fake code review, real workflow execution, automatic external Agent/MCP permission, reinstall, signing or publishing.
**Acceptance:** both branch ancestry included without unsafe conflict; tests/build/CI exact SHA, reviewer and native desktop E2E gates disclosed.

| Task | Actual worker | Scope | Acceptance | State |
| --- | --- | --- | --- | --- |
| INT-01 | TEAM_LEAD_SERIAL | merge PR #22 | safe visual R5 graph history included | [x] |
| INT-02 | TEAM_LEAD_SERIAL | merge PR #23 | R4 security and CI retention ordering included | [x] |
| INT-03 | TEAM_LEAD_SERIAL | all affected source | inspect conflicts, Node/Release/build + GitHub CI | [x] |
| INT-04 | TEAM_LEAD_SERIAL | docs and PR | evidence and reviewable stacked PR to #21 | [ ] |
| INT-05 | Independent reviewer PENDING | review/approval | required for develop protection | [ ] |

- [x] Task planner created before source changes.
- [x] PR #22 and #23 heads fully included without loss of commits.
- [ ] New merged HEAD passes cross-OS exact-SHA checks, ignored disclosed.
- [ ] Native GUI/installer release acceptance remains separate.
- [ ] Independent reviewer approval before protected merge.

**Team Lead decision:** GO for combined-source PR submission only. NO-GO for develop/main merge, native installer or R8 release until all required CI, real independent reviewer and native acceptance.

## Source integration evidence — 2026-10-22

- Base PR #21 had 6/6 successful required GitHub checks, but real independent reviewer approval is still pending.
- PR #22 had 6/6 successful checks. Merged its complete ancestry into isolated branch with no conflict; retained the read-only R5 graph.
- PR #23 at SHA 5a086ccd1604cb6731919b242b7c24af44a15214 contains R4 all-path approval gate plus the correction to non-deterministic Linux turn-change test/retention order. Merged complete ancestry without conflict.
- The original PR #23 Linux CI failure was due to turn-changes retention ordering, not DAG validation. Its corrected SHA CI was still running when this integration began.
- This integrated SHA must run its own 6/6 checks. It cannot inherit the previous CI conclusions.
- Source-level checkout/merges were the only Git mutations. No GUI, Agent, MCP, user-workspace checkout provisioning, signing, installer or release occurred.
- The local Mac was resource-constrained while rebuilding Rust; do not claim native Rust checks passed on the new SHA until actual test output is observed.
- Real operator consent, R4 safe executor integration, OS sandbox, crash recovery, native UI E2E and R6-R8 release gates remain NO-GO.

## Mac frontend evidence after merged source

- npm run build PASS. Vite's pre-existing large chunk advisory still appears.
- npm test: **156 PASS / 0 FAIL**.
- npm run test:release: **31 PASS / 0 FAIL**.
- npm run test:nexora:r0: **PASS** (fork/updater safety).
- git diff --check: PASS.
- Rust and native GUI on this new integrated SHA: PENDING GitHub CI. Earlier source-branch green results do not transfer to this new SHA.
