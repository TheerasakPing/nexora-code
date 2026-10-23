# NC-TEAM-003 — R1 Linux crash fixture ready-marker race (2026-10-24)

Repository: TheerasakPing/nexora-code. Parent PR #33; target PR #36, head `03b31921fe3f87a86fcd576ca8e54126fae87fe8`.
Team Lead: ChatGPT Integration Lead. Actual worker: TEAM_LEAD_SERIAL (no independent delegated agents). Independent reviewer required.

## Checklist Planner (created BEFORE source changes)
- [x] R1-LINUX-01 / QA analyst — exact-head CI failure: baseline run 39981633161, Linux job 120510140478; `r1_actual_os_kill_recovers_sqlite_session_and_never_replays_tool` returned `Conversation not found` at storage.rs:2723. Mac/Windows passed.
- [ ] R1-LINUX-02 / Rust test engineer — patch the crash fixture's publication handshake so a parent never kills a child between ready-file creation and content write. Test-only; production unchanged.
- [ ] R1-LINUX-03 / Test engineer — Unit/Contract: valid UUID and eight-ID payload published completely. Native E2E: real child kill/reopen and repetitions on Mac.
- [ ] R1-LINUX-04 / Build QA — Smoke: focused + full native Rust, Node, release contracts, Vite build, Clippy. Record exact commands and platform limits.
- [ ] R1-LINUX-05 / Team Lead — review diff, safety, crash semantics, tests and GO/NO-GO; no simulated independent approval.
- [ ] R1-LINUX-06 / CI owner — exact-head hosted macOS ARM64, Windows x64, Linux x64, installer baseline + independent review before protected merge.

## Root-cause hypothesis and safety boundary
The child uses `std::fs::write(path.with_extension("ready"), payload)`: creating the file precedes writing all bytes. The parent polls `ready.exists()` and may read an empty/partial payload then kill the writer. This is a test publication race, not proven production SQLite corruption.
Write complete data to an adjacent temporary marker, then atomically rename to `*.ready`; the parent cannot see a partial marker. Apply to both single and eight-session crash fixtures. Preserve real OS kill with open SQLite and recovery assertions.
No user DBs, runtime storage, permissions, secrets, protected branches, or force push.

## Exit / handoff
- [ ] Team Lead source GO with evidence. PR merge NO-GO without exact-SHA hosted gates and independent reviewer.
- [ ] R1 phase exit NO-GO pending installed native GUI/PTY/provider and platform endurance acceptance.
