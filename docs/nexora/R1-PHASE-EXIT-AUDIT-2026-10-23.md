# Nexora Code R1 phase-exit audit — Sequential R1 → R8

**Audit date:** 2026-10-23. **Team Lead:** TEAM_LEAD_SERIAL (no independent reviewer).
**Baseline:** PR #26 source head 8f8bd40 had 9/9 exact-head CI PASS (macOS arm64, Windows x64, Linux x64 unsigned previews).
**Active package:** `SEQ-R1-01` bounded command descendant cleanup; exact-head CI revalidation required after the new commit.

## R1 Master Checklist by actual acceptance

| R1 Master item | Narrow evidence | Exit status |
| --- | --- | --- |
| Case-variant .git restrictions | source + focused Rust coverage | Source-level completed |
| Mac unsigned engineering preview | DMG checksum, identity, M4 8-second process smoke | Engineering-only |
| Full filesystem symlink/TOCTOU safety | `read_file` and `image_data_url` anchor via OS handles, symlink adversarial tests | PARTIAL — Git operations, listing, caches and checkout replacements still pending |
| Supervised process groups, resource caps, bounded cleanup | `run_bounded` owned process group cleanup on success/timeout, ProcessGroup and Windows Job infrastructure | PARTIAL — native process-tree stress/cancel required |
| Provider/terminal start, stop, pause and resume | source tests and 14 Finder/launch unit fixtures | PENDING native GUI/PTY/provider E2E |
| Durable session checkpoint/recovery, orphan cleanup, crash diagnostics | existing Rust store/provider code + some source tests | PENDING crash-consistency native restart acceptance |
| Long multi-session hangs and stale generations | existing regression guards | PENDING multi-session soak and process recovery proof |
| Finder GUI PATH with Homebrew node/npx/uvx | existing `providers::launch::tests::`: 14/14 PASS on Mac M4 | PARTIAL — a real Finder-launched signed/native app scenario pending |
| Native macOS UI/PTY/provider E2E and crash tests | unsigned native Mac process smoke exists, no full GUI validation | BLOCKED until actual native interactive run with user-approved provider/session |

**Phase exit:** NO-GO. The phase must NOT be ticked complete due to passing Rust Unit, mock E2E or cross-platform installer CI alone.

## This iteration, scope and test triad

- **Bug:** A bounded command that finishes successfully after launching a background subprocess with redirected pipes previously returned while descendants remained alive. Timeout/stream-error paths already called `stop_process_group`, but the success path did not.
- **Fix:** On Unix, signal the *isolated subprocess process group* after the returned child has been reaped and its streams collected; on Windows, rely on the scoped `Job Object` `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`. Do not PID-kill an already reaped Windows child because its PID could be reused.
- **Unit/Contract:** Rust `workspace::tests::successful_bounded_process_reaps_background_process_group`: PASS. Existing timeout/stream fixtures still PASS. New fixture executes a POSIX shell creating a real `sleep 30` child with redirected streams, confirms parent exits 0, then polls native `ps` to ensure the descendant is gone or zombie. On unexpected live child it terminates the test-owned child.
- **E2E (native helper integration):** The test calls actual `run_process` with a real shell and system subprocess on macOS M4. This validates the process boundary, not the full application GUI, agent or user session workflow.
- **Smoke:** `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` **1068 PASS, 0 FAIL, 33 ignored**; `providers::launch::tests::` **14 PASS**, `npm test` **178 PASS**, `npm run test:release` **33 PASS**. Browser-only/Native GUI E2E remains PENDING.
- **Cross-platform CI:** exact-head Actions tests and 3 OS unsigned installers must be green *after push*. Windows Job Object behavior not declared device-accepted based only on Mac Rust.
- **Lead review:** inspected the process-group ownership and Windows PID reuse risk; narrowed the successful-exit cleanup to Unix only and preserved Windows RAII Job Object semantics.
- **Safety:** no actual user projects, keychain, app binaries or preference files changed. Test subprocess restricted to disposable short-lived `sleep` owned by test.

## R1 remaining blockers and order

1. **R1 P0:** Directory listing, Git stage/diff and file search workspace path race/rename hardening, including Windows junctions and verification on each target OS.
2. **R1 P1:** Real provider/terminal sessions, stop/restart/cancel/resume and no orphan process acceptance on an installed Mac .app. Include credentials only with user-approved environment, never logs.
3. **R1 P1:** Crash-consistent session checkpoint and load tests for many concurrent sessions and multiple agent generations.
4. **R1 QA:** Real Finder-launched native GUI PATH + node/npx/uvx and full PTY smoke. Mac previews are unsigned and Gatekeeper-rejected without user-per-app approval.
5. **Governance:** independent human reviewer before merging PR #26 to `develop`; all current-head CI checks green, and later Production signing/notarization still blocked by Apple credentials.

**Sequential dependency:** R2 starts only after R1 exit GO. R2–R8 phases remain queued, not presumed complete. This audit is not a promise of unsupervised background work.


## 2026-10-23 R1 follow-up — Unix directory enumeration

Directory listing anchored file descriptors added in `src-tauri/src/workspace.rs`.
Native Mac Rust workspace tests 33 PASS with 250-cycle adversarial symlink swap.
Full native Rust lib 1074 PASS, Node 186 PASS, Release contract 33 PASS;
Mac binary Cargo check and frontend build PASS. **Windows listing remains
pathname-based**; `file_search` and Git stage/diff remain separate risks.
This is a **partial** source-level P0 improvement only; R1 phase exit NO-GO
pending real Native GUI/provider/resume tests, Windows junction E2E and
independent review. Test commands/evidence: task planner
`2026-10-23-r1-workspace-directory-index-hardening.md`.
