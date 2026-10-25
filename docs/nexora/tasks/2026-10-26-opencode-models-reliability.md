# Nexora Code — R1 OpenCode discovery reliability, 2026-10-26

Scope: Nexora Code only. User requests complete all work; first reproducible blocker is OpenCode model discovery. Dedicated branch `fix/20261026-opencode-model-discovery` based on `origin/develop`.

- [x] Lead traced current source: `src-tauri/src/providers/extra_models.rs` launches `models --verbose` first under fixed 15-second deadline and never falls back on a timeout. Current OpenCode 2.1.0 help lists `models` and `--standalone`, no `--verbose` flag. On Mac original user HOME `opencode models` returned 79 IDs when allowed >12 s; native app times out intermittently.
- [ ] W1 fix minimal discovery routine: use current OpenCode plain listing as primary with bounded generous timeout, preserve verbose fallback only if plain fails (and no indefinite hang or output unbounded), do not leak metadata/secrets.
- [ ] W2 independent source QA plus regression tests verifying fallback and bounded cleanup; no independent approval claimed until actual review.
- [ ] Lead run Node build/tests, targeted Rust, check static safety, PR and CI. Do not overwrite installed app with a source-only patch; macOS installed binary requires rebuilt signed/unsigned test artifact.
- [ ] Native Mac GUI and full provider E2E, 3-OS release, security soak, branding owner acceptance remain open. No protected merge or Production claim without acceptance.

## Verified follow-up — 2026-10-26
- [x] W1: plain-first OpenCode catalog, timeout bounded to 35s, no duplicate probe when first call times out, legacy verbose fallback for immediate non-timeout failures. Source change in `extra_models.rs`.
- [x] Lead: macOS frontend build PASS; Node unit 210/210 PASS.
- [x] Initial independent read-only source review: **ACCEPT DEVELOPMENT**; reviewer requested focused retry-policy test.
- [x] Added focused Rust unit `discovery_timeout_never_triggers_another_slow_cli_probe` covering timeout/non-timeout classification and plain model input.
- [x] Initial GitHub PR #47, commit `172038f`: 9/9 CI SUCCESS (all three unsigned installers plus 3-OS native Rust), still Draft without human approval.
- [ ] Focused Rust test result for new test pending completion; rerun CI on next commit.
- [ ] Installed macOS binary still prior release and does not include PR #47. Native GUI model discovery remains unverified.
- [ ] All R0–R8 functional/native/security/branding Release gates still separate; no 100% claim.
