# NC-TEAM-003 — R1 AI review fixture safety follow-up (2026-10-24)

**Base:** PR #38 integration branch a823ab0. **Team Lead:** ChatGPT Integration Lead.
**Actual workers:** QA_REVIEW_AI_1 (read-only Codex CLI) identified three P2 test fixture concerns; TEAM_LEAD_SERIAL performs remediation and review. Human independent review still PENDING.
**Scope:** TEST ONLY storage.rs and terminal.rs. No runtime source logic, real user processes, network, credentials or release config changed.

## B. Checklist Planner — created before code edits
- [x] R1-QAFIX-01 / QA_REVIEW_AI_1 — identify diagnostic findings with file/line; issue: storage fixture child drop on panic can orphan, terminal fixture delayed reap flag may signal recycled PID, 8-PTY test lacks certain races.
- [ ] R1-QAFIX-02 / TEAM_LEAD_SERIAL — test-only scoped RAII Child guard for real OS crash E2E; on panic kill/wait test-owned children, avoid re-signalling already-reaped processes.
- [ ] R1-QAFIX-03 / TEAM_LEAD_SERIAL — terminal fixture mark reap at actual waiter completion with shared atomic, avoid stale PID teardown; add relevant regression checks with bounded waits.
- [ ] R1-QAFIX-04 / TEAM_LEAD_SERIAL — Unit/Contract, real subprocess E2E, smoke, full Mac Rust, Node, frontend, Clippy, Lead review.
- [ ] R1-QAFIX-05 / TEAM_LEAD_SERIAL — push verified patch to PR #38 only after CI installer run identity preserved; exact new HEAD CI still required.
- [ ] R1-QAFIX-REVIEW / INDEPENDENT_REVIEWER_PENDING — human security/native installed review and three-OS acceptance; NO-GO protected merge until completed.

## G. NC-TEAM-003 Verification Matrix
| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Lead Review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| R1-QAFIX-01 | QA_REVIEW_AI_1 | N/A (read-only) | N/A (read-only) | N/A (read-only) | GO diagnosis | Codex read-only review a823ab0; P2 findings in terminal.rs and storage.rs; not human approval |
| R1-QAFIX-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Source patch + actual child kill fixture |
| R1-QAFIX-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Real PTY and process reap tests |
| R1-QAFIX-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Integrated Mac suite and CI |
| R1-QAFIX-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | PR #38 after verified update |
| R1-QAFIX-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No actual human approval |

**NO-GO:** Production and protected merge until exact-head cross-OS CI and independent human review + installed native UI tests.
