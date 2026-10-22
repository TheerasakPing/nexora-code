# Nexora Code — Rebranding & Product Identity Master Plan

**Document:** NC-BRAND-001 · Version 1.0 · 2026-10-22
**Status:** Proposed; pending Product Owner approval
**Origin:** Fork of [PandaCode](https://github.com/aicodeking/pandacode)

## 1. Objective and brand governance

Rebrand this PandaCode fork as a distinct product without degrading existing functionality or compromising users' data, code, sessions, credentials, provider integrations, or update trust boundaries.

**Nexora Code remains a working name.** The final product name, brand mark, trademark checks, icon and visual identity require explicit Project Owner approval before public release. Do not interpret a working bundle name as signed-off branding.

**License/attribution is non-negotiable:** Show "This is a fork of PandaCode" and link to https://github.com/aicodeking/pandacode in README, About, and other required notices. Preserve upstream LICENSE and third-party acknowledgements.

## 2. Actual inventory — implemented versus pending

| Area | Current code state | Outstanding gate |
| --- | --- | --- |
| Display name, title and Tauri app ID | Source-level implemented: Nexora Code; distinct identifier | Native UI and 3-OS verification |
| About / License attribution | Source-level implemented | Screens and platform packaging review |
| Provider secure-store namespaces | Partially implemented | Test macOS, Windows and Linux OS stores |
| Upstream auto-updater | Disabled in fork | Fork-owned signing key, endpoint and release pipeline |
| Logo, splash, icon, tray, Dock, installers | Not signed off | Complete original visual asset family per platform |
| Design system | Not signed off | Tokens, typography, layout, shadows, light/dark, accessibility |
| Strings, errors, dialog and menus | Partly legacy | Full user-visible string inventory and text review |
| Rust executable | Upstream pandacode name retained | Controlled rename, native builds and compatibility test |
| Database | Legacy kingcode.sqlite3 retained | Backup, migration, rollback tests; no in-place rename |
| Internal tools | Legacy pandacode_* retained for compatibility | Protocol adapter or preserve stable contract |
| GitHub / support / website / privacy | Partial; local repo unpushed | Confirm ownership and valid pages; no invented URLs |
| Publisher signing / notarization | Not production-ready | New identities; 3-OS publisher verification |

"Implemented" means source code changed, not cross-platform release acceptance. A Mac ad-hoc-signed debug DMG is an engineering alpha, not a notarized production release.

## 3. Proposed design direction, not an approved asset

- Product tone: trustworthy, advanced, high-performance AI coding workbench.
- Visuals to explore: independent two-tone mark, clean modern typography, compact glass-inspired panels, disciplined shadows and restrained motion.
- UI readability beats decoration: status, safety approvals, agent timeline, workflows and MCP health must remain legible.
- Ship light, dark and system appearance; keyboard navigation, reduced motion and WCAG AA contrast for ordinary text.
- Prepare SVG masters and relevant high-resolution ICNS/ICO/PNG assets for macOS, Windows and Linux; cover tray, Dock, desktop shortcut, About, installer and website.
- Do not reuse protected upstream trademarks or present the fork as endorsed by PandaCode.

## 4. Work packages and mandatory acceptance gates

| Epic | Priority | Deliverable | Exit gate |
| --- | --- | --- | --- |
| BR-01 Brand Identity | P0 | Confirm product name, icon direction, tagline, trademark/domain assessment | Explicit Product Owner approval |
| BR-02 Visual Asset System | P1 | Logo masters, platform icons, About assets, splash, installation art | Asset coverage/legibility/HiDPI tests |
| BR-03 UI/UX Rebrand | P1 | Design tokens, theming, layouts, status components, accessibility | Visual regression, contrast, keyboard, reduced motion |
| BR-04 Product Copy & Identity | P0 | App titles, dialogs, links, package metadata, menus, notices | No unintended old-brand user copy; legal attribution stays |
| BR-05 Safe Migration | P0 | Data, credential, executable, tool-ID and database migration plan | Backups, regression, upgrade/downgrade and rollback pass |
| BR-06 Native Installer Brand | P0 | macOS DMG, Windows NSIS, Linux AppImage/DEB app identity | Real native install, uninstall, app coexistence and upgrades |
| BR-07 Release & Legal Gate | P0 | Signing, updater, SBOM, license and owner sign-off | 3-OS release evidence and approval |

These BR epics are additional to the core 24 planned engineering PRs. Do not silently mark them complete because the Tauri productName was changed.

## 5. Integration with engineering phases R0–R8

- R0: Inventory identity, legal notices and current fork separation; complete provisional brand brief.
- R1: Define data/Keychain/DB/executable compatibility policy; never silently migrate existing PandaCode secrets.
- R2–R4: Apply approved brand tokens to new MCP, Multi-Agent and Workflow components while preserving tool/session interfaces.
- R5: BR-02/BR-03 visual system implementation, light/dark, agent and MCP workbench UX; accessibility testing.
- R6: BR-04/BR-06 platform package metadata, icons and native desktop acceptance on all three OS.
- R7: Migration stress/rollback and brand/security regression; check license and hidden legacy update endpoints.
- R8: BR-07 production release with fork-owned signing, notarization, branded release assets and recorded approval.

**Dependencies:** BR-01 approval before final public design; BR-05 migration validation before executable/data changes; BR-06 native acceptance and BR-07 legal/signing before stable publication.

## 6. Test matrix

| Risk | Tests | Where |
| --- | --- | --- |
| Misidentified fork | Product Name, Bundle ID, executable, About and license notices | Mac M4, Win11 x64, Linux x64 |
| Visual usability | Themes, DPI/zoom, keyboard, screen reader, reduced motion, contrast | All three OS |
| Data conflict | Coexistence alongside original PandaCode; no shared private credentials | All three OS |
| Migration | Backup, invalid data, crash/restart, rollback, upgrade/downgrade | All three OS |
| Protocol break | Providers, agent tool IDs, sessions, MCP and worktrees | All three OS |
| Package errors | Icon, desktop menu, notification, uninstall, signature and checksum | DMG / NSIS / AppImage/DEB |
| Updater/legal failure | Fork-owned updater endpoint, signing key, copyright notices | Build and actual native artifacts |

**NO-GO:** Missing upstream attribution, unapproved final branding, old updater endpoints, source-based-only claims of native success, destructive data migration, broken provider/session contracts, unsigned stable package, or absent owner approval.

## 7. Working when the owner is away from the computer

Permitted: inventory, brand brief drafts, asset specifications, source code, static/unit tests, documentation, local branches/commits. Not permitted without explicit authorization: install/run GUI, alter OS settings/Keychain, publish, sign stable binaries, push remote changes, deploy or contact live servers. Mark native and owner checkpoints PENDING.

## 8. Definition of Done

Brand decision record, editable vector masters and asset inventory, theme and text specification, legal notice evidence, migration and rollback report, real three-platform native app captures and installer reports, fork update-signing verification, plus Project Owner and Release Reviewer approval. Each PR must explicitly distinguish source-implemented, native-tested and pending.
