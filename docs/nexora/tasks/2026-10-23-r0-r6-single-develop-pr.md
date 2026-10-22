# Task Checklist — Consolidate R0–R6 Into One Protected Develop PR

**Date:** 2026-10-23 Asia/Bangkok. Created before changes to the integration branch.
**User request:** "ทำไมไม่รวมสักที" — stop proliferating stacked PRs, consolidate usable work now.
**Team Lead / actual worker:** assistant, TEAM_LEAD_SERIAL only; no independent human GitHub reviewer exists yet.
**Target:** PR #21 integration/r0-r5-consolidated → protected develop; preserve protection and main.
**Input:** clean origin/fix/r6-nexora-bundle-identity (PR #25), containing full PR #24, #23 and #22 ancestry.
**Plan:** incorporate full branch history, verify branch ancestry and zero-conflict merge, push exact SHA on PR #21 to trigger all six required checks, then close redundant reviewed-source PRs only when appropriate; do not cancel active installer smoke runs.
**Hard gate:** develop protection requires 1 genuine independent approval, 6 passing up-to-date checks, admin enforcement. No fake approval, no force push, no bypass.
**Safety:** No live Agent/MCP/Workflow execution, no production release, no deployment, no user data migration, no installed system changes.
**Remaining release:** R6 native installer smoke, macOS validation, 3-OS GUI/provider E2E, R7 stress/rollback, R8 signing/notarization and owner approval.

| Item | Real owner | Success condition | Status |
| --- | --- | --- | --- |
| ONE-01 | TEAM_LEAD_SERIAL | task plan before integration | [x] |
| ONE-02 | TEAM_LEAD_SERIAL | integrate PR #22-25 full history into PR #21 and push | [x] |
| ONE-03 | TEAM_LEAD_SERIAL | confirm latest HEAD includes input commits and matches GitHub | [x] |
| ONE-04 | TEAM_LEAD_SERIAL | exact-SHA CI results; address failures not suppress tests | [ ] |
| ONE-05 | INDEPENDENT_REVIEWER_PENDING | genuine approval before protected merge | [ ] |
| ONE-06 | TEAM_LEAD_SERIAL | merge develop only when checks/review pass | [ ] |

**Initial GO/NO-GO:** GO to source consolidation; NO-GO to bypassing reviewer or claiming R8 completion.

## Integration performed

- Target branch integration/r0-r5-consolidated was clean before work (no source edits pending).
- Full branch history from PR #25 integrated by exact fast-forward, with no merge conflict or rewritten commits.
- Transitive ancestry explicitly verified for original PR #22 / #23 / #24 / #25 source refs.
- Parent commits and their CI evidence retained. PR #21 is the **single canonical develop-facing PR** after push.
- Keep GitHub head SHA fresh for all six required checks; older PR #21 PASS cannot substitute for a new SHA.
- No real independent GitHub reviewer exists: sole repository collaborator is TheerasakPing, who cannot approve own PR. This is an explicit protected-branch blocker, not permission to bypass the rule.
- R6 alpha macOS installer workflow was cancelled on earlier stacked PR, while Windows/Linux package CI passed; new exact-head smoke validation still required.
