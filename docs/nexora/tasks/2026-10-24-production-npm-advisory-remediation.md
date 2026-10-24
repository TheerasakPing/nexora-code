# NC-TEAM-003 — Production dependency vulnerability remediation (2026-10-24)

**Lead/worker:** TEAM_LEAD_SERIAL. **Review:** read-only QA_REVIEW_AI on adjacent native-window change; does not equal human security approval.
**Branch/worktree:** feat/production-gui-acceptance-20261024. Separate planner commit BEFORE dependency mutations.
**Read-only evidence:** npm ci on 2026-10-24 reported 4 advisories: 1 critical DOMPurify, 2 high Vite/source-map-js, 1 low KaTeX.
**Product:** Rust/Tauri v2 + React; do not regress original UI or disable CSP.

## Checklist planner
- [x] SEC-01 / TEAM_LEAD_SERIAL — run `npm audit --json` and categorize current advisories.
- [ ] SEC-02 / TEAM_LEAD_SERIAL — apply compatible patched versions for DOMPurify, Vite and source-map-js; review lockfile, avoid arbitrary forced semver-major updates.
- [ ] SEC-03 / TEAM_LEAD_SERIAL — investigate KaTeX patched major 0.19.1 for API/compatibility and minimize breaking changes.
- [ ] SEC-04 / TEAM_LEAD_SERIAL — rerun Node/Release/Frontend, `npm audit`, integration checks, Rust/Clippy; review vulnerable dependency flows.
- [ ] SEC-05 / TEAM_LEAD_SERIAL — CI audit gate fail closed on critical/high known advisory exposure before packaging, if real registry evidence available.
- [ ] SEC-06 / TEAM_LEAD_SERIAL — submit PR, hosted native CI/installer and QA-review; no protected merge before evidence.
- [ ] SEC-07 / HUMAN_REVIEW_PENDING — independent human security approval, three OS interactive GUI & production signing keys/owner acceptance.

## Verification matrix
| Task | Worker | Unit/Contract | E2E | Smoke | Lead | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| SEC-01 | TEAM_LEAD_SERIAL | N/A audit | N/A audit | N/A audit | GO diagnosis | 1 Critical, 2 High, 1 Low, NPM advisories |
| SEC-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | lockfile |
| SEC-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | KaTeX compatibility |
| SEC-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | test/audit outputs |
| SEC-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | CI audit |
| SEC-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | PR CI |
| SEC-07 | HUMAN_REVIEW_PENDING | PENDING | PENDING | PENDING | PENDING | release sign-off |

**GO/NO-GO:** critical/high known vulnerabilities block Production; signing/human/native GUI gates remain independently blocking.

## Source execution evidence (before hosted CI)
- npm audit pre-fix: critical DOMPurify, high Vite and source-map-js, low KaTeX.
- npm audit fix updated DOMPurify 3.4.17, Vite 7.3.7, source-map-js 1.2.2.
- explicit KaTeX 0.19.1 (from 0.16.47) for low advisory; math rendering imports API remained `renderToString`; visual E2E still pending.
- `npm audit --audit-level=high`: **0 vulnerabilities** locally; CI unsigned installer workflow now fail-closed on this gate.
- After dependency changes on Mac: `npm ci` PASS, Vite/TypeScript production build PASS, Node 210/210 PASS, release contracts 35/35 PASS. Rust and Clippy run separately; hosted exact PR SHA pending.
- This is an advisory snapshot, not a claim that future vulnerabilities or human supply-chain review cannot find new issues.
