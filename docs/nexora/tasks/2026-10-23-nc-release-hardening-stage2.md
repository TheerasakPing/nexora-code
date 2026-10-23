# NC-R8-002 — Release identity, safe distribution and native baseline

Date: 2026-10-23. User directive: “ที่นี้ทำให้มันสมบูร” (finish Nexora Code).
Team Lead: ChatGPT / TEAM_LEAD_SERIAL. Studio worker pool not ready; no false parallel workers or independent review.
Baseline: PR #26, source HEAD `6727f08`, 9/9 exact-head CI PASS (Mac ARM64, Windows x64, Linux x64).
Objective: close important *verifiable* release/identity work and document remaining production blockers, not conflate compile with complete product.

## Plan / checklist (create before source edits)

| Work | Owner | Acceptance | Status |
| --- | --- | --- | --- |
| ID-01 | TEAM_LEAD_SERIAL | Rename **shipped executable** only to nexora-code, preserve Cargo package/library names, session/database/tool keys and upstream attribution. Update release scripts/tests. | [ ] |
| SEC-01 | TEAM_LEAD_SERIAL | Audit/rebrand public website *without links to upstream PandaCode binaries or nonexistent signed Nexora releases*. Honest unsigned alpha labeling. | [ ] |
| NATIVE-01 | TEAM_LEAD_SERIAL | Verify Mac executable identity and no-user-data regression; test app package and startup on M4. | [ ] |
| QA-01 | TEAM_LEAD_SERIAL | Node/release/website and hosted six required native CI checks, record actual evidence; diagnose negatives. | [ ] |
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
