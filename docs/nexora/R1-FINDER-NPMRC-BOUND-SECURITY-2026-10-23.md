# R1 — Finder npm CLI search bounded config-reader acceptance

**Date:** 2026-10-23. **Phase:** R1 Runtime stability. **Scope:** src-tauri/src/providers/launch.rs only.

## Source bug and user-visible impact

When starting the macOS application through Finder, a GUI process commonly gets
minimal PATH. The provider launcher appends installed binary directories and
checks npm prefix stored in HOME/.npmrc. Formerly std::fs::read_to_string()
opened arbitrary path contents without a byte bound or FIFO protections.
An unusually large config could slow startup/allocate excess memory, while a
FIFO named .npmrc could stall the GUI thread waiting for a writer.

This fix does not execute npm, shell startup files or credentials, and does not
modify the user's .npmrc. It reads a **maximum 64 KiB** regular config file and
falls back to native executable discovery when a config is invalid.

## Implementation and security boundary

- Unix: open npmrc with O_NONBLOCK | O_NOFOLLOW | O_CLOEXEC; a named pipe
  cannot block and symlinks are rejected (including a symlink to a safe
  regular config file). Handle metadata must be a regular file; files above
  64 KiB are ignored; reads are bounded to 64 KiB + 1 and must decode UTF-8.
- Windows: use FILE_FLAG_OPEN_REPARSE_POINT on the actual handle and inspect
  FILE_ATTRIBUTE_REPARSE_POINT to reject symlink/junction final targets;
  otherwise identical file-type, size and UTF-8 guards. Actual Windows
  compiled/native smoke validation is required on exact-head CI.
- Environment NPM_CONFIG_PREFIX takes precedence over the file. Ordinary
  prefix=~/.npm-packages remains supported. Existing inherited PATH order
  and locations /opt/homebrew/bin, /usr/local/bin and .local/bin stay intact.
- No claim of OS-wide filesystem sandboxing; HOME parent directories, external
  binaries and installed Finder GUI user-clicks are separate trust boundaries.

## Test triad — actual evidence

- Unit / Contract (actual Mac M4 Rust): providers::launch::tests:: **16 PASS**;
  includes valid npm prefix parsing, NPM_CONFIG_PREFIX precedence,
  oversized config ignored, directory ignored and minimal-GUI-PATH fixture.
- E2E (native module-level filesystem): test creates real mkfifo HOME/.npmrc
  without a writer; bounded config read returns promptly in under 1s;
  symlink to a fake outside prefix rejected; CLI search retains default
  Homebrew path; no arbitrary config prefix trusted. No installed GUI E2E.
- Smoke: cargo test --locked --manifest-path src-tauri/Cargo.toml --lib
  **1115 PASS, 0 FAIL, 33 ignored**. npm run build PASS (existing
  Vite large-chunk advisory), npm test **208/208 PASS**, npm run
  test:release **33/33 PASS**, native Mac cargo check --bin nexora-code PASS.
- Team Lead source review: only a new bounded reader and fixture tests touch
  launch.rs. Temp test files only; no real Homebrew/npm config, user
  settings or secrets changed.

## Outstanding gates

- Current-head hosted 3-OS CI and artifact checks (not inherited from parent).
- Real Finder-launched installed Mac .app UI/PTY/provider smoke, and Windows
  native installed CLI discovery. Tests are integration fixtures, not those
  native installed scenarios.
- Compatibility: symlinked HOME/.npmrc files are intentionally ignored;
  users may use NPM_CONFIG_PREFIX if they intentionally manage such configs.
- No independent human reviewer; do not merge this stacked PR before actual
  review. R1 full phase still requires end-to-end provider crash/restart,
  multi-session stability, and all relevant workspace race security.
- R2–R8 gates remain independent and cannot be claimed complete from R1 tests.

**Lead verdict:** GO for narrow source PR, NO-GO R1 exit or production release.
