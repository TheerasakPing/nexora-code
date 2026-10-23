# R7 — SQLite Journal Recovery and Fault-Injection Tests

**Date:** 2026-10-23 · **Status:** Source regression suite, not OS-level R7 acceptance
**Repository:** Nexora Code fork of PandaCode

## Tests introduced

The staged internal R4 durable intent journal gains two Rust regression
scenarios, using only temporary SQLite files and no real Agents, MCP tools,
workspace Git effects or network:

1. **50 on-disk database reconnect cycles:** For each independent workflow,
   register a validated plan, mark a prerequisite successful and persist a
   pending mutating-effect intent; close SQLite and reopen it. Recovery must
   atomically mark the effect UNKNOWN exactly once, block dependent nodes,
   reject late completion and replay, preserve prior successful outcomes and
   maintain audit integrity. Final counts: 50 UNKNOWN, 200 audit events.
2. **Injected transactional audit failure:** A temporary SQLite trigger
   rejects an audit INSERT after a state UPDATE. Confirm record_intent and
   record_outcome roll back their state changes; failed recovery leaves
   RUNNING unchanged, not a false UNKNOWN. Once the trigger is removed,
   recovery marks UNKNOWN and later completion is rejected.

## Important limitations and no-go gates

SQLite reconnect is *not* an OS process-kill or power-failure test; an aborting
SQL trigger is not a simulated physical fsync outage. These source-level tests
do not exercise real worker effects, operator permission, process supervision
or true OS sandboxing. R7 is NOT complete.

Remaining R7/R8 release gates include:
- 50 genuine crash/cancel/resume cycles per macOS ARM64, Windows 11 x64,
  Linux x64 with bounded process termination and no external effect replay.
- Eight real Agent workers for two hours, collecting CPU, memory, process
  count, hangs and errors, and checking cancellation correctness.
- Full security/supply-chain assessment and actionable SBOM/exception record.
- Reviewed R4 native consent and real executor with reconciliation of UNKNOWN.
- Signed/notarized installers, trusted updater and rollback E2E.

## Evidence policy

The source test results and exact commit SHA must be recorded in the dated
Task Checklist Planner for this task. Tests passing on one Mac do not prove
three-platform native GUI E2E or signed R8 release readiness. Protected
develop requires all six current CI checks plus one independent Reviewer.

## Completed local source checks — Mac M4

Focused journal: 6 PASS; full Rust 1061 PASS / 33 ignored; CLI
integration 1 PASS; Clippy all-targets with denied warnings PASS;
Node 156 PASS; release tests 33 PASS; Vite build and R0 safety PASS.

Native exact-SHA GitHub CI and real crash/soak tests remain independent gates.
