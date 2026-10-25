#!/usr/bin/env bash
# Test-only: invoked inside a disposable GitHub-hosted Xvfb + DBus session.
set -euo pipefail
if [[ "${GITHUB_ACTIONS:-}" != true || "${RUNNER_ENVIRONMENT:-}" != github-hosted || -z "${DISPLAY:-}" ]]; then
  echo "ERROR: visible-window smoke requires a hosted private X display" >&2
  exit 2
fi
if [[ "$#" != 1 || ! -f "$1" ]]; then
  echo "ERROR: missing AppImage to test" >&2
  exit 2
fi
app="$1"
# Isolate AppImage and descendants into their own POSIX session/process group.
# The parent shell is never a member of this group.
setsid "$app" &
pid=$!
cleanup() {
  # Do not reap root until its separate group has been terminated: its PID
  # cannot be recycled while still owned by this shell.
  kill -TERM -- "-$pid" 2>/dev/null || true
  for ((n=0; n<10; n++)); do
    kill -0 "$pid" 2>/dev/null || break
    sleep 0.1
  done
  kill -KILL -- "-$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

# This is a private Xvfb DISPLAY, so no unrelated user windows can match.
for ((attempt=0; attempt<60; attempt++)); do
  if ! kill -0 "$pid" 2>/dev/null; then
    echo "ERROR: AppImage exited before an X11 native window appeared" >&2
    exit 1
  fi
  windows="$(xdotool search --onlyvisible --name '^Nexora Code$' 2>/dev/null || true)"
  for window in $windows; do
    geometry="$(xdotool getwindowgeometry --shell "$window" 2>/dev/null || true)"
    width="$(sed -n 's/^WIDTH=//p' <<<"$geometry")"
    height="$(sed -n 's/^HEIGHT=//p' <<<"$geometry")"
    if [[ "$width" =~ ^[0-9]+$ && "$height" =~ ^[0-9]+$ ]] && (( width >= 860 && height >= 580 )); then
      sleep 2
      kill -0 "$pid" 2>/dev/null || { echo "ERROR: app exited after creating a window" >&2; exit 1; }
      # Reject a flash of a native window that vanished while the process stayed alive.
      again="$(xdotool search --onlyvisible --name '^Nexora Code$' 2>/dev/null || true)"
      [[ " $again " == *" $window "* ]] || { echo "ERROR: app window vanished after creating it" >&2; exit 1; }
      refreshed="$(xdotool getwindowgeometry --shell "$window" 2>/dev/null || true)"
      [[ "$refreshed" == *"WIDTH=$width"* && "$refreshed" == *"HEIGHT=$height"* ]] || {
        echo "ERROR: app window geometry changed during stability check" >&2; exit 1;
      }
      echo "LINUX_NATIVE_WINDOW_PASS window=$window width=$width height=$height"
      exit 0
    fi
  done
  sleep 0.5
done
echo "ERROR: visible Nexora Code X11 window >= 860x580 was not found within 30s" >&2
exit 1
