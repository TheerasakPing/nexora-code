# PandaCode releases 0.3.17–0.3.50 — Nexora triage inventory

**Captured:** 2026-10-26. Each version below was read from its corresponding `releases/0.3.N.md` at upstream tag `v0.3.N`. This is a **release-notes triage**, not a source-code diff or proof that Nexora lacks a feature. For actual inclusion compare changed files and current Nexora implementation; classifications can change.

**Legend**
- `AUDIT`: likely already partially implemented; verify parity and preserve local contracts.
- `CANDIDATE`: selectively consider for local fork, after source/security review.
- `EXCLUDE`: remote/cloud/account capability conflicts with current Nexora local-first scope.
- `DEFER`: optional platform capability outside current Mac ARM64 / Windows x64 priority.
- No row is `ACCEPTED` or `PORT COMPLETE`.

| Upstream release | Highlight from upstream notes | Provisional triage | Nexora roadmap linkage |
| --- | --- | --- | --- |
| [0.3.17](https://github.com/aicodeking/pandacode/releases/tag/v0.3.17) | Duo simulator fold/GUI responsiveness | DEFER | C UI / device simulator |
| [0.3.18](https://github.com/aicodeking/pandacode/releases/tag/v0.3.18) | Verified `npx` installer; local mac signing | CANDIDATE: adapt only to Nexora keys | C Packaging / U7 |
| [0.3.19](https://github.com/aicodeking/pandacode/releases/tag/v0.3.19) | PandaCode Connect remote control | EXCLUDE | local-first rule |
| [0.3.20](https://github.com/aicodeking/pandacode/releases/tag/v0.3.20) | Remote task dispatch and clone; remote keychain fix | EXCLUDE remote; AUDIT credential isolation separately | A session safety |
| [0.3.21](https://github.com/aicodeking/pandacode/releases/tag/v0.3.21) | Command Code + Oh My Pi harnesses, lead effort, GitHub Actions UI | CANDIDATE / AUDIT | A providers / B teams |
| [0.3.22](https://github.com/aicodeking/pandacode/releases/tag/v0.3.22) | Remote persistence and remote model filtering | EXCLUDE remote; AUDIT model validation | A Model Discovery |
| [0.3.23](https://github.com/aicodeking/pandacode/releases/tag/v0.3.23) | Human-readable tool traces, tool summaries | AUDIT overlap with existing activity modules | B dashboard / U4 |
| [0.3.24](https://github.com/aicodeking/pandacode/releases/tag/v0.3.24) | Chats, named provider connections, design, skills, workflows, GitHub Copilot, search | AUDIT / CANDIDATE per feature | A+B+C |
| [0.3.25](https://github.com/aicodeking/pandacode/releases/tag/v0.3.25) | Windows ARM installer; team lead guidance | DEFER ARM; AUDIT team guidance | B Multi-Agent |
| [0.3.26](https://github.com/aicodeking/pandacode/releases/tag/v0.3.26) | Updated GLM catalog model IDs | CANDIDATE: verify real vendor catalog first | A Model Discovery |
| [0.3.27](https://github.com/aicodeking/pandacode/releases/tag/v0.3.27) | Cline harness; steering across ACP harnesses | CANDIDATE / AUDIT | A session/providers |
| [0.3.28](https://github.com/aicodeking/pandacode/releases/tag/v0.3.28) | Muse Code harness via MSP | DEFER unless supported CLI required | A Provider |
| [0.3.29](https://github.com/aicodeking/pandacode/releases/tag/v0.3.29) | Remote access credential store change | EXCLUDE remote; assess migration safety only | A Data Migration |
| [0.3.30](https://github.com/aicodeking/pandacode/releases/tag/v0.3.30) | OMP large catalog chunking; panel persistence | CANDIDATE / AUDIT | A catalog / C UI |
| [0.3.31](https://github.com/aicodeking/pandacode/releases/tag/v0.3.31) | Cloud agent providers and work sync | EXCLUDE | local-first rule |
| [0.3.32](https://github.com/aicodeking/pandacode/releases/tag/v0.3.32) | Bot icons and consolidated bot settings | DEFER / AUDIT | C UI |
| [0.3.33](https://github.com/aicodeking/pandacode/releases/tag/v0.3.33) | Menu bar running/approval/usage shortcuts | CANDIDATE | C UI |
| [0.3.34](https://github.com/aicodeking/pandacode/releases/tag/v0.3.34) | Local secret store and keychain migration | AUDIT SECURITY: no credential import without consent | A MCP/Data |
| [0.3.35](https://github.com/aicodeking/pandacode/releases/tag/v0.3.35) | Rich menu bar team/status & usage bars | CANDIDATE | B dashboard / C UX |
| [0.3.36](https://github.com/aicodeking/pandacode/releases/tag/v0.3.36) | Mobile client and desktop remote API | EXCLUDE | local-first rule |
| [0.3.37](https://github.com/aicodeking/pandacode/releases/tag/v0.3.37) | Mobile client and desktop remote API | EXCLUDE | local-first rule |
| [0.3.38](https://github.com/aicodeking/pandacode/releases/tag/v0.3.38) | Remote trust changes + mobile appearance | EXCLUDE | local-first rule |
| [0.3.39](https://github.com/aicodeking/pandacode/releases/tag/v0.3.39) | Remote Full access guardrail | EXCLUDE remote; audit permission rule applicability | A MCP security |
| [0.3.40](https://github.com/aicodeking/pandacode/releases/tag/v0.3.40) | Updater modes, per-connection usage, Cline console, cloud fixes | CANDIDATE local features, EXCLUDE cloud | A providers / C Updater |
| [0.3.41](https://github.com/aicodeking/pandacode/releases/tag/v0.3.41) | Attached-file chips and drag/drop fallback | AUDIT / CANDIDATE | C UX / U5 |
| [0.3.42](https://github.com/aicodeking/pandacode/releases/tag/v0.3.42) | Fix thread width in narrow window | CANDIDATE small fix | C UX |
| [0.3.43](https://github.com/aicodeking/pandacode/releases/tag/v0.3.43) | Devin CLI ACP, model-specific efforts, catalog cache, no empty discovery sessions | CANDIDATE with explicit provider opt-in | A Model Discovery / U3 |
| [0.3.44](https://github.com/aicodeking/pandacode/releases/tag/v0.3.44) | Remote follow-up, progress and image transport | EXCLUDE remote | local-first rule |
| [0.3.45](https://github.com/aicodeking/pandacode/releases/tag/v0.3.45) | Active/Needs review/Completed sidebar, inspector tabs, live diffs | CANDIDATE; compare sidebar and diff ownership | C UX / U5 |
| [0.3.46](https://github.com/aicodeking/pandacode/releases/tag/v0.3.46) | Immediate first message for remote task | EXCLUDE remote; audit generic local send semantics | A session |
| [0.3.47](https://github.com/aicodeking/pandacode/releases/tag/v0.3.47) | Team status, newest-first reads, database/request latency | CANDIDATE local optimizations; EXCLUDE phone link | B teams / C perf |
| [0.3.48](https://github.com/aicodeking/pandacode/releases/tag/v0.3.48) | Centered conversation handoff row | CANDIDATE | C UX |
| [0.3.49](https://github.com/aicodeking/pandacode/releases/tag/v0.3.49) | LAN/Tailscale pairing + Connect diagnostics | EXCLUDE; audit generic reconnect UX only | local-first rule |
| [0.3.50](https://github.com/aicodeking/pandacode/releases/tag/v0.3.50) | Live tool traces/cards/source chips; smoother streaming | AUDIT / CANDIDATE | B dashboard / C UI |

## Immediate U1 source-audit backlog
- [x] Read all 34 upstream release-note files and create a preliminary local-first triage.
- [ ] Pin and compare actual upstream source/commits per version to Nexora `develop`; add file paths and API/schema changes.
- [ ] Search current Nexora modules (not only sampled paths) for overlap; record ALREADY vs PORT vs ADAPT vs DEFER vs EXCLUDE with evidence.
- [ ] Identify third-party licenses and attribution requirements for any borrowed assets/code.
- [ ] Identify migration, provider permission, tool activity and native renderer contract changes.
- [ ] Assign real isolated workers and produce per-slice planner and test matrix.

Review [implementation plan](UPSTREAM-SYNC-U1-U7-2026-10-26.md) and [project SOP](SOP-DEVELOPMENT-RELEASE.md). No upstream merge or production release is authorized by this inventory.
