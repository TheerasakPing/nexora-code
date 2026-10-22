# Task Checklist — 2026-10-22-governance-ci-gate

- **Request:** Continue NC-TEAM-001 governance work. Ensure every new PR has a task planner and Team Lead review evidence, instead of relying on prose alone.
- **Created before implementation:** 2026-10-22
- **Priority:** P1 — governance reliability
- **Team Lead:** Acting assistant / Integration Lead
- **Worker mode:** TEAM_LEAD_SERIAL; no actual remote sub-agent delegation tool is available.
- **Source branch/worktree:** feat/governance-task-gate / nexora-code-governance-ci
- **Base:** docs/team-lead-planner-gate (stacked on unmerged PR #4)
- **Scope:** New safe Node verifier, tests, dedicated read-only GitHub PR check, documentation.
- **Out of scope:** App installation, native GUI, secrets, MCP execution, merging develop/main.
- **Acceptance:** Validator rejects missing or malformed planner, accepts a genuine new task planner, does not invent worker review; GitHub workflow read-only and safe for forks.
- **Dependencies:** NC-TEAM-001, PR #4; existing PRs #1–#3 opened before the rule must not be retroactively represented as compliant.
- **Risks:** A strict PR gate may block old PRs; grandfather exact PR numbers 1–3 only, visibly reported.

| ID | Task / deliverable | Owner | Files | Dependency | Status |
| --- | --- | --- | --- | --- | --- |
| GOV-GATE-01 | Implement a PR changed-task-planner verifier | TEAM_LEAD_SERIAL | scripts/nexora/check-task-planner.mjs | none | [x] |
| GOV-GATE-02 | Test positive/negative cases with isolated temp git repos | TEAM_LEAD_SERIAL | tests/task-planner-gate.test.mjs | GOV-GATE-01 | [x] |
| GOV-GATE-03 | Add an appropriately scoped GitHub Actions PR check | TEAM_LEAD_SERIAL | .github/workflows/nexora-task-governance.yml | GOV-GATE-02 | [x] |
| GOV-GATE-04 | Team Lead inspect and handoff, update rules/Planner | TEAM_LEAD_SERIAL | this planner, docs | All | [x] |

## Team Lead acceptance checklist

- [x] Each task reviewed for objective and scope.
- [x] Self-contained Node tests pass (including injection and path cases).
- [x] Existing source tests pass with code SHA and actual counts.
- [x] Workflow permissions, fetch depth and secrets usage reviewed.
- [x] Diff and file links checked; existing PR4 untouched.
- [x] Final review GO/NO-GO and independent reviewer status recorded.
- [x] PR/push result and any CI remaining pending recorded.

**Team Lead verdict:** GO for PR submission of a scoped stacked PR and source documentation handoff; NO-GO for merging protected branches or releasing before required CI and actual reviewer approval.

**Team Lead evidence (Mac Apple Silicon, 2026-10-22):** Acting assistant reviewed the changed script, tests, trusted-base workflow, SOP, AGENTS and docs. npm run build PASS (Vite large-chunk advisory); npm test **121 PASS / 0 FAIL**, release tests **31 PASS / 0 FAIL**, R0 identity check PASS, governance focused tests **7 PASS / 0 FAIL**, git diff --check PASS. No Rust application code changed in this patch.

**Actual agent roster:** TEAM_LEAD_SERIAL only; no independent worker or reviewer ran. Separate reviewer approval PENDING. PR and GitHub Actions check PENDING until pushed and verified. The workflow is not active as a required branch protection check until reviewed and merged into the base branch.


## Delivery and activation record

- **Source commit:** 7f36cb5 — governance verifier + safe workflow + tests.
- **Stacked PR:** https://github.com/TheerasakPing/nexora-code/pull/5
- **Base branch:** docs/team-lead-planner-gate (PR #4, itself targeting develop).
- **Local Git diff validator:** PASS against actual base/head SHAs for a new PR.
- **CI activation:** PENDING — base branch must first contain this workflow.
- **Independent reviewer approval:** PENDING; TEAM_LEAD_SERIAL review is not independent review.
- **Protected branch merge / app release:** NO-GO until separate checks and authorization.
