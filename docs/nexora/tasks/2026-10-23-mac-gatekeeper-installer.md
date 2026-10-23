# Task Checklist — Fix Gatekeeper installer guidance (Mac M4)

**Request / observed failure:** User reports macOS 27 dialog: "Could not install — This Mac could not verify the app, so nothing was installed" for Nexora Code ARM64 Preview.
**Date:** 2026-10-23 (Asia/Bangkok). **Priority:** P0 distribution usability/security.
**Team Lead:** ChatGPT, `TEAM_LEAD_SERIAL`. No independent workers assigned, reviewer still pending.
**Source baseline:** PR #26 head `306f7cd`; release artifact run 39890157346.
**Target:** Mac mini M4/macOS 27.2; do not alter system-wide security settings, existing /Applications installation, macOS user data, or Apple Keychain.

## Investigation and root-cause evidence
- [x] `codesign -dv` reports `Signature=adhoc`, `TeamIdentifier=not set`.
- [x] `spctl --assess --type execute -vv` **REJECTS** the mounted `Nexora Code.app`.
- [x] `xcrun stapler validate`: **no ticket stapled**, exit 65.
- [x] `spctl --status`: Gatekeeper **enabled**.
- [x] `security find-identity -v -p codesigning`: 0 valid identities.
- [x] Existing DMG SHA-256 already verified: `7574c3ff42c6de06e303f2d0bda3a343bb09c72675e91ec2b71b5825520a74a8`.

## Work packages and DoD
| Item | Owner | Gate | State |
| --- | --- | --- | --- |
| SEC-01 | TEAM_LEAD_SERIAL | Root cause: intact ad-hoc build != Gatekeeper approved | [x] |
| UX-01 | TEAM_LEAD_SERIAL | Embedded bilingual install instructions explaining drag-to-Applications + Apple per-app Open Anyway | [x] |
| SEC-02 | TEAM_LEAD_SERIAL | Extend DMG verifier to flag rejection rather than calling installability PASS | [x] |
| QA-01 | TEAM_LEAD_SERIAL | 4/4 focused tests; locally repack from verified CI app; sha256, mount, signature PASS, Gatekeeper REJECTED correctly | [x] |
| INT-01 | TEAM_LEAD_SERIAL | Push changes to consolidated PR #26 and get exact-head CI evidence | [ ] |
| PROD-01 | OWNER / APPLE DEVELOPER ACCOUNT | Developer ID sign, hardened runtime, notarize + staple, macOS Finder installer E2E | [ ] |
| REVIEW | Independent code reviewer (unassigned) | Review PR before merge to protected develop | [ ] |

## Decision / constraints
**GO:** safely explain Apple's native per-app Open Anyway exception **only after SHA-256 and source verification**; no automatic action changes Gatekeeper. Preview remains experimental.
**NO-GO:** claim a "fully installable trusted" notarized Mac app, modify `spctl --global-disable`, recursive `xattr -cr`, silent quarantine removal, generate fake Developer ID credentials, or bypass required independent review.

**Remaining baseline:** engineering acceptance 21/24 open; branding 7/7 open; detailed R0–R8 checks 61 open. These are overlapping views, never additive.
**Evidence and next steps:** to be updated after implementation and test.

## Verified fix and actual Mac test results

- **Source files:** `scripts/nexora/MAC-PREVIEW-INSTALL.txt`, `scripts/nexora/build-mac-preview-dmg.sh`, `scripts/nexora/verify-mac-preview-dmg.sh`, `tests/mac-preview-build.test.mjs`, `docs/nexora/MAC-GATEKEEPER-INSTALLATION.md`.
- `node --test tests/mac-preview-build.test.mjs`: **4/4 PASS**; static policy enforces no global security disablement or quarantine stripping.
- Verified original GitHub CI ARM64 app downloaded in previous task, copied to local workspace cache only (no /Applications modification), and repackaged with prominent bilingual README: **PASS**.
- New local DMG `release/mac-preview/Nexora-Code-0.1.0-macOS-arm64-PREVIEW-UNNOTARIZED.dmg` SHA256:
  `f4cc4dc71bc28029d73d34b1952a8522757619558c514c64768973a73a4caabf`.
  This is **different** from original CI DMG because README was added.
- `bash scripts/nexora/verify-mac-preview-dmg.sh`: SHA-256, volume mount, ARM64, signature and fork updater integrity **PASS**; `GATEKEEPER_ASSESSMENT=REJECTED` explicitly shown (expected, not a passed distribution gate).
- **Mac Gatekeeper assessment:** NO-GO for normal one-click installation. Only Apple's manual per-app Open Anyway after explicit verified-source consent; not an automated workaround.
- **Credentials:** Developer ID Application and approved Apple Notarization account must be provisioned by the project owner. No certificates or signing secrets were available in verified environment.
- **QA scope:** no live macOS visual UI acceptance or notarized package yet; independent reviewer not assigned. Do not lower product acceptance gates.
- **Team Lead decision:** GO to deliver user-facing diagnostic/instructions and safe unsigned preview (source-only); NO-GO stable Release / normal Gatekeeper install or protected develop merge without independent reviewer.


## G. NC-TEAM-003 — Subtask Verification Matrix (retrospective audit, not invented tests)

This planner existed **before NC-TEAM-003**. Historic checked [x] boxes
record earlier **narrow source-only** milestones and do NOT assert NC-TEAM-003
E2E/Smoke acceptance. Every historic subtask below is explicitly PENDING
until the Team Lead rechecks actual test command/OS/SHA/log and signs each.
Missing or unverified native evidence remains an acceptance NO-GO. The
retrospective entries are not fabricated executions or agent delegations.

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| SEC-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| UX-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| SEC-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| QA-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| INT-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |


**Team Lead GO/NO-GO (retrospective):** SOURCE PR GO for further review only.
Individual NC-TEAM-003 gates remain **PENDING** (not accepted); the prior
task's source-level status is not revoked or silently upgraded. Protected
merge and production NO-GO until genuine independent review / native gates.


### NC-TEAM-003 acceptance tracker (not inferred)

- [ ] NC-TEAM-003: Unit/Contract, E2E and Smoke verified per subtask.
- [ ] NC-TEAM-003: All applicable tests and native acceptance requirements passed.
- [ ] NC-TEAM-003: Real independent reviewer approved protected merge and release.

**Team Lead verdict:** SOURCE REVIEW GO for PR submission only; NC-TEAM-003 subtask test triads remain PENDING. Independent reviewer has NOT approved protected merge.
