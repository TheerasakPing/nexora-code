# Task Checklist — 2026-10-22-windows-governance-integration

- **Request:** Finish user-authorized governance + Windows CI integration while preserving protected develop review gates.
- **Created before implementation:** 2026-10-22.
- **Team Lead:** acting assistant, TEAM_LEAD_SERIAL; no independent Reviewer/Agent was available.
- **Priority:** P0 CI; P1 governance.
- **Scope:** Combine only reviewed PR #2 Windows Clippy patch and PR #4/#5 mandatory planner + single trusted CI verifier.
- **Base SHA / branch:** origin/develop at integration-branch creation; feat/integrate-governance-windows-gates.
- **Worktree:** nexora-code-governance-integration. Other sessions/worktrees remain untouched.
- **Exclusions:** PR #3 Warm Agent feature, PR #6 generator, main, installer, GUI, credentials, live MCP, production release.
- **Risk:** PR #2–#5 retain own histories and will not themselves merge without a real approving Reviewer; treat this as a reviewable integration candidate, not completed protected-branch merge.
- **Dependency:** GitHub CI passes including Windows 11 x64; genuine separate approval required by enforced develop Branch Protection.
- **DoD:** source contains reviewed Windows patch, SOP + Team Lead planner rule, safe trusted-base PR check, accurate PR handoff; full Mac/Node tests plus 3-OS GitHub checks; review/branch protection gates.

| Task | Actual Worker | Acceptance | Status |
| --- | --- | --- | --- |
| INTEG-01 | TEAM_LEAD_SERIAL | Cherry-pick/merge PR2 and PR5 histories without overwriting contributors | [ ] |
| INTEG-02 | TEAM_LEAD_SERIAL | Review final diff and policy/Windows behavior | [ ] |
| INTEG-03 | TEAM_LEAD_SERIAL | Run full available tests; capture ignored and CI pending | [ ] |
| INTEG-04 | TEAM_LEAD_SERIAL | Open integration PR and attach evidence | [ ] |
| INTEG-05 | INDEPENDENT_REVIEWER_PENDING | Review before protected develop merge | [ ] |

## Review and handoff

- [ ] Windows Clippy diagnostic matched known fix.
- [ ] No untrusted PR code executed by high-privilege workflow.
- [ ] No conflicting changes from separately owned PR #3/#6.
- [ ] Tests and git diff checks pass.
- [ ] Independent GitHub reviewer approves PR.
- [ ] Team Lead final GO for develop merge recorded.

**Team Lead verdict:** GO for integration source work and PR submission; NO-GO for protected merge before independent review and CI.
