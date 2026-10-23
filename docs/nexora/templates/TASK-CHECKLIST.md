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
- **Mandatory NC-TEAM-003:** One new planner per request; one verification row per actual subtask; real E2E + Smoke outcomes never implied by build/unit PASS.
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

## G. NC-TEAM-003 — Subtask Verification Matrix (REQUIRED in every planner)

> One row per subtask ID from section B (excluding the Team Lead-only
> final acceptance row). A worker cannot self-approve. Use PASS with exact
> command/log/OS/SHA evidence, FAIL, PENDING, BLOCKED + owner, or
> N/A: specific reason + Lead approval. N/A without reason is prohibited.
> Native E2E / live provider tests must be named separately from mock/browser E2E.

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| T-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Test command/OS/SHA/log and relevant N/A justification |
| T-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Test command/OS/SHA/log and relevant N/A justification |

**Subtask handoff rule:** do NOT tick section B `[x]` without a
Team Lead-verified triad. `PENDING` / `BLOCKED` / `FAIL` means NOT ACCEPTED.
When tests are genuinely inapplicable, write e.g. `N/A: docs-only, accepted
by Team Lead YYYY-MM-DD` in that test column and describe the document
link/build/render smoke in Evidence where feasible. The Team Lead must
verify the claimed CI/OS evidence, request repairs and retest failures.

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

## F. Mandatory remaining-work report — NC-TEAM-002

- **Snapshot date/time / source reference:** TBD
- **What changed this task (commit/PR/tests/OS):** TBD
- **Engineering acceptance:** TBD accepted / 24; TBD still pending (do not equate PR merge with acceptance)
- **Brand epics:** TBD accepted / 7; TBD still pending
- **R0–R8 detailed acceptance checks:** TBD passed / TBD total; TBD unchecked (may overlap with engineering/brand)
- **Open GitHub PRs / issues:** TBD PRs / TBD issues (separate from roadmap)
- **P0 blockers / real owner / unblock condition:** TBD
- **Next 1–3 highest-priority tasks / dependencies:** TBD
- **Evidence freshness and discrepancies:** VERIFIED as of YYYY-MM-DD, or UNVERIFIED with last known source
- **Team Lead delivery decision:** GO / NO-GO / PENDING
- **Protected merge decision:** GO / NO-GO / PENDING
- **Native acceptance decision:** GO / NO-GO / PENDING
- **Production release decision:** GO / NO-GO / PENDING

> Counts refer to different overlapping views and MUST NOT be added.
> A source-integrated PR is not automatically an accepted roadmap deliverable.

> Worker self-report is not an approval. Team Lead must review actual evidence
> and sign off BEFORE delivery, while main merge and release remain gated.
