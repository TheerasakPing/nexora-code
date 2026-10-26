# Nexora Code — Mac/Windows release handoff checklist (2026-10-26)

## Team Lead intake / NC-TEAM-001
- Request: continue work possible without connected Mac; prioritize Mac Apple Silicon and Windows 11 x64 installers.
- Source: `develop` at time of branch creation; GitHub run [40274425475](https://github.com/TheerasakPing/nexora-code/actions/runs/40274425475).
- Lead: ChatGPT; independent delegated worker agents: NONE; all tasks assigned `TEAM_LEAD_SERIAL`.
- Read-only review and documentation only. Do not touch user files, secrets, installed profiles or production updater.
- Native interactive testing, signing/notarization and independent human approval remain separate gates.

## Planner — tasks and ownership

| ID | Owner | Deliverable | Status |
| --- | --- | --- | --- |
| REL-01 | TEAM_LEAD_SERIAL | Recheck Mac and Windows job results at exact run SHA | PENDING: tunnel unavailable; cannot certify final results |
| REL-02 | TEAM_LEAD_SERIAL | Verify each downloadable artifact filename, size, checksum and boot smoke report | PENDING |
| REL-03 | TEAM_LEAD_SERIAL | Mac installed-app tests: Finder launch, Homebrew PATH, provider discovery, working model, terminal and MCP | BLOCKED: connected Mac unavailable |
| REL-04 | TEAM_LEAD_SERIAL | Windows 11 x64 installed-app tests: launch, Node discovery, no flashing terminals, model and terminal | BLOCKED: Windows machine unavailable |
| REL-05 | TEAM_LEAD_SERIAL | Backward-compatible session/database migration and clean rollback rehearsal using disposable copy | PENDING |
| REL-06 | TEAM_LEAD_SERIAL | Human security/branding/legal review and publisher signoff | PENDING |
| REL-QA | TEAM_LEAD_SERIAL | Independent evidence audit and final GO/NO-GO | PENDING |

## NC-TEAM-003: verification matrix

| Task | Unit/contract | E2E | Smoke | Lead review | Evidence |
| --- | --- | --- | --- | --- | --- |
| REL-01 | N/A: status audit | PENDING | PENDING | PENDING | inspect [run 40274425475](https://github.com/TheerasakPing/nexora-code/actions/runs/40274425475) |
| REL-02 | PENDING | PENDING | PENDING | PENDING | compare checksums and actual artifact bytes |
| REL-03 | N/A: installed-app QA | BLOCKED | BLOCKED | PENDING | real macOS Apple Silicon run required |
| REL-04 | N/A: installed-app QA | BLOCKED | BLOCKED | PENDING | real Windows 11 x64 run required |
| REL-05 | PENDING | PENDING | PENDING | PENDING | disposable data fixture only |
| REL-06 | N/A: human review | N/A: review only | N/A: review only | PENDING | separate identified human approval |

Do not mark a subtask complete without checked unit/contract, E2E and smoke evidence, or an explicit justified N/A accepted by Lead. Build success never substitutes for native interaction.

## Acceptance criteria (release blockers)
1. Both target installers produced **from exactly the same SHA**, with artifact links, SHA-256 checksums and retained provenance.
2. Mac installed GUI opens normally from Finder, without white screen; providers and available models discover correctly.
3. Windows installed GUI opens normally without repeated Node/terminal flashes and discovers models.
4. Sessions, user settings and original compatibility survive migration; rollback preserves a backup of real data.
5. MCP permissions/credentials are fail-closed and local-only behavior is preserved.
6. No outstanding critical CI failures, no P0 security issue, independent review recorded.
7. Signed/notarized *production* binaries and updater metadata are validated before calling anything a public release. Unsigned artifacts must be labelled test-only.

## Verified vs unverified
- Earlier macOS browser preview `tests/ui.spec.ts`: 124/124 passed at a QA worktree; not equivalent to native testing.
- At an earlier status check, Mac/Linux unsigned artifact jobs succeeded and Windows was still running. This is historical, **not** a current final outcome.
- GitHub PR [#49](https://github.com/TheerasakPing/nexora-code/pull/49) remains open at review time; focused animation browser tests claimed in its PR, but no native-provider acceptance implied.
- Current production-release gate: **NO-GO**; allow unsigned internal beta artifacts only after verifying their build and smoke reports.

## Team Lead handoff
- No worker-agent execution or independent code review claimed.
- Next priority: (1) capture final Windows CI and artifact evidence; (2) installed Mac native smoke; (3) Windows native smoke; then migration/security/release approval.
- Do not merge into protected `main` solely from this documentation.
