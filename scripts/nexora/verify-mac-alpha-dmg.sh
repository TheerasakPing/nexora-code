#!/usr/bin/env bash
set -euo pipefail
[[ "$(uname -s)" == Darwin ]] || { echo "macOS only" >&2; exit 2; }
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
VERSION="$(node -p "JSON.parse(require('fs').readFileSync('package.json','utf8')).version")"
DMG="$ROOT/release/internal-alpha/Nexora-Code-$VERSION-alpha-macOS-arm64-UNSIGNED.dmg"
if [[ "$#" -gt 0 ]]; then DMG="$1"; fi
[[ -f "$DMG" && -f "$DMG.sha256" ]] || { echo "Missing DMG or SHA256 evidence" >&2; exit 3; }
(cd "$(dirname "$DMG")" && shasum -a 256 -c "$(basename "$DMG").sha256") || { echo "Checksum mismatch" >&2; exit 4; }
mountpoint="$(mktemp -d /tmp/nexora-alpha-verify.XXXXXX)"
mounted=0
cleanup() {
  if [[ "$mounted" -eq 1 ]]; then hdiutil detach -quiet "$mountpoint" || true; fi
  rmdir "$mountpoint" 2>/dev/null || true
}
trap cleanup EXIT
hdiutil attach -quiet -readonly -nobrowse -mountpoint "$mountpoint" "$DMG"
mounted=1
APP="$mountpoint/Nexora Code.app"
[[ -d "$APP" && -L "$mountpoint/Applications" ]] || { echo "Invalid installer layout" >&2; exit 5; }
plist="$APP/Contents/Info.plist"
identifier="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$plist")"
bundle_version="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$plist")"
[[ "$identifier" == 'com.theerasakping.nexoracode' && "$bundle_version" == "$VERSION" ]] || { echo "Wrong identity/version" >&2; exit 6; }
codesign --verify --deep --strict "$APP"
main_exe="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$plist")"
[[ -f "$APP/Contents/MacOS/$main_exe" ]] || { echo "No executable" >&2; exit 7; }
lipo -archs "$APP/Contents/MacOS/$main_exe" | grep -qw arm64 || { echo "Not arm64" >&2; exit 8; }
if grep -aFq 'github.com/aicodeking/pandacode/releases/latest/download/latest.json' "$APP/Contents/MacOS/$main_exe"; then
  echo "Upstream update endpoint embedded in executable" >&2; exit 9
fi
echo "PASS: SHA256, read-only mount, signature seal, fork bundle ID, version, arm64, app layout"
echo "NOTE: ad-hoc signed debug artifact, not notarized or production approved"
