# Task Checklist — 2026-10-22-r3-schedule-preview-ui

- **Request:** Continue developing Nexora Code while governance integration waits for a legitimate reviewer; make R3 Schedule Preview visible and useful in the workers dashboard.
- **Created before code edits:** 2026-10-22.
- **Team Lead:** acting assistant / Integration Lead, TEAM_LEAD_SERIAL. No separately delegated Agent Workers.
- **Priority / stage:** R3/R5, P1.
- **Base:** origin/feat/r3-schedule-preview (PR #8), not main/develop.
- **Branch/worktree:** feat/r3-schedule-preview-ui / nexora-code-r3-schedule-preview-ui.
- **Scope:** Read-only Schedule Preview UI/TypeScript model/HTML, styling and unit tests. Do not edit Rust scheduler or PR #8's native API.
- **Dependency:** architect_preview_schedule IPC from PR #8; Windows turn_changes CI is being addressed in a separate worktree.
- **Risk:** Async replies may become stale on task changes; preview could be mistaken for execution authority; never render untrusted text as raw HTML.
- **DoD:** accessible opt-in preview, manual refresh, clear hypothetical label and stale-revision warning, escaped strings, no worker launches or state writes, loading/error UX, unit/UI/production build tests.
- **Out of scope:** live GUI launch, real MCP, installers, agent spawning, branch merge, protected-branch review bypass or production release.

| Task | Actual worker | Owned files and output | Acceptance | State |
| --- | --- | --- | --- | --- |
| UI-01 | TEAM_LEAD_SERIAL | src/schedule-preview.ts + tests | Safe deterministic HTML rendering with XSS tests | [x] |
| UI-02 | TEAM_LEAD_SERIAL | src/architect.ts and src/architect-types.ts | Read-only dialog + revision/race handling | [x] |
| UI-03 | TEAM_LEAD_SERIAL | src/architect.css + documentation | Accessible layout, responsive styles and docs | [x] |
| QA-01 | TEAM_LEAD_SERIAL | whole diff read-only review | Tests, Build, Git diff and PR handoff | [x] |
| REVIEW | Independent Reviewer pending | Actual GitHub PR review | Required prior to protected merge | [ ] |

- [x] Planner created before editing source; actual ownership and dependency recorded.
- [x] Team Lead inspects escaping, error paths and asynchronous state/focus.
- [x] UI tests, npm tests, build and release scripts pass, ignored counts disclosed.
- [x] Source changes do not trigger real agents or alter backend scheduler.
- [ ] PR opened and CI evidence recorded without claiming release acceptance.
- [ ] Independent review and cross-platform CI pass before protected merge.

**Team Lead verdict:** GO for scoped source PR submission after self-review, NO-GO for protected-branch merge/release without independent reviewer and native CI.

## Team Lead source acceptance (2026-10-22; Mac M4)

- **Actual worker:** TEAM_LEAD_SERIAL. No delegate or independent reviewer was available. Review performed by acting Team Lead, not passed off as independent approval.
- **Code review:** inspected src/architect.ts, src/architect.css, src/schedule-preview.ts and tests. Panel opt-in, IPC is read-only architect_preview_schedule, refresh returns stale warning if the team's revision changed, and responses after closing/switching the panel are discarded.
- **HTML safety:** tested escaping native reason/note and worker-provided names/IDs; error path uses textContent.
- **Performance/accessibility:** avoid re-painting identical forecast after every team event; loading/error status and manual refresh provided.
- **Focused tests:** 5 PASS / 0 FAIL.
- **Full Node unit suite:** 115 PASS / 0 FAIL.
- **Release-script tests:** 31 PASS / 0 FAIL.
- **TypeScript/Vite production build:** PASS with a pre-existing chunk-size advisory.
- **R0 identity/fork/updater safety check:** PASS.
- **Git diff validation:** PASS.
- **Native Windows/Linux/macOS GUI E2E:** PENDING; neither app installation nor live agent invocation was performed.
- **Review / merge:** PENDING actual GitHub CI and authorized independent approval. This work is stacked on PR #8 and does not touch develop or main.
