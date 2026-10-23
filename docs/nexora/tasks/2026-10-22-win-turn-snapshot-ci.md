# Task Checklist — Windows Turn Snapshot CI Regression

- **Request:** Continue Nexora Code development, fix two failing Windows Rust tests on integration PR #7 without bypassing CI/reviews.
- **Team Lead:** acting assistant, TEAM_LEAD_SERIAL. No independently delegated worker agent.
- **Branch/worktree:** fix/ci-windows-turn-snapshot / nexora-code-win-snapshot-ci.
- **Base:** origin/feat/integrate-governance-windows-gates, stacked PR on PR #7.
- **Priority:** P0 Windows CI blocker.
- **Out of scope:** GUI/app install, running MCP/agents, main/develop merge, broad rewriting of runtime scanner, PR #6 and PR #8.
- **Failure evidence:** GitHub Actions 39733118915 Windows job, turn_changes::tests::snapshot_limits_are_partial_without_false_deletions_or_unchanged_dirty_files at line 211 (0 instead of 1); retained_summary_survives_patch_eviction_and_duplicate_ids_roll_back at line 361 (Option None).
- **Root-cause hypothesis:** time-bounded fixture snapshot discovery invokes Git in intentionally non-Git temp directories; slow Windows process creation under parallel tests can exhaust 500 ms scan budget, causing missing file entries/diffs. Verify through adversarial unit tests; no security check relaxation.
- **Acceptance:** tests become reproducible on Windows CI; Git-backed fixtures keep Git semantics, plain fixtures safely bypass unrelated Git subprocess; full Mac Rust/Clippy regression, GitHub 3-OS CI; no test skip or timeout disabled.
- **Team Lead review:** code/test diff, time/permission impact, Git fixture behavior, CI links and source review required; independent approval remains PENDING.

| Task | Worker | Scope | Status |
| --- | --- | --- | --- |
| WTS-01 | TEAM_LEAD_SERIAL | Inspect scanner Git path and root cause | [x] |
| WTS-02 | TEAM_LEAD_SERIAL | Minimal fixture scanner fix / regression tests | [x] |
| WTS-03 | TEAM_LEAD_SERIAL | Unit, full Rust, Clippy: PASS; Windows CI pending | [x] |
| WTS-04 | TEAM_LEAD_SERIAL | Review, docs, PR, actual evidence/GO-NO-GO | [ ] |

- [x] Hypothesis and Git repo vs non-Git directory behavior documented; Windows root cause not claimed proven.
- [x] Unit regression asserts non-Git fallback and Git subdirectory with ignored entries.
- [x] Focused and full Mac source tests and git diff checks pass.
- [ ] Windows/macOS/Linux CI results attached before protected integration.
- [ ] Independent real reviewer approved before merge.

**Team Lead verdict:** GO for scoped source-only PR submission; NO-GO for protected merge until matching Windows CI and an independent reviewer approve.

## Team Lead Review Evidence

- Acting assistant audited both observed Windows failure lines and tracked source.
- This patch modifies only the test-only non-Git inventory fast path,
  regression tests, this task planner, and R1 Windows CI notes.
- Local Mac turn_changes tests: 35 PASS, 0 FAIL.
- Full Rust tests: 1008 PASS / 0 FAIL / 33 ignored; CLI integration: 1 PASS.
- All-target Clippy with -D warnings PASS.
- Exact Windows outcome on this patch is PENDING.
- Team Lead and implementer were the same agent (TEAM_LEAD_SERIAL);
  **no independent reviewer or separate worker** is claimed.
