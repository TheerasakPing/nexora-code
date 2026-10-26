# Nexora Code — Test artifact cleanup, 2026-10-26

## Intake / Team Lead NC-TEAM-001
- Request: merge PR #62/#63 and **always remove unneeded temporary test files** after finishing work.
- Owner: TEAM_LEAD_SERIAL (no real delegated worker agents claimed). Branch `docs/20261026-test-cleanup-policy`, based on `origin/develop`.
- Scope: generated, ignored `test-results/`, `playwright-report/`, `coverage/` only. Never delete tracked tests, source, migration fixtures, real user data, credentials, installer provenance, CI failure traces not yet captured, or other workers' active artifacts.
- Review and evidence must be captured **before** cleanup. Dry-run first, clean only a clean/idle owned worktree.
- Merge PRs through standard GitHub branch gates; auto-merge was configured, pending checks remain and must not be misrepresented.

## Checklist / delegation
- [x] Lead verified PRs #62/#63 are conflict-free, CI status inspected.
- [x] Lead marked PRs ready and enabled GitHub **auto-merge**, so checks/reviews must succeed before actual merge.
- [x] Build safe cross-platform cleanup helper; tracked files and symlinks are protected.
- [x] Add mandatory cleanup SOP NC-CLEAN-001 and evidence-retention exceptions.
- [x] Unit test CLI helper (3/3), dry-run and apply with an isolated disposable smoke marker.
- [x] Removed only generated test/build outputs from 4 task-owned worktrees; verified all older worktree Git statuses clean. Kept PR branches/worktrees because CI/review is pending.
- [ ] Independent PR review/merge approval and native OS acceptance (separate gates).

## NC-TEAM-003 verification matrix
| Subtask | Unit/contract | E2E | Smoke | Lead review |
| --- | --- | --- | --- | --- |
| Cleanup helper | PASS: 3/3 focused; 214/214 Node suite after Vite build | PASS: disposable Git-fixture dry-run/apply, tracked/symlink and unsafe-root cases | PASS: real worktree disposable `test-results/cleanup-smoke.txt` removed | PASS: Lead reviewed allowlist, symlink and tracked-file safeguards |
| SOP/documentation | N/A: documentation | N/A: documentation | PASS: NC-CLEAN-001 saved in SOP | PASS: Lead inspected boundaries and retention rules |
| PR #62/#63 merge | targeted local tests PASS | relevant browser/storage tests PASS | CI pending | independent review PENDING |

## GO/NO-GO
- Cleanup of ignored generated files: GO after evidence archived, safety checks.
- Unreviewed merge/production release: NO-GO; GitHub auto-merge remains subject to branch protection.
