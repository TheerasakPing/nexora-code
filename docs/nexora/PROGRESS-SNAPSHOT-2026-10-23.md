# Nexora Code — Evidence-based Remaining-Work Snapshot (2026-10-23)

> **Counting policy NC-TEAM-002.** Source: [Master Checklist](CHECKLIST-PLANNER.md), [Engineering Roadmap](ROADMAP.md), dated task plans and live GitHub PR/Issue metadata. This snapshot records *acceptance gaps*, not a percentage of source lines or commits written.
>
> **Captured:** 2026-10-23, Asia/Bangkok. **Status:** Engineering source integrated on `develop`; NOT production ready.

## 1. Current integration reality

- [PR #21](https://github.com/TheerasakPing/nexora-code/pull/21) ("consolidate R0–R6") is **closed and merged into develop**, head SHA `055b036db0b768267fe3a75e3dd935a5769fac69`. A merge proves integration, not 3-OS native acceptance.
- GitHub connected search returned **0 open PRs and 0 open issues** in this repository at snapshot time. These numbers are *not* the count of remaining roadmap work.
- The [R7 journal fault-injection plan](tasks/2026-10-23-r7-journal-fault-injection.md) reports Mac source tests (Rust 1061 passed, 33 ignored; Node 156 passed; release tests 33 passed). It explicitly excludes real crash/power-loss and live 8-worker tests.
- The [Master Checklist](CHECKLIST-PLANNER.md) still has a 2026-10-22 phase snapshot. Its 24 planned "engineering PRs" are *logical acceptance packages*; GitHub PR #1–#25 are a separate operational numbering sequence. Do not conflate them.

## 2. Remaining counts (separate overlapping scopes)

| Measure / denominator | Source-complete or checked | Still unchecked / not accepted | Method |
| --- | --- | --- | --- |
| 24 planned engineering acceptance packages | 3 marked **source complete only** (01, 02, 09) | **21/24 unchecked** | Count the `[ ]` rows in section 5 of Master Checklist. Even the 3 source-complete packages require downstream native/release gates. |
| 7 cross-phase rebranding epics | 0 fully approved | **7/7 pending** | BR-01 through BR-07 in section 6. |
| R0–R8 detailed phase checkbox items | 11 narrow checks ticked | **61 still unchecked** | Count `- [ ]` only within detailed R0–R8 sections 3–4, exclude NOW/PR/brand/repeated QA/sprint templates. |
| All checkboxes in entire Master Checklist | 12 checked | **112 unchecked** | Includes duplicated NOW, DoD, release/sprint templates; NOT 112 independent development deliverables. |
| GitHub open PRs / open issues | — | **0 / 0 open** | Live connected search on 2026-10-23; reflects queue only. |

**Never add the 21 + 7 + 61 or the 112 as though mutually exclusive:** acceptance packages, branding and detailed phase checks overlap.

### Detailed remaining R0–R8 acceptance checks

| Phase | Unchecked | Principal gap / work still needed |
| --- | ---: | --- |
| R0 | 4 | Remote CI evidence, review protection, final brand decision, native baseline |
| R1 | 7 | TOCTOU, process groups, durable session recovery, long-session stability, Finder PATH, Mac native E2E |
| R2 | 10 | Vault and trusted consent, stdio/HTTP transport, tools/resources/prompts, egress security, live native MCP |
| R3 | 7 | Real multi-agent scheduler/IPC, isolated execution, durable claims/mailbox, review gates, concurrent QA |
| R4 | 6 | Executable DAG with verified effects, real operator Approval, recovery/reconcile, no-replay native acceptance |
| R5 | 7 | Full Workflow Studio executor integration, dashboard, original branding, accessible native UX |
| R6 | 6 | Real macOS M4, Windows 11 x64 and Linux x64 installer and GUI/provider E2E, upgrades |
| R7 | 7 | Security/SBOM, 8 agents × 2h, 50 real crash/cancel/resume cycles per OS, release regression |
| R8 | 7 | Signing/notarization, fork-owned updater, provenance, migration/rollback, owner/security approval, stable release |
| **Total** | **61** | **Pending acceptance checks**, not independent issues or PRs |

## 3. Blocking gates

1. **R1/R2 P0 runtime security** — secure filesystem handles/TOCTOU, privileged operator consent, credential vault and transport supervision are not accepted for live use.
2. **R3/R4 actual execution acceptance** — preview/DAG/journal tests do not prove safe live workers or side-effect approvals; avoid replays of UNKNOWN effects.
3. **R6 native acceptance** — actual installed-app behavior on all three OS remains unverified by this snapshot.
4. **R7 stress/security** — two-hour eight-worker soak and true fault-injection/cancellation cycles are outstanding.
5. **R8 legal/publisher gates** — branding owner decision, signing, notarization, updater, rollback and explicit release approval outstanding.

## 4. Next three executable work packages

1. **P0 — CI audit & status reconciliation:** Collect exact develop commit, actual six required checks and branch protection/reviewer evidence; fix failures and update the Master Checklist only on verified evidence.
2. **P0 — Runtime permission/security:** Implement and test trusted MCP operator consent plus vault references; keep real server execution disabled until security gates close.
3. **P1 — R1/R3/R4 durability & isolation:** Finish native process/session safety, verified worktree ownership, real approval/executor and failure recovery in isolated workstreams.

Native OS, brand and release gates remain assigned to Project Owner/Independent Reviewer/Platform QA where appropriate and are explicitly PENDING.

## 5. Team Lead GO/NO-GO

- **GO:** Documentation/governance source review and submission of a PR to `develop`.
- **NO-GO:** Treating the integrated source as production ready; bypassing protected merge checks or real independent review; executing external MCP, signing, or shipping a stable installer.
- **Limitation:** The counts here reflect the dated Master Checklist's acceptance markers, which may lag code integrations. Reconcile each row against its required DoD before changing to `[x]`.

**Related policy:** [AGENTS.md](../../AGENTS.md) (NC-TEAM-001/002), [SOP](SOP-DEVELOPMENT-RELEASE.md), [Task template](templates/TASK-CHECKLIST.md).
