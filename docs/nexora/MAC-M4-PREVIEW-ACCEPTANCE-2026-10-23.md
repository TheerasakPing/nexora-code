# Nexora Code — macOS Apple M4 Release-mode Preview Acceptance

> 2026-10-23 / Team Lead: ChatGPT (TEAM_LEAD_SERIAL). Not a signed/notarized Production Release.
> Product remains a fork of PandaCode; retain original LICENSE and attribution.
> Source: consolidated PR #26 base SHA 654a3d9 + dedicated Mac changes.

## Environment

- Test host: Mac mini M4 (arm64) / 16 GB / macOS 27.2.
- Tooling: Rust 1.95.0, Node.js 26.8.2, Apple Command Line Tools.
- Signing identities: 0 valid Developer ID identities available; this means Developer ID + Apple notarization are NOT possible locally without separately provisioned credentials.

## Build defect found and remedied in source

On macOS 27 the original release profile's `strip = true` also applied
to Rust compile-time proc-macro dylibs. The resulting dylibs failed to load
(`can't find crate for zerofrom_derive`, `mis-aligned LINKEDIT string pool`).
This is a known macOS dyld/LLVM strip interaction, not a missing Cargo crate.

Our `[profile.release.build-override] strip = false` keeps compile-time macros
unstripped while the actual shipped Rust binary still uses the optimized
`[profile.release] strip = true` configuration. See rust-lang/rust#157750.

## Current evidence / gate

- TypeScript/Vite production bundle: **PASS**.
- Local Node unit tests: **159 PASS / 0 FAIL** (includes 3 Mac packaging checks).
- Release-script contract tests: **33 PASS / 0 FAIL**.
- R0 identity/updater isolation check: **PASS**.
- GitHub-hosted macOS ARM64 **optimized release-mode preview bundle: PASS**,
  [R6 artifact run 39890157346](https://github.com/TheerasakPing/nexora-code/actions/runs/39890157346).
- Exact artifact: `Nexora-Code-0.1.0-macOS-arm64-PREVIEW-UNNOTARIZED.dmg`.
  SHA-256: `7574c3ff42c6de06e303f2d0bda3a343bb09c72675e91ec2b71b5825520a74a8`.
- Downloaded artifact to the actual Mac mini M4 and **independently verified**
  SHA-256, `hdiutil verify`, read-only mount, app bundle ID, version, ARM64
  executable, `codesign --verify --deep --strict`, and updater isolation: **PASS**.
- Static bundle copied to a temporary folder and launched with isolated `HOME`:
  native process stayed alive 8 seconds, memory RSS ~85 MB: **PASS for bounded
  process startup only**.
- **Warning in native log:** macOS WebKit reported
  `sandbox_extension_issue_file_to_process ... Operation not permitted`
  when launched from temporary paths; actual WKWebView GUI functionality remains
  unverified. This is NOT a passing visual E2E result.
- Local optimized release build on macOS 27 was stopped after the independently
  built and validated GitHub bundle was available, to avoid redundant prolonged
  compilation. Do not claim a complete second local release build.
- Browser-preview UI regression (Settings, Terminal, Architect teams): **58 PASS**
  using Playwright Chromium on the Mac; this does NOT validate Tauri's native WKWebView.
- Native isolated HOME process startup smoke: **PASS (bounded process only)**.
- Native Mac WKWebView/terminal/provider/agent/MCP functional E2E: **PENDING** (sandbox warning requires investigation).
- Apple Developer ID, notarization, Gatekeeper distribution: **BLOCKED** (no signing identity).
- Independent GitHub code review: **PENDING**.
- R0–R8 production acceptance: **NO-GO**.

## Installer source and verification

On a real Apple Silicon Mac (no /Applications overwrite):

```bash
CARGO_BUILD_JOBS=4 bash scripts/nexora/build-mac-preview-dmg.sh
bash scripts/nexora/verify-mac-preview-dmg.sh
bash scripts/nexora/smoke-mac-preview.sh
```

Expected test artifact path:
`release/mac-preview/Nexora-Code-0.1.0-macOS-arm64-PREVIEW-UNNOTARIZED.dmg`
and adjacent `.sha256`. Run verifier before opening the DMG. The preview
uses an ad-hoc seal and **is not approved for public production distribution**.
The GitHub R6 test-only workflow can archive the same unsigned Mac preview for
short-retention download in GitHub Actions, with no GitHub Release created.

## Remaining acceptance to call it complete

- Qualified independent reviewer, full app behavior E2E, live provider/PTY/session recovery and crash under real macOS UI.
- Secured MCP operator consent/vault/transport; robust agent scheduler/worktrees and durable workflow execution.
- Final brand/logo and migration/coexistence policy, signed/notarized release and fork-owned updater.
- Actual release/rollback/telemetry-free deployment approval from Project Owner.

**GO:** source implementation, hosted optimized Mac preview, verified DMG and bounded Mac M4 process startup.
**NO-GO:** production label, trusted signed installer, protected develop merge without independent reviewer.
