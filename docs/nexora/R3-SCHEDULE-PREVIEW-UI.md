# R3 Schedule Preview — Workers panel UI

**Status:** Source-only user interface on top of PR #8 native schedule preview.

## Interaction

- Open the Architect & workers panel, then expand **Schedule preview · read only**.
- The panel explicitly requests `architect_preview_schedule` once on demand. It never invokes dispatch, resume, permission grant or any mutating operation.
- The result shows currently active workers, forecast waves and workers blocked by dependencies. Refresh is a deliberate user action.
- If a later TeamSnapshot changes its revision, the existing forecast is labeled stale. Refresh to recalculate.
- During loading, the button is disabled. Failures appear as text, not injected HTML. Responses from closed or switched sessions are ignored.
- Worker names, IDs, native reason strings and warnings are HTML-escaped before display.

## Acceptance evidence and remaining scope

- Pure rendering tests: active/blocked/waves, stale revision, empty states, hostile markup, missing worker fallback.
- Source TypeScript build, Node tests and release checks must pass on the feature branch.
- No GUI was launched, no live agents were run, and no native OS acceptance is claimed.
- This PR is stacked on `feat/r3-schedule-preview` (PR #8). Merge/deploy require real independent review and GitHub CI.
- Actual scheduler decisions remain authoritative; forecast waves do not implement per-agent worktree leases.
