# NC-TEAM-003 — R1 Linux crash fixture ready-marker race (2026-10-24)

Repository: TheerasakPing/nexora-code. Parent PR #33; target PR #36, head `03b31921fe3f87a86fcd576ca8e54126fae87fe8`.
Team Lead: ChatGPT Integration Lead. Actual worker: TEAM_LEAD_SERIAL (no independent delegated agents). Independent reviewer required.

## Checklist Planner (created BEFORE source changes)
- [x] R1-LINUX-01 / QA analyst — exact-head CI failure: baseline run 39981633161, Linux job 120510140478; `r1_actual_os_kill_recovers_sqlite_session_and_never_replays_tool` returned `Conversation not found` at storage.rs:2723. Mac/Windows passed.
- [x] R1-LINUX-02 / Rust test engineer — ready marker atomic publication for single and eight-session fixtures; test-only; production unchanged. Source reviewed.
- [x] R1-LINUX-03 / Test engineer — ready contract 1 PASS, both real process E2E PASS; 10 consecutive runs: 100 single-session + 50 eight-session OS kills PASS on Mac M4.
- [x] R1-LINUX-04 / Build QA — focused Rust 5 PASS; full Rust 1116 PASS / 0 FAIL / 33 ignored; Mac Clippy -D warnings PASS; Node 208 PASS; release 33 PASS; Vite build PASS after npm ci --ignore-scripts.
- [x] R1-LINUX-05 / Team Lead — inspected only production-unchanged test fixture, atomic marker contract, parent actual Child::kill and persisted recovery, Mac test evidence; GO for PR source patch ONLY, no outside reviewer.
- [ ] R1-LINUX-06 / CI owner — exact-head hosted macOS ARM64, Windows x64, Linux x64, installer baseline + independent review before protected merge.

## Root-cause hypothesis and safety boundary
The child uses `std::fs::write(path.with_extension("ready"), payload)`: creating the file precedes writing all bytes. The parent polls `ready.exists()` and may read an empty/partial payload then kill the writer. This is a test publication race, not proven production SQLite corruption.
Write complete data to an adjacent temporary marker, then atomically rename to `*.ready`; the parent cannot see a partial marker. Apply to both single and eight-session crash fixtures. Preserve real OS kill with open SQLite and recovery assertions.
No user DBs, runtime storage, permissions, secrets, protected branches, or force push.

## Exit / handoff
- [x] Team Lead source GO with local evidence. PR merge NO-GO without exact-SHA hosted gates and independent reviewer.
- [ ] R1 phase exit NO-GO pending installed native GUI/PTY/provider and platform endurance acceptance.

## NC-TEAM-003 Subtask Verification Matrix

| Subtask ID | Worker (actual) | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| R1-LINUX-01 | TEAM_LEAD_SERIAL | N/A (diagnostic log audit, no code) | FAIL observed (prior Linux hosted E2E) | N/A (diagnostic review only) | GO for diagnosis | Actual Linux job 120510140478 reported missing session; no assertion that failed CI passed |
| R1-LINUX-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Atomic temp-write + rename for both child fixtures; Rust focused suite 5/5; real OS kill works |
| R1-LINUX-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | macOS M4: 10 runs each of both process E2E fixtures: 150 test-owned child process kills, all PASS; 2 SQLite restart checks per cycle |
| R1-LINUX-04 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac full Rust 1116 PASS/33 ignored; Clippy all-targets -D warnings PASS; Node 208 PASS; Release 33 PASS; Vite build PASS |
| R1-LINUX-05 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Lead inspected test-only patch, no runtime behavior change, fixture OS kill still checked and scoped data only; source GO not external approval |
| R1-LINUX-06 | CI_PENDING | PENDING | PENDING | PENDING | PENDING | Exact new PR head CI and independent reviewer unavailable prior to push |

## Local commands and result ledger
- `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib storage::tests::r1_ -- --test-threads=4` — 5 PASS.
- 10 further iterations of `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib storage::tests::r1_ --quiet -- --test-threads=4` — 150 test-owned child kills, 0 FAIL on Mac M4; child OS process, not installed GUI/agents.
- `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib --quiet` — 1116 PASS, 0 FAIL, 33 ignored.
- `cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` — PASS.
- `npm test` — 208/208 PASS; `npm run test:release` — 33/33 PASS.
- First `npm run build` failed because checkout lacked local `tsc` binary; after `npm ci --ignore-scripts`, `npm run build` — PASS.
- `git diff --check` — PASS.
- Pending: exact-HEAD hosted Linux/macOS/Windows native tests and installed GUI/PTY/provider scenario; no independent reviewer granted.
