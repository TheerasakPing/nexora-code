# Nexora Code U1 source diff audit and first selective backport — 2026-10-26

## A. Team Lead intake
- User request: continue implementing PandaCode U1–U7 plan, start with source diff and safe backports.
- Base: `origin/develop` f0e18e8; fork pinned upstream base `30f9c823`; upstream `v0.3.50` `15541a5`.
- Workspace: `/Volumes/Data/workspace/git/nexora-code-upstream-v0350-audit` (isolated worktree).
- Lead: ChatGPT; real delegated workers: NONE (`TEAM_LEAD_SERIAL`).
- Security: preserve local-first, no cloud/remote import; no migration of real data, secrets or signed releases.
- Acceptance: full changed-path manifest, explicit source overlap, one safe local change if verified; actual tests.

## B. Work ownership
| ID | Owner | Owned work | State |
| --- | --- | --- | --- |
| U1-A | TEAM_LEAD_SERIAL | file-level diff manifest and provisional triage | [x] path audit only |
| U1-B | TEAM_LEAD_SERIAL | selective v0.3.48 centered handoff UX + regression | [x] browser scoped |
| U1-QA | TEAM_LEAD_SERIAL | review diff and test evidence; honest GO/NO-GO | [ ] independent review pending |

## G. NC-TEAM-003 verification matrix
| Task | Unit/Contract | E2E | Smoke | Lead review | Evidence |
| --- | --- | --- | --- | --- | --- |
| U1-A | PASS: count 758 paths, 539 added, 219 modified | N/A: path inventory docs-only, Lead 2026-10-26 | PASS: local Git produced 758 TSV rows, manual sum checked | PASS: Lead checked source tags and group counts | `docs/nexora/UPSTREAM-V0350-SOURCE-PATH-MATRIX.tsv`; full semantic review pending |
| U1-B | PASS: TypeScript/Vite + 211/211 Node tests | PASS: Chrome Playwright harness handoff 3/3 | PASS: browser preview harness switch, prompt preserved | PASS: Lead inspected CSS and test diff | upstream `v0.3.48:src/styles.css`; native installed app not tested |
| U1-QA | PASS: 43/44 release tests + 1 skip | PENDING: independent native | PENDING: signed installs | PENDING: independent human review | WebKit config excludes this case; separate native OS checks pending |

## E. Lead release gates
- Do not claim completed semantic audit only from filename comparison.
- Browser preview E2E is not installed native app E2E.
- Merge and production release remain NO-GO without evidence and independent review.
