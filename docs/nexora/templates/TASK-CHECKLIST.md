# Task Checklist Planner — YYYY-MM-DD-task-slug

> Copy this template to docs/nexora/tasks/YYYY-MM-DD-task-slug.md BEFORE
> editing source. Every user assignment creates a new checklist, including
> a one-file fix, documentation task or test task.
> Policy: [SOP NC-TEAM-001](../SOP-DEVELOPMENT-RELEASE.md).

## A. Intake — Team Lead creates FIRST

- **Request / concrete output:** TBD
- **Task ID / date:** TBD
- **Phase / priority:** TBD
- **Branch / base SHA / worktree:** TBD
- **Team Lead (real actor):** TBD
- **Available worker agents (real IDs or NONE):** TBD
- **Security / resource / OS boundaries:** TBD
- **Out of scope and required approvals:** TBD
- **Acceptance criteria:** TBD
- **Dependencies and blockers:** TBD

## B. Team Lead task decomposition and assignments

| ID | Deliverable / acceptance criteria | Real worker ID or TEAM_LEAD_SERIAL | Owned files / branch / worktree | Depends on | Status |
| --- | --- | --- | --- | --- | --- |
| T-01 | TBD | TBD | TBD | none | [ ] |
| T-02 | TBD | TBD | TBD | T-01 | [ ] |
| T-QA | Diff review and independent evidence verification | TEAM_LEAD | Read-only review | T-01,T-02 | [ ] |

Never invent agents or claim independent review without a real second reviewer.
If the available tools cannot create/delegate agents, record TEAM_LEAD_SERIAL.

## C. Evidence and acceptance checklist

- [ ] Created this task planner BEFORE implementing changes.
- [ ] Team Lead divided work, dependencies, priorities and acceptance gates.
- [ ] Each agent was actually assigned a non-overlapping file scope/worktree.
- [ ] Each worker returned a commit/patch, real tests, risks and remaining TODOs.
- [ ] Team Lead inspected every actual worker diff and shared contract.
- [ ] Security, data, secrets and side effects reviewed; permissions respected.
- [ ] Evidence attached: test command, SHA, OS, PASS/FAIL/IGNORED, CI URLs.
- [ ] Native E2E is explicitly PASS or PENDING (never inferred from mock tests).
- [ ] Failed tasks sent back for corrections and independently rechecked.
- [ ] Final Team Lead GO / NO-GO recorded with actual review identity.
- [ ] PR/delivery linked; protected main merge and release separately approved.

## D. Worker handoff ledger

| Task ID | Worker ID | Commit / patch | Tests / evidence | Risk / incomplete | Team Lead review |
| --- | --- | --- | --- | --- | --- |
| T-01 | TBD | TBD | TBD | TBD | PENDING |

## E. Final Team Lead review / handoff

- **Deliverables complete?** TBD
- **Actual reviewer and scope of diff inspected:** TBD
- **Security / quality / compatibility findings:** TBD
- **Commands, OS and test results verified:** TBD
- **CI / native three-OS status:** TBD
- **Owner approval for side effects/main/release:** TBD
- **Final GO / NO-GO:** PENDING
- **Team Lead / review date:** TBD
- **Commit / PR / file links:** TBD
- **Blockers and next tasks:** TBD

> Worker self-report is not an approval. Team Lead must review actual evidence
> and sign off BEFORE delivery, while main merge and release remain gated.
