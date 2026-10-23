# Task Checklist Planner — R2 operator-approved MCP Tool Execution (opt-in)

**Date / request:** 2026-10-23, user explicitly authorizes enabling MCP tool invocation for actual R2 tests.
**Owner:** ChatGPT Integration Lead, TEAM_LEAD_SERIAL (no engineering worker delegation API or independent reviewer available).
**Source:** PR #32 branch docs/nc-team-002-progress-accountability-20261023, isolated Mac worktree.
**Safety scope:** Opt-in manual tool execution ONLY. No blanket/grant-on-enable, no auto Agent tool access, no unreviewed credentialRef, no shell execution from renderer, no silent browser approvals. Existing native dialog must display exact server, tool and bounded arguments; reject config changes and selected tool absent from advertised catalog; one-time expiring in-memory ticket consumed for exact request, OS group cleanup.
**R2 phase exit:** Need 3-OS actual installed GUI + operator approval E2E, OS secure storage/secret handling, hostile executable/network assessments and real independent security review. Do not mark full phase GO from mock/browser/native unit-only checks.

## B. Decomposition and ownership
| ID | Deliverable / Acceptance | Worker | Files / worktree | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R2-EXEC-01 | Native tool-approval modal with server/tool/arguments, single-use intent ticket; no renderer consent token | TEAM_LEAD_SERIAL | mcp_security, mcp_inspector | prior R2 foundation | [ ] |
| R2-EXEC-02 | Typed tool executor in read-only transports; only explicit call path after verified advertised capability; scoped deadlines/caps | TEAM_LEAD_SERIAL | mcp_http, mcp_stdio, mcp_inspector | R2-EXEC-01 | [ ] |
| R2-EXEC-03 | UI manual tool action with explicit JSON preview, disable during use, escape all returned text | TEAM_LEAD_SERIAL | mcp-center.ts/css | R2-EXEC-02 | [ ] |
| R2-EXEC-04 | Real local stdio + pinned-loopback HTTP tool-call integration and deny/replay tests, browser E2E, Node/Rust smoke | TEAM_LEAD_SERIAL | tests, source | R2-EXEC-01..03 | [ ] |
| R2-EXEC-05 | Team Lead source inspection, PR push and exact-head cross OS CI | TEAM_LEAD_SERIAL | PR/evidence | R2-EXEC-04 | [ ] |
| R2-EXEC-REVIEW | Actual independent security review + real native installed app approval/deny E2E on Mac/Windows/Linux | INDEPENDENT_REVIEWER_PENDING | release acceptance | R2-EXEC-05 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix
| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R2-EXEC-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Native dialog and ticket replay/intent test pending |
| R2-EXEC-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Actual fixture transport calls + security bounds pending |
| R2-EXEC-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Browser UI explicit-confirm tests pending |
| R2-EXEC-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Full Rust/Node/E2E/smoke and logs pending |
| R2-EXEC-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | CI + PR source acceptance pending |
| R2-EXEC-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | Independent security and native GUI tests unavailable |

## C. Checklist
- [x] Fresh Planner written before modifying implementation, real team limitations recorded.
- [ ] Native tool approval requires interactive dialog and displays all user-supplied arguments.
- [ ] One-time scoped ticket consumed exactly once, denied commands never cross transport.
- [ ] UI can manually invoke a listed tool and see bounded escaped result; no automatic execution.
- [ ] Scoped real subprocess + HTTP E2E and full tests PASS.
- [ ] CI current SHA 3-OS passes; Team Lead inspects changed files.
- [ ] Independent review + installed OS acceptance; only then R2 Exit GO.

## E. Team Lead
**Team Lead verdict:** GO for narrow source implementation and PR submission only, pending actual tests. **GO/NO-GO:** R2 production/phase exit NO-GO pending independent reviewer, real native UI/permissions/credential lifecycle and full multi-server security acceptance.
