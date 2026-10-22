# Task Checklist — Linux Rust CI turn-change retention fix

**Date:** 2026-10-22, created before new source edits.
**Request:** Continue Nexora Code integration and fix observed CI failure.
**Priority:** P0 reliability / Linux CI, existing PR #23.
**Team Lead:** acting assistant, TEAM_LEAD_SERIAL; no independent worker/reviewer available.
**Branch:** fix/r4-approval-all-paths, existing owned branch as PR #23, isolated worktree.
**Evidence:** GitHub CI 39774220932 / job 119809288440; test turn_changes::tests::retained_summary_survives_patch_eviction_and_duplicate_ids_roll_back failed at src/turn_changes/tests.rs:394, expected oldest capture evicted but patch remained.
**Cause:** Eviction sorts by (created_at,id); second-resolution timestamp may tie; UUID order is not chronological.
**Design:** Evict oldest diff by monotonic auto-increment SQLite message_id and use deterministic timestamp skew regression; no weakened tests.
**Safety:** Preserve transaction, summaries, 128 MiB cap. No migrations or user file deletion.

| Task | Actual worker | Ownership | Verification | State |
| --- | --- | --- | --- | --- |
| LINUX-CI-01 | TEAM_LEAD_SERIAL | src-tauri/src/turn_changes.rs | oldest-first diff eviction | [x] |
| LINUX-CI-02 | TEAM_LEAD_SERIAL | src-tauri/src/turn_changes/tests.rs | skewed timestamp regression added, native runner still pending | [x] |
| LINUX-CI-QA | TEAM_LEAD_SERIAL | full diff | Rust, Clippy, Node, GitHub six checks | [ ] |
| LINUX-CI-REVIEW | independent reviewer pending | GitHub | protected merge gate | [ ] |

- [x] Planner created before source edit.
- [ ] Failing regression proven and repaired without ignore.
- [ ] Full Rust and Clippy pass, ignored tests disclosed.
- [ ] Exact SHA GitHub CI PASS on all six checks.
- [ ] Independent reviewer approval required before protected merge.

**Team Lead verdict:** GO for CI validation PR update, NO-GO for protected merge/release pending final checks and independent approval.

## Evidence and remaining risks

- Confirmed exact failure from GitHub run 39774220932 / job 119809288440: retention test panicked on expecting first evicted patch after two captures. This failure was **not** in the R4 approval logic.
- SQLite deterministic reproduction confirmed old sort (created_at, random UUID) chose message_id=2 before message_id=1 under clock skew. New monotonic message_id sort chose [1, 2]. Capture regression forces skew in first timestamp.
- Fixed native query preserves 128 MiB retention bound, transaction, summary rows and existing database schema.
- Mac M4 Rust test rebuild was observed running abnormally long; our own hung cargo/rustc and npm builds were terminated to avoid blocking the user's workstation. **The new patched code has NOT yet passed full local Rust/Node tests.** Do not cite previous Mac test passes for the new commit.
- Exact-head GitHub Linux/macOS/Windows CI is REQUIRED before declaring acceptance; one genuine independent reviewer is still required before protected merge.
- Reviewer should check semantics of oldest capture by monotonic autoincrement messages across sessions and the focused timestamp-skew test.
