#!/usr/bin/env bash
# Optimized macOS ARM64 engineering preview; NOT an approved signed/notarized stable release.
# Builds the release-mode .app, then creates the DMG without Finder/AppleScript.
set -euo pipefail
[[ "$(uname -s)" == Darwin && "$(uname -m)" == arm64 ]] || {
  echo "ERROR: requires an Apple Silicon macOS build host" >&2
  exit 2
}
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
VERSION="$(node -p "JSON.parse(require('node:fs').readFileSync('package.json','utf8')).version")"
BUNDLE_ID="com.theerasakping.nexoracode"
APP="$ROOT/src-tauri/target/release/bundle/macos/Nexora Code.app"
OUTDIR="$ROOT/release/mac-preview"
DMG="$OUTDIR/Nexora-Code-$VERSION-macOS-arm64-PREVIEW-UNNOTARIZED.dmg"
# Do not consume or inadvertently print secrets/production signing material.
if [[ -n "${APPLE_SIGNING_IDENTITY:-}" && "${APPLE_SIGNING_IDENTITY}" != "-" ]]; then
  echo "ERROR: preview packaging may not use Developer ID keys; build a separately approved production artifact." >&2
  exit 3
fi
if [[ -n "${TAURI_SIGNING_PRIVATE_KEY:-}" ]]; then
  echo "ERROR: updater signing keys are prohibited in an engineering preview." >&2
  exit 3
fi
if [[ "${1:-}" != "--skip-build" ]]; then
  APPLE_SIGNING_IDENTITY=- npm run tauri -- build --bundles app
fi
[[ -d "$APP" ]] || { echo "ERROR: release app bundle not found at $APP" >&2; exit 4; }

plist="$APP/Contents/Info.plist"
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$plist")" == "$BUNDLE_ID" ]] || {
  echo "ERROR: unexpected bundle identifier" >&2; exit 4
}
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$plist")" == "$VERSION" ]] || {
  echo "ERROR: bundle version mismatch" >&2; exit 4
}
EXECUTABLE="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$plist")"
[[ -f "$APP/Contents/MacOS/$EXECUTABLE" ]] || {
  echo "ERROR: no app executable" >&2; exit 4
}
lipo -archs "$APP/Contents/MacOS/$EXECUTABLE" | grep -qw arm64 || {
  echo "ERROR: built executable is not arm64" >&2; exit 4
}
mkdir -p "$OUTDIR"
STAGE="$(mktemp -d /tmp/nexora-preview-stage.XXXXXX)"
MOUNT="$(mktemp -d /tmp/nexora-preview-mount.XXXXXX)"
mounted=0
cleanup() {
  if [[ "$mounted" == 1 ]]; then /usr/bin/hdiutil detach -quiet "$MOUNT" || true; fi
  /bin/rmdir "$MOUNT" 2>/dev/null || true
  /bin/rm -rf "$STAGE"
}
trap cleanup EXIT

/usr/bin/ditto "$APP" "$STAGE/Nexora Code.app"
# An ad-hoc signature protects bundle integrity on the build host; it is NOT
# Apple Developer ID signing and does not satisfy Gatekeeper/notarization.
if ! /usr/bin/codesign --verify --deep --strict "$STAGE/Nexora Code.app" >/dev/null 2>&1; then
  /usr/bin/codesign --force --deep --sign - "$STAGE/Nexora Code.app"
fi
/usr/bin/codesign --verify --deep --strict "$STAGE/Nexora Code.app"
# Make Apple Gatekeeper limitations visible *before* a user tries to install.
# This is an unsigned engineering preview: the README is not a bypass.
cp "$ROOT/scripts/nexora/MAC-PREVIEW-INSTALL.txt" "$STAGE/READ ME - INSTALLATION.txt"
/bin/ln -s /Applications "$STAGE/Applications"
/usr/bin/hdiutil create -quiet -volname "Nexora Code Preview" -fs HFS+ \
  -srcfolder "$STAGE" -format UDZO -ov "$DMG"
/usr/bin/hdiutil verify -quiet "$DMG"
/usr/bin/hdiutil attach -quiet -readonly -nobrowse -noautoopen -mountpoint "$MOUNT" "$DMG"
mounted=1
[[ -d "$MOUNT/Nexora Code.app" && -L "$MOUNT/Applications" && -f "$MOUNT/READ ME - INSTALLATION.txt" ]] || {
  echo "ERROR: DMG layout invalid" >&2; exit 5
}
[[ "$(readlink "$MOUNT/Applications")" == "/Applications" ]] || { echo "ERROR: bad Applications link" >&2; exit 5; }
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$MOUNT/Nexora Code.app/Contents/Info.plist")" == "$BUNDLE_ID" ]] || {
  echo "ERROR: mounted app has wrong bundle ID" >&2; exit 5
}
/usr/bin/codesign --verify --deep --strict "$MOUNT/Nexora Code.app"
/usr/bin/hdiutil detach -quiet "$MOUNT"
mounted=0
(cd "$OUTDIR" && /usr/bin/shasum -a 256 "$(basename "$DMG")" > "$(basename "$DMG").sha256")
echo "MAC_PREVIEW_DMG=$DMG"
echo "MAC_PREVIEW_APP=$APP"
cat "$DMG.sha256"
echo "PREVIEW ONLY: Release optimized, ad-hoc signed, NOT Developer ID signed/notarized/production-approved."
