# Task Checklist Planner — 2026-10-23-nc-release-hardening-stage2

# NC-R8-002 — Release identity, safe distribution and native baseline

Date: 2026-10-23. User directive: “ที่นี้ทำให้มันสมบูร” (finish Nexora Code).
Team Lead: ChatGPT / TEAM_LEAD_SERIAL. Studio worker pool not ready; no false parallel workers or independent review.
Baseline: PR #26, source HEAD `6727f08`, 9/9 exact-head CI PASS (Mac ARM64, Windows x64, Linux x64).
Objective: close important *verifiable* release/identity work and document remaining production blockers, not conflate compile with complete product.

## Plan / checklist (create before source edits)

| Work | Owner | Acceptance | Status |
| --- | --- | --- | --- |
| ID-01 | TEAM_LEAD_SERIAL | Source/Cargo metadata and release scripts updated to ship nexora-code executable while keeping cargo pkg/lib, Session/DB/tool keys | [x] |
| SEC-01 | TEAM_LEAD_SERIAL | Website links only to fork-owned Actions preview and repository; fork/license provenance and disabled updater disclosed; Astro build PASS | [x] |
| NATIVE-01 | TEAM_LEAD_SERIAL | Verify Mac executable identity and no-user-data regression; test app package and startup on M4. | [ ] |
| QA-01 | TEAM_LEAD_SERIAL | Node 168/168, release 33/33, R0 PASS; website Astro build PASS; local Mac Cargo check and CI run pending | [ ] |
| AUDIT-01 | TEAM_LEAD_SERIAL | Update R0-R8 progress ledger with exact acceptance counts and blockers; push PR. | [ ] |
| MERGE-01 | INDEPENDENT_REVIEWER_PENDING | Qualified reviewer before protected `develop` merge under NC-TEAM-001, even though GitHub settings currently require 0 approving reviews. | [ ] |
| PROD-01 | PROJECT_OWNER_PENDING | Developer ID + notarization / clean Gatekeeper install, real all-OS GUI+provider/8-worker stress and final brand trademark/legal release acceptance. | [ ] |

## Security decisions

- No global Gatekeeper disablement, remote install or modification of users' `~/.pi`, Keychain, existing `/Applications`, database or session files.
- No automatic updater pointing at upstream PandaCode, and no misleading website binary links.
- Preserve `pandacode_lib`, `pandacode_*` tool names, legacy persisted keys and import/rollback capability. Executable identity may change only when packaging tests cover it.
- Do not self-approve PR; branch's required reviewer gate must be addressed explicitly and not misrepresented.

## Baseline completion and counts

As of opening this plan: engineering **21/24 unchecked**, branding **7/7 not fully accepted**, R0-R8 **61** phase detail boxes unchecked. These scopes overlap. R6 hosted unsigned builds passed; production native/install Gatekeeper remains NO-GO.

## Evidence and handoff (2026-10-23)

- Exact source baseline PR #26 before this patch: 9/9 source/installer GitHub checks PASS; no recorded independent reviews.
- GitHub `develop` branch protection enforces six CI contexts and administrator protection, **but required_approving_review_count was 0**. Governance NC-TEAM-001 still requires independent review before merge; do not silently self-approve.
- `cargo metadata --no-deps`: single executable target `nexora-code`, stable `pandacode_lib` rlib and `pandacode` Cargo package.
- `npm test`: **168/168 PASS** (4 new branding/executable/website origin guard tests).
- `npm run test:release`: **33/33 PASS**; `npm run test:nexora:r0`: PASS.
- `npm --prefix website run build`: PASS (4 Astro pages generated). Website no longer links to upstream PandaCode installers or claims notarized stable binary availability; original license provenance preserved.
- Browser full website tests under concurrent long-running Mac Rust check: mixed results/timeouts; cannot count as complete E2E. Review test artifacts and rerun without heavy compilation before acceptance.
- Real native Mac `cargo check --bin nexora-code`: in progress at source handoff; R6 hosted app packaging and cross-platform checks must be rerun for exact new commit before marking acceptance.
- **Next three:** (1) exact-head CI on Mac/Win/Linux and binary name inspections, (2) reliable serial browser/native GUI/PTY/provider restart tests, (3) qualified independent reviewer plus signing/notarization and documented Production owner approval.
- **Team Lead:** GO source submission/CI; NO-GO production, protected merge without independent reviewer, 100% project completed claim.


## G. NC-TEAM-003 — Subtask Verification Matrix (retrospective audit, not invented tests)

This planner existed **before NC-TEAM-003**. Historic checked [x] boxes
record earlier **narrow source-only** milestones and do NOT assert NC-TEAM-003
E2E/Smoke acceptance. Every historic subtask below is explicitly PENDING
until the Team Lead rechecks actual test command/OS/SHA/log and signs each.
Missing or unverified native evidence remains an acceptance NO-GO. The
retrospective entries are not fabricated executions or agent delegations.

| Subtask ID | Worker | Unit/Contract | E2E | Smoke | Team Lead Review | Evidence / N/A reason |
| --- | --- | --- | --- | --- | --- | --- |
| ID-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| SEC-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| NATIVE-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| QA-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| AUDIT-01 | TEAM_LEAD_SERIAL | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| MERGE-01 | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |
| PROD-01 | INDEPENDENT_REVIEWER_PENDING | PENDING | PENDING | PENDING | PENDING | Historic source evidence only; E2E/Smoke per-work-item review needs exact logs/OS/SHA, reviewer pending |


**Team Lead GO/NO-GO (retrospective):** SOURCE PR GO for further review only.
Individual NC-TEAM-003 gates remain **PENDING** (not accepted); the prior
task's source-level status is not revoked or silently upgraded. Protected
merge and production NO-GO until genuine independent review / native gates.


### NC-TEAM-003 acceptance tracker (not inferred)

- [ ] NC-TEAM-003: Unit/Contract, E2E and Smoke verified per subtask.
- [ ] NC-TEAM-003: All applicable tests and native acceptance requirements passed.
- [ ] NC-TEAM-003: Real independent reviewer approved protected merge and release.

**Team Lead verdict:** SOURCE REVIEW GO for PR submission only; NC-TEAM-003 subtask test triads remain PENDING. Independent reviewer has NOT approved protected merge.
