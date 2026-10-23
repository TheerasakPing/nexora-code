# Task Checklist Planner — Sequential R1 → R8 Execution

**Date:** 2026-10-23 / User instruction: “ทำไล่ไปให้เสร็จที่ละ R เลย ไล่จาก 1-8”
**Worktree:** /Volumes/Data/workspace/git/nexora-code-integration-20261023, PR #26. Latest accepted exact-head 9/9 CI for 8f8bd40, protected develop not yet merged.
**Lead/Delegation:** TEAM_LEAD_SERIAL because current MCP interface exposes no API to delegate full engineering subtasks to independent agents. No invented agents; no independent reviewer present.
**Rule:** NC-TEAM-001/002/003, each subtask needs Unit/Contract, E2E, Smoke, Team Lead review; no [x] without acceptance. Phase gates remain NO-GO if native or publisher evidence is missing. Strict *phase priority* not a claim of 8 finished phases in one turn.

## B. Team Lead tasks and ordering

| ID | Scope and DoD | Real Worker | Files/Worktree | Depends | State |
| --- | --- | --- | --- | --- | --- |
| SEQ-R1-01 | R1 process group bounded cleanup: no grandchild process leak when parent exits normally, with actual OS process regression | TEAM_LEAD_SERIAL | src-tauri/src/workspace.rs; Rust test | Prior R1 preview security | [ ] |
| SEQ-R1-02 | R1 native PATH/session/provider test audit; validate existing evidence and record missing native gates | TEAM_LEAD_SERIAL | R1 evidence, script/tests as needed | SEQ-R1-01 | [ ] |
| SEQ-R1-03 | R1 full Node/Rust/Contract and cross-OS smoke evidence; exact PR CI and Lead review | TEAM_LEAD_SERIAL | PR #26, this planner | SEQ-R1-01/02 | [ ] |
| SEQ-R2 | R2 MCP vault/approval/connection security — start only once R1 Exit accepted or record blocker | UNASSIGNED_PENDING_R1 | Phase backlog | R1 EXIT | [ ] |
| SEQ-R3 | R3 real scheduler, worktree owner, team mailbox + review | UNASSIGNED_PENDING_R2 | Phase backlog | R2 EXIT | [ ] |
| SEQ-R4 | R4 durable workflow journal, reconcile + no-replay | UNASSIGNED_PENDING_R3 | Phase backlog | R3 EXIT | [ ] |
| SEQ-R5 | R5 UI Studio/monitoring/accessibility/brand | UNASSIGNED_PENDING_R4 | Phase backlog | R4 EXIT | [ ] |
| SEQ-R6 | R6 native installed GUI/provider/PTY smoke all 3 OS | UNASSIGNED_PENDING_R5 | Native targets | R5 EXIT | [ ] |
| SEQ-R7 | R7 audit, 8 workers × 2h and 50 crash/restart cycles × OS | UNASSIGNED_PENDING_R6 | Native targets | R6 EXIT | [ ] |
| SEQ-R8 | R8 Developer ID/notarization, trusted updater, rollback/owner approve | PROJECT_OWNER_PENDING | Production | R7 EXIT | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| SEQ-R1-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Native Rust spawned-shell grandchild kill test pending |
| SEQ-R1-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Prior Finder PATH and runtime restart coverage audit pending |
| SEQ-R1-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Whole package, exact head 3-OS CI pending |
| SEQ-R2 | UNASSIGNED_PENDING_R1 | PENDING | PENDING | PENDING | PENDING | Phase strictly gated by R1 exit |
| SEQ-R3 | UNASSIGNED_PENDING_R2 | PENDING | PENDING | PENDING | PENDING | Phase strictly gated by R2 exit |
| SEQ-R4 | UNASSIGNED_PENDING_R3 | PENDING | PENDING | PENDING | PENDING | Phase strictly gated by R3 exit |
| SEQ-R5 | UNASSIGNED_PENDING_R4 | PENDING | PENDING | PENDING | PENDING | Phase strictly gated by R4 exit |
| SEQ-R6 | UNASSIGNED_PENDING_R5 | PENDING | PENDING | PENDING | PENDING | Real native all OS required |
| SEQ-R7 | UNASSIGNED_PENDING_R6 | PENDING | PENDING | PENDING | PENDING | Soak/crash stress unperformed |
| SEQ-R8 | PROJECT_OWNER_PENDING | PENDING | PENDING | PENDING | PENDING | Developer ID/notarization and production owner approval missing |

## C. Ordered acceptance checklist

- [x] New dated planner created before code edits, Team Lead serial fallback explained
- [ ] R1 process supervision cleanup unit, functional E2E and process smoke PASS
- [ ] R1 Finder/Terminal/session/provider, crash-resume and 3-OS acceptance matrix checked
- [ ] R1 exit GO from genuine reviewer and installed native Mac E2E; no outstanding P0/P1 process/session instability
- [ ] R2 exit GO, then R3, R4, R5, R6, R7, R8 in sequence with independent evidence
- [ ] Production release signing/notarization, owner legal/brand approval and safe updater

## E. Lead initial GO/NO-GO

**Team Lead verdict:** GO limited R1 source/test implementation. NO-GO R1 phase exit, protected develop merge and release absent genuine independent reviewer/native GUI/Provider and crash tests.
**Cross-cutting acceptance baseline:** 21/24 Engineering packages unchecked, 7/7 Branding epics awaiting final acceptance, 61 R0–R8 detailed checks unchecked; overlapping metrics, not progress percentages.
