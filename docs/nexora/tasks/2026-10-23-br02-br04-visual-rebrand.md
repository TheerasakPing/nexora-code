# Task Checklist — Visible Nexora Code Mac Rebranding (BR-02/BR-04)

**User:** 'ยังไม่เห็นรีแบรนดิ้งให้เลย' — previous Mac Preview still looks like PandaCode.
**Date:** 2026-10-23 Asia/Bangkok.
**Team Lead:** ChatGPT TEAM_LEAD_SERIAL (no actual independent reviewer/parallel worker invoked).
**Base:** integrated PR #26 `b52724e`. Native acceptance for both light/dark still pending.
**Brand:** Nexora Code (provisional, final BR-01 trademark and Product Owner approval pending).
**Visual concept:** independent two-tone midnight/teal/violet angular N monogram. SVG editable master, Dock/ICNS/Windows ICO/PNG, favicon and app sidebar/launcher brand.
**Do not change:** executable/protocol tool IDs `pandacode_*`, persisted user preferences/DB/Keychain, scripting backward compatibility, upstream license/attribution or signed update endpoint; do not silently replace installed app.
**Deliver:** coherent visible names, original icon asset family, package About display and installer + Mac preview actual build and asset audit.

| Package | Actual owner | Gate | Status |
| --- | --- | --- | --- |
| BR02-01 | TEAM_LEAD_SERIAL | Original SVG master, regenerated PNG/ICNS/ICO/Favicon, confirmed iconutil roundtrip | [x] |
| BR02-02 | TEAM_LEAD_SERIAL | Main Sidebar, Quick Launcher, welcome/board, About and palette updated; 23 selected Playwright UI assertions PASS | [x] |
| BR04-01 | TEAM_LEAD_SERIAL | Main app copy updated; website/full Rust-native review still not fully accepted | [x] |
| BR06-01 | TEAM_LEAD_SERIAL | Package identity / Mac Dock icon / installer DMG inspect | [ ] |
| QA-01 | TEAM_LEAD_SERIAL | Node 165/165, TypeScript/Vite PASS, Playwright 23/23, macOS icons generated; native artifact PENDING | [x] |
| QA-02 | INDEPENDENT_REVIEWER_PENDING | Independent review before protected `develop` merge | [ ] |
| BR01/07 | PRODUCT_OWNER_PENDING | Trademark/visual approval, Developer ID notarization, release | [ ] |

**Initial decision:** GO to implement provisional visible rebrand and upload reviewable Preview. NO-GO for production claim or protected branch bypass.
**Remaining baseline:** 21/24 engineering acceptance packages unchecked; 7/7 branding epics not fully accepted; 61 detailed R0–R8 items unchecked (overlap, not additive). Update once actual evidence is obtained.

## Source-level work evidence and handoff

- Regenerable original vector and icon exports: `brand/nexora-mark.svg`, `scripts/nexora/generate-brand-icons.sh`, `public/icon.svg`, `public/nexora-mark.svg`, `public/nexora-watermark.svg`, `src-tauri/icons/*`.
- Changed visible app branding in Sidebar, About, Quick Launcher, Welcome and Board, light/dark accent. Preserved original source attribution in About, README and LICENSE.
- `node --test tests/nexora-visible-brand.test.mjs`: 3/3 PASS; `npm test`: 165/165 PASS; `npm run build`: PASS (prior to final dark palette touch-up, must rerun).
- No database migration and no changes to existing user sessions, Keychain, `pandacode_*` RPC, localStorage keys or original PandaCode fork attribution.
- Independent brand approval: PENDING. Reviewer: PENDING. DMG and native macOS Dock icon acceptance: PENDING.
- **Team Lead verdict:** GO source-review/CI submission; NO-GO stable/production/native claim until native visual tests, Apple Developer ID notarization and genuine independent review.

## UI verification and remaining approval gates

- Browser Preview Playwright: **23/23 PASS** on selected UI, settings/appearance, slash commands and visual identity checks.
- Original source assets regenerated via rsvg-convert + iconutil + ImageMagick. `iconutil -c iconset` roundtrip for the committed ICNS succeeded on real Apple Silicon Mac.
- Source diagnostics: only upstream legal attribution and internal legacy token-saver marker intentionally retain user-facing PandaCode; stable `pandacode.*` storage keys and protocol tool IDs unchanged.
- 165/165 Node source tests pass; frontend TypeScript + Vite build PASS. Need branded Mac .app/DMG native inspection, actual Dock icon and LaunchServices test before R6 acceptance.
- **Team Lead verdict:** GO for PR source and unsigned preview artifact CI; NO-GO for merge without independent review or production without owner visual approval/notarization.
