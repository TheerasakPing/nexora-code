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
| FIX-01 | TEAM_LEAD_SERIAL | Pi agent launch doesn't auto-load user's extensions; explicit app bridge remains | [ ] |
| FIX-02 | TEAM_LEAD_SERIAL | Unit tests for isolation and bridge; clear timeout diagnosis | [ ] |
| QA-01 | TEAM_LEAD_SERIAL | macOS Pi RPC startup reproduction without login/model prompt + regression tests | [ ] |
| DOC-01 | TEAM_LEAD_SERIAL | Explain compatibility and per-user extension tradeoff without rewriting config | [ ] |
| INT-01 | TEAM_LEAD_SERIAL | Commit/push to PR #26 with required exact-head CI | [ ] |
| REVIEW | GENUINE_INDEPENDENT_REVIEWER_PENDING | Review/approval before merge to protected develop | [ ] |

**Initial decision:** GO for isolated app launch + source tests. NO-GO for production/native E2E/merge until explicit tests, independent review and 3-OS acceptance.
**Baseline remaining:** 21/24 engineering acceptance packages, 7/7 branding epics and 61 detailed R0–R8 checks unchecked (overlapping; cannot sum).
