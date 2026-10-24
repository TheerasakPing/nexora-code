# NC-TEAM-003 — R1 usable alpha integration (2026-10-24)

**Intent:** Produce test-only installable Nexora Code alpha while preserving protected develop/main gates.
**Team Lead:** ChatGPT Integration Lead; actual worker TEAM_LEAD_SERIAL. An independent reviewer has NOT signed off.
**Worktree:** `integration/r1-usable-preview-20261024`, created from PR #37 head `81c0884`.
**Inputs:** PR #32 includes R1+R2 baseline; #33 real OS crash; #34 Finder bounds; #37 native 8 PTYs; sibling #36 multi-session crash & atomic ready marker.
**Scope:** Merge sibling #36 into this isolated branch, prove combined unit/E2E/smoke, open integration PR into develop, dispatch unsigned installer CI on exact integration SHA. Never push main/develop directly or claim production-grade.
**DoD:** No lost sibling patch, no hidden conflicts; native exact-head cross OS CI; unsigned Mac ARM64 DMG, Windows 11 x64 installer and Linux x64 artifact tested per CI; links and checksum instructions for alpha only.

## B. Checklist Planner
- [x] R1-INT-01 / TEAM_LEAD_SERIAL — inspect current PR state & CI; both #36 and #37 exact-head CI PASS; 0 independent reviews.
- [ ] R1-INT-02 / TEAM_LEAD_SERIAL — merge branch #36 into branch #37-derived integration worktree; diff/ancestry audit and no overwrite.
- [ ] R1-INT-03 / TEAM_LEAD_SERIAL — Unit/Contract; native child process E2E for both storage and PTY, Smoke (Rust/Node/frontend/Clippy); Team Lead source review.
- [ ] R1-INT-04 / TEAM_LEAD_SERIAL — push isolated integration branch and create PR to develop; check exact-head hosted multi-OS CI.
- [ ] R1-INT-05 / TEAM_LEAD_SERIAL — workflow_dispatch test-only unsigned installers on exact branch; verify build jobs and downloadable artifacts; installed acceptance as documented.
- [ ] R1-INT-06 / INDEPENDENT_REVIEWER_PENDING — external code/security review and installed native GUI/provider/PTY soak before protected merge/production release.

## G. NC-TEAM-003 Subtask Verification Matrix
| ID | Actual worker | Unit/Contract | E2E | Smoke | Lead Review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| R1-INT-01 | TEAM_LEAD_SERIAL | N/A (read-only CI audit) | N/A (read-only CI audit) | PASS (prior exact head) | GO for diagnostic | PR #36/#37 cross-platform CI success, approval count zero |
| R1-INT-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Git merge and ancestry audit |
| R1-INT-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | New integration SHA local verification |
| R1-INT-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | New integration PR and hosted CI |
| R1-INT-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Test-only unsigned installer job & artifact |
| R1-INT-06 | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | Human/security and installed native acceptance |

**NO-GO:** protected main/develop merge, signed release and full R1 exit without independent review + installed cross-OS acceptance. Existing PRs remain open unless individually and safely reconciled.
