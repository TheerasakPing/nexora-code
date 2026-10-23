# Task Checklist Planner — 2026-10-23 NC-TEAM-002 Progress Accountability

> Created BEFORE changing policy files. Timezone: Asia/Bangkok. This is a documentation/governance task, NOT authorization to bypass branch protection.

## A. Intake — Team Lead
- **Request / concrete output:** "ปรับกฏเลย เหลืออักกี่งาน" — update mandatory agent rules to report remaining work after every task; provide a verified, non-duplicated backlog summary.
- **Task ID / date:** NC-TEAM-002 / 2026-10-23.
- **Phase / priority:** Cross-cutting governance / P1.
- **Repository / branch / base:** TheerasakPing/nexora-code; docs/nc-team-002-progress-accountability-20261023 from develop.
- **Team Lead (real actor):** ChatGPT acting as Integration Lead.
- **Available worker agents (real IDs or NONE):** NONE; TEAM_LEAD_SERIAL. No independent reviewer claimed.
- **Affected paths:** AGENTS.md; docs/nexora/SOP-DEVELOPMENT-RELEASE.md; docs/nexora/templates/TASK-CHECKLIST.md; docs/nexora/PROGRESS-SNAPSHOT-2026-10-23.md; this task planner.
- **Security/resources:** Docs only. No app run, credentials, external MCP, installers, user data, or OS changes.
- **Acceptance:** Mandatory completion/handoff report states completed, remaining, blocked, next priority, evidence, review status; reports both acceptance counts and PR counts without mixing; GitHub change submitted for review, not merged without independent approval.
- **Dependencies:** Existing NC-TEAM-001, Roadmap, checklist, and latest GitHub PR state.
- **Approval boundary:** No direct develop/main commit, auto-merge, release, deployment or bypass.

## B. Team Lead decomposition / ownership

| ID | Deliverable / acceptance | Actual worker | Owned files | Depends on | Status |
| --- | --- | --- | --- | --- | --- |
| T-01 | Review NC-TEAM-001 and current backlog evidence | TEAM_LEAD_SERIAL | Read-only | none | [x] |
| T-02 | Add explicit per-task remaining-work reporting rule | TEAM_LEAD_SERIAL | AGENTS.md, SOP | T-01 | [ ] |
| T-03 | Update Task Planner template to capture accurate remaining counts, blockers and evidence | TEAM_LEAD_SERIAL | docs/nexora/templates/TASK-CHECKLIST.md | T-02 | [ ] |
| T-04 | Document 2026-10-23 snapshot and counting method | TEAM_LEAD_SERIAL | docs/nexora/PROGRESS-SNAPSHOT-2026-10-23.md | T-01 | [ ] |
| T-QA | Inspect final diffs and syntax/consistency; record exact GitHub SHA and status | TEAM_LEAD_SERIAL | Read-only | T-02,T-03,T-04 | [ ] |
| T-REVIEW | Independent authorized approval and CI for protected merge | INDEPENDENT_REVIEWER_PENDING | PR | T-QA | [ ] |

## C. Evidence / acceptance checklist
- [x] New Task Planner created before policy-file edits.
- [x] One-writer scopes, risk and dependency mapping recorded; no invented workers.
- [ ] Updated rule preserves existing NC-TEAM-001 gates and explicit NO-GO semantics.
- [ ] Remaining counts verified from source and differentiated by type.
- [ ] Team Lead reviews the exact changed files and records GO for review submission or NO-GO.
- [ ] GitHub CI and independent review PASS for protected develop merge — currently PENDING.

## D. Worker handoff ledger
| Task | Worker | Evidence | Risk | Lead review |
| --- | --- | --- | --- | --- |
| T-01 | TEAM_LEAD_SERIAL | GitHub Roadmap, Checklist, PR #21 | Checklist statuses can lag source integrations | COMPLETE |
| T-02–T-04 | TEAM_LEAD_SERIAL | PENDING | Documentation consistency | PENDING |

## E. Team Lead final review / handoff
- **Source counts from checked roadmap (2026-10-23):** 24 planned engineering acceptance packages, 3 source-complete and 21 not fully accepted; 7/7 branding epics still pending; 61 unchecked R0–R8 phase checklist items. These are different measures, not additive. Whole living checklist has 112 unchecked boxes including duplicated NOW/QA/sprint templates.
- **Latest independent GitHub fact:** PR #21 (R0–R6 consolidation) is MERGED into develop; no claim that native gates passed.
- **Actual reviewer:** TEAM_LEAD_SERIAL (not independent).
- **Policy/QA findings:** PENDING.
- **CI/native three-OS:** PENDING. Prior Mac source-test reports are not this branch's CI.
- **Final GO/NO-GO:** PENDING; protected merge is NO-GO absent CI and independent approval.
- **Commit and PR URLs:** PENDING.
