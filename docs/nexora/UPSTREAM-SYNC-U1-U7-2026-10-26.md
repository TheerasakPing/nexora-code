# Nexora Code — PandaCode selective upstream integration (U1–U7)
**Plan ID:** NC-UPSTREAM-2026-10-26 · **Status:** APPROVED FOR PLANNING ONLY / NOT IMPLEMENTED · **Target:** PandaCode v0.3.50

## Why and source of truth
- Fork base declared in [Nexora roadmap](ROADMAP.md): PandaCode v0.3.16 at `30f9c823e3c38f472f89e8f8405575ecfe7173ee`.
- Proposed upstream **review** target: [PandaCode v0.3.50](https://github.com/aicodeking/pandacode/releases/tag/v0.3.50), release source `15541a5` (2026-10-26).
- The 34-number release gap **is not** a 34-feature gap; Nexora may already have equivalent functionality. Audit all release notes `0.3.17..0.3.50`, changed files, commits, and local equivalents before backport.
- Preserve fork attribution and license, Nexora visual identity, Tauri 2/Rust/TypeScript, local-first configuration, existing storage and CLI sessions, app bundle IDs, separate updater endpoints and signing keys.
- Do **not** `git merge upstream/main`, force-push, auto-copy third-party credentials, import remote pairing/cloud relay/account services, or assume a cherry-pick is safe.
- A validated upstream change may be *ported/adapted*, *already implemented*, *excluded intentionally*, or *deferred*. Record evidence and reason for each category.

## Observed starting evidence (not feature acceptance)
- Upstream `0.3.50` includes live trace rail, result cards with exit code/diff/search, source chips, and streaming partial-link handling.
- Upstream `0.3.49` mainly adds device pairing, LAN/Tailscale and relay; remote connection itself is **EXCLUDED** because Nexora scope is local-first. Only platform-neutral, locally applicable error recovery can be evaluated.
- Upstream `0.3.47` advertises newest-first loading, shorter DB contention, and team status; use as **design input** with local measurements, not a claimed win.
- Upstream `0.3.45` adds Active / Needs review / Completed sidebar groups, an inspector tab strip, and streaming diff list.
- Upstream `0.3.43` adds Devin CLI and five-minute model catalog caching / model-specific reasoning levels; validate CLI authorization and source before adding a new harness.
- Upstream `0.3.41` adds attachment chips and drop fallback.
- Local Nexora `develop` already contains `src/tool-activity.ts`, `src/turn-activity.ts`, `src/attachments.ts`, `src/sidebar-tree.ts` and `src/model-picker.ts`. These are **overlap candidates**, not proof of feature parity.
- Existing A–C development work (Model Discovery, MCP, Sessions, Migration, Multi-Agent, DAG, Dashboard, UI, Performance, Updater) remains independent; upstream tasks below **extend** A–C, not replace them.

## Execution roadmap (sequential gates; parallel only with isolated file ownership)
| Batch / phase | Priority | Implementation task and likely files | Dependencies | Deliverable / exit criterion | State |
| --- | --- | --- | --- | --- | --- |
| **U1 Source inventory / A+B+C** | P0 | Review `v0.3.16...v0.3.50` tags, release notes, diff, third-party deps, DB migration, native APIs; audit Nexora implementations | None | Versioned feature-to-file matrix: **ALREADY / PORT / ADAPT / EXCLUDE / DEFER**, owner, risk, conflicts and per-feature tests | **IN PROGRESS — notes sampled; full source diff pending** |
| **U2 Runtime, session and performance / A+C** | P0 | Profile startup and long threads; selectively adapt deterministic newest-first paging, bounded DB reads, stop/retry/reconnect state fixes where local | U1; R1 session/storage contracts | No regression in 8-session recovery, no orphan subprocesses, migration safety; measured latency and resource baseline before/after | PENDING |
| **U3 CLI/model discovery / A** | P0/P1 | Audit upstream Devin ACP adapter and model catalog caching; reconcile `src/model-picker.ts`, `src-tauri/src/providers.rs`, provider registries and stable model IDs | U1 and A Model Discovery fixes | Explicit opt-in new harness (if supported), no CLI login side effect from discovery, bounded timeout/cache invalidation, no empty sessions, clear failure UX | PENDING |
| **U4 Live traces, activity cards / B+C** | P1 | Diff v0.3.50 trace data schema and rendering against `src/tool-activity.ts`, `src/turn-activity.ts`; adapt tool card/error/diff/search, reduced-motion, streamed incomplete Markdown | U1; R4 journal and Agent Dashboard contracts | Safe escaped/size-bounded cards, stable stream ordering, last two open, folded older traces, diagnostics without credential leaks | PENDING |
| **U5 Task navigation/inspector/attachments / C** | P1 | Selectively port v0.3.45 sidebar groups and diff inspector, v0.3.41 chips/drop fallback; integrate Nexora brand, dark mode, Thai, accessibility | U1; U4 view contracts | Sidebar selection/filter persistence, no task loss, inspector keyboard/AX, docs/images safe previews, full-feature regression | PENDING |
| **U6 Integration, security & data parity / A+B+C** | P0 | Resolve conflicts and migrations on isolated fixtures; compare all changed APIs and permission modes; run focused unit/contract, browser E2E, native E2E and smoke | U2–U5 | Same-SHA CI + OS evidence, 8-worker/long-session gating where affected; independent review, zero unresolved P0 | PENDING |
| **U7 Internal beta packaging and release gate / C** | P0 | Rebuild Mac Apple Silicon and Windows x64 from verified `develop` SHA; provenance, SHA-256, updater isolation; optional signed production only after owner gate | U6; R6–R8 | Installs open from Finder/Windows, no data loss, provider/MCP/terminal sanity, rollback runbook; separate production authorization | PENDING |

## Mandatory acceptance / no regression contracts
1. **Local-only by default:** no upstream Connect, relay, account prompts, remote tunnel or phone feature silently enabled.
2. **Data safety:** schema version and WAL migration audited on disposable copies; old sessions/tools/model IDs remain readable. No edits to real user data to produce proof.
3. **Process safety:** bounded discovery, stop/cancel, stale generation fencing, no orphan process on failure; no side-effectful UNKNOWN call automatically replayed.
4. **MCP isolation:** no invoking an MCP tool without its exact user consent; vault credentials never leak into renderer, arguments, trace cards or logs.
5. **UI:** preserve Nexora design, Thai localization, dark/light contrast, keyboard navigation and reduced motion. Upstream UI assets are not automatically copied.
6. **Release independence:** Nexora GitHub/updater public key, signing secrets, bundle name and app data root remain fork-owned. Preserve legal attribution.
7. **Evidence:** every worker handoff names actual commit, tests, OS, code reviewer, E2E and smoke outcome. Builds and preview E2E do not substitute for installed native acceptance.

## Work assignment & review architecture
- **Integration Lead / reviewer:** owns classification, integration conflicts, scope, acceptance matrix and final NO-GO/GO.
- **Proposed workers when real agents are available:** RUNTIME (U2), PROVIDER (U3), ACTIVITY/UI (U4), PRODUCT UX (U5), QA/SECURITY (U6), RELEASE (U7). These are **roles**, not active agents.
- **Current execution:** `TEAM_LEAD_SERIAL` until a real delegated worker is attached. Each implementation worker must own non-overlapping paths/worktree; parallel PRs may merge only in dependency order after the Lead reviews diffs.
- Create **one new task planner per actual assignment** according to [SOP NC-TEAM-001](SOP-DEVELOPMENT-RELEASE.md), and add a subtask verification row for every implemented subtask per NC-TEAM-003.

## Verification / evidence required for each implementation PR
- [ ] File-level upstream diff and provenance (upstream tag SHA, local base SHA, old/new API or schema)
- [ ] Focused unit + contract tests PASS with command/OS/SHA
- [ ] Browser or service E2E PASS with command/OS/SHA (or **N/A + specific reason + Lead approval**)
- [ ] Smoke PASS including affected startup/stop/logout-free local routes (or reasoned N/A)
- [ ] Native Mac + Windows installed E2E PASS separately when affecting native runtime, provider or package
- [ ] Security/data compatibility review, UI accessibility and Thai/dark mode as relevant
- [ ] Team Lead diff review, CI checks, independent reviewer when required
- [ ] Changelog, rollback note and released installer hashes only after exact artifact verification

## Definition of done / progress reporting
- Do not mark U1 complete until the entire v0.3.17–v0.3.50 inventory has file-by-file classifications; sampling six releases is only a start.
- Do not mark U2–U5 complete merely because upstream already implements them: verify Nexora's own behavior and acceptance.
- U6 requires green current-SHA CI plus native security/data evidence; U7 internal beta requires both target artifacts; production requires separate R8 signing/notarization and owner authorization.
- **As of creation:** plan drafted; implementation **0 of 7 U-batches accepted**. This is not the overall Nexora completion percentage.

## References
- https://github.com/aicodeking/pandacode/releases/tag/v0.3.50
- https://github.com/aicodeking/pandacode/tree/v0.3.50/releases
- https://github.com/TheerasakPing/nexora-code
- [Nexora roadmap R0–R8](ROADMAP.md)
- [Current acceptance checklist](CHECKLIST-PLANNER.md)
