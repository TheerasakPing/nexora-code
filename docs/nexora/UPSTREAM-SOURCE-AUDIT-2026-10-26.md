# PandaCode v0.3.16 → v0.3.50: full changed-path source audit
Date: 2026-10-26 · Lead: ChatGPT · Repo: Nexora Code · Baseline: `origin/develop` at `f0e18e8`.

## Reproducibility
- Upstream old SHA `30f9c823e3c38f472f89e8f8405575ecfe7173ee`, new tag `v0.3.50` = `15541a51f3f5d8ee32d91349f9d5035a53c4cd19`.
- Commands: `git diff --no-renames --name-status -z OLD v0.3.50`; `git ls-tree -r -z v0.3.50`; `git ls-tree -r -z HEAD`.
- [Complete per-path 758-row matrix](UPSTREAM-V0350-SOURCE-PATH-MATRIX.tsv) contains upstream add/change, scope, blob-overlap status, and both SHA-1 Git blob IDs.

## Counts (full local Git, not GitHub Compare 300-file truncation)
| Group | File paths | Definition |
| --- | ---: | --- |
| Total changed or added | **758** | 539 added, 219 modified; no deletions under `--no-renames` |
| Core code and tests | **312** | `src/`, `src-tauri/`, `tests/`, `scripts/` |
| Mobile app | **209** | `mobile/`: intentionally excluded for desktop local-first cycle |
| Remote/Cloud paths | **59** | remote/cloud prefixes and related docs; excluded by default |
| Documentation and CI | **118** | `docs/`, `releases/`, `.github/` |
| Other paths | **60** | package metadata, website, assets, other components; manual triage |

## Fork overlap (Git object equality only)
- `UPSTREAM_ONLY` **540** paths: not present at the identical path in Nexora `develop`.
- `DIVERGED_CONTENT` **218** paths: both have the path, contents differ.
- `EXACT_BLOB_MATCH` **0** changed paths. This does **not** prove features absent or that they must be copied: Nexora's fork often owns different implementations and branding.

## Source-level integration decisions (provisional, never an automatic merge)
| Area | Upstream paths (examples) | Nexora paths | Decision |
| --- | --- | --- | --- |
| CLI discovery (U3) | `src-tauri/src/providers/{cline,commandcode,devin,muse}.rs` upstream-only | `src-tauri/src/providers.rs`, `src/model-picker.ts` diverged | PORT selectively after auth and model-cache contract review |
| Trace rendering (U4) | `src/tool-activity.ts`, `src/turn-activity.ts` diverged | same paths exist | ADAPT; avoid overwriting Nexora trace schema and escaping |
| Session/DB (U2) | `src-tauri/src/storage.rs` diverged | local schema and migration | SECURITY REVIEW first; do not merge migration SQL directly |
| Task navigation (U5) | `src/sidebar-sections.ts` upstream-only; `src/sidebar-tree.css` diverged | local task tree/state | ADAPT with persistence and existing project filter |
| Attachments (U5) | `src/message-attachments.ts` upstream-only | `src/attachments.ts` diverged | ADAPT only after persistence/data scope check |
| Update packaging (U7) | `src/app-updates.ts`, `.github/workflows/release.yml` diverged | fork-specific updater isolation | EXCLUDE upstream signing endpoints; only fork-owned rules |
| Remote/Cloud/Mobile | 209 mobile + 59 classified remote/cloud paths | local-first Nexora | EXCLUDE by default; inspect shared paths for inadvertent coupling |
| Handoff display (U5) | upstream `v0.3.48:src/styles.css`, selector `.provider-handoff` | pre-port marker left-aligned in Nexora | **PORTED** narrow CSS change and regression added in this branch |

## Known limits / next U1 substeps
- This manifest is exhaustive **by tracked path**, not exhaustive semantic/API/ABI review; 312 core files need changed-hunk classification per subsystem. Original upstream tag comparison returned only the first 300 paths, therefore was insufficient.
- No credentials, database migrations, signatures or remote-network operations were copied.
- Prioritize U3 provider catalog and U2 session storage first; collect native Mac and Windows evidence after merges to `develop`.
- Track diff origin `v0.3.48:src/styles.css`; behavioral validation comes from `tests/ui.spec.ts` harness-handoff case, not merely CSS equality.
