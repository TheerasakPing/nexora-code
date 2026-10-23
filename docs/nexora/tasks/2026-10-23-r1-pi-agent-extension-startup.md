# Task Checklist — R1 Pi Agent startup isolation on macOS M4

**Date:** 2026-10-23 Asia/Bangkok
**Request:** Fix user-reported Nexora Code "Agent initialization timed out" when Pi CLI reports extension host-provided peerDependency warnings.
**Team Lead:** ChatGPT Integration Lead — TEAM_LEAD_SERIAL (no independent worker delegated; no reviewer approval fabricated).
**Base:** canonical GitHub PR #26 head, Mac preview; project fork attribution preserved.
**Affected platform:** Mac mini M4, macOS 27.2; Pi CLI 1.4.0.
**P0:** Agent init reliability + avoid third-party extension code injecting undesired host actions into Nexora process.
**Out of scope:** Mutating/removing user-installed Pi extensions, rewriting their package.json, disabling Mac Gatekeeper, inventing login credentials or shipping a notarized build.

## Diagnosis with actual Mac evidence
- [x] Nexora Pi provider starts RPC by default and also loads all Pi user extensions from ~/.pi/agent settings.
- [x] User has many configured Pi extension packages, including pi-memory-md 0.1.38 and pi-manage-todo-list 0.2.0; both declare host-provided modules as dependencies rather than wildcard peers. No user files edited.
- [x] A clean Pi `get_state` RPC probe without discovered extensions responded in **2.5 s**.
- [x] A configured Pi `get_state` probe with default extensions did not respond in **18 s**, generated a flood of `extension_ui_request` messages and reproduced **two** host peerDependency warnings. Killed bounded probe. Thus warnings themselves are not necessarily fatal; startup extension overload is reproduced.
- [x] Pi RPC + *explicit Nexora permission bridge* + `--no-extensions --no-mcp` returned both `pandacode-bridge: ready` and successful `get_state` in **7.66 s**, without warnings.
- [x] Pi CLI docs document that explicit `--extension` flags remain enabled when auto-discovery is disabled.

## Work breakdown
| ID | Owner | Criteria | Status |
| --- | --- | --- | --- |
| FIX-01 | TEAM_LEAD_SERIAL | Pi agent launch doesn't auto-load user's extensions; explicit app bridge remains | [x] |
| FIX-02 | TEAM_LEAD_SERIAL | 2 Rust tests, 2 Node tests and Pi-specific timeout diagnosis added | [x] |
| QA-01 | TEAM_LEAD_SERIAL | Real Pi 1.4.0 + Nexora bridge RPC in 7.66 s; local npm 162/162 PASS; Rust focused tests RUNNING | [x] |
| DOC-01 | TEAM_LEAD_SERIAL | docs/nexora/PI-RPC-STARTUP-ISOLATION-2026-10-23.md delivered | [x] |
| INT-01 | TEAM_LEAD_SERIAL | Submit to PR #26 for protected exact-head CI; outcome PENDING | [ ] |
| REVIEW | GENUINE_INDEPENDENT_REVIEWER_PENDING | Review/approval before merge to protected develop | [ ] |

**Initial decision:** GO for isolated app launch + source tests. NO-GO for production/native E2E/merge until explicit tests, independent review and 3-OS acceptance.
**Baseline remaining:** 21/24 engineering acceptance packages, 7/7 branding epics and 61 detailed R0–R8 checks unchecked (overlapping; cannot sum).

## Team Lead review and verification

- **Reviewed source changes:** `src-tauri/src/providers/pi.rs`, `src-tauri/src/providers.rs`, new focused tests and Pi troubleshooting guide. No changes to the user's Pi settings or extension directories.
- **Pi RPC with explicit Nexora bridge and extensions disabled:** startup success 7.66 seconds on macOS 27.2, `extension_ui_request` setStatus for `pandacode-bridge=ready` and get_state success, no peerDependency warnings.
- **Default user Pi extensions:** reproduced multiple unfulfilled extension_ui_request events and no get_state response before the 18s probe limit; warnings emitted twice. Did not edit/uninstall or send any model prompts.
- **Node tests:** 162/162 PASS; `git diff --check` PASS.
- **Rust focused tests:** executing on local Mac at time of PR handoff; CI must confirm exact head before merging.
- **Security tradeoff:** user-installed Pi extensions stay enabled when running standalone `pi` in Terminal, but are not auto-loaded inside Nexora. Explicit app bridge survives via `--extension`; installed Pi skills/context data remains available.
- **Approval:** Team Lead GO for source-review PR; protected develop merge NO-GO until all required CI checks + *genuine independent reviewer*. Native app E2E and production readiness NO-GO pending Mac release build and signing/notarization.
- **Next 3 priorities:** exact-head CI and Mac unsigned test artifact, real Mac GUI/provider smoke test from rebuilt app, independent reviewer and release credentials. Historic backlog unchanged (21/24 engineering unchecked, 7/7 branding pending, 61 detailed R0–R8 checkboxes open; overlap).
