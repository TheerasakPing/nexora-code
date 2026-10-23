## Mandatory Nexora Code Team Lead checklist (NC-TEAM-001 / NC-TEAM-002 / NC-TEAM-003)

- **Task Planners:** link EVERY newly added docs/nexora/tasks/YYYY-MM-DD-task-slug.md (one for each request)
- **Team Lead (actual identity):** TBD
- **Real Agent IDs / ownership / worktrees:** TBD (or TEAM_LEAD_SERIAL)
- **Subtasks, dependencies, incomplete work:** TBD (link planner)
- **Test evidence:** exact commit SHA / OS / PASS / FAIL / IGNORED / CI URLs
- **NC-TEAM-003 per-subtask quality:** Unit/Contract, E2E, Smoke (separate per-worker results, commands/OS/logs and documented N/A or BLOCKED)
- **Team Lead Review PER subtask:** actual review of diff, security and the test triad; GO/NO-GO/PENDING for each row
- **Review decision:** PENDING / GO / NO-GO, date and actual reviewer
- [ ] Task planner created before implementation and updated throughout.
- [ ] Real worker changes and shared contracts inspected by Team Lead.
- [ ] Each subtask has a real worker ID or TEAM_LEAD_SERIAL, distinct E2E and Smoke evidence, and explicit Lead Review status.
- [ ] Tests/CI/security reviewed; N/A requires concrete justification and approval; BLOCKED/FAIL is not marked accepted.
- [ ] Defects sent for rework and reverified; pending native gates disclosed.
- [ ] Lead GO recorded before delivery; main/release approvals separately granted.

> Without a current planner and a documented Team Lead GO this change is
> not ready for merge. A worker's claim to have finished is not review.

## What and why

<!-- What does this change, and why? Link the issue: Fixes #123 -->

## How it was tested

<!-- Commands you ran and what you checked by hand. Add screenshots for UI changes. -->

- [ ] `npm run build` (typecheck + bundle)
- [ ] `npm test`
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` (if Rust changed)
- [ ] Relevant Playwright tests in Chrome and WebKit (if UI changed)

## Checklist

- [ ] Commit messages follow `<type>: <description>`
- [ ] Docs in `docs/` updated if behavior changed
- [ ] Preview mocks added in `src/preview.ts` for new IPC commands
- [ ] No telemetry, secrets or personal paths added
