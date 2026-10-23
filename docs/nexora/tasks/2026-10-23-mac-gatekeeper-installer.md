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
| UX-01 | TEAM_LEAD_SERIAL | Embedded bilingual install instructions explaining drag-to-Applications + Apple per-app Open Anyway | [ ] |
| SEC-02 | TEAM_LEAD_SERIAL | Extend DMG verifier to flag rejection rather than calling installability PASS | [ ] |
| QA-01 | TEAM_LEAD_SERIAL | Unit tests + actual DMG mounted, hash/code-signature, Gatekeeper label validated | [ ] |
| INT-01 | TEAM_LEAD_SERIAL | Push changes to consolidated PR #26 and get exact-head CI evidence | [ ] |
| PROD-01 | OWNER / APPLE DEVELOPER ACCOUNT | Developer ID sign, hardened runtime, notarize + staple, macOS Finder installer E2E | [ ] |
| REVIEW | Independent code reviewer (unassigned) | Review PR before merge to protected develop | [ ] |

## Decision / constraints
**GO:** safely explain Apple's native per-app Open Anyway exception **only after SHA-256 and source verification**; no automatic action changes Gatekeeper. Preview remains experimental.
**NO-GO:** claim a "fully installable trusted" notarized Mac app, modify `spctl --global-disable`, recursive `xattr -cr`, silent quarantine removal, generate fake Developer ID credentials, or bypass required independent review.

**Remaining baseline:** engineering acceptance 21/24 open; branding 7/7 open; detailed R0–R8 checks 61 open. These are overlapping views, never additive.
**Evidence and next steps:** to be updated after implementation and test.
