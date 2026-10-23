#!/usr/bin/env bash
# Non-destructive native process smoke test for an optimized macOS app.
# This is NOT a provider, terminal, MCP, GUI usability, or full native E2E test.
set -euo pipefail
[[ "$(uname -s)" == Darwin ]] || { echo "macOS only" >&2; exit 2; }
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
APP="${1:-$ROOT/src-tauri/target/release/bundle/macos/Nexora Code.app}"
[[ -d "$APP" ]] || { echo "Missing release app bundle: $APP" >&2; exit 3; }
PLIST="$APP/Contents/Info.plist"
EXE="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$PLIST")"
BINARY="$APP/Contents/MacOS/$EXE"
[[ -f "$BINARY" ]] || { echo "Missing binary" >&2; exit 3; }
/usr/bin/codesign --verify --strict --deep "$APP"
TEST_HOME="$(mktemp -d /tmp/nexora-m4-smoke-home.XXXXXX)"
mkdir -p "$TEST_HOME/Library/Application Support" "$TEST_HOME/Library/Caches" "$TEST_HOME/Library/Logs"
LOG="$ROOT/release/mac-preview/native-startup-smoke.log"
mkdir -p "$(dirname "$LOG")"
pid=
cleanup() {
  if [[ -n "$pid" ]] && /bin/kill -0 "$pid" 2>/dev/null; then
    /bin/kill -TERM "$pid" 2>/dev/null || true
    /bin/sleep 1
    /bin/kill -KILL "$pid" 2>/dev/null || true
  fi
  /bin/rm -rf "$TEST_HOME"
}
trap cleanup EXIT
# No user credentials, provider configs, or historical sessions in isolated HOME.
# No -n/open LaunchServices: the executable is started directly with its own bundle.
(
  cd "$TEST_HOME"
  /usr/bin/env -i HOME="$TEST_HOME" TMPDIR="$TEST_HOME" PATH="/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin" \
    "$BINARY"
) >"$LOG" 2>&1 &
pid=$!
sleep 8
if ! /bin/kill -0 "$pid" 2>/dev/null; then
  wait "$pid" || code=$?
  echo "FAIL: native app exited early (code ${code:-0}); see $LOG" >&2
  tail -30 "$LOG" >&2
  exit 4
fi
ps -o pid=,etime=,rss=,comm= -p "$pid" | tee -a "$LOG"
echo "PASS: app stayed running for 8 seconds in isolated HOME (bounded startup smoke only)"
echo "NOTE: successful GUI interactions, provider login and real MCP remain unverified"
