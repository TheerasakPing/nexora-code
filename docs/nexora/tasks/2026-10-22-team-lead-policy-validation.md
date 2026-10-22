# Task Checklist — 2026-10-22-team-lead-policy-validation

- **Request:** Continue the in-progress NC-TEAM-001 rule rollout, validate and deliver the patch.
- **Task ID:** GOV-CONT-20261022
- **Lead:** Acting Team Lead (assistant), serial implementation; no actual delegated worker agent.
- **Priority:** P1 governance.
- **Branch/worktree:** docs/team-lead-planner-gate, nexora-code-team-rules.
- **Dependencies:** [Initial governance task](2026-10-22-team-lead-planner-gate.md); SOP NC-TEAM-001.
- **Scope:** Validation, corrections, tests, review, PR and automated planner CI gate. No installation or protected branch merge.
- **DoD:** Correct links, truthful handoff identities, passing automated tests, reviewed diff, committed PR.
- **Approval boundary:** Source docs/branch PR allowed; no main merge, no production release.

| Task | Real owner | Deliverable | Acceptance | Status |
| --- | --- | --- | --- | --- |
| GOV-CONT-01 | TEAM_LEAD_SERIAL | Review existing governance PR and links | All relative paths valid | [x] |
| GOV-CONT-02 | TEAM_LEAD_SERIAL | Create CI planner guard and regression tests | Refuse missing/unapproved planners | [x] |
| GOV-CONT-03 | TEAM_LEAD_SERIAL | Source build and full tests | Node 120, release 31, Vite, R0 | [x] |
| GOV-CONT-04 | TEAM_LEAD_SERIAL | Review patches, open PR update | Diff checked and GO/NO-GO | [x] |

## Evidence / final review

- [x] Planner and agent instructions checked for all required fields.
- [x] Security and legacy upstream release guidance checked; no claim of independent review.
- [x] Links checked, governance focused tests 9/9, npm tests 120/120, release tests 31/31, build and R0 PASS on Mac M4.
- [x] Source diff checked, governance PR #4 already exists; final commit and check references are in PR history.
- [x] Team Lead source review GO for PR update; NO-GO for protected merge without required CI/reviewer.

**Team Lead verdict:** GO for PR submission; NO-GO for merge until required CI and independent reviewer pass.

**Actual worker:** TEAM_LEAD_SERIAL (assistant performing tasks sequentially), not independent review.
**Evidence (22 Oct 2026, Mac M4):** node --test governance tests 9/9 PASS;
npm test 119/119 PASS; npm run test:release 31/31 PASS; npm run build PASS;
npm run test:nexora:r0 PASS; git diff --check PASS.
**Automated gate limitation:** GitHub structure check alone cannot prove reviewer identity,
or enforce a protected branch rule. Branch protection and human reviewer needed.
**PR:** https://github.com/TheerasakPing/nexora-code/pull/4
**Runtime/native E2E:** NOT TESTED (documentation/governance only).

**Guard evidence:** New GH Actions Task Planner Gate checks only structural
file additions and Team Lead declaration. It cannot prove human review.
Required-check branch protection and independent reviewer remain PENDING.
