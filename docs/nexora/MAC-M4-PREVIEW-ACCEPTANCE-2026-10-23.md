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
- Local Node unit tests: **156 PASS / 0 FAIL**.
- Release-script contract tests: **33 PASS / 0 FAIL**.
- R0 identity/updater isolation check: **PASS**.
- Mac preview build and volume verification: **PENDING**, must be filled only from actual output.
- Native 8-second isolated HOME startup smoke: **PENDING**.
- Native Mac UI/terminal/provider/agent/MCP functional E2E: **PENDING**.
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

**GO:** source implementation, isolated build/verification, CI test artifacts.
**NO-GO:** production label, trusted signed installer, protected develop merge without independent reviewer.
