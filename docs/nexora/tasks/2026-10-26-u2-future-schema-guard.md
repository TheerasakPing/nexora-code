# Nexora Code U2 — protect databases from future-schema downgrades (2026-10-26)

## A. Team Lead intake
- Request: continue U2 Runtime/Session/Data Migration hardening.
- Source basis: `origin/develop` f0e18e8, branch `fix/20261026-session-runtime-u2`; isolated worktree.
- Discovery: `Store::open` initializes WAL/DDL before checking database `user_version`, and does not reject schemas newer than `SCHEMA_VERSION` (25).
- Lead: ChatGPT; no real delegated workers attached, assign `TEAM_LEAD_SERIAL`. Independent review remains pending.
- Goal: fail closed and avoid altering the database of a newer installed app; preserve recovery and current-schema migration.
- No access to real user DB; all cases use temporary test databases.

## B. Ownership
| ID | Deliverable | Owner | Status |
| --- | --- | --- | --- |
| U2-01 | Guard against newer SQLite user_version before any app DDL | TEAM_LEAD_SERIAL | [ ] |
| U2-02 | Regression verifying future DB remains intact, current DB opens | TEAM_LEAD_SERIAL | [ ] |
| U2-QA | Unit/contract, smoke and Lead diff review | TEAM_LEAD_SERIAL | [ ] |

## G. NC-TEAM-003 verification
| Task | Unit/Contract | E2E | Smoke | Lead review | Evidence |
| --- | --- | --- | --- | --- | --- |
| U2-01 | PENDING | PENDING | PENDING | PENDING | Rust Store::open test |
| U2-02 | PENDING | PENDING | PENDING | PENDING | Temp SQLite fixture |
| U2-QA | PENDING | PENDING | PENDING | PENDING | cargo logs / native integration |

## E. Gate
- Reject future schema non-destructively, with actionable error; never auto-migrate backwards.
- Opening older/current databases must remain backward compatible.
- Native installed-app acceptance and security review remain separate, not implied by unit results.

## D. Actual verification handoff — macOS Apple Silicon
- `cargo test --locked --manifest-path src-tauri/Cargo.toml newer_schema_refuses_downgrade_without_changing_database_bytes`: PASS, 1 new disk-backed SQLite test; original wrapper command incorrectly attempted to assign reserved zsh variable `status` after the cargo command, so wrapper returned 1. The Cargo log explicitly confirms test PASS, not test failure.
- `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib storage::tests`: **56 PASS, 1 ignored, 0 fail**. Includes current and legacy migration, actual process-kill SQLite recovery and previous schema fixture.
- `npm run test:nexora:r0`: PASS (fork attribution/updater isolation smoke, not native GUI).
- `cargo fmt --check`: FAIL on repository-wide pre-existing formatting differences, first in unmodified `agent_tools.rs`; no global reformat committed.
- `git diff --check`: pending.
- U2-01 Unit: PASS; E2E: PASS for SQLite file open/close/reopen fixture; Smoke: PASS R0. Lead: inspected focused diff, independent reviewer PENDING.
- U2-02 Unit: PASS; E2E: PASS for SQLite bytes equality and future-schema rejection on disk; Smoke: PASS current-schema/legacy regression. Lead: inspected test, independent reviewer PENDING.
- Native Finder/Windows installed interactive E2E, full multi-session runtime and production security review: PENDING.
- **NO-GO** to production; code ready for PR/CI review, not independently accepted for native release.
