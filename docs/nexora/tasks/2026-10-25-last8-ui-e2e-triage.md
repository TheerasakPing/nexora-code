# NC-TEAM-003 — Last eight macOS Browser UI E2E diagnostics
**Date:** 2026-10-25 Asia/Bangkok. **Actual lead/worker:** TEAM_LEAD_SERIAL. Human independent reviewer: PENDING.
**Input head:** 31254f59 from PR #44, stacked on PR #43. Existing 33/33 targeted UI, CI Rust 3-OS & installers pass.
**Evidence:** full macOS 4-shard workflow 40147908199 complete: Shard1 186/186 PASS, Shard2 183/186, Shard3 185/186, Shard4 181/186: 736 PASS / **8 FAIL** with 0 retries and 744 executed.
Failure classification:
- task deletion: missing Task actions menuitem 'Delete task…'.
- UI 100-message history and turn activity: project filter menu did not expose 'pandacode' radio item.
- UI first prompt from New task: heading 'What should we build in' not found.
- UI provider logos: expected visible claude-logo in assistant message not found.
- usage-limit: Resume at reset checkbox not found.
- Jev live: watched other-run lease expected to activate after session switch.
- responsive layout: header actions overflow 760px by 3px with 160% scale.

## Checklist Planner — commit before source changes
- [x] LAST8-01 / TEAM_LEAD_SERIAL: audit all 8 exact hosted stack traces and repeated Mac / Linux environment hints.
- [ ] LAST8-02 / TEAM_LEAD_SERIAL: reproduce each root cause in isolated Mac Chrome; repair only verified browser test fixtures or genuine UX layout.
- [ ] LAST8-03 / TEAM_LEAD_SERIAL: unit/contract + focused E2E + smoke per subtask; never remove assertions or suppress failures with retries.
- [ ] LAST8-04 / TEAM_LEAD_SERIAL: run all 744 tests across hosted Mac CI with 0 retries; 33/33, npm Node/Release/Build and 3-OS Rust/Installer.
- [ ] LAST8-05 / TEAM_LEAD_SERIAL: perform code/QA review, merge only after all checks pass; no claims of human security signoff.
- [ ] LAST8-06 / HUMAN_REVIEW_PENDING: signed/native installed production requires independent human approval and real provider workflows.

| Work item | Worker | Unit/Contract | Native/Browser E2E | Smoke | Lead |
| --- | --- | --- | --- | --- | --- |
| LAST8-01 | TEAM_LEAD_SERIAL | PASS classified 8 errors | Baseline 736/744 | hosted 4 shards ran | GO diagnosis |
| LAST8-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| LAST8-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| LAST8-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| LAST8-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING |
| LAST8-06 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO |
