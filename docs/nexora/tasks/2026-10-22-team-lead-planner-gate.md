# Task Checklist — 2026-10-22-team-lead-planner-gate

- **Request:** Require a checklist planner for every assigned Nexora Code task, Team Lead delegation to agents and Team Lead inspection before delivery.
- **Goal:** Make the operating rule visible at agent entrypoints, SOP, contributor PR process and a reusable per-task checklist.
- **Branch:** docs/team-lead-planner-gate (based on develop)
- **Team Lead / Integration Lead:** current assigned assistant (actual reviewer; no invented second agent).
- **Priority:** P1 / process governance
- **Status:** SOURCE REVIEW GO; PR review and CI pending
- **Owner approval:** User request grants documentation work and ordinary source branch/PR, not release or merge to main.
- **Dependencies:** docs/nexora/SOP-DEVELOPMENT-RELEASE.md; root AGENTS.md; CONTRIBUTING.md; existing PR template.
- **Side effects:** documentation-only; no install, GUI, MCP, secrets, build or release.

## Team Lead assignment and review checklist

| Task | Assigned worker | Scope | Acceptance | Status |
| --- | --- | --- | --- | --- |
| GOV-01 | Documentation worker (Team Lead executing serially) | docs/nexora/SOP-DEVELOPMENT-RELEASE.md | Mandatory lifecycle with review/GO-NO-GO | [x] |
| GOV-02 | Agent-rules worker (Team Lead executing serially) | AGENTS.md | Prominent rule for all coding agents | [x] |
| GOV-03 | Template worker (Team Lead executing serially) | docs/nexora/templates/TASK-CHECKLIST.md | Reusable per-request checkboxes and evidence | [x] |
| GOV-04 | PR gate worker (Team Lead executing serially) | CONTRIBUTING.md + .github/pull_request_template.md | PR requires planner and Team Lead review | [x] |
| GOV-05 | Team Lead / QA | This planner and all changed docs | Check links, no false approvals, diff, git status | [x] |

> Multi-agent runtime is not invoked for this document-only request; roles are described
> without inventing separate workers or independent approvals. When actual worker agents
> are available, Team Lead should delegate independent scopes and record their IDs.

## Final gate (fill with actual evidence)

- [x] Every task has been tracked and reviewed by Team Lead.
- [x] Markdown links and task template fields verified.
- [x] Git diff --check clean.
- [x] Checked existing provider/brand/upstream constraints remain intact.
- [ ] Commit SHA and PR link recorded (pending publication).
- [x] Team Lead outcome recorded as GO / NO-GO, with blockers if any.

**Team Lead verdict:** GO for submitting a documentation-only PR; NO-GO for protected-main merge or production release pending CI and actual reviewer approval.

**Review evidence (Apple M4, 2026-10-22):** Team Lead inspected AGENTS.md, SOP, CONTRIBUTING, PR template, new task template and Node regression tests. Policy links verified. npm run build: PASS; npm test: 114/114 PASS; npm run test:release: 31/31 PASS; npm run test:nexora:r0: PASS; git diff --check: PASS. No Rust application code changed.

**Agents used:** TEAM_LEAD_SERIAL, no independent worker or reviewer invoked. This absence is disclosed; an independent PR reviewer remains PENDING.

**Open checks:** Independent PR review, CI after push, owner-specific approvals for merge/release. No installer, native GUI, external MCP or live secrets accessed.
