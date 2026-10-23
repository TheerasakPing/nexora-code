# Task Checklist — 2026-10-23 Mac M4 Native Preview

**Request:** "ทำเลย ขอเวอชั่นเสร็จสมบูรบนแมค" — deliver the most complete safely verifiable native Mac M4 application possible, with an installer, tests, and honest release gate status.
**Date:** 2026-10-23 Asia/Bangkok.
**Project:** TheerasakPing/nexora-code (fork of PandaCode with original attribution preserved).
**Team Lead:** ChatGPT, TEAM_LEAD_SERIAL; no independently delegated worker or independent reviewer has been identified.
**Branch:** feat/mac-m4-native-preview-20261023, forked from the consolidated PR #26 head 654a3d9.
**Host:** Mac mini Apple M4 (arm64), macOS 27.2, 16 GB RAM.
**Priority:** P0 Mac native package and acceptance baseline, but no claim of full R8 production readiness.
**Scope:** Native macOS ARM64 release-mode .app and readable/verifiable preview DMG; isolated launch smoke, frontend/Rust tests and installer evidence; do not modify installed /Applications app or user credentials. Tighten packaging tests/documents as necessary.
**Risk:** Runtime/GUI and provider/MCP features may remain incomplete; missing Apple Developer ID/notarization blocks production.
**Dependencies:** NC-TEAM-001/002, six required GitHub checks, APP identity/version, macOS tooling and available disk.
**Approval boundary:** No bypass of develop reviewer gate; no production auto-updater; no signing with keys not explicitly configured. A test-only artifact or preview distribution is not equivalent to stable publication.

| ID | Work | Actual owner | Acceptance criteria | Status |
| --- | --- | --- | --- | --- |
| MAC-01 | Audit Mac build prerequisites and upstream release identity | TEAM_LEAD_SERIAL | Host/toolchain/product identity inventory | [x] |
| MAC-02 | Implement a deterministic no-Finder release-mode DMG packager | TEAM_LEAD_SERIAL | Packager, verifier and static contract tests delivered; real volume result pending | [x] |
| MAC-03 | Run source regression and production frontend bundle | TEAM_LEAD_SERIAL | Node 159 PASS, release 33 PASS, focused Mac 3 PASS, R0 PASS, Vite/tsc PASS | [x] |
| MAC-04 | Build optimized macOS ARM64 .app and DMG | TEAM_LEAD_SERIAL | GitHub-hosted optimized preview DMG + SHA256 checked on local M4 | [x] |
| MAC-05 | Run non-destructive native startup/exit and installer smoke | TEAM_LEAD_SERIAL | App process alive 8 sec isolated HOME; WebKit sandbox warning, real GUI E2E PENDING | [x] |
| MAC-06 | Record final validation, blockers and reviewer gate | TEAM_LEAD_SERIAL | Exact artifact SHA, CI/Playwright evidence and NO-GO release blockers documented | [x] |
| MAC-REVIEW | Genuine independent code reviewer / notarization approval | INDEPENDENT_REVIEWER_PENDING | Required before protected integration and production release | [ ] |

**Initial Team Lead decision:** GO to implement source/test-only Mac installer. NO-GO for production label or protected branch merge without genuine independent review and remaining R0–R8 gates.

## NC-TEAM-002 Remaining-work ledger (baseline)
- 24 engineering acceptance packages: 3 marked source complete, 21 not accepted.
- 7 branding epics: 7 pending.
- R0–R8 detailed acceptance checks: 61 unchecked. These counts overlap.
- GitHub PR #26 is open and CI 6/6 passes; no independent approval. PR #27 closed after its commits entered #26.
- After this task, update actual evidence and blockers here; do not count a preview DMG as R6/R8 acceptance.

**Build issue diagnosed:** macOS 27 dyld rejects stripped Rust proc-macro dylibs: `E0463` and `mis-aligned LINKEDIT string pool`. Existing `[profile.release] strip=true` inadvertently applies to host proc macros. Implemented `[profile.release.build-override] strip=false`; app release profile remains stripped. A clean local build is underway; no success claim until completion.

**Source-level evidence:** `npm test` 156/156 PASS, `npm run test:release` 33/33 PASS, `npm run test:nexora:r0` PASS, `node --test tests/mac-preview-build.test.mjs` 3/3 PASS, `bash -n` PASS, `git diff --check` PASS, frontend tsc+Vite production PASS. GitHub R6 workflow now calls Mac-specific no-Finder preview packager.

**Team Lead source review:** reviewed Cargo release override, unsigned installer checks, CI artifact-only policy and no native-key requirement. No stable update channel, no independent reviewer, no shell egress authorization added.
**Team Lead verdict:** SOURCE REVIEW GO for PR/CI submission only. Protected branch merge NO-GO without independent reviewer, even if all six checks pass. Mac Native acceptance NO-GO until real preview build and GUI/protocol checks. Production release NO-GO, Developer ID certificates unavailable.

**Current risk:** Mac build in-progress; 0 validated Apple Developer ID signing identities; 61 detailed roadmap checks pending by historical checklist (overlap with 21/24 engineering and 7/7 branding).

## Native Mac M4 evidence and final review — 2026-10-23

- **Host:** Mac mini M4 arm64, macOS 27.2, 16 GB RAM.
- **GitHub Mac builder:** R6 unsigned preview job PASS at [run 39890157346](https://github.com/TheerasakPing/nexora-code/actions/runs/39890157346), 9m21s.
- **Artifact:** `Nexora-Code-0.1.0-macOS-arm64-PREVIEW-UNNOTARIZED.dmg` plus .sha256, verified hash `7574c3ff42c6de06e303f2d0bda3a343bb09c72675e91ec2b71b5825520a74a8`.
- **Independent Mac M4 verification:** sha256, `hdiutil verify`, mount readonly, code signature seal, bundle version/ID and ARM64 + upstream updater isolation PASS.
- **Native startup:** 8 seconds alive from DMG-mounted and copied-to-temp app, isolated HOME, RSS ~85 MB. **Not a real GUI acceptance**; observed WebKit `sandbox_extension_issue_file_to_process` warning. No installed app was overwritten.
- **Browser UI preview:** targeted Architect, Settings and Terminal Playwright 58/58 PASS (browser only). Node full suite 159/159 PASS; release scripts 33/33 PASS; R0 safety PASS.
- **DevID/notarization:** 0 valid Apple Developer ID certificates; Production signing BLOCKED. Genuine independent reviewer absent. Source checks and CI are not live provider, multi-worker and migration proof.
- **Local optimized Rust:** macOS 27 proc-macro strip defect reproduced; fixed with release build-override. GitHub MAC hosted build PASS. Local redundant 27.2 build was stopped after using GitHub bundle on real Mac; no second compiled local artifact claimed.
- **Completed tasks in this round:** MAC-01 through MAC-06 *for the limited preview DoD*; MAC-REVIEW still PENDING. R6/R8 production acceptance not closed.
- **Remaining backlog:** 21/24 engineering acceptance packages unchecked (3 source-only complete); 7/7 BR epics and 61 detailed R0–R8 checks unchecked per dated master snapshot. Categories overlap, not additive.
- **Top next priorities:** (1) trusted independent reviewer and protected integration; (2) native WKWebView sandbox/GUI/providers/PTY test; (3) secure R1/R2 process recovery, MCP vault/consent and Apple Developer ID notarization.
- **Team Lead GO/NO-GO:** GO to deliver test-only Mac ARM64 preview bundle and source PR; NO-GO to say feature-complete/Production; NO-GO to protected develop merge without independent review; NO-GO to enable unsigned automatic updater.


## G. NC-TEAM-003 — Subtask Verification Matrix (retrospective audit, not invented tests)

This planner existed **before NC-TEAM-003**. Historic checked [x] boxes
record earlier **narrow source-only** milestones and do NOT assert NC-TEAM-003
E2E/Smoke acceptance. Every historic subtask below is explicitly PENDING
until the Team Lead rechecks actual test command/OS/SHA/log and signs each.
Missing or unverified native evidence remains an acceptance NO-GO. The
retrospective entries are not fabricated executions or agent delegations.

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| MAC-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| MAC-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| MAC-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| MAC-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| MAC-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| MAC-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| MAC-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |


**Team Lead GO/NO-GO (retrospective):** SOURCE PR GO for further review only.
Individual NC-TEAM-003 gates remain **PENDING** (not accepted); the prior
task's source-level status is not revoked or silently upgraded. Protected
merge and production NO-GO until genuine independent review / native gates.


### NC-TEAM-003 acceptance tracker (not inferred)

- [ ] NC-TEAM-003: Unit/Contract, E2E and Smoke verified per subtask.
- [ ] NC-TEAM-003: All applicable tests and native acceptance requirements passed.
- [ ] NC-TEAM-003: Real independent reviewer approved protected merge and release.
