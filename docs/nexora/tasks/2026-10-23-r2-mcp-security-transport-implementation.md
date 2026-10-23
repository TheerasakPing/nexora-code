# Task Checklist Planner — R2 MCP security, transport and inspector implementation

**Date / User request:** 2026-10-23 / “ทำ R2ให้เสร้จ” (finish R2).
**Lead:** ChatGPT, TEAM_LEAD_SERIAL. Nexora Studio workspace has no exposed task-delegation API; no independent reviewer or falsely assigned engineers. Work is serialized and split into bounded packages.
**Location / PR:** isolated worktree `/Volumes/Data/workspace/git/nexora-code-integration-20261023`, PR #26 into develop. Branch remote may be concurrently advanced; compare remote SHA before any push. No changes to user's installed app, private credentials, local Keychain, app database, or existing original checkout.
**Goal:** implement as many security-gated R2 deliverables as testable now. Do not connect to untrusted servers, start external binaries, or collect/store user secrets until verified trusted UI consent, secure vault and transport controls exist.
**R2 completion gate:** ALL native/security consent + isolated credentials + stdio and pinned Streamable HTTP lifecycle + inspector + real OS E2E + reviewer. Source-only implementation or CI does NOT close R2. R1 remains NOT accepted; explicit R2 priority requested supersedes previous phase order for code work, not for release acceptance.

## B. Lead decomposition, ownership and DoD

| ID | Deliverable / acceptance | Actual worker | File scope | Dependency | Status |
| --- | --- | --- | --- | --- | --- |
| R2-01 | Inventory audit: verify current Registry, ACL, disabled live executor and backlog, identify leaks | TEAM_LEAD_SERIAL | src-tauri/src/mcp_*.rs / docs | None | [ ] |
| R2-02 | Implement bounded fail-closed JSON-RPC MCP frame and catalog parser + pagination/capability/diagnostic constraints | TEAM_LEAD_SERIAL | src-tauri/src/mcp_protocol.rs, lib.rs | R2-01 | [ ] |
| R2-03 | Add strict credential-reference-only schema; no inline secret/argv/env/token export; preserve v1 registry compatibility | TEAM_LEAD_SERIAL | src-tauri/src/mcp_registry.rs, frontend model/tests if needed | R2-01 | [ ] |
| R2-04 | Model operator consent expiry/revocation, explicit request/attempt audit and safe offline mock transport tests; don't grant renderer live exec | TEAM_LEAD_SERIAL | src-tauri/src/mcp_security.rs / mcp_permissions.rs | R2-01 | [ ] |
| R2-05 | Unit/contract, native Rust mock E2E, Node guards, frontend/build and smoke on Mac M4; record results | TEAM_LEAD_SERIAL | tests/, docs/nexora, this planner | R2-02..04 | [ ] |
| R2-06 | Lead diff/security inspection and safe PR integration after remote fetch, exact-head 3-OS CI | TEAM_LEAD_SERIAL | PR #26 and evidence docs | R2-05 | [ ] |
| R2-REVIEW | Actual independent reviewer, Mac/Windows/Linux installed native operator consent/transport/secret E2E | INDEPENDENT_REVIEWER_PENDING | Approval gate | R2-06 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R2-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Baseline source audit and live gate smoke pending |
| R2-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Rust protocol mock frames and pagination denial tests pending |
| R2-03 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Registry schema fixture, migration and no-leak contract pending |
| R2-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Consent/expiry/replay and mock denied executor pending |
| R2-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Node/Rust/real subprocess mock E2E and smoke pending |
| R2-06 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Git diff review and exact SHA CI pending |
| R2-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | Real native E2E and independent review absent |

## C. Checklist

- [x] Planner created first, actual ownership and serial fallback recorded
- [ ] Scope R2 existing code + vulnerabilities reviewed
- [ ] Parser, credential-reference validation and consent safety packages tested
- [ ] Native local test/CLI/mock E2E and smoke with exact SHA recorded
- [ ] PR safely rebased/merged with concurrent remote branch
- [ ] Native all-OS 3-platform E2E and independent reviewer completed
- [ ] R2 phase acceptance GO with real secure servers, no missing critical gates

## E. Lead initial verdict

**GO:** narrow R2 implementation, fixtures and source-level review.
**NO-GO:** live external MCP use, credential import, protected merge, R2 Production acceptance until reviewed, signed off and full native/security E2E.
**Reference:** docs/nexora/CHECKLIST-PLANNER.md R2 (10 unchecked), docs/nexora/PROGRESS-SNAPSHOT-2026-10-23.md; other phase counts overlap.
