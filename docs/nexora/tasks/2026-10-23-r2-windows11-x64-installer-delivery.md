# Task Checklist Planner — R2 Windows 11 x64 Installer Delivery

**Date/user:** 2026-10-23, “บิ้วเวอชั่นวินโดมาด้วย” (also build Windows version).
**Team Lead:** ChatGPT, TEAM_LEAD_SERIAL. No independent reviewer or delegated worker API assigned; no false parallel-agent claims.
**Baseline:** PR #32 source commit `778a5bdca43b8443664aefb88abbbd4e85b922c1`; Github Actions run [39952189822](https://github.com/TheerasakPing/nexora-code/actions/runs/39952189822), Windows job success at `UNSIGNED-TEST-ONLY-windows-x86_64-778a5bdca43b8443664aefb88abbbd4e85b922c1`. All installer bundles engineering preview only.
**Safety:** Do not execute untrusted Windows binaries on Mac, overwrite existing Mac apps, install into Windows user's device, use credentials or sign with production secrets. Extract to an isolated folder and inspect filename, size, static package metadata, SHA-256, job logs and release smoke evidence. NO claim of native Windows GUI acceptance without a Windows 11 actual test.

## B. Team Lead decomposition and acceptance

| ID | Work package | Real worker | Acceptance | Status |
| --- | --- | --- | --- | --- |
| R2-WIN-01 | Source/CI identity + checks verify exact Windows x64 GitHub run | TEAM_LEAD_SERIAL | Exact SHA matches, Windows job successful, requested feature committed | [x] |
| R2-WIN-02 | Download signed-by-GitHub artifact ZIP and inspect installer package contents | TEAM_LEAD_SERIAL | ZIP valid, check installer types, architecture and actual package integrity | [ ] |
| R2-WIN-03 | Static build verification + E2E/Smoke evidence from Windows CI logs | TEAM_LEAD_SERIAL | Real CI packages + startup smoke and checks verified; native GUI E2E explicitly blocked | [ ] |
| R2-WIN-04 | Copy test installer to Downloads separated by exact SHA; checksum and setup notes | TEAM_LEAD_SERIAL | Preserve original files, copy verified, clear unsigned-only / SmartScreen caution | [ ] |
| R2-WIN-05 | Team Lead review, link Actions, PR evidence integration and exact-head CI | TEAM_LEAD_SERIAL | Review verification matrix, report GO/NO-GO truthfully | [ ] |
| R2-WIN-REVIEW | Independent security review and installed real Windows 11 x64 GUI/MCP E2E | INDEPENDENT_REVIEWER_PENDING | Requires real Windows machine/user, approval/deny clicks, unsigned warning validation and tool execution | [ ] |

## G. NC-TEAM-003 — Subtask verification matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| R2-WIN-01 | TEAM_LEAD_SERIAL | PASS | N/A: build identity | PASS | GO | Windows job success, exact commit SHA |
| R2-WIN-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Archived package inspection/checksum required |
| R2-WIN-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Windows CI job logs + startup smoke |
| R2-WIN-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Copy & verify exact archive and files |
| R2-WIN-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Source/CI reviewer + PR handoff |
| R2-WIN-REVIEW | INDEPENDENT_REVIEWER_PENDING | BLOCKED | BLOCKED | BLOCKED | NO-GO | Real installed Windows 11 GUI, MCP dialog/tool approval and independent security reviewer unavailable |

## C. Checklist

- [x] Create & commit fresh planner BEFORE download, transformation or distribution.
- [x] Exact source SHA and successful Windows CI build identified.
- [ ] Download ZIP from Github Actions and verify integrity.
- [ ] Inspect Windows installer executable/MSI/NSIS package and CI smoke.
- [ ] Deliver verified Windows engineering package with SHA256 + safety/readme.
- [ ] Team Lead independent source / package review evidence and PR notes.
- [ ] Real installed Windows 11 GUI/MCP E2E + independent reviewer acceptance.

## E. Team Lead release verdict

**GO:** engineering Windows x64 preview artifact download and offline user test after checksum verification.
**NO-GO:** production release; unsigned binary, SmartScreen, no Authenticode signing, no reviewed real Windows 11 native GUI MCP E2E, independent reviewer pending. R2 phase exit not approved.
