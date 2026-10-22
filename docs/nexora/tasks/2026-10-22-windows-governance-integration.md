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
| INTEG-01 | TEAM_LEAD_SERIAL | Merge PR2 and PR5 histories without overwriting contributors | [x] |
| INTEG-02 | TEAM_LEAD_SERIAL | Review final diff and policy/Windows behavior | [x] |
| INTEG-03 | TEAM_LEAD_SERIAL | Run full local tests; record ignored and remote CI pending | [x] |
| INTEG-04 | TEAM_LEAD_SERIAL | Open integration PR and attach evidence | [x] |
| INTEG-05 | INDEPENDENT_REVIEWER_PENDING | Review before protected develop merge | [ ] |

## Review and handoff

- [x] Windows Clippy diagnostic matched known fix.
- [x] Source review: trusted base SHA checkout, no PR-executed code in governance workflow.
- [x] No conflicting changes from separately owned PR #3/#6.
- [x] Local Node/Rust/test/Clippy/Build and git diff checks pass; remote CI pending.
- [ ] Independent GitHub reviewer approves PR.
- [ ] Team Lead final GO for develop merge recorded.

**Team Lead verdict:** GO for integration source work and PR submission; NO-GO for protected merge before independent review and CI.

## Completed local evidence and outstanding gate — 2026-10-22

- **Combined integration PR:** https://github.com/TheerasakPing/nexora-code/pull/7
- **Team Lead source review:** acting assistant (TEAM_LEAD_SERIAL), reviewed final changed files, Windows OS conditions, trusted-base workflow, inherited PR histories and source test outputs.
- **Mac M4 Node/Vite:** npm run build PASS; npm test **129 PASS / 0 FAIL**; npm run test:release **31 PASS / 0 FAIL**; R0 tests PASS.
- **Mac M4 Rust:** cargo test --locked: **1006 PASS, 0 FAIL, 33 ignored**, Rust CLI integration **1 PASS**. cargo clippy --all-targets --locked -- -D warnings: PASS.
- **Known warning:** existing Vite large-chunk advisory, not a test failure.
- **Windows:** PR #2 individual branch checked successful six CI gates including Windows; **combined PR #7 cross-platform CI is still PENDING**.
- **Ownership:** no separate worker agents or independent reviewer available (only TheerasakPing is a collaborator). Never infer approval.
- **Branch rule evidence:** develop requires all six exact CI checks, one genuine approving Reviewer, strict up-to-date base, conversation resolution, admin enforcement, no force pushes/deletions.
- **Decision:** **GO for PR submission and continued CI review only; NO-GO for develop merge or release until an authorized independent reviewer approves and combined CI passes**.
