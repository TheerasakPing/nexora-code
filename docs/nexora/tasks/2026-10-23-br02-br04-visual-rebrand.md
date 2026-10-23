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
| BR02-01 | TEAM_LEAD_SERIAL | Create original vector logo source and PNG/ICNS/ICO/Favicon variants | [ ] |
| BR02-02 | TEAM_LEAD_SERIAL | Update main sidebar/launcher, icon CSS and palette, light/dark | [ ] |
| BR04-01 | TEAM_LEAD_SERIAL | Replace user-facing PandaCode copy, preserve upstream notices and protocol IDs | [ ] |
| BR06-01 | TEAM_LEAD_SERIAL | Package identity / Mac Dock icon / installer DMG inspect | [ ] |
| QA-01 | TEAM_LEAD_SERIAL | Build/Node/frontend/Rust policy tests, UI screenshots if feasible, actual macOS artifact evidence | [ ] |
| QA-02 | INDEPENDENT_REVIEWER_PENDING | Independent review before protected `develop` merge | [ ] |
| BR01/07 | PRODUCT_OWNER_PENDING | Trademark/visual approval, Developer ID notarization, release | [ ] |

**Initial decision:** GO to implement provisional visible rebrand and upload reviewable Preview. NO-GO for production claim or protected branch bypass.
**Remaining baseline:** 21/24 engineering acceptance packages unchecked; 7/7 branding epics not fully accepted; 61 detailed R0–R8 items unchecked (overlap, not additive). Update once actual evidence is obtained.
