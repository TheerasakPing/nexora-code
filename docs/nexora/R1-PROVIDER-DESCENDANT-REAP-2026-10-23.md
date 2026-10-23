# R1 Provider Process Group — normal completion and cancellation

**Date:** 2026-10-23. **Team Lead:** TEAM_LEAD_SERIAL, no independent worker delegated.

## Risk and mitigation

Native provider runs use process groups. A provider may exit successfully
while a child stays alive in that group. R1 must ensure the descendant stops
when the run is over, not only on timeout. The existing ProcessGroup guard
already performs cleanup on Drop. This package adds real process regressions
to exercise normal completion and async task cancellation.

Defense in depth: on Unix kill_group now rejects group IDs 0, 1 and values
outside signed 32-bit positive range before negative signal delivery.
Mistaken kill(-1, SIGKILL) could affect unrelated permitted processes.

## Test scope

- Normal-exit E2E (native subprocess): a shell in its own group starts a
  background sleep child, redirects output, exits normally. The test verifies
  that the child is running, then drops the group guard and checks it stops.
  Repeated three times using temporary, test-owned process IDs only.
- Cancellation E2E: abort an async task with a process-group guard and check
  that its subprocess and process registry are cleaned.
- Unit/Contract: invalid special group boundaries and static source tests.
- Smoke: targeted tests, full Rust library/Node suites and CI per SHA.
- Review: inspect actual diff and test logs. No user process or configuration
  changes. Existing Windows Job Object handling remains unchanged.

This does NOT prove prevention of descendants intentionally detaching into
a new process session, native Finder GUI/session/provider restart, Windows
native lifecycle, long-duration soak or independent human review.

**GO:** source-level bounded process cleanup validation after tests.
**NO-GO:** R1 phase exit, R2 advancement and Production without native
installed-app acceptance and independent reviewer.
