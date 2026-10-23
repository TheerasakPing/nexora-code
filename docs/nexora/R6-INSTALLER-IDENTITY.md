# R6 — Provisional Nexora Code Installer Identity and Unsigned Smoke Builds

**Date:** 2026-10-22 · **Status:** source-ready for CI validation, not an approved Production Release
**Brand:** Nexora Code remains provisional, subject to Project Owner confirmation and trademark/legal checks.
**Upstream attribution:** This product forks PandaCode (https://github.com/aicodeking/pandacode).

## Fixed source mismatch

Tauri declares productName = "Nexora Code" and bundle identifier
com.theerasakping.nexoracode. The old release scripts still selected
PandaCode versioned DMG / NSIS / AppImage filenames and verified
PandaCode.app inside a DMG. These expectations were incompatible with the
declared Tauri product identity and could stop packaging even when Rust and
frontend unit tests passed.

The release contracts now expect versioned Tauri bundles for the current
provisional productName. Stable output asset labels are:

- macOS ARM64: NexoraCode-macOS-arm64.dmg
- Windows x64: NexoraCode-Windows-x64-setup.exe
- Linux x64: NexoraCode-Linux-x86_64.AppImage

The builder derives the exact macOS application directory from validated
Tauri productName, instead of assuming a PandaCode.app. macOS window title
in the platform-specific config also matches the provisional product name.
The default updater manifest repository is the fork's repository, not an
upstream vendor. No updater or GitHub Release was enabled by this change.

The compiled Rust executable is still named pandacode. This is deliberate
for compatibility: renaming it now could affect private state, helper paths,
on-disk data and Windows startup smoke. A controlled executable/data/secure-
store migration requires a separate BR-05 design and native E2E signoff.

## Optional GitHub-hosted Alpha Build

The path-filtered workflow .github/workflows/nexora-r6-alpha.yml attempts
native unsigned packaging on ephemeral macOS ARM64, Windows x64 and Linux
x64 runners. No signing credentials, production publisher identity or
persistent checkout credentials are provided.

- Builds with the existing bounded release scripts and fork/update checks.
- macOS packaging verifies an ad-hoc app signature and the DMG contents.
- Windows and Linux smoke install/start on disposable hosted runners with
  isolated test HOME and bounded process lifetimes.
- Uploads unsigned, short-retention Actions artifacts and SHA256 sidecars,
  clearly prefixed UNSIGNED-TEST-ONLY; does not create a GitHub Release,
  publish an updater manifest, install on the user's device or deploy anything.
- The workflow may only report PASS after three real native package builds
  finish on the exact head SHA; adding this file does not prove installers
  were actually built.

## Explicit release NO-GO gates

R6 native packages need real device GUI testing for fonts, menus, sessions,
terminal, provider/MCP integration, worktree writes and preferences, plus
coexistence and migrations alongside the original PandaCode app. Windows
WebView2, Linux AppImage EGL/Wayland and macOS Gatekeeper/notarization
acceptance must be verified on actual end-user machines.

R8 still requires an approved brand/legal review, a fork-owned trusted
updater, signed/notarized installers, independent code review, migration
and rollback tests, 3-OS reliability evidence and owner signoff. Unsigned
PR artifacts are engineering experiments, not production downloads.
