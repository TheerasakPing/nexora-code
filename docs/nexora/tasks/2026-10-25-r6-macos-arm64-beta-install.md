# Nexora Code — macOS arm64 beta installation, isolation and remaining gates (2026-10-25)

## A. Intake / Lead
- **User:** "เหลืออะไรอีกบ้าง ตดตั้งใ้ใช้งานบนแมคก่อนได้ไหม" — enumerate remaining work and install **Nexora Code ONLY** on connected Apple-silicon Mac for user testing.
- **Project:** `TheerasakPing/nexora-code` (fork PandaCode); **not Nexora Studio**. Branch `ops/20261025-nexora-code-mac-beta` based on PR #45 code SHA `c2239b0`.
- **Environment:** macOS 27.2 arm64, Mac currently has **`/Applications/Nexora Code.app`** version 0.1.0 with app bundle id `com.theerasakping.nexoracode` and `/Applications/Nexora Studio.app`. Neither may be overwritten, uninstalled, killed or modified; do not read user credentials, shell secrets, session stores.
- **Approach:** Prefer GitHub Action-built **UNSIGNED-TEST-ONLY macOS arm64 artifact** from Nexora Code PR #44 checked at run 40161607533, SHA `669e9bc`; PR #45 since parent changes only docs and E2E test, not app source. Verify workflow/checkout SHA, archive content/bundle identity, resource integrity/signing. If safe, install **separate named beta** into `~/Applications/Nexora Code Beta.app` only if nonexistent; consider shared bundle identifier/app data risk. Do not run with user's real agent credentials or access home project data.
- **Scope:** Just source-based tester install and smoke, no protected merge / publishing / production claim, no altering updater/server/harness settings.
- **Owner delegation:** T-01 Lead artifact verification; T-02 independent Codex read-only safety examiner (external app provenance/installed app collision); T-03 Lead isolated beta install and native launch smoke; T-04 Lead repo acceptance ledger.
- **DoD:** artifact source exactly matches current production code, SHA/architecture/bundle verifiable, current installed app intact, beta installed and basic process GUI evidence (or marked BLOCKED), honest remaining-work counts and links.
- **Risks:** unsigned ad-hoc test binary Gatekeeper; duplicate bundle ID can share preferences/keychain and updater. If identifier or data cannot be isolated, install-only/no launch, or launch under truly isolated macOS test user instead of risking personal data. No Gatekeeper bypass nor approval dialogs.
- **Dependencies:** GitHub artifact readable/unexpired; macOS runtime can launch in user GUI session; disk free >= 500 MB (34GiB currently).
## B. Tasks and assignments
| ID | Deliverable | Real worker/Lead | Ownership | Depends on | Status |
|---|---|---|---|---|---|
| T-01 | Compare PR #45 code to #44 build SHA, exact artifact and archive contents | TEAM_LEAD_SERIAL | Read-only GH+archive; this checklist | none | [ ] |
| T-02 | Independent installer/data/updater sandbox safety audit | Codex CLI independent read-only, if available | source+CI+Mac app metadata | T-01 | [ ] |
| T-03 | Install separate Mac beta app only when safe; smoke process/window/GUI no secret access | TEAM_LEAD_SERIAL | `~/Applications/Nexora Code Beta.app` (not `/Applications`) | T-01,T-02 | [ ] |
| T-04 | Verified remaining R0-R8/engineering/brand/PR accounting and report | TEAM_LEAD_SERIAL | checklist + GH read-only | T-03 | [ ] |
| T-QA | Lead validate hash, no clobber, native QA, explicit GO/NO-GO | TEAM_LEAD_SERIAL | read-only QA + evidence | T-01..T-04 | [ ] |
## G. NC-TEAM-003 verification matrix
| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A |
|---|---|---|---|---|---|---|
| T-01 | TEAM_LEAD_SERIAL | PENDING | N/A: provenance metadata only, Lead to verify | PENDING: archive extraction/hash | PENDING | GH run 40161607533 |
| T-02 | Codex read-only | PENDING | N/A: read-only safety analysis | N/A: safety review does not run app | PENDING | exact review log |
| T-03 | TEAM_LEAD_SERIAL | PENDING: SHA/bundle checks | PENDING: native GUI/window | PENDING: launch/liveness | PENDING | Mac beta location, evidence |
| T-04 | TEAM_LEAD_SERIAL | PENDING: source count | N/A: accounting only | N/A: accounting only | PENDING | dated snapshot and PR links |
## D. Handoff and security
- No review is called independent until actual Codex read-only response. Source plan goes in an ops branch; no forced PR #45 rerun from unrelated installation documents.
## E. Final gate
- Delivery GO/NO-GO PENDING; Mac native first-run GO/NO-GO PENDING; protected merge NO-GO until reviews, Production NO-GO. No claim full native 3-OS QA from one Mac run.
## F. Mandatory remaining-work snapshot
- Engineering packages: UNVERIFIED / 24 until inspecting living master.
- Branding epics: UNVERIFIED / 7 pending master.
- R0-R8 detailed: UNVERIFIED; cannot conflate with PR checks.
- GitHub PR/Issues: PENDING current GH evidence.
- Source-integrated, native-tested, production-ready separately evaluated.

## Verified result — 2026-10-25 22:xx Asia/Bangkok
### GitHub provenance and source equivalence
- [x] T-01 GitHub Actions **Nexora Code** run `40161607533`, conclusion SUCCESS, checked-out head SHA `669e9bc948329a42e6802f742a246a6ae8910b74`, on PR #44 branch. Original artifact `UNSIGNED-TEST-ONLY-darwin-aarch64-124aeb976cbf92d67cfe4744fd7b7ffbc5948c3a` extracted `Nexora-Code-0.1.0-macOS-arm64-PREVIEW-UNNOTARIZED.dmg` and companion checksum.
- [x] DMG SHA-256 **d6aa7cde812b298fa89a4b904e94dcf2b02224dd2959accfc6e188d4448edcbd**, verified against supplied `.sha256`. `hdiutil attach -readonly -nobrowse` mounted with valid filesystem checksums. App's `nexora-code` Mach-O arm64 binary SHA-256 **79ebbba9b2e40fe2ca5298154a6efc49e93aa59817d15b55e7e8f81859f55a13**. App version `0.1.0`. Signature `codesign --verify --deep --strict` PASS; `spctl -a` **REJECTED** (ad hoc, unnotarized) as warned in readme. No Gatekeeper override performed.
- [x] Source diff from CI build `669e9bc` to PR #45 `c2239b0` contains **only two task Markdown files plus `tests/ui.spec.ts`**, hence no application source behavior delta; the Preview artifact is production-source equivalent, not necessarily a release from current PR45 CI.
### Existing install / risk
- [x] Old `/Applications/Nexora Code.app` 0.1.0 has CFBundleExecutable `pandacode` and SHA-256 **4147fa8e97bca2ba2b55513f0f864aff9fc4da0ef3a605f81b523fc4b3ef7ae4**. Old `/Applications/Nexora Studio.app` is a different app. Both remained untouched; original Nexora Code binary SHA-256 rechecked unchanged after copying.
- [x] T-02 independent real Codex read-only reviewer `/tmp/nxc-mac-beta-install-independent-review.log` verdict **REQUEST CHANGES / NO SAFE LAUNCH** in current user's macOS account. DMG new and old app both have `com.theerasakping.nexoracode` bundle ID, so app data directory and `kingcode.sqlite3` + macOS Keychain services can overlap. Startup invokes `discard_empty_drafts()`; do not assume read-only GUI start. Updater verified disabled via `src/app-updates.ts` R0 config. Separate renamed .app does **not** isolate app data. Ad hoc unsigned Gatekeeper rejects app; do not disable global security.
### Actual Mac installation and triad
- [x] T-03 **COPY INSTALL ONLY**, no GUI launch: `ditto --rsrc --extattr` installed `/Users/baemon/Applications/Nexora Code Beta.app` (24MB), and separately `/Users/Shared/Nexora Code Beta.app` (24MB) for an isolated standard test user. Neither destination pre-existed. Both bundles verified with `codesign --verify --deep --strict` and exact binary SHA match; Original Nexora Code app checksum unchanged; no process spawned and no user/Keychain/agent session data accessed.
- [ ] **Native app E2E and launch Smoke BLOCKED** for current user by duplicate Bundle ID/data/Keychain collision + unsigned/non-notarized Gatekeeper policy. Needs an existing or newly user-created **separate macOS standard test account**, no real provider tokens, source trust confirmation, and user-driven Apple Security Open Anyway if needed. No alternate account was created or logged in by automation. User may open only after performing this test-account setup. A source or installed artifact is not native GUI PASS.
- [x] T-04 GH PR45 hosted source checks on commit `c2239b0`: **11/11 SUCCESS** (Typecheck+unit; frontend; Expanded UI; macOS Full UI 4/4; Rust Linux; Native Rust macOS arm64, Windows x64, Linux x64). PR45 remains Draft stacked on #44 -> #43 -> `develop`; no protected branch merge. GitHub live open PR list **12** (#28,29,30,31,33,34,35,36,37,43,44,45); GitHub Issues is **DISABLED** (not zero completed).
### NC-TEAM-003 verification matrix — actual
| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence |
|---|---|---|---|---|---|---|
| T-01 | TEAM_LEAD_SERIAL | PASS: DMG and published checksum, bundle ID/version/arm64, exact source diff | N/A: provenance analysis, Lead accepted | PASS: readonly DMG mount, `codesign --verify`; **not a GUI Smoke** | GO for artifact identity only | GitHub run 40161607533, hash above |
| T-02 | Independent Codex read-only | PASS: app config + actual legacy app inspection | N/A: static threat review, Lead accepted | N/A: review must not launch user app | **NO-GO** for launch in current Mac account | `/tmp/nxc-mac-beta-install-independent-review.log` |
| T-03 | TEAM_LEAD_SERIAL | PASS: installed-arm64 code sign and SHA checks; old checksum unchanged | BLOCKED: no isolated test account/session | BLOCKED: Gatekeeper/no isolated app data | **GO for non-running copy; NO-GO for native acceptance** | `ditto`, `codesign`, `shasum`, no PID |
| T-04 | TEAM_LEAD_SERIAL | PASS: checked dated master + live PR count and checks | N/A: reporting only, Lead accepted | N/A: reporting only, Lead accepted | GO for honest status report | GH API & `docs/nexora/PROGRESS-SNAPSHOT-2026-10-23.md` |
### Remaining work / acceptance counts (overlapping, do NOT sum)
- **Engineering 24 acceptance packages:** latest dated master snapshot 2026-10-23 **3 SOURCE-COMPLETE ONLY / 24**, **21 unchecked**. It has not been re-reviewed against all acceptance DoDs today; those numbers describe checkmarks, not finished feature count.
- **Brand BR-01–BR-07:** **0 approved / 7**, 7 pending owner acceptance.
- **R0–R8 detailed acceptance:** **11 narrow checks ticked / 72**, **61 unchecked** by 2026-10-23 snapshot; the current master still has 12 checked and 112 unchecked across all sections (including duplicated DoD/template rows). Treat 61 unchecked as backlog requiring fresh native/review evidence, not estimate of coding percent.
- **Current GH PRs:** 12 open, including PR45->44->43 stacked UI and PR28–37 R1 chains and PR35 develop->main; no merges in this task. GitHub Issue tracker disabled, not 0/0.
- **Top blockers:** (1) safe macOS beta isolation and real native GUI E2E (separate test account now; future beta bundle ID + provider/keychain service isolation); (2) R1/R2 native runtime/permissions and MCP vault/consent, R3/R4 real multi-agent/DAG durability; (3) Windows/Linux/macOS installation/upgrades, GUI/provider E2E; (4) R7 security + 8-workers ×2h and power/crash simulation; (5) R8 Apple Developer ID notarization, signing/SBOM/updater/rollback/human security and Owner signoff.
### Team Lead final decision
- **Install-copied-artifact: GO / DONE**, signed checksum/provenance and no overwrite or launch; **Mac native usability: NO-GO pending isolated test account launch and real GUI acceptance**; **Protected merge: NO-GO** without project owner and unstacked PR review even though PR45 CI green; **Production release: NO-GO** until R0–R8 applicable DoDs, independent human and 3-OS native release checks. No claims of signed production or 100%.
