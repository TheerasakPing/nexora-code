# Nexora Code — OpenCode model discovery in isolated Mac Beta

2026-10-25. User-reported `OpenCode model discovery timed out` in Nexora Code Beta Isolated. This is Nexora Code ONLY.

- [x] Reproduced `opencode models` under actual beta HOME `/private/tmp/nexora-code-beta-home.TjPaLxkf` and sanitized PATH: 8 sec bounded test hit 124, emitted zero model IDs. OpenCode binary exists `/opt/homebrew/bin/opencode` (symlink to user-local binary). Never read credentials.
- [x] Source traced to `src-tauri/src/providers/extra_models.rs`: `tokio::time::timeout(Duration::from_secs(15))`; generic error disguises beta missing OpenCode login/config. Beta launcher deliberately uses ephemeral fresh HOME and clears env; cannot automatically see the original OpenCode auth or models. We should NOT copy tokens from original account or silently widen PATH/credential access.
- [ ] W1 Lead: provide precise, non-sensitive Beta error/help guidance in provider model discovery. No fake available model names, no raising timeout blindly. Distinguish timeout from CLI missing/login and suggest explicitly configuring separate OpenCode in Beta HOME.
- [ ] W2 Lead: add test contracts to ensure Beta doesn't access legacy credentials and normal builds unchanged, with focused Rust unit tests and frontend tests if practical.
- [ ] Independent read-only reviewer: inspect privacy and regressions; native real provider authentication requires explicit user action.
- [ ] Lead: cargo test targeted, Node smoke/build, GitHub push and report. Existing running Beta is built from an earlier commit so must rebuild to ship modifications; don't claim installed app fixed until then.

Acceptance: UI explains Beta isolation/timeout without exposing original credential data; testable; safe local noninteractive test; production behavior unchanged; no secrets logged. No Production go-live until native GUI and provider auth E2E pass.
