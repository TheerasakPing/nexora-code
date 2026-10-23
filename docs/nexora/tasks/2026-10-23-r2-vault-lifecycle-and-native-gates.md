# Task Checklist Planner — NC-TEAM-003 R2 Vault Lifecycle, Native Acceptance and Transport Hardening

**Request:** 2026-10-23 — Continue R2 feature implementation without waiting for CI to finish; report upon completion of this synchronous work session.
**Lead:** ChatGPT Integration Lead; actual assignment TEAM_LEAD_SERIAL because task-delegating worker API absent. No invented independent reviewer or Windows native desktop access.
**Base:** PR #32 / branch docs/nc-team-002-progress-accountability-20261023 at 8b3058e, integration worktree /Volumes/Data/workspace/git/nexora-code-integration-20261023.
**Rules:** Write planner before code edits. Unit/Contract + E2E + Smoke per subtask; Lead review before handoff. Don't touch installed app, Keychain or user credentials during development/tests. Never bypass real operator consent and don't claim R2 production acceptance without genuine Windows/Mac/Linux GUI test and reviewer.
**Implementation work:** Credential lifecycle (server-bound opaque references, secure-store native UX, atomic/rollback handling), safe credential-protected manual HTTP catalog/calls ONLY after actual Native Approval, denial/secret masking, status/error diagnostics, regression tests. No autonomous Agent MCP access, no fake credentials, no production changes.
**Stop at code delivery:** CI may run asynchronously without waiting to finish. Coding implemented and R2 acceptance GO are separate states.

## B. Team Lead decomposition and assignment

| Subtask ID | Deliverable / acceptance | Actual worker | Ownership | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R2-VLT-01 | Audit real credential/transport gaps and secure-store deletion error semantics | TEAM_LEAD_SERIAL | mcp_vault.rs, mcp_registry.rs, docs | baseline | [x] |
| R2-VLT-02 | Native approve provision/rotate/revoke for server-scoped OS vault refs, safe UI with no secret echo and rollback | TEAM_LEAD_SERIAL | mcp_vault.rs, new mcp_vault_commands.rs, mcp_center.ts, lib.rs | 01 | [x] |
| R2-VLT-03 | Add opted-in HTTPS/loopback HTTP Bearer header on native-approved catalog/manual call, no secrets in registry or logs | TEAM_LEAD_SERIAL | mcp_http.rs, mcp_inspector.rs | 02 | [x] |
| R2-VLT-04 | Rust/mock vault+header/unit, negative E2E, UI contract/browser smoke, full Rust/Node/Release tests | TEAM_LEAD_SERIAL | Rust and JS tests, docs | 02/03 | [x] |
| R2-VLT-05 | Team Lead diff, separate exact-SHA PR push, record Windows/Linux CI not awaited | TEAM_LEAD_SERIAL | PR #32, planner | 04 | [ ] |
| R2-VLT-REVIEW | Independent security review, installed native OS approval, real Keychain/Win Credentials/Linux Secret Service, hostile MCP server and production go/no-go | INDEPENDENT_REVIEWER_PENDING | R2 phase exit | 05 | [ ] |

## G. NC-TEAM-003 — Verification matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R2-VLT-01 | TEAM_LEAD_SERIAL | PASS | N/A: Team Lead approved source audit; no running feature | PASS | GO | Registry and OS vault deletion audit; Lead diff reviewed 2026-10-23 |
| R2-VLT-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac native module with in-memory secure-store 2/2, Registry 14/14, browser mocked 5/5; actual installed OS dialog E2E pending separately |
| R2-VLT-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 actual local TCP HTTP Bearer header fixture and server-bound vault inspector E2E; HTTP 6/6 |
| R2-VLT-04 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Rust 1111/1111, 33 ignored, Node 208, Release 33, browser 5, Vite PASS, Cargo check PASS |
| R2-VLT-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Exact SHA PR handoff and review |
| R2-VLT-REVIEW | INDEPENDENT_REVIEWER_PENDING | BLOCKED | BLOCKED | BLOCKED | NO-GO | Requires real OS installer/credentials and independent human reviewer |

## C. Gate checklist

- [x] Fresh planner and team assignment first
- [x] R2-VLT-01/02/03 implementation and scoped source tests reviewed locally
- [x] Contract tests deny raw token in registry, result, stdout, stdio argv; user-entered token transiently crosses local Tauri IPC
- [ ] Config change and synthetic leak negative tests PASS; real native dialog denial and locked OS vault remain BLOCKED
- [ ] Unit, scoped E2E, smoke logged, commit pushed
- [ ] Native Mac+Windows+Linux installed GUI/Keychain/Secret Service and independent security signoff

## E. Lead GO/NO-GO

**Team Lead verdict:** GO for scoped source development / PR; NO-GO R2 production/phase exit without independent review and native installed OS acceptance. Never make auto-agent tool calling implicit.

## Source handoff status

Team Lead GO for source PR; full R2 Production NO-GO pending genuine independent reviewer and installed Mac/Windows/Linux acceptance. Scope and logs: docs/nexora/R2-VAULT-HTTP-AUTH-2026-10-23.md.
