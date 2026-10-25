# R1 — Bound provider IPC when process exits but child retains output stream

**2026-10-23.** Applies to Rust provider session lifecycle (Mac/Linux/Windows).
**Scope:** `src-tauri/src/providers.rs`, not all R1 phase gates.

## Reproduced failure mode

The provider leader process can exit but spawn a helper/background descendant
that inherits the provider's stdout. Even though the leader has ended, the pipe
remains open and `stdout.read()` does not return EOF. After initialization,
the old select loop had no parent-exit detection and could wait indefinitely
for an output frame, leaving an Agent session appearing busy.

## Fix and boundaries

- Poll the owned child status every 200 ms, avoiding blocking reads.
- If the provider leader exited, allow 1 second to drain buffered completed
  frames before reporting a specific diagnostic. Existing run cleanup then
  calls `finish_child`, stops the owned process group and clears outstanding
  host-tool jobs and permissions. User Stop still returns success when stopped.
- If an agent is alive and legitimately silent, no new response deadline is
  imposed. The watchdog only triggers after the child **has exited**.
- A separate process tree escaped into a new session/process group remains
  outside this group's kill rights; R1 requires further OS process containment
  and full native GUI/provider crash/recovery E2E.
- No provider credential, live network, installed app, Keychain or user
  workspace touched.

## Test triad

**Unit/contract:** Node static guard confirms the active watchdog, correct
grace window and existing final cleanup; Rust provider test verifies the error
case. **Scoped native E2E:** real macOS `sh` provider-like process forks a
`sleep` child inheriting stdout/stderr, parent exits, and test requires the
agent run to end within five seconds, with the background child no longer
running. **Smoke:** full Rust `--lib`, Node, Vite, Release checks and
exact-head 3-OS GitHub CI. A mock-protocol process E2E is *not* installed GUI,
provider API login or session resume E2E.

## Remaining R1 exit gates

Durable multi-session checkpoints/crash-recovery, terminal PTY stop/resume in
installed Mac GUI, provider login turns, cross-OS native child-tree cleanup,
workspace stage/diff TOCTOU, Finder PATH live verification, plus an independent
reviewer. Keep R1 Exit **NO-GO** until this acceptance evidence exists.
