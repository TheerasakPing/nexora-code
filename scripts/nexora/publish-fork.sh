#!/usr/bin/env bash
# Publish existing reviewed local R0 branch once GitHub CLI is authenticated.
# Refuses to merge, tag, release or modify upstream.
set -euo pipefail
root="$(git rev-parse --show-toplevel)"
cd "$root"
[[ "$(git branch --show-current)" == "feat/r0-foundation" ]] || { echo "Switch to feat/r0-foundation first"; exit 2; }
[[ -z "$(git status --porcelain)" ]] || { echo "Commit or clean local changes first"; exit 2; }
[[ "$(gh api user --jq .login)" == "TheerasakPing" ]] || { echo "Run gh auth login as TheerasakPing"; exit 3; }
if ! gh repo view TheerasakPing/nexora-code --json nameWithOwner >/dev/null 2>&1; then
  gh api -X POST repos/aicodeking/pandacode/forks -f name=nexora-code -F default_branch_only=false --jq .full_name
fi
for i in $(seq 1 30); do
  if gh api repos/TheerasakPing/nexora-code --jq '.parent.full_name // ""' 2>/dev/null | grep -qx aicodeking/pandacode; then break; fi
  sleep 4
done
[[ "$(gh api repos/TheerasakPing/nexora-code --jq '.parent.full_name // ""')" == "aicodeking/pandacode" ]] || { echo 'Repository exists but is not expected fork'; exit 4; }
[[ "$(git remote get-url origin)" == "https://github.com/TheerasakPing/nexora-code.git" ]] || { echo "Unexpected remote origin"; exit 4; }
git push --set-upstream origin feat/r0-foundation
gh pr create --repo TheerasakPing/nexora-code --draft --base main --head feat/r0-foundation --title "R0: Nexora Code fork identity and secure release baseline" --body "Nexora Code v0.1.0 engineering branch. Upstream v0.3.16 pinned. Fork attribution, isolated Tauri identity/Keychain, safe disabled upstream updater, native CI and roadmap. No production release. Native three-OS acceptance and fork-owned signer remain blocking."
