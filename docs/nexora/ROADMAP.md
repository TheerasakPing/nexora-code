# Nexora Code — Engineering Roadmap

**This is a fork of PandaCode** — https://github.com/aicodeking/pandacode.
Pinned PandaCode v0.3.16 source SHA: 30f9c823e3c38f472f89e8f8405575ecfe7173ee.

Development, multi-agent, MCP, quality gates, native acceptance and release operations follow [SOP NC-SOP-001](SOP-DEVELOPMENT-RELEASE.md).

| Phase | Work | Gate |
|---|---|---|
| R0 | Fork, identity, attribution, CI, disable upstream updater | original functionality unaffected; native build smoke |
| R1 | Rust runtime, process cleanup, session durability, path security | crash, stop/resume and recovery pass |
| R2 | MCP registry/stdio/HTTP, tool/resource/prompt inspector, vault | secure multi-server transport and approvals |
| R3 | bounded agent scheduler, worktree isolation, mailbox/review | four agents parallel with no unsafe writes |
| R4 | typed Workflow DAG, durable journal, no-replay | crash and UNKNOWN block unverified effects |
| R5 | workflow studio, agent teams and observability UX | real desktop E2E and accessibility |
| R6 | native macOS M4, Windows 11 x64, Linux x64 | actual clean install and UI/provider/terminal tests |
| R7 | security, dependency, 8-agent 2h performance soak | no critical risks, bounded memory and processes |
| R8 | signed production release, updater, rollback | release evidence / publisher approval |

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
