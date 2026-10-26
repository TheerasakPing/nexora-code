# Nexora Code PR #44 — macOS shard 4 New Task readiness race

## Evidence
- Shard 4 attempt 1 failed on `applies a delayed project switch` immediately after `page.reload(); new-task-button.click()`: expected New Task heading absent, prior conversation remained. Local repeat 3/3 PASS indicated timing-dependent failure.
- Rerun attempt 2 failed instead on `changes the empty task project` and `ignores canceled task folder selection`: attached Playwright DOM snapshots show old conversation still active after clicking New Task. Same entrypoint, different cases.
- Root cause: initial shell renders New Task button while `boot()` is still asynchronous; `state.projects` and `state.providers` are initially empty. `createTask()` handles an early click with `if (!active) return`, silently losing intent.

## Checklist planner / review
- [x] Lead: inspect original and rerun GitHub logs and snapshot evidence. Only code-level root cause within app bootstrap, not arbitrary test sleep.
- [x] W1 UI: initially disable New Task before boot; enable after projects/providers/session hydration and render are finished. Do not change existing click/stop handler.
- [x] W2 Browser E2E: inject controlled pending `list_projects`, assert New Task disabled while boot pending, release promise, assert button enabled and New Task heading visible. No actual provider credentials.
- [x] Focused delayed boot E2E repeat 3/3 PASS on Mac.
- [ ] Wider targeted project/draft E2E, frontend/Node/Release tests pending.
- [ ] Independent review, GitHub macOS full shard 4 and cross-OS CI; no merge into develop without passing gates.

## Mac verification after race fix
- [x] Targeted affected New Task, project switch, delayed switch and both canceled folder operations E2E: **15/15 PASS** (`--repeat-each=3 --workers=2`).
- [x] Frontend `npm run build`: PASS.
- [x] Node unit `npm test`: **210/210 PASS**.
- [ ] Source change goes to existing stacked PR #44; fresh exact-head CI pending and human acceptance not granted.
