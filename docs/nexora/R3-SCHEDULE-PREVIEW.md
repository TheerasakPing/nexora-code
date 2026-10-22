# R3 — Multi-Agent Schedule Preview (Read-only Foundation)

**Date:** 2026-10-22 · **Status:** source-only foundation, not production enabled.
**Upstream:** This is a fork of PandaCode — https://github.com/aicodeking/pandacode.

## What was added

A new architect_preview_schedule native IPC command retrieves a team's saved
TeamSnapshot and projects queued worker waves without changing any session,
task state, tool authority, agent process or database records.

| JSON field | Purpose |
| --- | --- |
| revision | Saved team revision represented by the preview |
| maxParallel | Effective concurrency limit (zero/unlimited expands to team size) |
| activeWorkerIds | Workers currently running or waiting |
| waves | Ordered proposed sets of queued worker IDs |
| blocked | Queued workers with missing, failed or unresolved dependencies |
| note | Explicit caveat that this is a prediction, not execution |

The simulation shares the runtime's architect file-scope conflict policy: two
writers claiming overlapping paths cannot be grouped in the same wave, but
a read-only worker can overlap a writer. Prerequisites must complete in prior
waves; order is stable. Unknown/cyclic dependencies report blocked, no hang.

## Safety and limitations

- No process/network execution, secrets, mutations, native file writes or
  new permissions. Command reads an existing team using the same scoped
  local Tauri IPC system as architect_get.
- Actual scheduler remains source of truth. The preview assumes currently
  active workers eventually finish successfully. Real runs can differ due to
  other teams, changes, failed jobs, permissions and process availability.
- Native IPC is available; Team Dashboard UI is deferred to R5.
- Does NOT provide actual per-agent worktrees/leases, native 3-OS acceptance,
  security approvals, or permission to launch workers.

## Source test commands

    cargo test --manifest-path src-tauri/Cargo.toml --locked architect::preview
    cargo test --manifest-path src-tauri/Cargo.toml --locked
    cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings

Verified locally on Mac M4: 9 focused preview tests PASS; persisted team
nonmutation regression PASS; full Rust unit suite 1016 PASS/0 FAIL/33 ignored,
CLI integration 1 PASS; all-target Clippy PASS.

Windows/Linux CI, GUI/IPC E2E and multi-worker acceptance remain pending.
