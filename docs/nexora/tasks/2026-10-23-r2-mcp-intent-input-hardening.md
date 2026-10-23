# Task Checklist — R2 MCP Intent Input Hardening

**User request:** "รวมเลย แล้วทำงานที่เหลือให้เสร้จ" — merge governance PR when allowed and advance outstanding Nexora Code roadmap tasks.
**Date / Task:** 2026-10-23 Asia/Bangkok / R2-MCP-INPUT-01.
**Team Lead:** ChatGPT Integration Lead; **workers:** TEAM_LEAD_SERIAL, NONE independently delegated.
**Branch / base:** fix/r2-mcp-intent-input-validation-20261023 from origin/develop.
**Priority:** P0 R2 security hardening; no live external MCP launch or secrets.
**Scope:** Harden invalid/control-character session identity and tool name inputs to the existing source-only approval intent ledger without expanding execution permissions. Regression tests and risk/evidence document.
**Out-of-scope:** Building production consent flow, credential vault, live MCP servers, direct main/develop push, stable release.
**Dependencies:** Existing mcp_security.rs ToolIntent from_config; Rust tests; GitHub CI and independent review.
**DoD:** Valid well-formed inputs preserved; empty/control/Unicode-laundered identities fail closed; no approval path unlocked; focused test, full test where feasible; PR with review gate.

| Work | Owner | Files | Gate | State |
| --- | --- | --- | --- | --- |
| SEC-01 | TEAM_LEAD_SERIAL | src-tauri/src/mcp_security.rs | Analyze existing validator and design tight input guard | [x] |
| SEC-02 | TEAM_LEAD_SERIAL | src-tauri/src/mcp_security.rs | Add explicit boundary tests and guards | [ ] |
| SEC-03 | TEAM_LEAD_SERIAL | docs/nexora/R2-INPUT-HARDENING.md | Document security boundary and limitations | [ ] |
| QA | TEAM_LEAD_SERIAL | read-only diff/test output | Review against regression and policy | [ ] |
| EXTERNAL | Independent reviewer (not assigned) | PR | Genuine independent review and exact-head CI | [ ] |

**Team Lead verdict:** NO-GO until implementation and actual test evidence.
**Current remaining baseline:** 21/24 engineering packages unchecked; 7/7 branding epics; 61 detailed R0–R8 phase checks. Overlapping categories, not additive.
**Task report:** Track actual completed tests, exact SHA, independent-review status, blockers and next priorities in this planner.
