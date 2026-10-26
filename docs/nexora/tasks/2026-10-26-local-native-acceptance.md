# Planner — Native macOS installation acceptance and release transparency

Date: 2026-10-26 · Scope: Nexora Code only · Worktree: `feat/20261026-local-native-acceptance`

## Lead and agent responsibilities

- [x] Integration lead: verify PR #43 merged into `develop` (`6654a3b`) and installer smoke for all 3 OS targets passed
- [x] Runtime QA worker: inspect installed macOS app, bundle identifier, process, code signature and signing identities
- [x] Data QA worker: run read-only SQLite integrity on existing data; back up using SQLite backup API and verify independent backup integrity
- [ ] Native QA worker: add repeatable, non-mutating local installed-app acceptance script, with mocked gate logic tests
- [ ] Security/release reviewer (lead): verify no credentials or database contents are collected or exposed by the script, and unsigned native builds cannot be represented as signed/notarized
- [ ] QA: build, tests, local smoke check, clean diff, commit, push PR
- [ ] Human independent reviewer: review package security and approve externally before production tag
- [ ] Apple Developer operator: provide valid Developer ID Application identity, signing/notarization credentials and publish entitlement
- [ ] Native UI tester: grant macOS Accessibility to test runner and capture actual interaction evidence

## Known acceptance evidence

- Current Mac installation: `/Applications/Nexora Code.app`, running, bundle `com.theerasakping.nexoracode`
- SQLite source `PRAGMA integrity_check = ok`; SQLite backup also `ok`; data remains on user machine
- `security find-identity -p codesigning` reports zero valid identities
- Current `.app` is ad-hoc signed, not Developer ID signed or notarized
- AppleScript `System Events` UI test is blocked by Accessibility permissions, not marked passed
- CI unsigned installer smoke run 40261370962: macOS ARM64, Windows x64, Linux x64 all successful

Acceptance must distinguish runnable local unsigned app from externally distributable notarized production release.
