# NC-TEAM-003 — R1 AI review fixture safety follow-up (2026-10-24)

**Base:** PR #38 integration branch a823ab0. **Team Lead:** ChatGPT Integration Lead.
**Actual workers:** QA_REVIEW_AI_1 (read-only Codex CLI) identified three P2 test fixture concerns; TEAM_LEAD_SERIAL performs remediation and review. Human independent review still PENDING.
**Scope:** TEST ONLY storage.rs and terminal.rs. No runtime source logic, real user processes, network, credentials or release config changed.

## B. Checklist Planner — created before code edits
- [x] R1-QAFIX-01 / QA_REVIEW_AI_1 — identify diagnostic findings with file/line; issue: storage fixture child drop on panic can orphan, terminal fixture delayed reap flag may signal recycled PID, 8-PTY test lacks certain races.
- [x] R1-QAFIX-02 / TEAM_LEAD_SERIAL — scoped test-only RAII Child guard on both crash fixtures: owned subprocess killed/reaped on unwind; already-reaped child never re-signalled. Native crash E2E 5 PASS.
- [x] R1-QAFIX-03 / TEAM_LEAD_SERIAL — TestShell teardown uses owned Terminal registry close, never raw PID; waiter marks closed/reaped on OS wait; new concurrent native write-vs-close regression, existing 8 PTY E2E PASS. This does NOT cover installed production reader GUI.
- [x] R1-QAFIX-04 / TEAM_LEAD_SERIAL — Mac M4 Rust full 1120 PASS/0 FAIL/33 ignored, Clippy all targets -D warnings PASS, R0 PASS, Node 208 PASS, release contracts 33 PASS, frontend build PASS; Lead source review GO for test-only patch.
- [ ] R1-QAFIX-05 / TEAM_LEAD_SERIAL — push verified patch to PR #38 only after CI installer run identity preserved; exact new HEAD CI still required.
- [ ] R1-QAFIX-REVIEW / INDEPENDENT_REVIEWER_PENDING — human security/native installed review and three-OS acceptance; NO-GO protected merge until completed.

## G. NC-TEAM-003 Verification Matrix
| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Lead Review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| R1-QAFIX-01 | QA_REVIEW_AI_1 | N/A (read-only) | N/A (read-only) | N/A (read-only) | GO diagnosis | Codex read-only review a823ab0; P2 findings in terminal.rs and storage.rs; not human approval |
| R1-QAFIX-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO source | macOS focused 5/5 actual child-kill/storage recovery tests, full Rust 1120/1120; unwinding guard test-only code reviewed |
| R1-QAFIX-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO source | Native 8-PTY test PASS, new concurrent write-vs-close test PASS in full suite, no raw PID fixture destructor |
| R1-QAFIX-04 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO source | Mac full Rust 1120 pass/33 ignored, Clippy PASS, Node 208, Release 33, Vite build/R0 PASS; hosted exact head PENDING |
| R1-QAFIX-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | PR #38 after verified update |
| R1-QAFIX-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No actual human approval |

**NO-GO:** Production and protected merge until exact-head cross-OS CI and independent human review + installed native UI tests.
