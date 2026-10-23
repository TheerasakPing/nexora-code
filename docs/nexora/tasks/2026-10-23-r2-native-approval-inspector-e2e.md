# Task Checklist Planner — R2 Native Approval, MCP Inspector and Integration Acceptance

**Date/Request:** 2026-10-23, “ทำให้ครบจนปิดเฟสนี้ได้”.
**PR/worktree:** follow-up PR #32 to develop, `/Volumes/Data/workspace/git/nexora-code-integration-20261023`.
**Baseline:** b05567c on PR #32, remote matches; 6 visible source+platform checks PASS, native GUI/real server E2E not accepted.
**Lead/actual workers:** ChatGPT Integration Lead, `TEAM_LEAD_SERIAL`; Nexora Studio exposes no actual subtask agent delegation API. No independent reviewer assigned. Do not claim external parallel agents exist.
**Requirements:** NC-TEAM-001/002/003. This planner is committed before any feature code modification. No production activation until a genuine native consent boundary is reviewed. No external user credentials, user's installed app, Mac Keychain, personal projects, or production servers touched by tests.
**DoD for R2:** all 10 unchecked R2 gates must meet production-level E2E/security acceptance across Mac/Windows/Linux, with independent review. Partial implementation may GO for source PR while R2 remains NO-GO.

## B. Team Lead decomposition and ownership

| ID | Deliverable/acceptance | Real worker | Owned files | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R2-NA-01 | Audit native UI dialog, current R2 transport and permission/credential boundaries; update threat model | TEAM_LEAD_SERIAL | docs/nexora, source audit | none | [x] |
| R2-NA-02 | Build native human-dialog-based action-scoped single-use approval flow with no renderer-minted consent | TEAM_LEAD_SERIAL | src-tauri/src/mcp_*.rs, lib.rs | R2-NA-01 | [ ] |
| R2-NA-03 | Wire read-only catalog/discovery inspector command and safe UI with diagnostics, preserved fail-closed behavior for tool invocations | TEAM_LEAD_SERIAL | src-tauri/src/mcp_*.rs, src/mcp-center* | R2-NA-02 | [x] |
| R2-NA-04 | Test real short-lived local stdio/HTTP fixtures, malicious streams, approval cancellation/replay, no credentials in UI, native smoke Mac M4 | TEAM_LEAD_SERIAL | Rust and Node tests | R2-NA-02/03 | [x] |
| R2-NA-05 | Lead inspect each diff and triad, run full release/Node/Rust suites and build, push PR #32 with exact-head CI evidence | TEAM_LEAD_SERIAL | planner and PR | R2-NA-04 | [ ] |
| R2-NA-REVIEW | Genuine independent security review + real installed Mac/Windows/Linux native MCP E2E and final R2 approval | INDEPENDENT_REVIEWER_PENDING | production review gate | R2-NA-05 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R2-NA-01 | TEAM_LEAD_SERIAL | PASS | N/A: source audit; Team Lead approved, no running implementation | PASS | GO | Rust native DialogExt API source audited; R2-NATIVE-INSPECTOR-ACCEPTANCE document reviewed; build smoke Mac M4 PASS |
| R2-NA-02 | TEAM_LEAD_SERIAL | PASS | BLOCKED | PASS | NO-GO | Native consent + stable configuration source tests PASS, but real installed GUI user Approve/Deny click E2E and security accessibility review NOT performed; owner native platform QA |
| R2-NA-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 Rust Inspector 6 scoped tests incl real stdio process + local HTTP, Playwright browser-mock 2/2 PASS, Node guard 3/3 PASS, Vite build PASS; native installed GUI E2E separate BLOCKED |
| R2-NA-04 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 full Rust 1100 pass 0 fail 33 ignored; Node 201 pass, Release 33, Frontend + native cargo check PASS, SSE fixture 5 PASS; real native dialog/secure vault still unaccepted |
| R2-NA-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | PR exact-head cross-OS CI and Lead review pending |
| R2-NA-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No independent source review / all-OS native acceptance yet |

## C. Verification plan

- [x] Fresh dated task plan committed before source edits.
- [x] R2-NA-01/03 source and security inspection reviewed; R2-NA-02 real native dialog approval E2E BLOCKED.
- [ ] All subtasks have real native E2E/Smoke + Lead GO (R2-NA-02 dialog on installed app remains BLOCKED; mocked browser E2E is not native).
- [ ] Full project regression, macOS check, exact SHA GitHub 3 OS checks green.
- [ ] Independent reviewer + installed GUI/OS vault/real MCP security E2E accepted.
- [ ] Native trusted approval and secure multi-server actual E2E satisfy R2 exit, R2 mark GO.

## E. Initial Lead GO/NO-GO

**Team Lead verdict:** GO for source implementation/tests and PR update; NO-GO R2 phase exit, protected develop merge or Production without real independent review and native platform acceptance. Prior R1 phase remains unaccepted, per strict sequential roadmap.

## Next required OS assets

Windows 11 x64 real GUI, Linux x64 graphical session + Secret Service, macOS installed signing/test identity, independently reviewed operator-consent UI and a controlled actual MCP network testbed.

## Delivery ledger — source GO, R2 phase exit NO-GO

- [x] Mac M4: Rust full `cargo test --locked --lib --quiet` **1100 passed / 0 failed / 33 ignored**.
- [x] Browser-mocked Playwright MCP inspector `npx playwright test tests/mcp-inspector.spec.ts --workers=1`: **2 passed**; malicious HTML remains inert.
- [x] Real OS-function fixture E2E: ephemeral local stdio executable and HTTP listener return Tools/Resources/Prompts; Rust inspector 6/6; bounded HTTP SSE 5/5.
- [x] Node `npm test`: **201 passed**; `npm run test:release` 33 passed, Frontend Vite PASS, Mac `cargo check --bin nexora-code` PASS.
- [x] Team Lead inspected type contracts, renderer denial, native dialog code, full persisted-config equality on approval, no uncontrolled tools/call and no real secrets/external servers accessed.
- [ ] **R2-NA-02 native-dialog E2E** remains BLOCKED: Requires installed GUI and actual human confirm/deny tests (not simulated browser IPC).
- [ ] **R2-NA-05 exact-head 3-OS GitHub CI and PR QA** pending after push.
- [ ] **R2-NA-REVIEW independent source/security review** and real Mac/Windows/Linux transport + secure-store acceptance pending.

**Source GO for PR review only. Phase R2 completion NO-GO.**
See `docs/nexora/R2-NATIVE-INSPECTOR-ACCEPTANCE-2026-10-23.md`
for operator workflow and six unaccepted R2 security gates.
