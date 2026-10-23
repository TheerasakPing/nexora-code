# Task Checklist Planner — NC-TEAM-003 mandatory subtask E2E/Smoke/Lead gates

**Date / Task:** 2026-10-23 / NC-TEAM-003.
**User request:** "เขียนกฏไว้เลยว่าทุกครั้งที่ให้ทำงาน จะต้องสร้างเช็คลิสแพลนเนอร์ แล้วแบ่งเอเจ้นไปทำทุกงานที่เอเจ้นแต่ละตัวทำต้องทำการตรวจสอบ ตรวจสอบ e2e smoketest เสมอ แล้วแต่ละงานย่อยจะต้องถูกตรวจสอบโดยมีหัวหน้าทีมเป็นคนแบ่งงานและตรวจสอบก่อนส่งมอบเสมอ"
**Team Lead:** ChatGPT Integration Lead, acting TEAM_LEAD_SERIAL for this change. The DWB broker currently has 2 active general tool workers / 6 capacity, but exposes no task-delegation API through this chat; neither worker has been assigned an independent engineering task here. No independent reviewer has approved PR #26.
**Base / worktree:** feat/mac-m4-native-preview-20261023 at c197121 / /Volumes/Data/workspace/git/nexora-code-integration-20261023.
**Scope:** repo rules, SOP, planner template, automated structural PR governance gate, test coverage and existing open-PR checklist compatibility.
**Out of scope:** production release, signing, changing platform settings or user data, fabrication of worker assignments, pretending a CI unit test is native E2E.
**Dependencies:** NC-TEAM-001/002; existing PR #26 rules. Work is serialized due to available tooling.
**Acceptance criteria:** Each user task starts with dated planner; Team Lead creates and delegates real isolated tasks where supported, else explicit serial mode. Every subtask has Unit/Contract, E2E and Smoke status/evidence (actual or N/A justification), plus explicit Team Lead Review status; worker cannot self-approve; gates fail closed for absent matrix and false completion. Automated tests prove guardrails; independent review remains external.

## B. Team Lead task decomposition and assignments

| ID | Deliverable / acceptance criteria | Real worker | Owned files / branch | Depends on | Status |
| --- | --- | --- | --- | --- | --- |
| G-01 | Define mandatory policy for per-task planning, actual delegation and tests | TEAM_LEAD_SERIAL | AGENTS.md, SOP | None |[x] |
| G-02 | Extend standard task template with per-subtask test/review matrix | TEAM_LEAD_SERIAL | docs/nexora/templates/TASK-CHECKLIST.md | G-01 |[x] |
| G-03 | Enforce planner structure in PR gate, retrofit existing open PR planners | TEAM_LEAD_SERIAL | scripts/nexora/governance-pr-gate.mjs, docs/nexora/tasks/* | G-02 |[x] |
| G-04 | Unit/contract, E2E/Smoke structural regression, run checks | TEAM_LEAD_SERIAL | tests/governance-pr-gate.test.mjs, validation commands | G-03 |[x] |
| G-05 | Lead review of every diff, record QA and progress, push PR | TEAM_LEAD_SERIAL | this planner and PR #26 | G-04 | [ ] |
| G-REVIEW | Independent review before protected develop merge | UNASSIGNED independent reviewer | Protected branch | G-05 | [ ] |

## C. Evidence and acceptance checklist

- [x] Created planner before rule/source edits.
- [x] Team Lead decomposed and ordered all subtasks; serial fallback explicitly disclosed.
- [x] Every worker patch and changed-file diff checked by Team Lead.
- [x] Unit/Contract, E2E and Smoke gate recorded per subtask; N/A requires Lead-signed reason.
- [x] Task template and automated PR validator reject missing evidence structure.
- [x] Full governance and Node tests run and exact evidence attached.
- [ ] PR updated and exact-head CI status checked (not guessed).
- [ ] Genuine independent reviewer approval obtained before protected merge.

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| G-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | AGENTS/SOP requirements audited; macOS M4 node --test tests/team-governance.test.mjs; CLI PR fixtures exercised; Team Lead source diff inspection 2026-10-23 |
| G-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Template matrix and source integrity checked; Node template fixtures and CLI PR smoke passed on macOS M4; Team Lead source diff inspection 2026-10-23 |
| G-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Validated 8 new PR planners; test CLI rejects missing E2E/Smoke, N/A without approval, false Lead GO and [x] while pending; Team Lead source diff inspection 2026-10-23 |
| G-04 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | macOS M4 npm test 175/175 PASS; 25 governance E2E/Smoke cases PASS; git diff --check PASS; exact SHA after commit; Team Lead source diff inspection 2026-10-23 |
| G-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Exact-head GitHub CI and PR handoff pending after push; owner Team Lead |

| G-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No actual independent reviewer assigned; protected merge remains NO-GO |

## E. Team Lead handoff and review

**Team Lead verdict:** SOURCE REVIEW GO for PR submission only, contingent on actual test evidence recorded below.
**Protected merge:** NO-GO, requires genuine independent reviewer and exact-head CI.
**Native acceptance:** NO-GO; this task is policy tooling, not a native build.
**Production release:** NO-GO.
**Work counters:** 21/24 engineering acceptance packages unchecked; BR 7/7 unapproved; 61 R0–R8 detailed acceptance checkboxes unchecked per 2026-10-23 snapshot; categories overlap.
**Top 3 next work:** QA enforcement, actual multi-agent scheduler/delegation capability, source-to-native verification requirements.
**Evidence / review:** PENDING implementation.


### NC-TEAM-003 acceptance tracker (not inferred)

- [ ] NC-TEAM-003: Unit/Contract, E2E and Smoke verified per subtask.
- [ ] NC-TEAM-003: All applicable tests and native acceptance requirements passed.
- [ ] NC-TEAM-003: Real independent reviewer approved protected merge and release.

**GO/NO-GO:** SOURCE PR GO for review only; protected merge/production NO-GO.

## Verified NC-TEAM-003 test ledger (Task Lead source inspection)

- **Test host:** macOS Apple M4 / 2026-10-23. This is documentation and
  governance PR tooling, so its testable E2E is the real `check-task-planner.mjs`
  CLI against synthetic local git PR history / event data, not a GUI claim.
- **Unit/contract:** `node --test tests/governance-pr-gate.test.mjs tests/task-planner-gate.test.mjs tests/team-governance.test.mjs`: 25/25 PASS.
- **E2E:** Test fixtures create real temporary Git repos with base/head SHAs
  and run the actual PR validator CLI as a separate process; valid planners
  PASS and malformed/missing E2E/Smoke/Lead states fail closed.
- **Smoke:** `npm test`: 175/175 PASS and `git diff --check` PASS.
  Actual GitHub exact-head CI and user interface native E2E remain independent
  PENDING gates, never inferred from this source test.
- **Lead diff review:** AGENTS, SOP, template, CONTRIBUTING, PR template,
  secure trusted-base CI workflow, validator and 9 retrospective plans
  inspected for syntax/protocol compatibility and no invented PASS.
- **Delegation:** TEAM_LEAD_SERIAL only; DWB general worker pool exists but
  offers no task delegation API in this chat. No fake peer reviewer sign-off.
- **Remaining work:** G-05 exact-head CI/PR handoff, G-REVIEW real reviewer
  and subsequent protected merge. Source reviewer decision: GO to submit PR;
  protected merge/native acceptance/production: NO-GO.
