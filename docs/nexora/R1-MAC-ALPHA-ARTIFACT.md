# macOS M4 Engineering Alpha (NOT PRODUCTION)

This is a fork of PandaCode: https://github.com/aicodeking/pandacode.

## Mac M4 test evidence
- Tauri Debug .app creation: PASS.
- LaunchServices app open: PASS (app process started).
- Test app shutdown: PASS.
- Tauri default graphical DMG bundling: BLOCKED in Finder AppleScript.
- Headless DMG packaging: PASS.
- Ad-hoc signing + code resource seal: PASS after re-signing packaged bundle.
- Read-only DMG mount, bundle ID validation, Applications symlink: PASS.
- Source Rust: 984 passed, 0 failed, 33 ignored (+ CLI integration 1 passed).

## Reproduce

Run: bash scripts/nexora/build-mac-alpha-dmg.sh

Output under release/internal-alpha:
Nexora-Code-0.1.0-alpha-macOS-arm64-UNSIGNED.dmg
and the adjacent SHA256 file.

The Tauri bundle retains its legacy Rust executable name pandacode.
Product name and macOS bundle ID are Nexora-specific. Rename the executable
and Rust package deliberately in a separate compatibility-tested PR.

## Production release blockers

This is a Debug build, signed ad-hoc only. It is not Apple Developer ID signed,
not notarized, and not verified on a clean external Mac. It is not approved for
stable channel publication. Windows/Linux acceptance is outstanding.
Use an audited release pipeline and owned signing identities for v1.0.

The original upstream publishing workflow remains disabled until fork-owned
signing keys and publishing policies are established.

## Independent artifact verification

The new read-only verification script, scripts/nexora/verify-mac-alpha-dmg.sh,
checks SHA256, mounts the existing DMG read-only, verifies the ad-hoc code
seal, fork-specific bundle identifier, app version, Apple arm64 executable,
Applications shortcut and absence of the literal upstream update URL in the main
binary. No GitHub upload, signing-key generation or installed-system change occurs.
Result on Apple M4: PASS, 22 October 2026. This is not a substitute for
native Finder UI acceptance or Apple Developer ID notarization.
