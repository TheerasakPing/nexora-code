# Nexora Code — Engineering Roadmap

**This is a fork of PandaCode** — https://github.com/aicodeking/pandacode.
Pinned PandaCode v0.3.16 source SHA: 30f9c823e3c38f472f89e8f8405575ecfe7173ee.

Development, multi-agent, MCP, quality gates, native acceptance and release operations follow [SOP NC-SOP-001](SOP-DEVELOPMENT-RELEASE.md).
Track actionable tasks and release gates in [Checklist Planner NC-PLAN-001](CHECKLIST-PLANNER.md).
Local R0–R2 merge evidence: [2026-10-22 integration report](INTEGRATION-R0-R2-2026-10-22.md).

| Phase | Work | Gate |
|---|---|---|
| R0 | Fork, provisional identity, attribution, CI, updater isolation and brand inventory | source baseline and identity/legal inventory review |
| R1 | Rust runtime, process cleanup, session durability, path security | crash, stop/resume and recovery pass |
| R2 | MCP registry/stdio/HTTP, tool/resource/prompt inspector, vault | secure multi-server transport and approvals |
| R3 | bounded agent scheduler, worktree isolation, mailbox/review | four agents parallel with no unsafe writes |
| R4 | typed Workflow DAG, durable journal, no-replay | crash and UNKNOWN block unverified effects |
| R5 | workflow studio, agent teams, observability UX and full rebrand design system BR-02/03 | brand and accessibility E2E on native desktop |
| R6 | macOS M4, Windows 11 x64, Linux x64 branded installers BR-04/06 | native install, identity, UI/provider/terminal checks |
| R7 | security, dependency, 8-agent 2h performance soak | no critical risks, bounded memory and processes |
| R8 | signed production release, fork-owned updater, rollback and rebrand BR-07 | security, legal, branding, 3-OS evidence and publisher approval |

## Mandatory Rebranding Workstream BR-01 through BR-07

Rebranding is a cross-phase engineering workstream, not optional visual polish.
Nexora Code is a provisional working name until approved by the Project Owner.

BR-01 Brand identity approval; BR-02 logo and platform icon assets;
BR-03 design system, light/dark and accessibility; BR-04 user-facing copy,
About, menus and package metadata; BR-05 safe data/executable/tool migration;
BR-06 branded macOS/Windows/Linux installers; BR-07 legal, signing and release gate.

These are additional to the 24 planned engineering PRs. Basic source-level Tauri
renaming does not mean full rebranding, migration or native release acceptance passed.
See [Rebranding & Product Identity Master Plan](REBRANDING-PLAN.md) for owners,
dependencies, status inventory, acceptance tests and explicit NO-GO criteria.

## 24 Engineering PRs

01 attribution; 02 app identity/update isolation; 03 native CI; 04 provider regression;
05 Rust modularity; 06 process supervision; 07 session recovery; 08 path security;
09 MCP registry; 10 stdio/Streamable HTTP; 11 tools/credentials; 12 MCP inspector;
13 Multi-Agent scheduler; 14 isolated worktrees; 15 team messaging/checkpoints;
16 review + merge gates; 17 typed workflow DAG; 18 journal/executor;
19 Workflow Studio; 20 Agent Dashboard; 21 Mac acceptance;
22 Windows acceptance; 23 Linux acceptance; 24 security and release gates.

## Architecture commitments

Retain Tauri v2, Rust backend and vanilla TypeScript initially; no unnecessary React rewrite.
Keep upstream provider/session command compatibility. Preserve original LICENSE and user-visible
fork attribution. Isolate credentials, data and updates. Never auto-retry an UNKNOWN mutating
tool call. Do not treat a per-agent Git worktree as an OS security sandbox.
