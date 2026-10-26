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

## A. Intake / acceptance boundaries

- **Exact user request:** "ทำต่อสิจนว่าจะเสร็จ" — continue Nexora Code to completion.
- **Task ID / date:** NC-NATIVE-20261026 / 2026-10-26.
- **Phase / priority:** R6 local macOS native and production release gates / P1.
- **Branch / base SHA / worktree:** `feat/20261026-local-native-acceptance` from `6654a3b`, `/Volumes/Data/workspace/git/nexora-code-native-acceptance`.
- **Team Lead (real actor):** ChatGPT integration lead / `TEAM_LEAD_SERIAL` (no real agent delegation tool was invoked).
- **Available worker agents:** NONE assigned; descriptions of QA worker *roles* above are task classifications, **not claims of independent agents**.
- **Scope:** Add non-mutating command-line acceptance probe, automated parser tests, operator checklist and upstream-doc warning. Only Nexora Code files are changed.
- **Out of scope:** Modifying production data, login tokens, macOS privacy permissions, publishing releases, signing without an Apple identity, or falsely asserting a human approval.
- **DoD:** Valid installed bundle/process/SQLite recognized; mismatched bundle, missing process and corrupted SQLite rejected; ad-hoc signature cannot count as signed release; explain manual blockers.
- **Dependencies:** PR #43 merged; current installed app; SQLite CLI; successful CI unsigned installer smoke.

## B. Team Lead decomposition and assignments

| ID | Deliverable / acceptance criteria | Real worker ID or TEAM_LEAD_SERIAL | Owned files / branch / worktree | Depends on | Status |
| --- | --- | --- | --- | --- | --- |
| T-01 | Read-only installed app verifier and gate logic unit tests | TEAM_LEAD_SERIAL | `scripts/release/verify-installed-macos*.mjs`, `package.json` / separate acceptance worktree | none | [x] |
| T-02 | Native operator checklist, upstream warning and recorded evidence | TEAM_LEAD_SERIAL | `docs/nexora/NATIVE-MACOS-ACCEPTANCE.md`, `docs/releasing.md` / same worktree, serial | T-01 | [x] |
| T-QA | Lead diff inspection; check scripts contain no credential reads and report blocked release gates | TEAM_LEAD_SERIAL (self-review, not independent) | entire diff, read-only | T-01, T-02 | [x] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| T-01 | TEAM_LEAD_SERIAL | PASS — `npm run test:release`: 40 pass, 1 existing skip; parser-specific 4/4 pass | PASS — `npm run test:native:installed` on live macOS arm64 validates actual installed bundle, running process and SQLite | PASS — direct CLI output `localSmokePass=true`, signed/notarized gates BLOCKED | GO — serial lead code review (not independent) | Mac 2026-10-26; app `com.theerasakping.nexoracode`, local SQLite `ok`; `npm test` 211/211, `npm run build` PASS; no private data exported |
| T-02 | TEAM_LEAD_SERIAL | N/A: documentation-only, lead accepted 2026-10-26, link and script contract checked against current files | PASS — `npx playwright test tests/token-saver.spec.ts --workers=1`: 3/3 browser-mock E2E, not native GUI | PASS — `npm run test:native:installed` includes installed app and SQLite acceptance; docs paths exist | GO — serial lead doc verification (not independent) | Upstream release page now clearly marked as **not** Nexora deployment instructions; real native GUI E2E BLOCKED by macOS Accessibility permission |

## C. Evidence checklist

- [x] Planner committed before source edits (`fcdd243`).
- [x] Roles and serial dependencies recorded; no fictitious separate agents claimed.
- [x] Tests executed: Unit 211/211 PASS; release contract 40 PASS / 1 existing skip; frontend build PASS; browser mock 3/3 PASS; installed native process and database read-only smoke PASS.
- [x] Source and database are not changed by the verifier; no plaintext credentials or account data are logged.
- [x] Host installer smoke across three OS targets confirmed in GitHub run `40261370962`.
- [x] Code/diff checked and release approval segregated from local smoke.
- [ ] Independent human production review: reviewer unavailable.
- [ ] Native GUI end-to-end acceptance: macOS Accessibility permission required.
- [ ] Signed/notarized production release: Developer ID identity missing.

## D. Worker handoff ledger

| Task ID | Worker ID | Commit / patch | Tests / evidence | Risk / incomplete | Team Lead review |
| --- | --- | --- | --- | --- | --- |
| T-01 | TEAM_LEAD_SERIAL | Current worktree diff to be committed | Release contract 40/1, actual macOS native check PASS, build and Node unit green | Native GUI is not verified and cannot be conflated with CLI smoke | GO for PR submission only |
| T-02 | TEAM_LEAD_SERIAL | Current worktree diff to be committed | Upstream guidance warning, operator checklist, Ponytail browser 3/3 | Signing and human independent review cannot be self-approved | GO for PR submission only |

## E. Team Lead final review / handoff

- **Actual reviewer:** ChatGPT integration lead (serial self-inspection), **not independent**.
- **Review scope:** New script does only read-only macOS commands; no path outside the installed app and app-data DB is written. The code does not read keychain secrets and never treats ad-hoc signing as Developer ID.
- **Risk:** Finder UI and actual interactive provider/model selection are not automated in this work item. Updater signing, macOS notarization and human review are blocked externally.
- **CI:** New branch CI PENDING until pushed. PR #43 merged into `develop`; prior GitHub installer smoke successful on three platform runners.
- **Final GO/NO-GO:** GO for separate PR review and source integration **after CI**. NO-GO for production release and for marking live GUI E2E accepted.
- **Date:** 2026-10-26.

## F. NC-TEAM-002 remaining-work ledger

- **Snapshot / references:** 2026-10-26; `docs/nexora/CHECKLIST-PLANNER.md` and `docs/nexora/PROGRESS-SNAPSHOT-2026-10-23.md` are older than this update; detailed phase counts **UNVERIFIED**.
- **Acceptance (this task):** 2 implementation/documentation subtasks accepted out of 2, including specific release blocking assertions; 3 externally gated items remain (native GUI manual acceptance, human reviewer, Developer ID/notarization). These groups overlap with the broader roadmap and must not be summed as project percentage.
- **Engineering 24-package and seven-branding-epic accepted counts:** UNVERIFIED, not inferred from PR #43 integration.
- **R0–R8 detailed item counts:** UNVERIFIED pending refreshed master checklist evidence.
- **Open PRs/issues:** UNVERIFIED for the repository as a whole; this task creates one separate PR.
- **Top unblock actions:** (1) owner grants macOS Accessibility for approved UI automation; (2) Apple Developer administrator provisions signing/notarization; (3) independent collaborator reviews and accepts release.
- **Delivery:** GO for submitting code and local runbook; **Protected merge:** PENDING new PR checks; **Native GUI acceptance:** NO-GO until operator evidence; **Production release:** NO-GO until signing/notarization and independent approval.
