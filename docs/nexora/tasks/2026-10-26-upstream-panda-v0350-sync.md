# Task checklist planner — 2026-10-26-upstream-panda-v0350-sync
**NC-TEAM-001 / 002 / 003** · **Request:** add a staged upstream update plan to Nexora Code (not to merge upstream wholesale).

## A. Intake — created by Team Lead
- **Source of requirements:** PandaCode `v0.3.50`, current Nexora roadmap `R0–R8`, user-requested A/B/C workstreams.
- **Date / issue:** 2026-10-26; `UPSTREAM-U1-U7`.
- **Target:** `aicodeking/pandacode` release source `15541a5`, original Nexora fork base `v0.3.16` / `30f9c823e3c38f472f89e8f8405575ecfe7173ee`.
- **Branch/worktree:** `docs/20261026-upstream-sync-u1-u7`, GitHub remote branch; no Mac workstation required for document-only changes.
- **Lead:** ChatGPT (the active assistant). **Real worker agents:** NONE connected / delegated; all substeps are `TEAM_LEAD_SERIAL`.
- **Boundaries:** No secret access, service startup, user storage modifications, signing or production release; local-first/no remote login.
- **Acceptance:** two linked plans, complete upstream release-note triage `0.3.17..0.3.50`, integration dependencies, ownership, exit gates; GitHub diff review. No false claims about code implementation, native E2E or independent reviewer.
- **Out of scope:** actual selective backports, imports, tests on real hardware, protected merges.

## B. Task decomposition and assignments
| ID | Deliverable / test | Real worker | Owned files | Depends on | Status |
| --- | --- | --- | --- | --- | --- |
| U1-01 | Read and classify 34 upstream release notes, with version links | TEAM_LEAD_SERIAL | `docs/nexora/UPSTREAM-RELEASE-INVENTORY-2026-10-26.md` | None | [x] notes triage created |
| U1-02 | Compare commit/file diffs and verify Nexora feature parity | TEAM_LEAD_SERIAL | future U1 diff appendix / separate worktree | U1-01 | [ ] |
| U2 | Runtime, sessions, SQLite perf backports | TEAM_LEAD_SERIAL | Rust runtime/storage future PR | U1-02 | [ ] |
| U3 | New CLI providers and discovery hardening | TEAM_LEAD_SERIAL | provider and model-picker future PR | U1-02 | [ ] |
| U4 | Activity trace schema, tool cards and streaming UX | TEAM_LEAD_SERIAL | activity renderer future PR | U1-02 | [ ] |
| U5 | Sidebar/inspector/attachments/accessibility/Thai | TEAM_LEAD_SERIAL | UI future PR | U1-02; U4 | [ ] |
| U6 | Integration, security, data/migration and cross-platform validation | TEAM_LEAD_SERIAL | test harness and staging worktree | U2–U5 | [ ] |
| U7 | Local beta package, rollback, optional production release | TEAM_LEAD_SERIAL | release automation future PR | U6 | [ ] |
| U-QA | Lead diff/security and external reviewer gate | TEAM_LEAD_SERIAL | read-only | U1–U7 as implemented | [ ] |

**Proposed only (not assigned):** Native/Rust Agent → U2, Provider Agent → U3, Activity/UI Agent → U4–5, QA/Security Agent → U6, Release Agent → U7. A future Lead must bind each to a real agent ID and non-overlapping worktree before claiming delegation.

## C. Evidence & acceptance checklist
- [x] This task planner created alongside upstream planning-only changes.
- [x] Scope decomposed by dependency, bounded ownership and stop conditions.
- [x] Release-note source URLs used for all 34 versions; remote/full cloud features excluded by default.
- [ ] Full version-to-version *source* diff with changed paths and compatible contracts.
- [ ] Actual implementation agents with independent patch handoff (none currently assigned).
- [ ] Unit/contract evidence for implementation PRs and all required browser E2E.
- [ ] Separate real installed macOS / Windows native E2E and smoke.
- [ ] Security/data migration/permissions review and no critical regressions.
- [ ] Independent human security, legal and publisher approval for production.
- [ ] Lead approves protected merge / release only with same-SHA evidence.

## G. NC-TEAM-003 — subtask verification matrix
| ID | Worker | Unit/Contract | E2E | Smoke | Lead Review | Evidence / reason |
| --- | --- | --- | --- | --- | --- | --- |
| U1-01 | TEAM_LEAD_SERIAL | N/A: release-note docs only | N/A: docs only, approved by Lead 2026-10-26 | PASS: GitHub stored docs; re-read required before PR merge | PENDING: PR diff | all 34 release notes read, classification provisional |
| U1-02 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | full SHA/diff/feature parity audit |
| U2 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Rust unit, CLI native E2E, long session/restart smoke |
| U3 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | CLI mocks, actual discovery, provider permissions, Mac/Windows |
| U4 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | formatter unit, trace stream browser E2E, native smoke |
| U5 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | accessibility, Thai, dark mode and selection E2E |
| U6 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | 3-OS CI, migration fixtures, native E2E and security |
| U7 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | artifacts/installer hash, same-SHA checks and rollback |

No implementation task is accepted while its verification triad is PENDING. Unit/E2E and smoke are separate; GitHub Build PASS alone cannot certify installed native app.

## D. Worker handoff ledger
| ID | Worker | Commit / patch | Test evidence | Risks / incomplete | Lead review |
| --- | --- | --- | --- | --- | --- |
| U1-01 | TEAM_LEAD_SERIAL | document commits on planning branch | 34 upstream tagged release notes examined | source diff and PR review pending | PENDING |
| U2–U7 | none yet | N/A | N/A | implementation pending | PENDING |

## E. Lead acceptance / handoff
- **Completed deliverable:** planning and preliminary release-note inventory, not code porting.
- **External reviewer:** NOT ASSIGNED; no independent review claimed.
- **CI / tests:** implementation E2E, smoke and native checks **NOT RUN** for this planning task.
- **Security:** no runtime code or user data modified; future permission/updater/migration gates explicitly tracked.
- **Release decision:** **NO-GO** production until U6/U7 and R8 gates.
- **Protected merge:** **PENDING** PR review and documentation gate.

## F. Mandatory NC-TEAM-002 remaining-work report
- **Source snapshot:** PandaCode 0.3.50, local Nexora `develop` inspected 2026-10-26.
- **Engineering 24 PR packages / brand 7 epics / R0–R8 acceptance:** not recomputed in this planning-only task; refer to authoritative current project checklist; do not translate 7 U-batches into overall percent.
- **U-batches accepted:** 0/7. **U1 release-note triage:** drafted; *source audit incomplete*.
- **Top priorities:** U1-02 detailed source diff → U2 and U3 independent gates → U4/U5; maintain local-first and data-safety constraints.
- **P0 blockers:** unreviewed upstream API/schema changes, unavailable installed native evidence, independent release approval.
- **Review decision:** planning GO, implementation/release NO-GO until evidence.
