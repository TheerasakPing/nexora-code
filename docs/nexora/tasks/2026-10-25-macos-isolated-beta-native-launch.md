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

## Resumed work after Mac reconnect — 2026-10-25
- Mac workspace re-bound successfully to this exact Nexora Code worktree. Both scoped W1 and W2 agent worktrees retained their previous outputs; Lead integrated W1 beta Tauri overlay and React contract test, W2 beta Keychain guards and bot/VM idle restrictions. No Nexora Studio code or installer was used.
- Tauri config `src-tauri/tauri.beta.conf.json`: different macOS bundle ID `com.theerasakping.nexoracode.beta` and app title; preserves base native window geometry. Cargo `beta-isolation` opt-in feature, build wrapper `scripts/nexora/build-mac-beta.mjs` ALWAYS passes `--features beta-isolation`; in-app setup refuses bundle ID / feature mismatch before opening stores.
- Beta native secure store `secret_store.rs`: cannot get/set/remove; `provider_settings.rs` macOS Keychain getter/setter/delete and GLM usage getter fail closed; `usage_keychain.rs` blocked; `agent_usage.rs` returns Beta-disabled message without spawning provider CLI; `lib.rs` Beta startup does not widen provider CLI PATH or start bots/VM idle watchers.
- Test launcher `scripts/nexora/launch-mac-beta.sh`: checks bundle identifier, signature and executable, makes fresh `/private/tmp/nexora-code-beta-home.XXXXXXXX`, runs with `env -i` and minimal PATH. Only for first-run GUI smoke; do NOT click Agent/Provider/usage tasks or mistake it for security sandbox. CoreGraphics exact-PID read-only GUI witness `scripts/nexora/mac-beta-window-probe.swift` typechecked PASS.
- Lead Mac Host: `npm run build` PASS, `npm test` 210/210 PASS, `npm run test:release` 36 PASS + one platform skip from earlier host run; Beta config/build/startup/launcher Node contracts **4/4 PASS** `/tmp/nxc-beta-guard-final2.log`, `zsh -n` PASS; `git diff --check` PASS. Rust Native Beta `tauri build --debug --features beta-isolation --bundles app` process is compiling, **not yet** a GUI pass. Global `cargo fmt --all --check` shows unrelated pre-existing formatting differences, not treated as an application build failure.
- First independent security reviewer /tmp/nxc-beta-independent-review.log REQUEST CHANGES; second /tmp/nxc-beta-review-delta.log REQUEST CHANGES; Lead fixed all source issues. Final independent read-only reviewer `/tmp/nxc-beta-review-final3.log` confirms **ACCEPT DEVELOPMENT / SAFE_TO_RUN_ISOLATED_NATIVE_GUI** for **limited no-click GUI smoke via sanitized launcher only**. It does **not** authorize Agent credentials/tasks, not signed nor notarized Production, and it did not itself launch GUI. Mac launch must wait for actual built bundle inspection.
- [x] T01 source config + contract delivered; [x] T02 source Keychain/Usage guards delivered; [x] T03 independent read-only review ACCEPT for limited Beta window smoke; [ ] Native macOS bundle build/code signature/hash and first GUI smoke remain PENDING; [ ] user acceptance of Agent/Provider UI, Production/native 3-OS release remains PENDING.
