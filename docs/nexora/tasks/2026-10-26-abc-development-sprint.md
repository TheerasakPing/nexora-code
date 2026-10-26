# Nexora Code A–C implementation sprint — 2026-10-26

## Team Lead intake
- User request: finish Phase A (Model Discovery, MCP, Sessions, Migration), Phase B (Multi-Agent, Workflow DAG, Dashboard), Phase C (UI, Performance, Updater, Packaging).
- Scope this change: robust Model Discovery error recovery; remaining A–C work tracked rather than falsely marked complete.
- Branch: `fix/20261026-opencode-discovery-recovery`, base `develop`.
- Worker availability: no remote Mac worker tunnel; assign `TEAM_LEAD_SERIAL`. No fictitious parallel agents.
- Protect session data, credentials, command compatibility, and updater keys; no native distribution from an untested patch.

## Work decomposition
| Task | Worker | Owned area | Status |
| --- | --- | --- | --- |
| A-01 | TEAM_LEAD_SERIAL | OpenCode catalog failed-response recovery in UI | IN PROGRESS |
| A-02 | TEAM_LEAD_SERIAL | MCP native live server/consent/cancellation audit | PENDING |
| A-03 | TEAM_LEAD_SERIAL | Session crash recovery, orphan process cleanup | PENDING |
| A-04 | TEAM_LEAD_SERIAL | Data migration backup/rollback on disposable fixture | PENDING |
| B-01 | TEAM_LEAD_SERIAL | Multi-agent scheduler, ownership, mailbox | PENDING |
| B-02 | TEAM_LEAD_SERIAL | Workflow journal, no-replay, restart | PENDING |
| B-03 | TEAM_LEAD_SERIAL | Agent Dashboard and observer UX | PENDING |
| C-01 | TEAM_LEAD_SERIAL | UI translation, accessibility, design consistency | PENDING |
| C-02 | TEAM_LEAD_SERIAL | Startup and heavy-session perf | PENDING |
| C-03 | TEAM_LEAD_SERIAL | Updater, package signing/notarization and rollback | PENDING |
| QA | TEAM_LEAD_SERIAL | Review actual diff, unit, E2E, smoke and native status per item | PENDING |

## NC-TEAM-003 evidence gate
| Task | Unit/contract | E2E | Smoke | Lead review |
| --- | --- | --- | --- | --- |
| A-01 | PENDING | PENDING | PENDING | PENDING |
| A-02/A-03/A-04 | PENDING | PENDING | PENDING | PENDING |
| B-01/B-02/B-03 | PENDING | PENDING | PENDING | PENDING |
| C-01/C-02/C-03 | PENDING | PENDING | PENDING | PENDING |

- Stop condition: code changes cannot be called complete until tests and Team Lead review pass, with native installed-app E2E named separately.
- Independent human review is required for production and cannot be represented by this assistant.
- Release decision **NO-GO** until all critical gates have evidence.
