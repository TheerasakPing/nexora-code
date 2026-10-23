# Task Checklist Planner — R2 operator-approved MCP Tool Execution (opt-in)

**Date / request:** 2026-10-23, user explicitly authorizes enabling MCP tool invocation for actual R2 tests.
**Owner:** ChatGPT Integration Lead, TEAM_LEAD_SERIAL (no engineering worker delegation API or independent reviewer available).
**Source:** PR #32 branch docs/nc-team-002-progress-accountability-20261023, isolated Mac worktree.
**Safety scope:** Opt-in manual tool execution ONLY. No blanket/grant-on-enable, no auto Agent tool access, no unreviewed credentialRef, no shell execution from renderer, no silent browser approvals. Existing native dialog must display exact server, tool and bounded arguments; reject config changes and selected tool absent from advertised catalog; one-time expiring in-memory ticket consumed for exact request, OS group cleanup.
**R2 phase exit:** Need 3-OS actual installed GUI + operator approval E2E, OS secure storage/secret handling, hostile executable/network assessments and real independent security review. Do not mark full phase GO from mock/browser/native unit-only checks.

## B. Decomposition and ownership
| ID | Deliverable / Acceptance | Worker | Files / worktree | Depends | Status |
| --- | --- | --- | --- | --- | --- |
| R2-EXEC-01 | Native tool-approval modal with server/tool/arguments, single-use intent ticket; no renderer consent token | TEAM_LEAD_SERIAL | mcp_security, mcp_inspector | prior R2 foundation | [ ] |
| R2-EXEC-02 | Typed tool executor in read-only transports; only explicit call path after verified advertised capability; scoped deadlines/caps | TEAM_LEAD_SERIAL | mcp_http, mcp_stdio, mcp_inspector | R2-EXEC-01 | [x] |
| R2-EXEC-03 | UI manual tool action with explicit JSON preview, disable during use, escape all returned text | TEAM_LEAD_SERIAL | mcp-center.ts/css | R2-EXEC-02 | [x] |
| R2-EXEC-04 | Real local stdio + pinned-loopback HTTP tool-call integration and deny/replay tests, browser E2E, Node/Rust smoke | TEAM_LEAD_SERIAL | tests, source | R2-EXEC-01..03 | [x] |
| R2-EXEC-05 | Team Lead source inspection, PR push and exact-head cross OS CI | TEAM_LEAD_SERIAL | PR/evidence | R2-EXEC-04 | [ ] |
| R2-EXEC-REVIEW | Actual independent security review + real native installed app approval/deny E2E on Mac/Windows/Linux | INDEPENDENT_REVIEWER_PENDING | release acceptance | R2-EXEC-05 | [ ] |

## G. NC-TEAM-003 — Subtask Verification Matrix
| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| R2-EXEC-01 | TEAM_LEAD_SERIAL | PASS | BLOCKED | PASS | NO-GO | Native dialog code/intent-bound ticket and replay tests PASS; real installed Mac/Windows/Linux human approve/deny E2E not run (owner native QA) |
| R2-EXEC-02 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac ARM64 Rust mcp_inspector 10/10 PASS with actual disposable stdio/loopback HTTP tool call, advertised-tool enforcement, 128KiB caps, no real external credentials; Team Lead source diff checked |
| R2-EXEC-03 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac ARM64 Playwright browser mock 4/4 PASS: user Run Once, precise JSON args, malformed JSON denial, remote markup escaped; mock Tauri IPC not native dialog |
| R2-EXEC-04 | TEAM_LEAD_SERIAL | PASS | PASS | PASS | GO | Mac ARM64 Rust full 1104 PASS / 0 failed / 33 ignored, Node 204 PASS, Release 33 PASS, Vite PASS; Linux test real descendant check retained after accepting Broken pipe diagnostic |
| R2-EXEC-05 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | CI + PR source acceptance pending |
| R2-EXEC-REVIEW | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | Independent security and native GUI tests unavailable |

## C. Checklist
- [x] Fresh Planner written before modifying implementation, real team limitations recorded.
- [ ] Native tool approval requires interactive dialog and displays all user-supplied arguments.
- [x] One-time scoped ticket consumed exactly once in mocked/native-Rust tests, denied commands never cross transport.
- [x] UI can manually invoke a listed tool and see bounded escaped result in browser-mocked IPC E2E; real native UI acceptance BLOCKED.
- [x] Scoped real stdio subprocess + pinned HTTP localhost E2E and local Rust/Node/Release smoke PASS.
- [ ] CI current SHA 3-OS passes; Team Lead inspects changed files.
- [ ] Independent review + installed OS acceptance; only then R2 Exit GO.

## E. Team Lead
**Team Lead verdict:** GO for narrow source implementation and PR submission only, pending actual tests. **GO/NO-GO:** R2 production/phase exit NO-GO pending independent reviewer, real native UI/permissions/credential lifecycle and full multi-server security acceptance.

## Verified local handoff / outstanding blockers

- [x] Lead source review of exact tool intent, no renderer consent token, native dialog full argument display, config revalidation, advertised-tool catalog and one-use ticket.
- [x] Rust local `cargo test --locked --lib --quiet`: 1,104/1,104 PASS (33 ignored), Node 204/204 PASS, release 33/33 PASS, Vite build PASS.
- [x] Rust `mcp_inspector::tests::` 10/10 PASS, stdio + HTTP fixture actual tools/call executes test-only fixture (no production files/credentials).
- [x] Playwright browser mock `tests/mcp-inspector.spec.ts` 4/4 PASS; display output via textContent, never execute HTML strings.
- [x] Linux provider regression scope: allow safe bounded Broken pipe or exited stdout diagnostic but always require no remaining live descendant.
- [ ] Real native OS modal human accept/deny E2E (Mac, Windows, Linux).
- [ ] Exact head CI and installer checks after push (R2-EXEC-05).
- [ ] True independent security review, real credentials, adversarial real servers, R2 Exit/production approval.

**Team Lead GO/NO-GO:** GO for PR source integration; **NO-GO** for full native R2 release acceptance. R2-EXEC-01 blocked on human native dialog E2E and requires a new review before its [x] can be set.
Documentation: `docs/nexora/R2-MANUAL-TOOL-EXECUTION-2026-10-23.md`.

## Final Mac M4 source/test evidence (2026-10-23)

- Rust Unit/Contract + scoped E2E: `CARGO_TARGET_DIR=/Volumes/Data/workspace/git/nexora-code/src-tauri/target cargo test --locked --manifest-path src-tauri/Cargo.toml --lib --quiet`: **1104 PASS / 0 failed / 33 ignored**.
- `node --test tests/nexora-r2-manual-tool-gate.test.mjs`: **3 PASS**; `npm test`: **204 PASS**; `npm run test:release`: **33 PASS**; `npm run build`: PASS; `cargo check --locked --bin nexora-code`: PASS.
- `npx playwright test tests/mcp-inspector.spec.ts --workers=1`: **4 PASS** using mocked Tauri IPC only; actual native user approval **BLOCKED**.
- Integration tests spawn an ephemeral `sh` fixture with an actual tool-call marker only under /tmp, and a localhost HTTP server that must receive `tools/call` only after catalog advertisement. Denied/unknown tool and wrong or reused ticket never run any tool.
- Linux previously failing provider lifecycle test updated to accept legitimate bounded `Broken pipe` or exited-producer diagnostics, while retaining the process descendant liveness assertion. Exact SHA Linux CI still pending.
- **Lead review:** sources confined to native mcp_inspector/mcp_http/mcp_stdio/mcp_security/JS UI and test fixture, no renderer-supplied approval bool, no model API/network external or user data/secret access. Source PR submission GO. **No-go** for R2 production until installed GUI and independent security review.
