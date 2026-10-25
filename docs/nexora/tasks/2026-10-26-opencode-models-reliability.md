# Nexora Code — R1 OpenCode discovery reliability, 2026-10-26

Scope: Nexora Code only. User requests complete all work; first reproducible blocker is OpenCode model discovery. Dedicated branch `fix/20261026-opencode-model-discovery` based on `origin/develop`.

- [x] Lead traced current source: `src-tauri/src/providers/extra_models.rs` launches `models --verbose` first under fixed 15-second deadline and never falls back on a timeout. Current OpenCode 2.1.0 help lists `models` and `--standalone`, no `--verbose` flag. On Mac original user HOME `opencode models` returned 79 IDs when allowed >12 s; native app times out intermittently.
- [ ] W1 fix minimal discovery routine: use current OpenCode plain listing as primary with bounded generous timeout, preserve verbose fallback only if plain fails (and no indefinite hang or output unbounded), do not leak metadata/secrets.
- [ ] W2 independent source QA plus regression tests verifying fallback and bounded cleanup; no independent approval claimed until actual review.
- [ ] Lead run Node build/tests, targeted Rust, check static safety, PR and CI. Do not overwrite installed app with a source-only patch; macOS installed binary requires rebuilt signed/unsigned test artifact.
- [ ] Native Mac GUI and full provider E2E, 3-OS release, security soak, branding owner acceptance remain open. No protected merge or Production claim without acceptance.
