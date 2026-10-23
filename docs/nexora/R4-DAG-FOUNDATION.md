# R4 — Versioned Workflow DAG Validator and Read-only Preview

**Created:** 2026-10-22 · **Status:** source foundation, not an executor
**Upstream:** Nexora Code forks PandaCode — https://github.com/aicodeking/pandacode.

## Functional scope

A Rust module with typed workflow schema version 1 and read-only Tauri IPC
command workflow_validate_plan accepts a proposed plan, validates it, and
returns topologically sorted proposed execution waves and approval/effect
metadata. It neither stores nor runs tasks.

Permitted node kinds: agent, mcp_tool, condition, approval, test, review, git,
notification, artifact.

- Reject unsupported future schema versions and unknown node fields/types.
- Require 1–128 unique, safe node IDs (case variants rejected).
- Limit each node to 16 known, unique, non-self dependencies and 160-byte titles.
- Reject cycles, malformed references and plans above 128 KiB.
- Require an upstream explicit approval node on **every root-to-node
  dependency path** leading to an effectful node (agent, mcp_tool, test, git,
  notification and artifact). An unrelated sibling approval does not qualify,
  nor does a join of an approved branch and an unapproved branch. Two
  independently approved branches may join safely, and a gate after a join
  may gate all incoming paths. This structural rule is fail-closed.
- Approval declarations describe plan structure only; they do not prove
  human consent, establish tool authority, or grant execution privileges.
- Return ordered wave IDs that obey dependencies and preserve stable ordering.

### Example JSON candidate

    {
      "schemaVersion": 1,
      "nodes": [
        {"id": "check", "kind": "condition", "title": "Inspect state"},
        {"id": "approve", "kind": "approval", "title": "Human review", "dependsOn": ["check"]},
        {"id": "apply", "kind": "git", "title": "Propose patch", "dependsOn": ["approve"]},
        {"id": "review", "kind": "review", "title": "Inspect patch", "dependsOn": ["apply"]}
      ]
    }

This is only a candidate plan. No actual manual approval, Git mutation,
tool call or permission grant happens when validating it.

## Security and durable recovery limitations

**This is NOT a workflow runner.** A JSON approval node does not prove
any person actually approved an operation. Native execution remains disabled.

Before R4 can execute:
1. Persist plans and immutable intent records transactionally.
2. Enforce real human/operator consent at the native permission boundary.
3. Durable state machine with effect fences: PENDING, READY, RUNNING,
   SUCCEEDED, FAILED, CANCELLED and UNKNOWN, recording intent before effects.
4. Never auto-replay uncertain mutating outcomes after a crash; UNKNOWN
   requires explicit operator reconciliation.
5. Bounded retries only for proven read-only/idempotent actions; cancellation,
   process cleanup and permission diff.
6. Fault injection, native Mac/Windows/Linux E2E and rollback evidence.

## Verification

Source tests cover approval ancestry, unapproved mutating node rejection,
cycles, unknown/duplicate/self dependencies, schema and ID validation,
unknown fields, stable DAG waves and no state mutations. Full Rust/Clippy,
frontend quality, CI and native GUI acceptance remain separate release gates.

No GUI, live MCP, real Agent Worker or external workflow actions were run.

## Verified local source results — 2026-10-22

- Focused DAG/approval/schema tests: 8 PASS.
- Full Rust unit suite: 1014 PASS / 0 FAIL / 33 ignored; CLI integration 1 PASS.
- Rust all-target Clippy denied warnings: PASS.
- Node frontend tests: 110 PASS; release tests: 31 PASS.
- Vite build and R0 security/identity verification PASS.
- macOS/Windows/Linux GitHub CI for this exact SHA and native GUI E2E: pending.

## R4 approval-join regression hardening — 2026-10-22

The validator previously used an ANY-input approval-reachability check. A
non-effectful join could become approval-marked merely because one incoming
branch passed a declared Approval node; a later effectful descendant would
then pass even if another branch remained unapproved.

The corrected check propagates approval only when ALL predecessor paths are
approval-marked, except that the Approval node itself establishes the
structural gate for all incoming paths. Empty predecessor sets are not approved.

Tests verify rejection of direct and transitive mixed-approval bypasses
for all six potentially effectful node types, while allowing two independent
approved branches or one approval following a joined set of prerequisites.
The R4 native IPC remains read-only; genuine operator consent enforcement
must still be implemented separately before any workflow execution.
