#!/usr/bin/env bash
# Internal engineering artifact only: ad-hoc signed macOS DMG without Finder/AppleScript.
# NOT a Developer ID signed, notarized or Production-ready installer.
set -euo pipefail
[[ "$(uname -s)" == "Darwin" && "$(uname -m)" == "arm64" ]] || {
  echo "This packaging script requires an arm64 macOS build host." >&2
  exit 2
}
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
VERSION="$(node -p "JSON.parse(require('node:fs').readFileSync('package.json','utf8')).version")"
BUNDLE_ID="com.theerasakping.nexoracode"
APP="$ROOT/src-tauri/target/debug/bundle/macos/Nexora Code.app"
OUTDIR="$ROOT/release/internal-alpha"
DMG="$OUTDIR/Nexora-Code-$VERSION-alpha-macOS-arm64-UNSIGNED.dmg"
mkdir -p "$OUTDIR"

# Tauri's normal DMG layout uses Finder AppleScript and may hang without GUI access.
npm run tauri -- build --debug --bundles app

STAGE="$(mktemp -d /tmp/nexora-alpha-stage.XXXXXX)"
MOUNT="$(mktemp -d /tmp/nexora-alpha-mount.XXXXXX)"
mounted=0
cleanup() {
  if [[ "$mounted" -eq 1 ]]; then hdiutil detach -quiet "$MOUNT" || true; fi
  rmdir "$MOUNT" 2>/dev/null || true
  rm -rf "$STAGE"
}
trap cleanup EXIT

/usr/bin/ditto "$APP" "$STAGE/Nexora Code.app"
codesign --force --deep --sign - "$STAGE/Nexora Code.app"
codesign --verify --deep --strict "$STAGE/Nexora Code.app"
ln -s /Applications "$STAGE/Applications"
hdiutil create -quiet -volname "Nexora Code Alpha" -fs HFS+ -srcfolder "$STAGE" -format UDZO -ov "$DMG"

hdiutil attach -quiet -readonly -nobrowse -mountpoint "$MOUNT" "$DMG"
mounted=1
test -d "$MOUNT/Nexora Code.app"
test -L "$MOUNT/Applications"
test "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$MOUNT/Nexora Code.app/Contents/Info.plist")" = "$BUNDLE_ID"
codesign --verify --deep --strict "$MOUNT/Nexora Code.app"
hdiutil detach -quiet "$MOUNT"
mounted=0

(cd "$OUTDIR" && shasum -a 256 "$(basename "$DMG")" > "$(basename "$DMG").sha256")
echo "Nexora Code internal alpha DMG verified: $DMG"
echo "SHA-256:"
cat "$DMG.sha256"
echo "IMPORTANT: ad-hoc signed Debug build; NOT notarized or Production Ready."
