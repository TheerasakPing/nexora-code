# R1 — Real OS process kill / SQLite restart acceptance (2026-10-23)

**Scope:** R1 storage startup recovery and in-flight tool *audit state*, source-level integration on Mac M4. Not an installed desktop GUI or an external agent/provider, not physical host power-loss.

## Actual test mechanism

The new Rust test in src-tauri/src/storage.rs launches a **separate copy of the native test executable**, opens a unique on-disk SQLite Store in WAL mode, creates a disposable project, and commits an active session with its first prompt, one RUNNING tool event and one COMPLETED tool event. After the child writes its unique ready marker, the parent confirms the child is alive and terminates **that child** with std::process::Child::kill (POSIX SIGKILL on macOS/Linux, TerminateProcess on Windows).

The parent awaits OS child termination before re-opening the exact same database, twice per cycle. It asserts:

- 10 actual OS kill/restart cycles; alternating RUNNING and WAITING session statuses, without any network or user project activity.
- Session becomes INTERRUPTED and stage remains IN_PROGRESS; no silent completion, retry, or side effect replay.
- Exactly one saved user turn and the two original tool records remain; no duplicated prompt/tool messages.
- The in-flight tool becomes FAILED and retains prior partial output with one interruption notice; another re-open does not append a second notice.
- Previously completed tool activity stays COMPLETED with unchanged output.

This is *not* just dropping a SQLite connection inside one long-lived process: the original process remains killed while the second process opens the database. No production user credentials, sessions, or external tools are exercised.

## Evidence

- macOS M4 real OS crash E2E (storage module integration): cargo test --locked --manifest-path src-tauri/Cargo.toml --lib r1_actual_os_kill_recovers_sqlite_session_and_never_replays_tool -- --nocapture: **1 PASS / 10 SIGKILL cycles**, two DB opens each cycle, 0 failures.
- Full native Rust library: **1113 PASS, 0 FAIL, 33 ignored**, cargo test --locked --manifest-path src-tauri/Cargo.toml --lib --quiet.
- Node contract smoke: **208 PASS, 0 FAIL** after npm run build (fresh worktree needed dist). Vite production build PASS (existing chunk-size advisory). Release contracts **33 PASS**.
- Team Lead source review: fixture child remains alive with real DB before SIGKILL, signals ready only after tool/session writes, parent limits child startup to 12 seconds, kills only the spawned child, waits for exit, uses unique temporary fixture directories. No production code behavior changed; this adds a regression acceptance test.
- Hosted Windows and Linux native test runs **PENDING** on the exact new PR head. No claim of a real Windows 11 installed-app test.

## R1 exit and next phase policy

**R1 phase exit NO-GO** until full TOCTOU path coverage (not merely file preview/list/search), installed Mac GUI/PTY/provider start-stop-resume/crash, many-session stress, and real owner-approved QA; *test process* recovery does not prove an installed app will recover its providers.

**Protected merge NO-GO** until actual independent reviewer and exact-head CI. PR #32 remains open with staged R2 capabilities; this R1 test branch is based on that PR head to avoid unrelated code overwrites, and must be rebased/retargeted after its parent PR merges. No R2-R8 phase is declared done.
