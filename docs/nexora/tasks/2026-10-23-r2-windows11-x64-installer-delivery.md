# Task Checklist Planner — R2 Windows 11 x64 Installer Delivery

**Date/user:** 2026-10-23, “บิ้วเวอชั่นวินโดมาด้วย” (also build Windows version).
**Team Lead:** ChatGPT, TEAM_LEAD_SERIAL. No independent reviewer or delegated worker API assigned; no false parallel-agent claims.
**Baseline:** PR #32 source commit `778a5bdca43b8443664aefb88abbbd4e85b922c1`; Github Actions run [39952189822](https://github.com/TheerasakPing/nexora-code/actions/runs/39952189822), Windows job success at `UNSIGNED-TEST-ONLY-windows-x86_64-778a5bdca43b8443664aefb88abbbd4e85b922c1`. All installer bundles engineering preview only.
**Safety:** Do not execute untrusted Windows binaries on Mac, overwrite existing Mac apps, install into Windows user's device, use credentials or sign with production secrets. Extract to an isolated folder and inspect filename, size, static package metadata, SHA-256, job logs and release smoke evidence. NO claim of native Windows GUI acceptance without a Windows 11 actual test.

## B. Team Lead decomposition and acceptance

| ID | Work package | Real worker | Acceptance | Status |
| --- | --- | --- | --- | --- |
| R2-WIN-01 | Source/CI identity + checks verify exact Windows x64 GitHub run | TEAM_LEAD_SERIAL | Exact SHA matches, Windows job successful, requested feature committed | [x] |
| R2-WIN-02 | Download official GitHub Actions artifact ZIP and inspect contents | TEAM_LEAD_SERIAL | SHA256 matches CI, NSIS archive extracts and x86-64 payload proven | [x] |
| R2-WIN-03 | Static build verification + E2E/Smoke evidence from Windows CI job steps | TEAM_LEAD_SERIAL | Real Windows hosted CI silent install + app startup 30 sec verified; native GUI E2E explicitly blocked | [x] |
| R2-WIN-04 | Copy test installer to Downloads separated by exact SHA; checksum and setup notes | TEAM_LEAD_SERIAL | Copied 11 MB EXE + SHA256, TH/EN README, optional echo test fixture, verification note, zipped bundle and rehashed; original installations untouched | [x] |
| R2-WIN-05 | Team Lead source/installer review, GitHub Actions links and PR evidence integration | TEAM_LEAD_SERIAL | Verified downloaded payload against exact source build, safe docs-only PR integration in progress | [x] |
| R2-WIN-REVIEW | Independent security review and installed real Windows 11 x64 GUI/MCP E2E | INDEPENDENT_REVIEWER_PENDING | Requires real Windows machine/user, approval/deny clicks, unsigned warning validation and tool execution | [ ] |

## G. NC-TEAM-003 — Subtask verification matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R2-WIN-01 | TEAM_LEAD_SERIAL | PASS | N/A: Team Lead approved CI build identity; no separate feature interaction required | PASS | GO | Windows job success, exact commit SHA |
| R2-WIN-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | 7z extracted NSIS, embedded nexora-code.exe PE32+ AMD64 (0x8664); installer SHA256 verified against Actions sidecar |
| R2-WIN-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Windows job #120409093015 succeeded, identity/release contracts, NSIS build, silent Windows install, 30s app-process survival; actual GUI MCP tool click E2E unavailable |
| R2-WIN-04 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | User Downloads Windows EXE + SHA+ TH/EN guide+ ZIP; shasum -c PASS; no files overwritten; Mac extracted-and-verified payload, no Windows execution on Mac |
| R2-WIN-05 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Exact SHA 778a5bd; checkout/delivery reviewed; PR updated with evidence; independent human review remains NO-GO for release |
| R2-WIN-REVIEW | INDEPENDENT_REVIEWER_PENDING | BLOCKED | BLOCKED | BLOCKED | NO-GO | Real installed Windows 11 GUI, MCP dialog/tool approval and independent security reviewer unavailable |

## C. Checklist

- [x] Create & commit fresh planner BEFORE download, transformation or distribution.
- [x] Exact source SHA and successful Windows CI build identified.
- [x] Download ZIP from Github Actions and verify integrity.
- [x] Inspect NSIS package embedded x86-64 executable and Windows CI startup smoke.
- [x] Deliver verified Windows engineering package with SHA256 + Thai/English readme.
- [x] Team Lead source / package inspection and PR evidence recorded (NOT an independent reviewer).
- [ ] Real installed Windows 11 GUI/MCP E2E + independent reviewer acceptance.

## E. Team Lead release verdict — GO/NO-GO

**Team Lead verdict:** GO for PR submission and unsigned Windows preview distribution only.

**GO:** engineering Windows x64 preview artifact download and offline user test after checksum verification.
**NO-GO:** production release; unsigned binary, SmartScreen, no Authenticode signing, no reviewed real Windows 11 native GUI MCP E2E, independent reviewer pending. R2 phase exit not approved.


## Delivery verified 2026-10-23 (19:37 Bangkok time)

- Windows source SHA: 778a5bdca43b8443664aefb88abbbd4e85b922c1.
- GitHub Actions unsigned three-OS run: https://github.com/TheerasakPing/nexora-code/actions/runs/39952189822 (Windows job SUCCESS).
- Windows job: https://github.com/TheerasakPing/nexora-code/actions/runs/39952189822/job/120409093015.
- Artifact: UNSIGNED-TEST-ONLY-windows-x86_64-778a5bdca43b8443664aefb88abbbd4e85b922c1.
- Installer: NexoraCode-Windows-x64-setup.exe, about 11 MiB, NSIS-3 self-extracting setup.
- SHA-256 installer: 0514fa7835853b5ad0b51ac48bf3a00d2371b297daf8fbd17625e135ecca76c2 (CI sidecar == downloaded copy == delivered copy).
- 7z offline extract: PASS, embedded executable nexora-code.exe verified x86-64 PE32+ COFF (Machine 0x8664), SHA256 e374918a407a44a14bfdec3e5fd4b90f824040f5a26ae27e8b3b72b067d1d989.
- Windows GitHub-hosted job checks: npm ci, identity/release contracts, native unsigned installer build, silent NSIS install to disposable temporary profile, expected nexora-code.exe present, native app remained alive 30 seconds and artifact upload ALL SUCCESS (job steps inspected).
- Delivery folder on user Mac: /Users/baemon/Downloads/Nexora-Code-Windows11-x64-RunOnce-20261023-778a5bd/
- Delivery ZIP on user Mac: /Users/baemon/Downloads/Nexora-Code-Windows11-x64-RunOnce-20261023-778a5bd.zip
- ZIP SHA256: 79679b9f03043a37274a9a029e6c450e09cb66e7062fdaa8d1f15af3551ff19d
- User-facing instructions: READ-ME-FIRST-WINDOWS11-TH-EN.txt, RELEASE-VERIFICATION-2026-10-23.md, optional mcp_echo_test.py.
- Actual GUI user approvals, MCP tools/call and independent security review on installed Windows 11: BLOCKED, NOT TESTED. Installer remains UNSIGNED preview; do not claim Production.
