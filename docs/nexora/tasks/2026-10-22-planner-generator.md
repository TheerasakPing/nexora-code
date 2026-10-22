# Task Checklist — 2026-10-22-planner-generator

- **Request:** Continue Nexora governance workflow by adding a safe new planner CLI.
- **Team Lead:** TEAM_LEAD_SERIAL (no worker delegation used in this task).
- **Acceptance:** CLI creates a unique task planner, preserves template fields, refuses overwrite, validates slug and date, and includes regression tests.
- **Scope:** scripts/nexora/new-task-plan.mjs, tests/new-task-plan.test.mjs, this plan only.
- **Branch:** feat/governance-planner-generator, isolated worktree.
- **Risk:** Low; filesystem writing constrained to docs/nexora/tasks with no overwrites.
- **Dependency:** NC-TEAM-001 on docs/team-lead-planner-gate branch.

## Task assignment and Team Lead gate

- [x] CLI implementation — TEAM_LEAD_SERIAL.
- [x] Unit tests for date/slug/path collision — TEAM_LEAD_SERIAL.
- [x] Team Lead inspect actual diff and run tests.
- [ ] Commit and open PR after GO decision.

**Team Lead verdict:** GO for PR submission after source review; NO-GO for merge to develop/main pending GitHub CI and independent review.

**Verification:** node --test tests/new-task-plan.test.mjs: 4/4 PASS; git diff --check PASS. Reviewed exclusive-create, validation and actual diffs. Source-only test, no GUI or installed application. Independent worker/reviewer not available; TEAM_LEAD_SERIAL. CI/native remain PENDING.
