# Nexora Code — Installed macOS Acceptance and Production Gates

This is an operator checklist for **Nexora Code**, not an authorization to publish a signed release. The baseline engineering integration is `develop` at `6654a3b` (PR #43). The same PR's R6 unsigned installer smoke, run `40261370962`, passed on macOS ARM64, Windows x64 and Linux x64. Those CI checks do not prove a signed, notarized or human-operated native application.

## Read-only local installation smoke

Run on the target Apple Silicon Mac, with the application already open:

```sh
node scripts/release/verify-installed-macos.mjs
```

The verifier checks the app bundle identifier, arm64 executable, running native process and the existing SQLite database with a **read-only** `PRAGMA integrity_check`. It separately reports the Developer ID signature, stapled notarization ticket, native GUI interaction and independent human-review gates. **It never starts or stops processes, opens credentials, or modifies the app, database or settings.** A `localSmokePass: true` means the currently installed native process and database pass these limited checks; it never implies `productionReady: true`.

A safe database snapshot while the app runs must be made via SQLite's online backup API rather than copying the `.sqlite3` alone while a WAL is active. Keep backup files off GitHub, outside the repository and off screenshots. Before upgrade or rollback, verify the backup with `PRAGMA integrity_check` and preserve the original app in a separate path. Never log user session contents, tokens, prompts or keychain entries.

## Native GUI acceptance — still requires human/operator access

- [ ] Enable macOS **System Settings → Privacy & Security → Accessibility** for the approved test runner (and Screen Recording when necessary). The operator must grant this; do not change privacy permissions silently.
- [ ] Open the *installed* `/Applications/Nexora Code.app`; inspect rendered main window, sidebar, project switcher and settings without a white/blank page.
- [ ] Open Settings and switch language; verify Thai labels where supported and no layout clipping.
- [ ] Use **Saver → Ponytail → Lite / Full / Ultra** with a disposable local task; verify the outgoing guidance and ability to switch off. Do not use paid providers without authorization.
- [ ] Discover installed local model/CLI, including OpenCode; verify timeout and actionable error recovery. Confirm no provider login is required for local features.
- [ ] Restart the app and verify a pre-existing safe task plus settings and drafts survived. Do not delete or overwrite any real sessions.
- [ ] Exercise start, cancel, recover and concurrent tasks using disposable data and obtain app-level logs without secrets.
- [ ] Run equivalent manual install/launch/data checks on actual Windows 11 x64 and Linux machines; hosted unsigned installer smoke is not equivalent.

## Distribution acceptance — separate GO/NO-GO

- [ ] Qualified independent reviewer approves functional correctness, persistence, security boundaries and installer provenance on the protected release candidate.
- [ ] Developer ID Application certificate is installed in the signing environment and identity verified (`security find-identity -v -p codesigning`). This must be managed by the Apple Developer account owner, not copied into source or chat.
- [ ] Sign each distributable binary and nested executable; validate with `codesign --verify --deep --strict --verbose=2`.
- [ ] Notarize through the Apple-authorized operator and verify with `xcrun stapler validate` and `spctl --assess --type execute`. Never claim a successful notarization without a confirmed ticket/status.
- [ ] Sign updater artifacts using the existing trusted updater key, publish only to the approved Nexora Code repository, and validate rollback from a clean installation.
- [ ] Obtain owner release approval and record exact signed artifact hashes and native OS evidence. Do not publish a `v*` tag or production update until every applicable gate is accepted.

**As-of 2026-10-26 (verified on target Mac):** Local install, live process and database integrity passed; a SQLite online backup was made and its integrity verified. Zero valid Developer ID identities were present; the installed app had an ad-hoc signature. Native GUI automation was blocked by macOS Accessibility permission. Independent production acceptance was not obtained. Consequently: **local use YES, production distribution NO-GO**.
