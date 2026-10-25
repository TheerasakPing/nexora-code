# Nexora Code — Production Release Preflight

**As of:** 2026-10-24. **Source:** Rust / Tauri v2 / React. **Status:** RELEASE NO-GO; unsigned engineering Alpha is available, not trusted production.
**Lead:** TEAM_LEAD_SERIAL. A signed stable release is not allowed without independent human and real target-device acceptance.
**Current baseline:** PR #38 merged into develop (6dd348b); PR #39 acceptance docs merged (6541b); PR #40 production GUI-security improvements awaiting hosted CI/merge.

## Production release go/no-go checks
- [ ] PR #40 exact SHA passes CI, hosted three installer jobs and Team Lead QA. Release from a clean protected main commit, not from PR branch.
- [ ] Independent qualified human (not patch author) reviews source, supply chain, credentials and native-platform QA and explicitly approves.
- [ ] Installed Mac M4 GUI: WebView rendered content, navigation, permissions, provider/MCP/PTY, session recovery, crash/restart and safe rollback validated.
- [ ] Installed **Windows 11 x64** GUI: WebView2 runtime, ConPTY, dialogs, provider/MCP, cleanup, install/uninstall, SmartScreen publisher trust and crash E2E.
- [ ] Linux desktop x64: AppImage through FUSE on target distro, Wayland and X11 paths, portal permission, provider/MCP/PTY and application lifecycle E2E.
- [ ] Real 8-provider soak (2 hours), repeated native crash/restart on each target OS; inspect no duplicate commands, no stale children or lost sessions.
- [ ] Security: `npm ci && npm audit --audit-level=high`, Rust locked tests/Clippy, artifact scanning/SBOM, current dependency provenance, actual SHA256 manifest.
- [ ] In-app updater: production public key match, Tauri signed artifact on every platform, authenticated update manifest, migration/rollback behavior and test restores.
- [ ] Team Lead logs exact source SHA, 3 OS artifacts, E2E/Smoke results and human reviewer approval; project owner manually authorizes version/tag/release notes.

## Cryptographic and distribution prerequisites (not yet provisioned)
- **macOS:** Apple Developer Program membership, Developer ID Application P12 (not ad hoc), private key password, notarization API credentials and stapled .app/.dmg; `spctl` accept on a clean Mac.
- **GitHub Mac CI:** protect `APPLE_CERTIFICATE` (base64), `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`, `APPLE_API_KEY`, `APPLE_API_ISSUER`, `APPLE_API_PRIVATE_KEY` as masked environment secrets. Existing `scripts/release/ci-apple.mjs` creates a temporary keychain; do not print key material.
- **Windows:** procure trusted Authenticode publisher certificate/signing service, integrate verifiable Win11 NSIS signing + signature validation. Tauri *updater* signature is **not** Windows Authenticode/SmartScreen trust.
- **Tauri updater:** provision `TAURI_SIGNING_PRIVATE_KEY` and password through protected release environment, store verified public key only in built app. Never ship private key in repo/artifacts.
- **Linux:** verify package provenance and signing/update metadata; run FUSE and desktop portal smoke on supported distro, not just Xvfb extract-and-run.
- **GitHub:** configure protected production deployment environment with a reviewer; no untrusted pull-request signing key exposure. Repository Issues are disabled, track via PR checklists.
