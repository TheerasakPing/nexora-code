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
