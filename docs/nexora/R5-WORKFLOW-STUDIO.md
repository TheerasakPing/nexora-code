# R5 Workflow Studio — Read-only DAG Validation UI

**Scope/date:** 2026-10-22, based on consolidated R0–R4 PR #18.
**Status:** Source-tested candidate, not yet native GUI accepted.

## Using Workflow Studio

From the main workspace toolbar choose Workflow Studio, edit the JSON sample
with schemaVersion, nodes and dependsOn edges, and choose Validate plan.
The dialog calls only the native R4 workflow_validate_plan validator. It
shows proposed dependency waves, declared Approval nodes and potentially
effectful nodes. Reset example restores the sample. Escape/Close dismisses
the dialog. Editing immediately invalidates any previous result.

Malformed JSON, missing fields, unsupported native schema and inputs above
128 KiB produce explicit errors. The UI does not fabricate success when the
native validation backend is unavailable.

All editable and native-supplied strings use DOM text nodes, not HTML
injection. The UI provides accessible labels, a live result region and
an asynchronous generation guard so a late old response cannot replace a
newly edited plan.

## Safety boundaries

**Validate does not Execute.** The dialog does not call MCP, Agent dispatch,
Git mutations, local file writes, workflow journal or native permission
approval. An Approval node in JSON is only a declared dependency; it cannot
provide actual operator consent.

R4 validator is pure/read-only; SQLite intent journal is internal-only.
Secure operator identity/consent, durable side effects, UNKNOWN reconciliation,
OS sandbox and native workflow executor all remain pending future requirements.

No real GUI install/run, live Worker/MCP action or production deployment took
place during source tests. Native macOS M4, Windows 11 x64 and Linux GUI E2E,
GitHub exact-head CI and independent human reviewer approval remain required.

## Local Mac acceptance evidence

- Focused input/async-state tests: 8 PASS including malformed/oversized input,
  native rejection, stale response rejection and hostile source text.
- Full Node unit suite: 151 PASS, 0 FAIL.
- Release-script suite: 31 PASS, 0 FAIL.
- TypeScript/Vite production build PASS (existing large-chunk advisory).
- R0 identity / fork attribution / updater safety check PASS.
