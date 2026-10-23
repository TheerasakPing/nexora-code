# Nexora Code — BR-02/BR-04 Mac Visual Rebrand Evidence

**Date:** 2026-10-23. **Status:** source implemented, native acceptance pending, final owner trademark/logo approval pending.
**Product:** provisional Nexora Code identity, fork of PandaCode with legally required attribution retained.

## Why the previous Mac preview still looked like PandaCode

Although the Tauri package productName / bundle identifier, About heading and npm package had been renamed, the old `public/icon.svg` favicon, native `src-tauri/icons` assets, main Sidebar `PandaCode` wordmark, Quick Launcher, welcome illustrations, palette and many visible captions were still upstream branded. A renamed DMG was not a complete rebrand.

## Original provisional brand assets

- Editable master: `brand/nexora-mark.svg` — angular N, cyan and violet on midnight indigo, designed for 16px through 1024px.
- `public/icon.svg` / `public/nexora-mark.svg`: identical master for app favicon/UI.
- `public/nexora-watermark.svg`: non-bamboo watermark for main screens.
- `src-tauri/icons/icon.icns`: native macOS Dock/Finder icon.
- `src-tauri/icons/icon.ico`: Windows taskbar/installer icon.
- `src-tauri/icons/32x32.png`, `128x128.png`, `128x128@2x.png`: cross-platform PNG.
- Reproduction: `bash scripts/nexora/generate-brand-icons.sh` on a Mac with `rsvg-convert`, `magick`, `iconutil`; all generated assets are committed so CI does not require this local script.

## Visible UI changes

- Page title, Sidebar logo/wordmark, Quick Launcher, new-task empty state, project board, About visual mark, colorful accent, light/dark frame and non-bamboo background illustration.
- Human-visible provider, integrations, assistance, settings, import, bot and update copy changed to **Nexora Code**.
- Clear original source attribution and upstream LICENSE links remain visible under About, README and LICENSE. Do not disguise third-party provenance.
- User preference keys, stable `pandacode_*` tool names, stored DB/Keychain namespaces, legacy token-saver context markers and executable compatibility paths remain unchanged to avoid erasing sessions or breaking integrations.
- Existing user theme preference (VS Code theme, wallpaper, custom swatch) remains supported; the default built-in accent is now cyan, not yellow.

## Evidence / release gates

- Editable SVG, ICNS/ICO and PNG asset presence, identity/source legal guard in `tests/nexora-visible-brand.test.mjs`.
- Frontend TypeScript + Vite build, Node regression and Playwright visual identity tests: see actual task planner/PR for final results.
- `BR-01` final brand/trademark approval: pending owner; logo is a **provisional design**, no ownership/trademark clearance claimed.
- `BR-02` source asset family: present; 3-platform HiDPI/native acceptance pending.
- `BR-03` complete accessibility/light-dark contrast and UI design system: pending native E2E.
- `BR-04` main Mac GUI/source copy: updated; website copy, language audit, Rust-native text review pending.
- `BR-05` migrations: intentionally avoided; backup/rollback/secure-store regression pending.
- `BR-06` branded Mac Installer and Windows/Linux packages: CI and actual device verification pending.
- `BR-07` Developer ID signing/notarization and release: not ready.

**Decision:** GO to build **branded unsigned Preview** for owner review; NO-GO to call production-ready or bypass review/notarization. Historical 21/24 engineering acceptance packages unchecked, 7/7 branding epics unaccepted, 61 detailed phase checks unchecked (overlapping, not additive).
