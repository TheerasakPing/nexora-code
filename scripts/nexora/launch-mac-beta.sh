#!/bin/zsh
# Nexora Code isolated test launcher. Never launches Nexora Studio or production Nexora Code.
set -euo pipefail
if [[ "$(uname -s)" != Darwin || "$(uname -m)" != arm64 ]]; then
  echo 'BLOCKED: requires macOS arm64' >&2; exit 3
fi
app="$HOME/Applications/Nexora Code Beta Isolated.app"
plist="$app/Contents/Info.plist"
bin="$app/Contents/MacOS/nexora-code"
if [[ ! -f "$plist" || ! -x "$bin" ]]; then
  echo 'BLOCKED: isolated Nexora Code Beta app is not installed' >&2; exit 3
fi
bundle_id="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$plist" 2>/dev/null || true)"
if [[ "$bundle_id" != 'com.theerasakping.nexoracode.beta' ]]; then
  echo 'BLOCKED: app identifier is not the isolated beta' >&2; exit 3
fi
if ! /usr/bin/codesign --verify --deep --strict "$app" 2>/dev/null; then
  echo 'BLOCKED: code signature integrity check failed' >&2; exit 3
fi
# Startup app data is separated by Tauri bundle identifier; CLI HOME and
# provider environment are also replaced. No legacy Claude/Pi token files.
# Fresh HOME outside the login user's home. No inherited CLAUDE_CONFIG_DIR,
# PI_CODING_AGENT_DIR, provider API keys, or other credential overrides.
sandbox_home="/private/tmp/nexora-code-beta-home.$(/usr/bin/id -u)"
if [[ -L "$sandbox_home" ]]; then
  echo 'BLOCKED: Beta HOME may not be a symbolic link' >&2; exit 3
fi
if [[ ! -e "$sandbox_home" ]]; then
  /usr/bin/install -d -m 700 "$sandbox_home"
fi
if [[ ! -d "$sandbox_home" || -L "$sandbox_home" ||
      "$(/usr/bin/stat -f '%u' "$sandbox_home")" != "$(/usr/bin/id -u)" ||
      "$(/usr/bin/stat -f '%Lp' "$sandbox_home")" != 700 ]]; then
  echo 'BLOCKED: Beta HOME must be a private directory owned by the current user (0700)' >&2; exit 3
fi
/usr/bin/env -i \
  HOME="$sandbox_home" USER="$(id -un)" LOGNAME="$(id -un)" \
  TMPDIR="/tmp/" LANG=en_US.UTF-8 \
  PATH='/usr/bin:/bin:/usr/sbin:/sbin' \
  "$bin"
