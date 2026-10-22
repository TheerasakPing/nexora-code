# R4 — Crash-consistent Intent Journal (Internal Foundation)

**Date:** 2026-10-22 · **Status:** Staged internal-only Rust module; not wired to live executors
**Product lineage:** Nexora Code forks PandaCode (https://github.com/aicodeking/pandacode).

## Durable design

The module workflow_journal.rs uses SQLite transactional persistence. Each
journal instance attaches to a SQLite connection supplied by its owner.
This is intentionally **not exposed** through Tauri IPC, providers or Agent
tool execution. Production schema migration and permission integration are
blocked pending separately reviewed R4 gates.

- A validated, immutable WorkflowPlan is recorded once by its unique run ID
  and SHA-256 plan hash, with every node initially PENDING.
- Recording intent transitions a dependency-ready node to RUNNING and writes
  an immutable intent audit event in the SAME SQLite transaction, before a
  future trusted dispatcher would perform an external effect.
- Once RUNNING, re-claiming the same node fails.
- Completion requires exact run/node, generation and unique intent key and
  RUNNING status; duplicate or late results fail.
- On restart, unresolved RUNNING intents are atomically changed to UNKNOWN
  with audit events. UNKNOWN never counts as SUCCEEDED, so dependent nodes
  do not become ready and attempts to retry an UNKNOWN node fail.
- A FAILED node also prevents dependent execution. No implicit reconciliation
  or automatic retry is available.
- Repeated run IDs, duplicated effect keys and unsafe ID/key formats are
  rejected; failed SQLite transactions roll back cleanly.

## Important security boundary

**This module alone does not authorize any operation.** A syntactic approval
node, or a caller marking it SUCCEEDED, is not proof of user authorization.
The journal's state transitions are future executor primitives and MUST NOT
be wired to real mutating tools without native secure operator consent,
signed/verified permission grants, scoped processes, idempotency handling and
OS-enforced filesystem boundaries.

No recovery path currently resolves UNKNOWN automatically. Before production,
a real human/operator must reconcile uncertain outcomes with external
evidence, and a reviewed durable state machine must bind that decision to
the exact immutable plan, node, process generation and event identity.

## Remaining release work

- Integrate migrations with the owned app database, storage permissions and
  backup/restore; stress test SQLite failure modes and abrupt OS termination.
- Add fail-closed permission/approval verification before reserving any
  mutating intent, and bind intent to actual provider tool ID/capabilities.
- Ensure no external side effect starts before durable commit acknowledgment;
  add power loss / fsync checks and real idempotency keys.
- Reconcile UNKNOWN through a separately authorized human review workflow.
- Native Mac M4, Windows 11 x64 and Linux integration with secure workers,
  crash/restart injection, 50-cycle test gate and zero unintentional replays.
- Full Workflow Studio GUI and accessibility tests.

## Source verification

Four dedicated SQLite tests cover crash-to-UNKNOWN, dependency fencing,
stale generations, duplicate outcomes, invalid keys, transaction rollback
and on-disk reopen recovery. Full Rust, Clippy, build and GitHub CI are
tracked in the accompanying Task Checklist. No GUI, MCP, agents, external
side effects or user data migrations were performed.

## Verified local source evidence

- Focused SQLite durability tests 4 PASS.
- Full Rust tests 1018 PASS / 0 FAIL / 33 ignored; CLI integration 1 PASS.
- All-target Clippy -D warnings PASS.
- Node frontend tests 110 PASS; release tests 31 PASS.
- Production Vite build and R0 safety check PASS.
- Windows/Linux CI for this exact branch and native GUI/forced-OS-kill tests PENDING.
