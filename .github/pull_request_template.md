## Mandatory Nexora Code Team Lead checklist (NC-TEAM-001)

- **Task Planners:** link EVERY newly added docs/nexora/tasks/YYYY-MM-DD-task-slug.md (one for each request)
- **Team Lead (actual identity):** TBD
- **Real Agent IDs / ownership / worktrees:** TBD (or TEAM_LEAD_SERIAL)
- **Subtasks, dependencies, incomplete work:** TBD (link planner)
- **Test evidence:** exact commit SHA / OS / PASS / FAIL / IGNORED / CI URLs
- **Review decision:** PENDING / GO / NO-GO, date and actual reviewer
- [ ] Task planner created before implementation and updated throughout.
- [ ] Real worker changes and shared contracts inspected by Team Lead.
- [ ] Tests/CI/security reviewed; non-applicable checks explained.
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
