# R3 Task Checklist — Interrupted Worker Worktree Claims

- **Created before code changes:** 2026-10-22.
- **User request:** Continue Nexora Code development after consolidated PR; advance R3 crash safety without disabling branch checks.
- **Team Lead:** acting assistant (TEAM_LEAD_SERIAL). Independent worker/reviewer unavailable.
- **Priority:** P0 recovery safety; R3 Phase 3.
- **Branch / worktree:** feat/r3-orphan-claim-audit; /Volumes/Data/workspace/git/nexora-code-r3-claim-audit.
- **Base:** integration/r0-r4-consolidated (PR #18).
- **Scope:** read-only audit of durable preparing worktree claims against Git branch and actual registered checkout; typed classification, explicit errors and evidence, tests, docs.
- **Out of scope:** automatic retry, deletion, prune, branch/worktree creation, real Agent process, public IPC, GUI install, MCP, develop/main merge or release.
- **Acceptance:** all Git/DB inspection read-only; no table creation on read; missing Git repo or I/O failures never reported as confirmed absence; branch-only and linked checkout distinguished; no Git metadata spoof accepted; tests with temp Git repo and persisted SQLite, all checks green.
- **Security:** A preparing claim is UNKNOWN by definition; presence of a branch is not proof of success. Never auto-authorize reconciliation.

| Task | Worker | Ownership | Acceptance | Status |
| --- | --- | --- | --- | --- |
| R3-C-01 | TEAM_LEAD_SERIAL | src-tauri/src/workspace.rs | bounded read-only branch/worktree inspection | [x] |
| R3-C-02 | TEAM_LEAD_SERIAL | src-tauri/src/architect/reconcile.rs and architect.rs | DB claim report + no effects + tests | [x] |
| R3-C-03 | TEAM_LEAD_SERIAL | docs/nexora/R3-CLAIM-RECOVERY-AUDIT.md | human triage + limits | [x] |
| R3-C-QA | TEAM_LEAD_SERIAL | full diff review | Rust, Clippy, Node, CI, PR evidence | [x] |
| R3-C-REVIEW | INDEPENDENT_REVIEWER_PENDING | GitHub review | genuine approval before protected merge | [ ] |

- [x] Plan created before edits and isolated scope maintained.
- [x] Branch-only, real linked worktree, missing and corrupt metadata tested.
- [x] Errors classified UNVERIFIED, never absence/success; no mutating operations.
- [x] Full tests, denied-warning Clippy and source review complete.
- [ ] PR opened with exact result, CI and reviewer gates.
- [ ] Protected-branch merge and release remain blocked until real approval.

**Team Lead GO/NO-GO:** GO for isolated source PR submission. NO-GO for protected merge or production execution/release pending exact-head GitHub CI, authorized independent Reviewer and native OS acceptance.

## Real Team Lead acceptance evidence (2026-10-22, Mac M4)

- **Real worker:** TEAM_LEAD_SERIAL. No independent Agent Worker or reviewer ran.
- **Focused recovery regression:** 6 PASS; branch-only/missing, legitimate linked checkout, removed/prunable checkout, unknown project, invalid ref, no schema creation.
- **Full Rust library:** 1056 PASS, 0 FAIL, 33 Ignored; Rust CLI integration 1 PASS.
- **Strict Rust Clippy:** all-targets -D warnings PASS.
- **Frontend:** 143 PASS, 0 FAIL; release tests 31 PASS; TypeScript/Vite build and R0 updater/identity checks PASS.
- **Team Lead scope review:** Only read-only Git and SQLite inspection plus internal-only module registration. All unknown Git states stay unverified; every PREPARING claim still needs human triage.
- **Explicitly not done:** actual Agent provisioning/recovery, reconciler mutation, native GUI run, real external MCP, signing, installation, native 3-OS E2E or protected branch merge.
- **GitHub exact-head CI/independent reviewer:** PENDING at submission, to be recorded separately.
