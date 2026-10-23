# macOS Installer: "This Mac could not verify the app" (Apple Silicon)

**Observed 2026-10-23 on Mac mini M4 / macOS 27.2.**
This is a distribution-signing failure for the Nexora Code engineering Preview,
**not proof of a CPU incompatibility or corrupted DMG**.

## Exact verified diagnosis

Against the GitHub Actions test artifact
`Nexora-Code-0.1.0-macOS-arm64-PREVIEW-UNNOTARIZED.dmg`:

| Check | Actual result |
| --- | --- |
| `shasum -a 256 -c <file>.sha256` | PASS |
| `hdiutil verify`, mount, ARM64 identity | PASS |
| `codesign --verify --deep --strict` | PASS (bundle seal only) |
| `codesign -dv --verbose=4` | `Signature=adhoc`, `TeamIdentifier=not set` |
| `spctl --assess --type execute --verbose=4` | **REJECTED** |
| `xcrun stapler validate` | No notarization ticket (exit 65) |
| `spctl --status` | assessments enabled |
| Valid Developer ID code-signing identities | 0 present |

The earlier Mac acceptance verified the binary and checksum but **did not
exercise Gatekeeper**. Therefore a pass in `codesign --verify` cannot be
reported as "installation passes Gatekeeper" or "publicly installable".

## Safe owner-only Preview trial (manual, never automatic)

1. Verify that the archive was obtained from the correct repository and
   check the DMG's SHA-256 against the sidecar; do not trust an arbitrary copy.
2. Open the DMG in Finder. **Drag `Nexora Code.app` to Applications** (do not
   attempt to run directly from the mounted read-only DMG). Do not overwrite
   an existing application or its data without backup.
3. Try opening `Nexora Code.app` from Applications once.
4. If macOS blocks it, go to **System Settings → Privacy & Security → Security
   → Open Anyway**, available for approximately one hour after the blocked
   attempt. Confirm only if you intentionally accept the Preview security risk.
5. If no "Open Anyway" entry is shown, retry launching the copied app from
   Applications to prompt a new app-specific entry. If the Mac is managed by
   an organization, follow administrator policy rather than bypassing it.

Apple support (macOS 27):
https://support.apple.com/en-ie/guide/mac-help/mh40617/27/mac/27

**Never use** `sudo spctl --master-disable`, global Gatekeeper disablement,
recursive `xattr -cr`, automatic removal of `com.apple.quarantine`,
or silent user-consent simulation. Doing any of these does not magically
notarize the app and is not an acceptable shipping workaround.

## Actual fix required for normal public installation

The release owner must provision a paid Apple Developer Program account,
a valid **Developer ID Application** signing identity and a permitted
Apple notarization credential. Use a fully reproducible build, hardened
runtime, Developer ID signature, `xcrun notarytool submit --wait` for an
approved submission and `xcrun stapler staple` the DMG or app, then verify
both `spctl --assess` and stapler on an **untrusted quarantined download**
on a clean Mac M4. The existing production-release script has initial
signing/notarization support but must pass this real device gate and
independent review before any stable channel is published.

Apple documentation:
- https://developer.apple.com/developer-id/
- https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution

## Team Lead acceptance and backlog (2026-10-23)

- **GO:** verified preview disk image, checksum and native process smoke;
  manual informed owner-only trial via Apple Privacy & Security UI.
- **NO-GO:** one-click public installer, accepted Gatekeeper, notarized
  production release, or protected `develop` merge without real review.
- 21/24 engineering acceptance packages remain unchecked; 7/7 brand epics
  remain unchecked; 61 detailed R0–R8 checklist items remain unchecked.
  These are overlapping categories and **must not be summed**.
- Next actions: real Apple Developer ID / notarization (owner); native
  WebKit/provider E2E (engineering); independent reviewer for PR #26.
