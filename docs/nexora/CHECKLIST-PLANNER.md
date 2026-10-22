# Nexora Code — Checklist Planner / แผนงานและรายการตรวจสอบ (R0–R8)

> **Document:** NC-PLAN-001 · **Version:** 1.0 · **Created:** 2026-10-22
> **Repository:** [TheerasakPing/nexora-code](https://github.com/TheerasakPing/nexora-code)
> **Working branch:** develop · **Product name:** Nexora Code (provisional)
> **Source:** Fork of [PandaCode](https://github.com/aicodeking/pandacode), upstream v0.3.16
> **Planning policy:** [SOP NC-SOP-001](SOP-DEVELOPMENT-RELEASE.md) · [Roadmap](ROADMAP.md) · [Rebranding Plan](REBRANDING-PLAN.md)

This is a **living checklist**, not a claim that R0–R8 or production deployment is complete.
Only check an item after its evidence and acceptance gate are recorded. Do not invent completion percentages or dates.

## สรุปสำหรับติดตามงาน (ภาษาไทย)

**จุดประสงค์:** ใช้ไฟล์นี้เป็นแผนงานหลักของ Nexora Code สำหรับติดตามงานตาม Roadmap ไม่ใช่หลักฐานว่าโครงการเสร็จ 100% โดยเครื่องหมาย [x] หมายถึงมีหลักฐานของงานที่ระบุไว้เท่านั้น ส่วน [ ] หมายถึงยังไม่ผ่านเกณฑ์หรือยังทำไม่ครบ

| เฟส | สถานะปัจจุบัน | สิ่งที่ต้องทำต่อ |
| --- | --- | --- |
| R0 — พื้นฐาน Fork | เสร็จเฉพาะ Source Baseline | ตรวจ GitHub CI ทุก OS และตั้งกฎป้องกัน main |
| R1 — ความเสถียร | บางส่วน | แก้ Process ค้าง, Session Recovery และ Filesystem TOCTOU |
| R2 — MCP | บางส่วน | Credential Vault, Approval, Connection Manager, Tool Inspector |
| R3 — Multi-Agent | ยังไม่ผ่าน Acceptance | Scheduler, Worktree, Handoff, Review Gate |
| R4 — Workflow DAG | ยังไม่ผ่าน Acceptance | Durable Journal, Checkpoint, No-Replay |
| R5 — UX/UI | ยังไม่ผ่าน Acceptance | Branding, Workflow Studio, Agent Dashboard, Accessibility |
| R6 — 3 Platforms | ยังไม่ผ่าน Native E2E | macOS M4, Windows 11 x64 และ Linux x64 |
| R7 — Hardening | ยังไม่ผ่าน | Security Audit, 8-Agent Soak 2 ชม., Crash/Resume |
| R8 — Production | BLOCKED | Sign/Notarize, Updater, Rollback, อนุมัติ Release |

**Checklist ที่ควรเริ่มทันที (P0/P1)**

- [ ] ตรวจผล GitHub Actions ทั้ง macOS, Windows และ Linux ให้ได้ผลสุดท้าย พร้อมเก็บ Log ของแต่ละ OS
- [ ] กำหนด Required CI และ Reviewer ก่อน Merge เข้า main
- [ ] ตรวจสอบและแก้ปัญหา Process/Session ค้างหรือไม่คืนทรัพยากรหลัง Stop/Restart
- [ ] เพิ่ม Crash Recovery และทดสอบการอ่านเขียน Workspace อย่างปลอดภัยเมื่อเจอ Symlink Race
- [ ] ออกแบบการอนุมัติ MCP แบบผูกกับ Session/Server/Tool (ห้ามใช้ enabled=true แทนสิทธิ์)
- [ ] เพิ่ม Credential Vault และห้ามบันทึก Secret ลง Config, Argument, Renderer หรือ Logs
- [ ] สร้าง MCP stdio/HTTP Connection Manager หลัง Security Gate ผ่าน และทดสอบกับ Mock ก่อน
- [ ] ขออนุมัติชื่อแบรนด์ โลโก้ และแนวทางออกแบบจาก Project Owner

**วิธีอัปเดต:** หลังแต่ละ PR ให้เปลี่ยน [ ] เป็น [x] เฉพาะเมื่อ Acceptance Tests ผ่านแล้ว และแนบ Commit SHA, CI Run, ผู้ตรวจ และข้อจำกัดที่ยังเหลือไว้ในส่วนท้ายของเอกสาร ห้ามเดาเปอร์เซ็นต์ความคืบหน้าจากจำนวนไฟล์ที่แก้

## How to use this planner

- [x] means the defined, narrow deliverable has verifiable source/commit evidence.
- [ ] means remaining work, including any item marked **PARTIAL**, **BLOCKED**, or **IN PROGRESS**.
- Use P0 = security/data-loss/release blocker, P1 = reliability/core capability, P2 = feature/UX.
- A designated **Role** is a suggested owner, not confirmation a person/agent is assigned.
- Record PR link, test report, reviewer, platform and actual completion date before ticking a gate.
- A feature implemented in source is **not** equivalent to native-tested, approved or production-ready.
- Do not mark OS acceptance based on build, GitHub CI or emulation alone.

## 1. Snapshot — 22 October 2026

| Workstream | Status | Verified evidence / next gate |
| --- | --- | --- |
| R0 — fork/source foundation | **PARTIAL / source baseline present** | Identity separation, attribution, update isolation; CI and branding signoff pending |
| R1 — stability & filesystem | **PARTIAL** | Path checks and unsigned Mac alpha artifact; lifecycle, recovery and TOCTOU unresolved |
| R2 — Advanced MCP | **PARTIAL / configuration-only** | Registry, MCP Center, fail-closed preflight, network policy; no live connections |
| R3 — Multi-Agent | **PLANNED** | Bounded scheduler and worktree ownership not accepted |
| R4 — Workflow Engine | **PLANNED** | Durable DAG/journal/no-replay not accepted |
| R5 — Product UX | **PLANNED** | Full design system, workflows, accessibility and rebrand |
| R6 — 3-OS packages | **PLANNED** | Native Mac, Windows 11 x64, Linux x64 installations and tests |
| R7 — security / performance | **PLANNED** | Audit, fault injection, soak and resource gates |
| R8 — production release | **BLOCKED** | Signing, notarization, 3-OS evidence, rollback, approval |
| Brand BR-01–BR-07 | **PROPOSED / PARTIAL** | Final product identity requires Project Owner approval |

**Git baseline:** R0–R2 integration commit **2223764**, pushed to origin/develop.
**GitHub CI snapshot:** [Run 39724720602](https://github.com/TheerasakPing/nexora-code/actions/runs/39724720602) at the time of drafting: Frontend **success**, macOS/Windows/Linux Rust jobs **in progress**. Check live status; this is not a final CI conclusion.
**Local integration evidence:** [R0–R2 Report](INTEGRATION-R0-R2-2026-10-22.md): Node 110 PASS; release tests 31 PASS; Rust 1006 PASS, 33 ignored; CLI integration 1 PASS; Clippy PASS.
**Important:** main remains distinct from develop; no approved production installer or live Nexora MCP service.

## 2. NOW — critical path / next working batch

Suggested owners are roles. Choose one item per agent/worktree, avoid concurrent edits of shared native contracts.

- [ ] **NOW-01 / P0 / CI-Release:** Confirm all jobs in the [cross-platform Actions run](https://github.com/TheerasakPing/nexora-code/actions) finish; capture per-OS result and logs. **DoD:** no red required checks, or tracked failure with root cause and owner. **Depends:** none.
- [ ] **NOW-02 / P0 / Integration:** Enable protected-branch merge policy and review gate for main; confirm develop CI is required before promotion. **DoD:** settings/evidence recorded; no direct unreviewed release merges. **Depends:** NOW-01.
- [ ] **NOW-03 / P1 / Rust Runtime:** Audit native child-process trees, PID/generation and shutdown for stop, pause, crash and restart. **DoD:** orphan-process regression and bounded cleanup verified. **Depends:** none.
- [ ] **NOW-04 / P1 / Sessions:** Persist session state and checkpoints with crash-consistent recovery. **DoD:** stop/relaunch/resume tests; no duplicated mutating effects. **Depends:** NOW-03.
- [ ] **NOW-05 / P0 / Filesystem:** Close symlink/TOCTOU gaps in workspace file operations using safe OS handles. **DoD:** adversarial race tests pass on supported operating systems. **Depends:** none.
- [ ] **NOW-06 / P0 / MCP-Security:** Implement trusted **operator consent** and action-scoped approvals (project/session/server/config/tool); no approval path for renderer or agent spoofing. **DoD:** denied-by-default and revoked/expired/replay test cases. **Depends:** R2 inventory and policy foundation.
- [ ] **NOW-07 / P0 / MCP-Security:** Implement OS credential references and redaction; no secrets in registry JSON, argv, renderer state or logs. **DoD:** secure-store fixtures and negative-leak tests. **Depends:** NOW-06 contract.
- [ ] **NOW-08 / P0 / MCP-Runtime:** Design real stdio and Streamable HTTP connection supervisor with kill/timeout/response caps. **DoD:** mocked lifecycle and no unapproved connection. **Depends:** NOW-05 through NOW-07.
- [ ] **NOW-09 / P1 / QA:** Establish CI artifacts for Rust/Node test counts, known ignored tests, Clippy and failure triage. **DoD:** reproducible report per SHA and platform. **Depends:** NOW-01.
- [ ] **NOW-10 / P0 / Brand:** Decide whether provisional product name, legal/trademark checks and visual direction are approved. **DoD:** signed-off BR-01 decision. **Depends:** Owner review.

**No-GO during this batch:** do not start actual MCP servers, import secrets, migrate user data, enable the original PandaCode updater, merge to main or label the unsigned Mac alpha DMG as production.

## 3. R0–R2 — detailed checklists

### R0 — Fork and engineering baseline

- [x] Pin PandaCode upstream source and preserve attribution/LICENSE.
- [x] Establish fork-specific Tauri identity, application storage and updater isolation in source.
- [x] Establish native CI workflow and Node/Rust verification scripts.
- [x] Create GitHub fork and push integrated develop branch.
- [x] Create [SOP](SOP-DEVELOPMENT-RELEASE.md) and [local integration report](INTEGRATION-R0-R2-2026-10-22.md).
- [ ] Complete all remote macOS, Windows and Linux **CI** jobs with stored run evidence.
- [ ] Confirm independent GitHub repo ownership, CI protections, reviewers and branch rules.
- [ ] Resolve final brand decision and product identity approval (BR-01).
- [ ] Establish native E2E and regression baseline, not just source-only tests.

**R0 exit:** native CI green, baseline fixtures documented, identity/security review signed off.

### R1 — Stability, processes and filesystem

- [x] Add case-variant Git metadata/path validation to workspace operations.
- [x] Produce and verify unsigned/ad-hoc Mac alpha packaging artifact **for engineering only**.
- [ ] Prevent symlink-swap, TOCTOU and unsafe workspace race behavior.
- [ ] Implement supervised process groups, resource caps and bounded cleanup.
- [ ] Verify terminal/provider session start, stop, pause and resume.
- [ ] Implement durable session checkpoint/recovery, orphan cleanup and crash diagnostics.
- [ ] Fix long-running multi-session hangs and stale generations under load.
- [ ] Verify CLI PATH when app is launched via Finder, especially Homebrew node/npx/uvx.
- [ ] Native macOS UI/PTY/provider E2E and crash tests.

**R1 exit:** native restart/recovery and child process shutdown pass on Mac; no known P0/P1 stability regression.

### R2 — MCP security, transport and inspector

- [x] Persist isolated, typed MCP Registry with strict backend validation.
- [x] Add MCP Center configuration editor for stdio / Streamable HTTP.
- [x] Add fail-closed native preflight; enabled does **not** grant execution.
- [x] Add source-only IP/DNS target policy and loopback literal/port hardening.
- [ ] Implement vault-backed credential references (not inline secrets).
- [ ] Implement trusted operator approvals for connection, discovery and each action class.
- [ ] Enforce one-use, expiring, scoped authorization at the executor boundary.
- [ ] Add stdio process supervision, isolated environment and process-tree cleanup.
- [ ] Implement Streamable HTTP client with pinned socket peers, verified TLS hostname and redirect denial.
- [ ] Handle initialize/capability negotiation, tools/list, resources/list and prompts/list.
- [ ] Implement tool/resource/prompt inspector with diagnostics and status.
- [ ] Add bounded timeouts/output, pagination caps, cancellation, retries and audit trails.
- [ ] Verify SSRF/rebinding, symlink races, malformed streams, permission bypass and secret leaks.
- [ ] Native MCP approval and server lifecycle E2E across all supported platforms.

**R2 exit:** secure multi-server connections with explicit reviewed permissions and native test evidence. Existing provider-managed MCP remains separate until intentionally integrated.

## 4. R3–R8 — future release stages

### R3 — Multi-Agent scheduler and isolation (Priority P1)

- [ ] Specify typed task graph, ownership, limits and scheduling policy.
- [ ] Implement bounded worker queue and per-agent process/session identity.
- [ ] Assign each worker its own Git worktree, file scope and write lease.
- [ ] Implement agent mailbox, checkpoint, cancellation and generation fencing.
- [ ] Add reviewer/merge gates and conflict-resolution workflow.
- [ ] Test **four workers running together** without unsafe concurrent writes.
- [ ] Test queue limits and graceful handling of 1/4/8/16-worker loads.

**R3 gate:** four-worker concurrent acceptance, safe handoff, bounded memory and no silent cross-agent writes.

### R4 — Durable Workflow DAG (Priority P1)

- [ ] Define typed DAG nodes and versioned workflow schema; validate cycles and inputs.
- [ ] Add durable execution journal with intent before side effects.
- [ ] Implement checkpoint, retry policy and idempotency when supported.
- [ ] Treat uncertain mutating outcomes as **UNKNOWN**; block automatic replay.
- [ ] Add approval/reconcile node handling and crash-safe resume tests.
- [ ] Verify power-loss / force-kill / stale generation recovery scenarios.

**R4 gate:** UNKNOWN blocks dependent effects; crash cannot create duplicate mutations.

### R5 — Studio UX, observability and rebrand (Priority P1–P2)

- [ ] Approve two-tone brand mark, icon family and design tokens.
- [ ] Build Workflow Studio with clear node states and error traces.
- [ ] Build Agent Team Dashboard (roles, queues, running jobs, budgets, handoffs).
- [ ] Implement MCP health/permission/credential UX with audit visibility.
- [ ] Complete light/dark/system styles and compact accessible layouts.
- [ ] Test keyboard, screen readers, reduced motion, contrast and visual regressions.
- [ ] Verify source and native performance do not regress versus baseline.

**R5 gate:** user workflows are comprehensible, accessible and native GUI E2E-tested.

### R6 — Native installers on three OS (Priority P0)

- [ ] **Mac Apple Silicon M4:** signed DMG (initial QA may use unsigned internal alpha), Finder launch, WKWebView, PTY, Keychain and Provider E2E.
- [ ] **Windows 11 x64:** NSIS EXE, WebView2, ConPTY, PATH discovery, OS credential storage and process cleanup.
- [ ] **Linux x64:** AppImage/DEB target, WebKitGTK, X11/Wayland, PTY and Secret Service.
- [ ] Verify install, first run, multi-session, close/reopen, coexistence and uninstall on each OS.
- [ ] Test product name, icons, About, attribution and safe user-data migration.
- [ ] Collect native acceptance report, screenshots, logs and checksums per OS.

**R6 gate:** native installers and full smoke/GUI tests pass on real platform targets; a CI compile alone is insufficient.

### R7 — Security, stress and release candidates (Priority P0)

- [ ] Security threat model and audit of IPC, tools, local files, Git, network, credentials and updates.
- [ ] Dependency/supply-chain review; SBOM and vulnerability disposition.
- [ ] Run **8 workers for 2 hours** and record peak memory, CPU, process counts, hangs and errors.
- [ ] Execute **50 crash/cancel/resume cycles per OS** with no duplicated side effects.
- [ ] Meet SOP performance budget and failure/cancellation limits.
- [ ] Close all Critical/P0 issues; record any formally accepted residual risks.
- [ ] Freeze release candidate and complete regression suite on all three OS.

**R7 gate:** no open critical release risk and reproducible stress/security evidence.

### R8 — Production approval, signing, updater and rollback (Priority P0)

- [ ] Confirm project name, legal/attribution, licenses and owned signing identities.
- [ ] Configure Nexora-owned updater endpoint/public keys; verify upstream updater isolation.
- [ ] Sign and notarize macOS release; verify Windows and Linux publishing identities/packages.
- [ ] Generate checksums, SBOM, release notes and provenance attestations.
- [ ] Test upgrade, reinstall, rollback and prior-version user data preservation.
- [ ] Obtain Security Reviewer and Product Owner approval.
- [ ] Publish stable installers and perform post-release smoke/incident monitoring.

**R8 gate:** release owner signs off on 3-OS install evidence, signing, security, recovery and rollback.

## 5. Mapping to the planned 24 engineering PRs

These checkboxes track the **complete PR acceptance**, not whether some source code already exists.
No upstream/legacy work is counted as completed merely because it is included in the fork.

| PR | Planned scope | State | Prerequisite / exit note |
| --- | --- | --- | --- |
| 01 | Attribution and legal provenance | [x] Source complete | Native final branding review remains under BR |
| 02 | Fork identity and safe updater isolation | [x] Source complete | Fork updater remains disabled |
| 03 | 3-OS CI baseline | [ ] IN PROGRESS | Live Actions result required |
| 04 | Provider regression acceptance | [ ] PARTIAL | Native provider tests pending |
| 05 | Rust architecture modularity | [ ] PLANNED | Preserve IPC compatibility |
| 06 | Process supervisor | [ ] PLANNED | R1 native fault injection |
| 07 | Session checkpoint/recovery | [ ] PLANNED | R1 resume/no duplicate effects |
| 08 | Workspace path security | [ ] PARTIAL | TOCTOU + OS-level attack tests |
| 09 | MCP Registry | [x] Source complete | Configuration only, not runtime |
| 10 | MCP stdio / Streamable HTTP | [ ] PLANNED | R2 secure lifecycle |
| 11 | MCP tools and credential vault | [ ] PLANNED | Owner approval + OS stores |
| 12 | MCP inspector | [ ] PLANNED | Read-only discovery and diagnostic UI |
| 13 | Bounded multi-agent scheduler | [ ] PLANNED | R3 concurrency |
| 14 | Isolated worktrees and file leases | [ ] PLANNED | No cross-agent collision |
| 15 | Agent mailbox/checkpoints | [ ] PLANNED | State and generation fencing |
| 16 | Reviewer/merge gates | [ ] PLANNED | No unsafe automatic merge |
| 17 | Typed workflow DAG | [ ] PLANNED | R4 schema/cycle validation |
| 18 | Durable journal/executor | [ ] PLANNED | UNKNOWN/no replay |
| 19 | Workflow Studio UX | [ ] PLANNED | R5 native UX |
| 20 | Agent Team Dashboard | [ ] PLANNED | R5 accessibility |
| 21 | Mac M4 native acceptance | [ ] PLANNED | R6 real Mac GUI |
| 22 | Windows 11 x64 acceptance | [ ] PLANNED | R6 real Windows GUI |
| 23 | Linux x64 acceptance | [ ] PLANNED | R6 real Linux GUI |
| 24 | Security + production release gates | [ ] PLANNED | R7–R8 signoff |

## 6. Mandatory brand workstream BR-01–BR-07

- [ ] **BR-01 / P0:** Owner approves final product name, identity and trademark/domain review.
- [ ] **BR-02 / P1:** Master SVG/PNG/ICNS/ICO, app/tray/taskbar/installer assets complete.
- [ ] **BR-03 / P1:** UI design system, theming, accessibility and visual regression pass.
- [ ] **BR-04 / P0:** User-facing strings/menus/metadata fully reviewed; upstream attribution retained.
- [ ] **BR-05 / P0:** DB, Keychain, executable, tool ID and migration/rollback tests pass.
- [ ] **BR-06 / P0:** Fully branded native packages with coexistence/uninstall tests on three OS.
- [ ] **BR-07 / P0:** Legal, publisher identity, production signing and release approval.

See [NC-BRAND-001](REBRANDING-PLAN.md). Provisional source-level rename is not brand approval.

## 7. Testing and release evidence checklist

### Every PR (Definition of Done)

- [ ] Requirement, impacted OS, risk, acceptance criteria and owner recorded.
- [ ] Branch/worktree ownership and shared interface changes reviewed.
- [ ] Unit/contract tests for normal, invalid, cancellation and failure cases included.
- [ ] TypeScript build, Rust tests/Clippy and Git diff checks pass as applicable.
- [ ] Secrets scanned/redacted; threat model updated for native side effects.
- [ ] Reviewer checks compatibility, regressions, docs and rollback.
- [ ] GitHub CI status and exact commit SHA are attached before merge.
- [ ] Source-complete vs native-accepted status explicitly distinguished.

### Three-platform acceptance matrix

| Gate | macOS Apple Silicon M4 | Windows 11 x64 | Linux x64 |
| --- | --- | --- | --- |
| CI source tests | [ ] Verify remote result | [ ] Verify remote result | [ ] Verify remote result |
| Native install / first launch | [ ] | [ ] | [ ] |
| Providers, terminal, tools | [ ] | [ ] | [ ] |
| Credential store, MCP approvals | [ ] | [ ] | [ ] |
| Multi-session stop/resume/crash | [ ] | [ ] | [ ] |
| Rebranding/accessibility | [ ] | [ ] | [ ] |
| Upgrade / rollback / uninstall | [ ] | [ ] | [ ] |
| Signed production artifacts | [ ] | [ ] | [ ] |

### Final merge/release gates

- [ ] G0 — static analysis, formatting, TypeScript/Vite, Clippy: pass.
- [ ] G1 — unit/contract tests: pass (ignored cases disclosed).
- [ ] G2 — isolation/integration/failure tests: pass.
- [ ] G3 — native E2E: pass on each supported OS.
- [ ] G4 — reliability/security: pass, no P0.
- [ ] G5 — signed production installers, updater and rollback: pass.
- [ ] Required GitHub checks plus review approval recorded.
- [ ] Release Candidate freeze, changelog, checksums and provenance recorded.
- [ ] Project Owner explicitly approves merging into main and releasing.

## 8. Suggested execution order and dependency chain

| Order | Epic | Depends on | Main proof |
| --- | --- | --- | --- |
| 1 | Confirm GitHub CI / protect main | Remote pushed | Per-OS successful checks |
| 2 | R1 process/session reliability | Source baseline | Fault + restart regression |
| 3 | R1 safe filesystem operations | Path validation | Symlink/TOCTOU test |
| 4 | R2 consent and credential vault | Registry/policy | Deny-by-default, no leak |
| 5 | R2 transport/client and inspector | R1 + vault + consent | Native approved MCP lifecycle |
| 6 | R3 scheduler + worktrees | R1 process + R2 policy | Four-agent concurrency |
| 7 | R4 DAG + durable journal | R3 task lifecycle | UNKNOWN replay prevention |
| 8 | R5 GUI/rebrand | Owner BR-01 approval + R3/R4 | UX/accessibility acceptance |
| 9 | R6 three-OS packages | R1–R5 feature freeze | Native install reports |
| 10 | R7 reliability/security | R6 platform parity | Soak/crash/security reports |
| 11 | R8 stable release | R7 + BR-07 | Signed artifacts and owner GO |

**Parallelizable:** BR-01 approval, UI design specs, source-only fixtures, CI triage and isolated R1/R2 workstreams.
**Do not parallel-write:** shared IPC contract, schema migrations, signing manifests and native tool policy without Integration Lead coordination.

## 9. Weekly planning template (copy for every sprint)

| Field | Fill in |
| --- | --- |
| Sprint / dates | TBD — no imposed deadline |
| Objective | TBD |
| Integration lead | TBD |
| Source baseline SHA | TBD |
| P0 blockers | TBD |
| R1 worktree owner | TBD |
| R2 worktree owner | TBD |
| R3/R4 worktree owner | TBD |
| CI link / SHA | TBD |
| Security reviewer | TBD |
| Native acceptance owner | TBD |
| Decision: GO / NO-GO | NO-GO until required evidence |

### Sprint checklist

- [ ] Confirm branch SHA, remote and worktree are correct.
- [ ] Pick at most three highest-priority deliverables with verifiable DoD.
- [ ] Assign one writer per file scope and list API/schema dependencies.
- [ ] Implement source changes with focused tests; no hidden network or GUI side effects.
- [ ] Run relevant native-independent tests; capture actual counts and ignore list.
- [ ] Review security/rollback and commit small patches.
- [ ] Push reviewed branch, monitor GitHub CI and triage failures.
- [ ] Update this planner's status, evidence link, owner, blocker and next action.

## 10. Decisions and blockers log

| ID | Decision / blocker | Owner | State | Evidence / action |
| --- | --- | --- | --- | --- |
| D-001 | Final product name and logo not approved | Project Owner | OPEN | BR-01 decision required |
| D-002 | Real MCP execution blocked pending vault + operator consent | MCP/Security | OPEN | R2 security plans |
| D-003 | Production updater/signing identity unavailable | Release Reviewer | OPEN | R8 security gate |
| D-004 | Actual Windows/Linux native acceptance unavailable | QA/Platform | OPEN | R6 evidence |
| D-005 | GitHub CI cross-platform result not final at drafting | CI Lead | IN PROGRESS | [Actions](https://github.com/TheerasakPing/nexora-code/actions) |
| D-006 | main remains unchanged; develop is engineering integration | Integration Lead | INTENTIONAL | [Develop](https://github.com/TheerasakPing/nexora-code/tree/develop) |

## 11. References and change history

- [Engineering Roadmap R0–R8](ROADMAP.md)
- [NC-SOP-001](SOP-DEVELOPMENT-RELEASE.md)
- [Rebranding & Product Identity Plan](REBRANDING-PLAN.md)
- [R0–R2 source integration evidence](INTEGRATION-R0-R2-2026-10-22.md)
- [MCP Center UI](R2-MCP-CENTER-UI.md)
- [MCP Permission Preflight](R2-MCP-PERMISSION-PREFLIGHT.md)
- [MCP Egress Hardening](R2-MCP-EGRESS-HARDENING.md)
- [Live GitHub Actions](https://github.com/TheerasakPing/nexora-code/actions)

| Version | Date | Change |
| --- | --- | --- |
| 1.0 | 2026-10-22 | Initial evidence-based R0–R8 checklist, PR map, branding, CI and release planner |

> **Security boundary:** This planner authorizes no external tool execution, secret migration, installation, release signing, production deployment or unreviewed merge. Follow the SOP and ask for the required permissions.
