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
| G-01 | Define mandatory policy for per-task planning, actual delegation and tests | TEAM_LEAD_SERIAL | AGENTS.md, SOP | None | [ ] |
| G-02 | Extend standard task template with per-subtask test/review matrix | TEAM_LEAD_SERIAL | docs/nexora/templates/TASK-CHECKLIST.md | G-01 | [ ] |
| G-03 | Enforce planner structure in PR gate, retrofit existing open PR planners | TEAM_LEAD_SERIAL | scripts/nexora/governance-pr-gate.mjs, docs/nexora/tasks/* | G-02 | [ ] |
| G-04 | Unit/contract, E2E/Smoke structural regression, run checks | TEAM_LEAD_SERIAL | tests/governance-pr-gate.test.mjs, validation commands | G-03 | [ ] |
| G-05 | Lead review of every diff, record QA and progress, push PR | TEAM_LEAD_SERIAL | this planner and PR #26 | G-04 | [ ] |
| G-REVIEW | Independent review before protected develop merge | UNASSIGNED independent reviewer | Protected branch | G-05 | [ ] |

## C. Evidence and acceptance checklist

- [x] Created planner before rule/source edits.
- [x] Team Lead decomposed and ordered all subtasks; serial fallback explicitly disclosed.
- [ ] Every worker patch and changed-file diff checked by Team Lead.
- [ ] Unit/Contract, E2E and Smoke gate recorded per subtask; N/A requires Lead-signed reason.
- [ ] Task template and automated PR validator reject missing evidence structure.
- [ ] Full governance and Node tests run and exact evidence attached.
- [ ] PR updated and exact-head CI status checked (not guessed).
- [ ] Genuine independent reviewer approval obtained before protected merge.

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| G-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | SOP check + governance integration assertions after implementation |
| G-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Template rendering/fixture parser + CI smoke |
| G-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Gate fixtures and actual PR diff simulated |
| G-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | test runner results and CLI E2E smoke |
| G-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | git diff --check, PR metadata + commit SHA |

## E. Team Lead handoff and review

**Team Lead verdict:** SOURCE REVIEW GO for PR submission only, contingent on actual test evidence recorded below.
**Protected merge:** NO-GO, requires genuine independent reviewer and exact-head CI.
**Native acceptance:** NO-GO; this task is policy tooling, not a native build.
**Production release:** NO-GO.
**Work counters:** 21/24 engineering acceptance packages unchecked; BR 7/7 unapproved; 61 R0–R8 detailed acceptance checkboxes unchecked per 2026-10-23 snapshot; categories overlap.
**Top 3 next work:** QA enforcement, actual multi-agent scheduler/delegation capability, source-to-native verification requirements.
**Evidence / review:** PENDING implementation.
