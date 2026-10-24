# NC-TEAM-003 — Expanded UI Functional E2E (2026-10-24)

**Lead:** TEAM_LEAD_SERIAL. **Actual worker:** TEAM_LEAD_SERIAL. Human independent reviewer: PENDING (do not manufacture).
**Base:** PR #40 `45b706b`, retains Rust/Tauri v2/React and security advisory fixes. Branch `test/expanded-ui-e2e-r1-20261024`.
**User requirement:** make expanded UI Functional E2E (Terminal, Settings, Agent Teams and MCP Inspector) pass, with regression evidence.
**Observed failure:** previous Mac Playwright 33-case run had missing initial session and textbox (snapshot showed `Open a project to get started`), multiple failures. The final suite result was not captured. Hypothesis: preview bootstrap/test fixture race or mock interception; do not assume a root cause without E2E logs.
**Current tool limitation:** Mac tunnel disconnected; use disposable GitHub-hosted Linux Chromium tests, saved Playwright trace/screenshot logs, then verify locally if tunnel reconnects.

## B. Checklist Planner (committed BEFORE source/test edits)
- [x] UI-E2E-01 / TEAM_LEAD_SERIAL — inspect prior failing browser snapshots, source boot flow and seed fixture; document suspected race not conclusion.
- [ ] UI-E2E-02 / TEAM_LEAD_SERIAL — add reproducible Linux Playwright Chromium workflow for exactly the 4 suites + trace and failure artifacts. Must not access real user data.
- [ ] UI-E2E-03 / TEAM_LEAD_SERIAL — run on hosted CI, capture exact failing cases, repair preview/test fixture readiness only as warranted; no skipping or suppressing assertions.
- [ ] UI-E2E-04 / TEAM_LEAD_SERIAL — 33/33 expanded functional E2E required with no retries masking failures. Repeat on exact source HEAD and check isolated Chromium context.
- [ ] UI-E2E-05 / TEAM_LEAD_SERIAL — ensure npm build, Node/Release contracts and R0 Rust 3 OS CI stay green, QA review diffs; record test files/run IDs.
- [ ] UI-E2E-06 / TEAM_LEAD_SERIAL — open stacked PR targeting PR #40 branch, hand off to Team Lead review before merging; production signing and human independent review remain separate NO-GO.
- [ ] UI-E2E-07 / HUMAN_REVIEW_PENDING — human security/GUI approval and installed native full-app acceptance.

## G. Required per-subtask verification
| Task | Real worker | Unit/Contract | Functional E2E | Smoke | Lead review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| UI-E2E-01 | TEAM_LEAD_SERIAL | N/A diagnostic | Prior failure captured | Prior Mac failures | GO diagnostic | E2E snapshots showed empty project selection |
| UI-E2E-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | hosted CI workflow |
| UI-E2E-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | failed specs and fix diff |
| UI-E2E-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | hosted 33/33 pass |
| UI-E2E-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | build/CI |
| UI-E2E-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | reviewed PR |
| UI-E2E-07 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | NO-GO | reviewer + 3 OS installed |

**Policy:** no production-release claims until genuine independent review, signed updater/publisher keys, real installed Win11/Linux/Mac provider/UI E2E. No CI falsification or generic test skips.
