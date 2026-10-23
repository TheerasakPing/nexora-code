# Task Checklist — Consolidate R0–R4 into a Reviewable Integration PR

**User request:** Merge the working Nexora Code development into one usable foundation and finish as much as the evidence permits.
**Created before source changes:** 2026-10-22, Bangkok timezone.
**Team Lead:** acting assistant, TEAM_LEAD_SERIAL. No independent Agent Worker or eligible independent GitHub reviewer is available.
**Branch/worktree:** integration/r0-r4-consolidated / nexora-code-final-integration.
**Base:** origin/feat/r3-worker-worktree-provisioner (PR #16, stacked on PRs #15/#14).
**Target:** protected develop, never main or a production release.
**Priority:** P0 source/CI integrity and safe release governance.
**Scope:** consolidate R0-R4 source PRs and missing R1 warm disposal, R0–R8 roadmap checklist, safe planner generator, validate with CI and fix only demonstrated integration failures.
**Authorization:** User approved merge, but enforced independent-review and 6 required checks remain in effect; do not bypass protections or invent approval.
**Security limits:** No GUI/native installer, secret exposure, external MCP, real worker launch, live Git worktree provision or production signing.

| Task ID | Actual worker | Scope | Acceptance | Status |
| --- | --- | --- | --- | --- |
| INT-A | TEAM_LEAD_SERIAL | Inherited PRs #14–#16 | Ancestry verified, all 6 original checks green | [x] |
| INT-B | TEAM_LEAD_SERIAL | PR #3 warm process race | Merge patch, tests, never drop safeguards | [x] |
| INT-C | TEAM_LEAD_SERIAL | PR #1 roadmap checklist and PR #6 planner generator | Merge, fix conflicts, verify generator security/test | [x] |
| INT-D | TEAM_LEAD_SERIAL | Combined native Rust/Clippy, frontend/Release/R0 tests | Zero new regressions; ignored tests disclosed | [x] |
| INT-E | TEAM_LEAD_SERIAL | GitHub branch + consolidated PR targeting develop | Publish and verify remote SHA, CI and required checks | [ ] |
| INT-F | EXTERNAL_REVIEWER_PENDING | Genuine independent code review | Required approval before protected merge | [ ] |
| INT-G | TEAM_LEAD_SERIAL | Release readiness report | Truthful GO/NO-GO; no fake Production Release | [ ] |

## Integration acceptance and handoff checklist

- [x] Fresh task planner created before modifying source.
- [x] Every included source PR's ancestry checked.
- [x] Team Lead reviews all merge conflict resolutions and safety boundaries.
- [ ] Full tests/Clippy and GitHub CI checked on **exact integrated SHA**.
- [ ] Required human reviewer approval obtained (self-review is not enough).
- [ ] Protected develop merge performed only when GitHub branch rules allow.
- [ ] Native GUI E2E, installers, signing and rollback evidence available before any Production Release.

**Team Lead decision:** GO for consolidated PR submission, NO-GO for protected merge or Production Release until genuine reviewer and new CI evidence / R5-R8 native acceptance.

## Mac source acceptance evidence — 2026-10-22

- Merge-base ancestry verified for PR #16, #14, #3, #1 and #6 and their included source history.
- No source merge conflicts for new R1 Warm, R0-R8 checklist or planner generator additions.
- Rust locked tests 1050 PASS / 0 FAIL / 33 ignored; CLI integration 1 PASS; Clippy -D warnings PASS.
- Node 143 PASS / 0 FAIL; Release tests 31 PASS; Task Planner generator focused tests 4 PASS; build and R0 identity/updater safety PASS.
- Team Lead reviewed R1 epoch/lock ordering and Planner exclusive-create source. No independent reviewer or Agent Worker was actually invoked.
- No real GUI, live Agent/MCP, user-repository write, native OS sandboxing, signed installer or production deployment was tested.
- Six new GitHub CI checks on final consolidated SHA and one genuine independent approving reviewer remain required. Older PR results are not final-head acceptance.
- Full release / blockers report: docs/nexora/CONSOLIDATED-INTEGRATION-2026-10-22.md.
