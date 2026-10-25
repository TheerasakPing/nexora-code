#!/usr/bin/env bash
# CI macOS arm64 beta artifact from exact opt-in feature build, NOT Production.
set -euo pipefail
[[ "$(uname -s)" == Darwin && "$(uname -m)" == arm64 ]] || { echo 'BLOCKED: requires Mac ARM64'; exit 3; }
root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"
app="$root/src-tauri/target/debug/bundle/macos/Nexora Code Beta.app"
plist="$app/Contents/Info.plist"
[[ -d "$app" && -f "$plist" ]] || { echo 'NO BETA APP WAS BUILT'; exit 2; }
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$plist")" == 'com.theerasakping.nexoracode.beta' ]] || { echo 'UNSAFE BUNDLE ID'; exit 2; }
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$plist")" == 'nexora-code' ]] || { echo 'UNEXPECTED EXECUTABLE'; exit 2; }
lipo -archs "$app/Contents/MacOS/nexora-code" | grep -qw arm64 || { echo 'WRONG CPU'; exit 2; }
codesign --verify --deep --strict "$app"
mkdir -p release/darwin-aarch64
archive='release/darwin-aarch64/Nexora-Code-Beta-0.1.0-macOS-arm64-ISOLATED-UNNOTARIZED.app.zip'
/usr/bin/ditto -c -k --sequesterRsrc --keepParent "$app" "$archive"
(cd release/darwin-aarch64 && shasum -a 256 "$(basename "$archive")" > "$(basename "$archive").sha256")
echo "NEXORA_MAC_BETA_ARTIFACT=$archive"
echo 'TEST ONLY: ad-hoc signed / unnotarized, never overwrite existing Nexora Code app.'
