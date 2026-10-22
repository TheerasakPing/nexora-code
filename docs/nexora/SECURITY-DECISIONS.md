# R0 Security decisions

Do not trust the original PandaCode updater public key. No update checks or automatic
installation until Nexora-owned signing keys are generated and separately reviewed.
The app identifier separates OS application data and the provider Keychain service
names are fork-specific. Never import users' original secrets automatically.

Git worktree isolation and scheduler file-scopes are not sufficient for native OS
sandboxing. R1 and R3 must add and test meaningful execution containment.
