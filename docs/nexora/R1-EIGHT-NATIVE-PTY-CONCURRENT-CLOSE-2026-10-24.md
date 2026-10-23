# R1 — Eight real native PTY sessions, concurrent close regression

**Date:** 2026-10-24 Asia/Bangkok. **Phase:** R1, stability/terminal; independent source-review requirement still applies.
**Branch:** test/r1-eight-pty-concurrent-close-20261024 (stacked on PR #34, deliberately isolated from PR #36 Storage crash work).

## Risk / exact integration exercised

Real users can have multiple shell/agent-console PTYs open at once and close them
concurrently. A Terminal shared registry lock, OS signal handling or a blocked
native reader can leave children alive, deadlock a closure or allow input after
close. The existing Rust tests checked one terminal at a time; the new test
introduces eight real PTYs inside one shared registry and simultaneous
close calls to expose cross-session interference.

The test creates eight real /bin/sh children through portable_pty and real
native PTY master/slave handles. Half execute sleep 30; half wait for stdin
after emitting a ready marker. All terminals are moved into a single shared
Terminals map with distinct IDs and PIDs, preserving the production lock.
Eight threads synchronize at a barrier and call the production close()
function concurrently. The test asserts every call responds, every child has
been reaped by the native reader child.wait(), the registry is empty,
and write() fails for every closed terminal. Each operation has a bounded
timeout, so deadlocks become failures. No actual user project, provider,
credential, system config, shell startup file, network or tool is touched.

The fixture also tracks when a PID has been reaped to avoid signalling a
potentially recycled PID in the fixture destructor after a successful close.

## Actual Mac M4 test evidence

- Unit/Contract: targeted Rust eight_shared_pty_sessions_close_concurrently_without_orphans PASS.
- Native OS integration E2E (not installed GUI): targeted test launched 8
  real PTYs and waited for 8 shell children to exit; PASS.
- Repeat/soak fixture: same native Rust test binary 20/20 PASS with eight
  unique shells per run = 160 observed fixture shell processes. Each
  run confirms shared registry cleanup/reaping and post-close input rejected.
  This is neither eight real AI agents nor a two-hour stress test.
- Regression/Smoke: full Rust library 1116 PASS / 0 FAIL / 33 ignored,
  Mac Clippy --all-targets -- -D warnings PASS, Vite production build
  PASS (existing chunk advisory), Node tests 208/208 PASS,
  Release Contract 33/33 PASS.
- Team Lead diff inspection: test-only Rust change to terminal.rs; real
  /bin/sh fixture, 8 barrier synchronization, and safe PID disarm after
  child reaping. No production behavior modified.
- Not covered locally: installed macOS Tauri WKWebView, user-launched Finder
  GUI, real provider/credential/agent resume, Windows ConPTY native E2E,
  8 real independent agents or 50 real app crash/recovery cycles per OS.

## Exact-head gate and governance

The source changes need exact-commit macOS/Linux CI. Windows must compile
the cfg-unix-only fixture and continue passing the normal suite. Do not
declare a platform test complete until its hosted job has passed. Parent
PR #34 and related R1 patch chain remain subject to independent human
review and protected develop merge policy.

**Lead:** TEAM_LEAD_SERIAL; source GO for PR submission only.
**NO-GO:** full R1 phase exit, installed GUI/PTY/provider E2E, production,
R2–R8 phase acceptance and protected merge without independent review.
