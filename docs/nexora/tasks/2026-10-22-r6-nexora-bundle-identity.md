# Checklist Planner — R6 Nexora Code Unsigned Installer Identity

**Date:** 2026-10-22; written before source edits.
**Request:** Continue R0–R8 on connected Mac M4, advance 3-platform installer readiness.
**Team Lead:** acting assistant, TEAM_LEAD_SERIAL. No independently available Agent Workers/reviewers.
**Priority:** P1 release reliability, R6 and rebranding BR-04/BR-06.
**Branch/worktree:** fix/r6-nexora-bundle-identity; nexora-code-r6-package-identity.
**Base:** origin/integration/r0-r5-security-graph (stacked on PR #24).
**Observed mismatch:** Tauri productName is Nexora Code, but scripts/release/common.mjs expects PandaCode bundle filenames and build.mjs looks for PandaCode.app inside a DMG.
**Scope:** sync expected output package names for macOS ARM64, Windows 11 x64 and Linux x64; update release tests; ensure fork-owned manifest fallback; align macOS window title; add a disposable-runner unsigned alpha packaging workflow with smoke checks, no publishing or signing secrets.
**Compatibility:** Cargo/Rust binary remains pandacode for now; runtime protocol/session/DB identifiers unchanged, tool contracts unaffected, attribution retained.
**Out of scope:** upload release, notarize/sign, auto-update, brand art approval, user app install, vendor credential changes, protected branch merge.
**Acceptance:** npm run test:release, npm run build, R0/Node tests, CI on exact new SHA, native package E2E remains separate.

| Task | Actual assignee | Files | Gate | State |
| --- | --- | --- | --- | --- |
| R6-01 | TEAM_LEAD_SERIAL | scripts/release/common.mjs | Correct three bundle/updater asset names and fork-safe manifest default | [x] |
| R6-02 | TEAM_LEAD_SERIAL | scripts/release/build.mjs | Correct macOS app/DMG verification without renaming existing binary | [x] |
| R6-03 | TEAM_LEAD_SERIAL | scripts/release/*.test.mjs | Release name expectations and boundary regression | [x] |
| R6-04 | TEAM_LEAD_SERIAL | docs/nexora/R6-INSTALLER-IDENTITY.md | Limits, test and acceptance evidence | [x] |
| R6-05 | TEAM_LEAD_SERIAL | src-tauri/tauri.macos.conf.json | macOS window title matches provisional Nexora bundle | [x] |
| R6-06 | TEAM_LEAD_SERIAL | .github/workflows/nexora-r6-alpha.yml | 3-OS ephemeral unsigned package/smoke artifacts, no secrets or release | [x] |
| R6-QA | TEAM_LEAD_SERIAL | diff + Node/build/release/R0 | tests and PR | [x] |
| R6-REVIEW | independent reviewer pending | GitHub | protected merge only after actual approval | [ ] |

- [x] Planner created before editing and isolated from other feature worktrees.
- [x] Tests prove productName, DMG app path and expected package names agree.
- [x] Existing Rust binary/DB/schema unchanged; no upstream updater release URLs.
- [x] Local Node/Release/Build/R0 verified; exact-head GitHub package and Rust checks PENDING.
- [ ] Native installed GUI/package acceptance remains outstanding.
- [ ] No protected merge/release without independent review.

**Team Lead verdict:** GO for R6 unsigned packaging smoke PR submission, NO-GO for protected merge, signed distribution and R8 Production Release.

## Mac source acceptance evidence (2026-10-22)

- Actor: TEAM_LEAD_SERIAL only; no separate coding agents, security reviewer or manual app user test.
- Source review: synced Tauri productName, macOS window title, versioned DMG/NSIS/AppImage expected filenames, provisional NexoraCode stable asset labels and fork updater default; intentionally retained pandacode executable and on-disk contract.
- Release tests: 33 PASS / 0 FAIL, including new identity alignment and safe macOS application path tests.
- Full Node unit tests: 156 PASS / 0 FAIL.
- TypeScript/Vite production build: PASS (pre-existing >500KB chunk advisory).
- R0 fork/updater safety check: PASS.
- R6 YAML parsed with Python PyYAML; matrix targets macOS ARM64/Windows x64/Linux x64, read-only permissions, no GitHub Release command or signing secret inputs.
- Diff hygiene: git diff --check PASS.
- **Not tested here:** actual Tauri GUI launch, installation, signed/notarized DMG/NSIS/AppImage, provider/MCP runtime, Windows/Linux UX, automatic updater. PR workflow requires successful native packaging before engineering alpha files exist.
- GitHub CI on exact PR head SHA and independent GitHub reviewer: PENDING. No production readiness claim.
