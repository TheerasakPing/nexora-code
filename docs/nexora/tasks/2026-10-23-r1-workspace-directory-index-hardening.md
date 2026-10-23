# Task Checklist Planner — R1 Workspace directory and search safety

**Date:** 2026-10-23. **Request:** Continue R1 sequentially toward R8, no skipping failed phase acceptance.
**Team Lead:** TEAM_LEAD_SERIAL (no actual task-delegation API or independent reviewer available in this session).
**Base:** Integration source 093f397 following two safely merged provider lifecycle tracks. PR #26 remote head 2c33f95 at beginning of this task.
**Scope:** One step of R1 directory listing/search security using real filesystem tests; integrate previous R1 provider fixes and submit exact-head CI.
**Threat model:** Directory symlink swap after path validation or during enumeration may expose external filenames; cached search may return stale/outside paths. Enforce that listing never walks outside a project and avoid inventing native GUI acceptance.
**Constraints:** No writes to user home projects or installed app, no credential access, preserve legacy public IPC. No unsafe global security bypass. Windows CI compilation/smoke is not native Windows GUI E2E.

## B. Task assignments and acceptance

| ID | Acceptance / owned area | Actual worker | Depends | Status |
| --- | --- | --- | --- | --- |
| R1-DIR-01 | Audit list_files plus file_search symlink/race behavior and document precise scope | TEAM_LEAD_SERIAL | None | [x] |
| R1-DIR-02 | Implement bounded safe directory listing for supported OS with no-follow scoped traversal | TEAM_LEAD_SERIAL | R1-DIR-01 | [x] |
| R1-DIR-03 | Real filesystem symlink adversarial E2E, unit, smoke and Team Lead source review | TEAM_LEAD_SERIAL | R1-DIR-02 | [x] |
| R1-DIR-04 | Integrate the already merged R1 provider patch, PR handoff and exact-head CI evidence | TEAM_LEAD_SERIAL | R1-DIR-03 | [ ] |
| R1-DIR-REVIEW | Independent approval before protected develop merge | INDEPENDENT_REVIEWER_PENDING | R1-DIR-04 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R1-DIR-01 | TEAM_LEAD_SERIAL | PASS | N/A: docs/security audit only | PASS | GO | Lead approved N/A 2026-10-23: reviewed list_files and file_search pathways; scoped directory code only, smoke docs+tests verify coverage; SHA in PR |
| R1-DIR-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | macOS M4 Rust workspace::tests 33/33 + actual 250-cycle directory replacement race, Unix fdopendir/fstatat no-follow, Mac cargo check PASS, Windows old path remains pending separate OS acceptance |
| R1-DIR-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac M4 Node 186/186 PASS, Rust full lib 1074 PASS/0 FAIL/33 ignored, Release 33 PASS, frontend Vite PASS, scoped native filesystem E2E 250 swaps, Team Lead diff inspection 2026-10-23 |
| R1-DIR-04 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | PR #26 exact-head 9 CI checks and signoff pending |
| R1-DIR-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | No independent reviewer available |

## Acceptance checklist

- [x] Fresh planner prior to source edits; Team Lead serial fallback recorded
- [x] Confirmed list_files TOCTOU threat; Windows, file_search and Git separately pending
- [x] Scoped native Unix filesystem E2E with 250 symlink swaps, no external filenames; Mac M4
- [x] Source diff reviewed by Team Lead and local Rust/Node/Contract/Frontend tests pass; hosted CI pending
- [ ] Nine exact-head GitHub Actions checks PASS
- [ ] Independent review, native installed GUI/session acceptance, R1 exit GO

## Team Lead handoff

**Team Lead verdict:** GO for narrow R1 code/test work; protected branch merge, R1 phase exit and Production NO-GO until independent reviewer and native acceptance.
**Historic overlapping backlog:** engineering 21/24 packages pending, 7/7 branding signoff pending, 61 R0–R8 detailed acceptance boxes unaccepted. These are not additive.


## Verification ledger — 2026-10-23, Mac mini Apple M4

- **Rust Unit/Contract:** `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib workspace::tests:: --quiet`: **33/33 PASS** after Unix FD listing change.
- **Scoped native E2E:** Actual temp directory replacement to out-of-project symlink **250 cycles**, repeated `list_files` concurrently. No path outside project was enumerated; nested dir, non-ASCII/literal filename, symlink omission and Git metadata exclusion tests passed. This is a Rust OS-file-handle integration test, **not** the native installed GUI/agent E2E.
- **Full Rust:** `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib --quiet`: **1074 PASS, 0 FAIL, 33 ignored**. `cargo check --locked --bin nexora-code`: PASS on Mac M4.
- **Node Unit + code guard:** `npm test` **186 PASS**, `node --test tests/nexora-r1-directory-anchored.test.mjs` **3 PASS**. `npm run test:release` **33 PASS**.
- **Frontend smoke:** `npm run build`: PASS with existing Vite large-chunk advisories.
- **Compatibility and boundedness:** Existing 500-entry sorted listing cap and IPC shape retained. Unix now omits symlinks even if target inside project by deliberate secure default; no user project was changed.
- **Known remaining security:** Windows directory listing is still pathname-based and not declared race-safe; `file_search`, Git stage/diff and OS live GUI/PTY/provider need separate R1 acceptance. No Apple notarization or independent reviewer.
- **Team Lead review:** Inspected OS-handle ownership, `dup/fdopendir/closedir` cleanup, `fstatat(AT_SYMLINK_NOFOLLOW)`, no-follow directory walk and bound; SOURCE GO for narrow R1 Unix listing. Protected develop merge, R1 exit and production **NO-GO**.
- **PR stage:** R1-DIR-04 exact-head GitHub CI and independent review still PENDING as of source submission. Current-head CI statuses must be checked before acceptance.

**GO/NO-GO release gates:** SOURCE GO for review of the Unix directory
handle patch; exact-head hosted CI PENDING. R1 full phase exit NO-GO,
protected develop merge NO-GO and Production NO-GO. Native Windows
listing junction E2E and genuine independent review remain missing.
