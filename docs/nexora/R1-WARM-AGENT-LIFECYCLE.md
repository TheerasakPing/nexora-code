# R1 — Warm Agent Disposal and Late-Spawn Race

**Date:** 2026-10-22 · **Status:** source regression hardening
**Origin:** This is a fork of PandaCode — https://github.com/aicodeking/pandacode.

## Failure mode

Prewarmed Claude Code processes are created asynchronously to reduce startup
latency between turns. Previous discard(session_id) removed only an already
cached spare. If a background prewarm was still starting when the session was
deleted, invalidated or reconfigured, that older task could later cache its
new child process because the launch generation had not changed.

This is a stale-process lifetime bug, not proof of the wider long-session
hang root cause. Process supervision, process-tree cleanup and application
crash recovery remain separate R1 acceptance requirements.

## Fix

- discard() now increments the launch generation before clearing the cache.
- Generation comparison and insertion are performed together using the
  same lock order: LAUNCHES, then SPARES.
- A late child from an invalidated generation is dropped, preventing its
  entry into the warm-agent cache.
- Existing idle expiry and cache capacity logic are preserved.
- A new Unix regression test models stale prewarm completion, current
  generation caching and a second discard in one sequence.

## Source verification (Apple Silicon Mac)

- New focused lifecycle test: PASS.
- Rust full suite: 1007 passed, 0 failed, 33 ignored.
- Additional Rust CLI integration: 1 passed, 0 failed.
- Rust all-target Clippy with -D warnings: PASS.

No GUI, app installation or real MCP connection was used. Windows and Linux
build/tests must independently confirm this change; native GUI/session
acceptance remains pending.
