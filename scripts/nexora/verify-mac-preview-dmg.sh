#!/usr/bin/env bash
# Independent verifier for optimized ad-hoc macOS preview (not a notarized release).
set -euo pipefail
[[ "$(uname -s)" == Darwin ]] || { echo "ERROR: verifier requires macOS" >&2; exit 2; }
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
VERSION="$(node -p "JSON.parse(require('node:fs').readFileSync('package.json','utf8')).version")"
DMG="${1:-$ROOT/release/mac-preview/Nexora-Code-$VERSION-macOS-arm64-PREVIEW-UNNOTARIZED.dmg}"
[[ -f "$DMG" && -f "$DMG.sha256" ]] || { echo "ERROR: DMG/checksum missing: $DMG" >&2; exit 3; }
(cd "$(dirname "$DMG")" && /usr/bin/shasum -a 256 -c "$(basename "$DMG").sha256")
/usr/bin/hdiutil verify -quiet "$DMG"
MOUNT="$(mktemp -d /tmp/nexora-verify-preview.XXXXXX)"
mounted=0
cleanup() {
  if [[ "$mounted" == 1 ]]; then /usr/bin/hdiutil detach -quiet "$MOUNT" || true; fi
  /bin/rmdir "$MOUNT" 2>/dev/null || true
}
trap cleanup EXIT
/usr/bin/hdiutil attach -readonly -nobrowse -noautoopen -quiet -mountpoint "$MOUNT" "$DMG"
mounted=1
APP="$MOUNT/Nexora Code.app"
[[ -d "$APP" && -L "$MOUNT/Applications" && -f "$MOUNT/READ ME - INSTALLATION.txt" ]] || { echo "ERROR: invalid DMG layout or missing Gatekeeper instructions" >&2; exit 4; }
[[ "$(readlink "$MOUNT/Applications")" == "/Applications" ]] || { echo "ERROR: incorrect Applications symlink" >&2; exit 4; }
PLIST="$APP/Contents/Info.plist"
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$PLIST")" == "com.theerasakping.nexoracode" ]] || {
  echo "ERROR: wrong app identifier" >&2; exit 4
}
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$PLIST")" == "$VERSION" ]] || {
  echo "ERROR: wrong version" >&2; exit 4
}
EXE="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$PLIST")"
[[ -f "$APP/Contents/MacOS/$EXE" ]] || { echo "ERROR: missing app executable" >&2; exit 4; }
lipo -archs "$APP/Contents/MacOS/$EXE" | grep -qw arm64 || { echo "ERROR: not ARM64" >&2; exit 4; }
/usr/bin/codesign --verify --deep --strict "$APP"
signature_details="$(/usr/bin/codesign -dv --verbose=4 "$APP" 2>&1)"
if ! grep -q '^Signature=adhoc$' <<< "$signature_details" || grep -q '^Authority=' <<< "$signature_details"; then
  echo "ERROR: DMG app is not strictly ad-hoc signed" >&2
  exit 4
fi
# Codesign integrity validation is not Apple Gatekeeper distribution trust.
# Never silently suggest that passing `codesign --verify` means installable.
if /usr/sbin/spctl --assess --type execute --verbose=2 "$APP" >/dev/null 2>&1; then
  echo "GATEKEEPER_ASSESSMENT=ACCEPTED (does NOT imply notarized or production approved)"
else
  echo "GATEKEEPER_ASSESSMENT=REJECTED (expected for ad-hoc unsigned preview)"
  echo "WARNING: automatic Finder installation can be rejected by this Mac."
  echo "See README in DMG; Apple allows informed per-app approval via Privacy & Security."
fi
if grep -aFq 'github.com/aicodeking/pandacode/releases/latest/download/latest.json' "$APP/Contents/MacOS/$EXE"; then
  echo "ERROR: original upstream update endpoint embedded in app" >&2; exit 4
fi
/usr/bin/hdiutil detach -quiet "$MOUNT"
mounted=0
echo "PASS: checksum, volume, version, identifier, arm64 executable, ad-hoc bundle seal, updater endpoint isolation (NOT Gatekeeper installation approval)"
echo "RELEASE NO-GO: not Apple Developer ID signed/notarized; no real provider/session/3-OS acceptance proven."
