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
| MAC-02 | Implement a deterministic no-Finder release-mode DMG packager | TEAM_LEAD_SERIAL | Ad-hoc seal, dmg verify/mount, app identity, checksum, no upstream updater | [ ] |
| MAC-03 | Run source regression and production frontend bundle | TEAM_LEAD_SERIAL | Actual PASS counts and logs | [ ] |
| MAC-04 | Build optimized macOS ARM64 .app and DMG | TEAM_LEAD_SERIAL | Real filesystem artifact and SHA-256 | [ ] |
| MAC-05 | Run non-destructive native startup/exit and installer smoke | TEAM_LEAD_SERIAL | Process/log evidence with bounded lifetime, no installed app overwritten | [ ] |
| MAC-06 | Record final validation, blockers and reviewer gate | TEAM_LEAD_SERIAL | Exact SHA, Mac evidence, detailed remaining-work ledger and GO/NO-GO | [ ] |
| MAC-REVIEW | Genuine independent code reviewer / notarization approval | INDEPENDENT_REVIEWER_PENDING | Required before protected integration and production release | [ ] |

**Initial Team Lead decision:** GO to implement source/test-only Mac installer. NO-GO for production label or protected branch merge without genuine independent review and remaining R0–R8 gates.

## NC-TEAM-002 Remaining-work ledger (baseline)
- 24 engineering acceptance packages: 3 marked source complete, 21 not accepted.
- 7 branding epics: 7 pending.
- R0–R8 detailed acceptance checks: 61 unchecked. These counts overlap.
- GitHub PR #26 is open and CI 6/6 passes; no independent approval. PR #27 closed after its commits entered #26.
- After this task, update actual evidence and blockers here; do not count a preview DMG as R6/R8 acceptance.

**Team Lead verdict:** PENDING, must be revised after actual tests and final review.
