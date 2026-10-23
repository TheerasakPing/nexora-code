# Task Checklist Planner — R1 bounded Finder npmrc CLI discovery

**Date:** 2026-10-23. **Request:** “ต่อไปเลย”: keep completing R1 milestones prior to R2–R8 exit.
**Lead/Worker:** TEAM_LEAD_SERIAL (only available engineering worker); no fabricated delegation or independent human reviewer.
**Branch/worktree:** fix/r1-finder-npmrc-bounded-20261023, /Volumes/Data/workspace/git/nexora-code-r1-npmrc-bounded-20261023; stacked on PR #33, which depends on PR #32. Never overwrite other engineering worktrees.
**Scope/threat:** CLI discovery at desktop Finder launch parses HOME/.npmrc synchronously using unbounded read_to_string. Oversized regular file can spike app startup latency/memory; Unix FIFO can block opening forever; non-file and swapped symbolic links can be unsafe. Change solely path discovery, not credential handling.
**Acceptance:** Fast bounded read <=64KiB, reject nonregular files and symlinks; no blocking Unix FIFO; preserve valid prefix= parsing and configured environment precedence and PATH search; fallback to native node prefix; no hidden shell invocation. Windows final handle reparse protections must compile on hosted CI. No mutation to installed user dotfiles.
**Security limits:** Config file may be modified concurrently; only file-handle check prevents loading bytes from a nonregular or symlink target. Parent HOME path is user's control, not security sandbox; no global filesystem TOCTOU fixed.

## B. Decomposition/ownership

| ID | Work | Actual worker | Owned paths | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R1-NPMRC-01 | Add bounded, fail-closed npmrc reader with safe OS open flags | TEAM_LEAD_SERIAL | src-tauri/src/providers/launch.rs | none | [x] |
| R1-NPMRC-02 | Add real OS FIFO and oversized-file tests; valid prefix and fallback assertions | TEAM_LEAD_SERIAL | src-tauri/src/providers/launch.rs tests | R1-NPMRC-01 | [x] |
| R1-NPMRC-03 | Unit/Contract + CLI E2E + Smoke regression, Team Lead source audit | TEAM_LEAD_SERIAL | src-tauri/src/providers/launch.rs, docs/nexora/ | R1-NPMRC-02 | [x] |
| R1-NPMRC-04 | Push stacked PR, obtain exact-head 3-platform CI | TEAM_LEAD_SERIAL | GitHub PR, planner | R1-NPMRC-03 | [ ] |
| R1-NPMRC-REVIEW | independent reviewer, native installed Finder and production acceptance | INDEPENDENT_REVIEWER_PENDING | PR/OS | R1-NPMRC-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-NPMRC-01 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 launch tests 16/16 + normal prefix parse, npm env precedence; 1115 Rust all PASS; Lead source review of bounded 64KiB opener. Windows native CI PENDING |
| R1-NPMRC-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Actual Mac M4 mkfifo no-writer read under 1 second, oversize and symlink bypass blocked; module filesystem E2E 16/16, no user data; Lead reviewed fixture |
| R1-NPMRC-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Cargo full Rust 1115 PASS/0 fail/33 ignored, Node 208 PASS, Release 33 PASS, npm build PASS, Mac cargo check PASS; documented limits in R1-FINDER-NPMRC-BOUND-SECURITY-2026-10-23.md; Lead GO for source PR |
| R1-NPMRC-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | new PR exact HEAD CI pending |
| R1-NPMRC-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | genuine independent review unavailable |

## C. Checklist

- [x] New dated task planner before source change, actual serial worker, file scope, acceptance
- [x] Bounded npmrc read and normal prefix precedence
- [x] Real FIFO/oversized/symlink fixture tests and timeout smoke
- [x] Full Rust/Node and Release validation, Team Lead review documented
- [ ] Exact head Windows/macOS/Linux CI and independent reviewer
- [ ] Real installed Mac Finder -> Node/npx/uvx E2E and R1 full phase exit

## E. Team Lead GO/NO-GO

**Team Lead verdict:** GO for scoped source implementation/tests/PR submission after review. **GO/NO-GO:** GO source work, NO-GO native installed UX, complete R1 exit, protected merge, production release until independent reviewer and native evidence.


## Local verification / Team Lead handoff

- **Actual runner:** macOS M4, Rust stable aarch64-apple-darwin.
- **Unit/Contract:** 16 focused provider launcher tests PASS; existing path and configured prefix behavior retained.
- **Native filesystem E2E (narrow):** mkfifo with no writer and symlink to outside prefix; no IPC or real Finder app clicks. The bounded reader never blocks; unrelated user dotfiles left intact.
- **Smoke:** 1115 Rust lib tests PASS, 33 ignored, 0 failures; 208 Node PASS, 33 Release PASS, Vite build PASS, Mac native cargo check PASS.
- **Team Lead GO for PR** source/test submission after examining exact launch.rs diff. **NO-GO** protected merge without independent reviewer and exact-head CI. **NO-GO** installed GUI/native Finder acceptance or R1 full phase exit.
- **Worker assignments:** actual TEAM_LEAD_SERIAL only; not multiple independent agents.
