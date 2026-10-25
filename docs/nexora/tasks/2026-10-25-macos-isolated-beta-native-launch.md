# Task Checklist — Nexora Code macOS isolated beta first launch
Date: 2026-10-25 Asia/Bangkok. Request: "ต่อเลยรอใช้งานแล้วนะ". Product **Nexora Code only**; fork PandaCode. Base PR45 c2239b0.
## A. Intake / Lead
- Goal: truly usable Mac arm64 Beta, with **separate Bundle ID/app data AND keychain credential namespace or disabling credential access**, and no mutation to `/Applications/Nexora Code.app` or Nexora Studio. Previous ad-hoc same-ID beta copies exist but NOT launched, reviewer NO SAFE LAUNCH.
- Branch `feat/20261025-mac-beta-isolated`; integration worktree `/Volumes/Data/workspace/git/nexora-code-mac-isolated`, base `c2239b0`.
- Constraints: macOS 27.2 arm64, 34GiB free (Cargo existing target large); Tauri 2; no bypass Gatekeeper/security dialogs, no existing app kill, no real secrets/agent operations, no production release.
- Distinct identifier `com.theerasakping.nexoracode.beta` via separate Tauri config. All direct Keychain reads/writes of other apps must be prevented (cfg beta feature / denial or alternate scoped service). Prove isolated app data path and no global keychain/direct usage read at startup. Off by default for production.
- Source changes require actual code-level reviewer, smoke/GUI-native whenever safe; can do install-only if reviewer refuses launch. Native startup may need user interactive Gatekeeper approval, report BLOCKED not PASS.
- No merge PR45/develop/main, no production signing or release.
## B. Actual assignments / dependencies
| ID | Deliverable + acceptance | Owner | Files | Depends | State |
|---|---|---|---|---|---|
| T01 | Separate Beta Tauri config and feature; macOS app identity unique, no collision | W1 specialized implementation agent (to be launched) | `src-tauri/tauri.beta.conf.json`, `src-tauri/Cargo.toml`, scoped tests if appropriate | none | [ ] |
| T02 | Audit and protect Keychain, usage lookups, bots/autostart from user credentials under Beta; fail-closed and tests | W2 specialized implementation agent (to be launched) | `src-tauri/src/secret_store.rs`, `src-tauri/src/provider_settings.rs`, `src-tauri/src/usage_keychain.rs`, `src-tauri/src/lib.rs`, credential tests | none | [ ] |
| T03 | Independent read-only review and diff integration, then Mac build and safe smoke | Reviewer + Team Lead | inspection/QA artifacts | T01,T02 | [ ] |
| T04 | Dated accounting, install guide, native acceptance and user handoff | Team Lead | planner and standalone notes | T03 | [ ] |
## C. Evidence checklist
- [x] New planner written before editing production files.
- [ ] Agents actually delegated nonoverlapping scopes and worktrees; Lead examined diffs.
- [ ] Mac arm64 isolated Tauri config and binary SHA/config verified.
- [ ] Frontend TypeScript/Node contracts PASS.
- [ ] Rust targeted tests/build passes; isolated macOS .app code-signed integrity and actual GUI process smoke or BLOCKED.
- [ ] Independent reviewer GO for safe launch.
- [ ] No existing app/data modified; no real provider credentials accessed.
## G. NC-TEAM-003 per-subtask matrix
| ID | Worker | Unit/Contract | E2E | Smoke | Lead Review | Evidence |
|---|---|---|---|---|---|---|
| T01 | W1 | PENDING | PENDING native Mac | PENDING app bundle | PENDING | code SHA/QA |
| T02 | W2 | PENDING | PENDING native credential avoidance | PENDING safe launch | PENDING | code SHA/QA |
| T03 | independent reviewer + Lead | PENDING | PENDING | PENDING | PENDING | reviewer log |
| T04 | Lead | N/A: documentation only pending Lead signoff | N/A: report only pending signoff | PENDING guide link | PENDING | dated ledger |
## D. Handoff
- Independent reviewers not conflated with Codex writing workers.
## E. Go/no-go
- Delivery PENDING; isolated beta native GUI PENDING; merge NO-GO; production NO-GO.
## F. Remaining acceptance accounting
- Latest verified source-of-truth snapshot 2026-10-23: engineering packages 3 source-complete/24 (21 still unchecked), brand 0 accepted/7, R0–R8 11 checked/72 (61 unchecked), counts overlapping. Current PR45 CI 11/11 green; native Mac installed GUI not accepted. Do not invent completion percentage.
