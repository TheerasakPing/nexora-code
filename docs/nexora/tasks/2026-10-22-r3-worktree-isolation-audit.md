# R3 Task Checklist — Git Worktree Isolation Audit

**Created:** 2026-10-22, before implementation.
**User goal:** Continue Nexora Code R0–R8 roadmap, prioritizing real R3 worktree isolation without bypassing stability/review gates.
**Task:** R3-ISO-01 · Worktree Isolation Audit and secure source evidence.
**Team Lead:** acting assistant (TEAM_LEAD_SERIAL). No real delegated independent agents/reviewers available.
**Worktree and branch:** nexora-code-r3-isolation-audit; feat/r3-worktree-isolation-audit, based on integration/r3-r4-windows-stabilized PR #14.
**P1 scope:** read-only native Audit API inspecting Architect Worker sessions and verifying registration of linked Git worktrees, per-worker isolation status, conflicts and warnings. Pure unit and temporary Git repo native integration tests; documentation.
**Out of scope:** Create/remove worktrees, launch Agent/GUI/MCP, merge PRs, move user files, credentials, sandbox enforcement, automatic worktree provisioning.
**Critical fact:** Existing worker dispatch copies parent.worktree_path. An independent path string does not prove a real isolated checkout or OS sandbox. This audit must not mark inherited workspace as isolated.
**Dependencies:** R3 File Ownership Guard PR #11, R3 Preview/UI PR #10, and Windows-stabilized integration PR #14.
**Acceptance:** classify shared-lead, shared-worker, verified-linked-worktree, not-verified; compare canonical paths; reject symlinks/unregistered Git metadata; no mutating operations; report honest release blockers. Preserve current scheduling and maxParallel behavior.
**Approval:** User permits source/branch PR, not protected develop/main merge or native app install/release.

| Task | Actual worker | Owned file scope | Done when | Status |
| --- | --- | --- | --- | --- |
| ISO-01 | TEAM_LEAD_SERIAL | src-tauri/src/workspace.rs | Verify Git worktree registration via trusted Git metadata/read-only | [x] |
| ISO-02 | TEAM_LEAD_SERIAL | src-tauri/src/architect/isolation.rs | Read-only per-Team audit, typed statuses and tests | [x] |
| ISO-03 | TEAM_LEAD_SERIAL | src-tauri/src/architect.rs and lib.rs | Wire read-only IPC with no worker scheduling mutations | [x] |
| ISO-04 | TEAM_LEAD_SERIAL | docs/nexora/R3-WORKTREE-ISOLATION-AUDIT.md, src/architect.ts, src/architect.css, src/isolation-audit.ts and tests | Accessible audit UI, docs and safety tests | [x] |
| ISO-QA | TEAM_LEAD_SERIAL | full diff and evidence | Focused and full Rust/Node, Clippy, PR | [x] |
| ISO-REVIEW | INDEPENDENT_REVIEWER_PENDING | GitHub PR review | Required before protected merge | [ ] |

- [x] Planner existed before source changes.
- [x] Positive genuine Git-linked worktree proof and negative spoofed/unregistered Git path tests.
- [x] Shared checkout warnings test and no data/state mutation.
- [x] Full test matrix and strict Clippy PASS; CI pending honestly disclosed.
- [x] Team Lead inspected every actual diff; no fictitious multi-agent review.
- [x] PR opened for review without protected branch merge.
**Team Lead decision:** GO for source-only PR submission after scoped Team Lead self-review; NO-GO for protected branch merge/release until independent reviewer and six required native CI gates.

## Local Team Lead review and evidence — Mac M4, 2026-10-22

- **Actual worker(s):** TEAM_LEAD_SERIAL. No independently spawned coding Agent or Reviewer was available. This is not an independent review.
- **Scope reviewed:** read-only Git registration helper, isolation snapshot/typed native IPC, frontend opt-in markup/escape, async cancellation on panel/task changes, no runtime worker launch or state mutation.
- **Rust audit tests:** 7 PASS including real temp Git worktree and DB-owned Store checks.
- **UI renderer tests:** 5 PASS, hostile strings remain escaped; fallback and stale revision warnings pass.
- **Full Mac Rust:** 1044 PASS / 0 FAIL / 33 Ignored; CLI integration 1 PASS.
- **All-target Clippy -D warnings:** PASS.
- **Node:** 139 PASS / 0 FAIL; release-script 31 PASS / 0 FAIL; production build and R0 safety PASS.
- **Native GUI E2E and OS 3-platform acceptance:** PENDING, not inferred from source tests.
- **Automatic worker-specific worktree provisioning, crash cleanup, filesystem sandbox and merge approval:** NOT IMPLEMENTED by this PR and remain explicit R3 roadmap blockers.
- **Review/CI:** independent GitHub Reviewer and exact-head CI remain PENDING at PR submission.

## Published PR and live CI evidence

- **PR #15:** https://github.com/TheerasakPing/nexora-code/pull/15
- **Source implementation SHA:** dbfcffb1665faf4fed2719d38ebe3be2b8ae5a80
- **Base:** integration/r3-r4-windows-stabilized (PR #14); the existing
  integration PR's six CI checks were SUCCESS when the audit PR was opened.
- **CI on this PR:** GitHub 3-OS checks began running after push; PENDING at
  this record. Previously passing PR #14 cannot substitute for this head SHA.
- **GitHub Reviewer:** PENDING. No self-approval, force merge, protected
  develop/main mutation, native GUI install, real Agent process or OS sandbox.
