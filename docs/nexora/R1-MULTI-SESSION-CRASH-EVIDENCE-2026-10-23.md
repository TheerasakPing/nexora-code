# R1 Multiple Session crash/restart persistence evidence

**Date:** 2026-10-23. **Scope:** native SQLite Store process-level integration test, not installed GUI/real eight running AI agent providers.

The test launches the *actual Rust test executable as a separate child process*, writes eight active sessions to the same temporary SQLite database with one pending tool each (running/waiting alternating), writes a ready marker only after the writes, and the parent explicitly checks child still alive before terminating it using Child::kill. The parent waits for the child to exit, then re-opens the same DB twice per cycle. Five cycles create five independent temporary databases and terminate five disposable child processes.

Per session, each reopen asserts interrupted state, exactly two original messages (one prompt and one tool), the tool marked failed and its partial output retained, and exactly one interruption notice. This protects a previously implemented startup recovery path against multi-session regression and no-replay failures.

**Results on Mac M4:**
- Narrow native process integration test: 1/1 PASS; 5 process crashes, 8 sessions each, 2 reopen checks each => 80 session/restart assertions groups. No actual provider processes were run; agents were persisted session records.
- Full Rust Library tests: 1115 PASS, 0 FAIL, 33 ignored.
- Frontend build PASS; Node 208/208 PASS; Release Contract 33/33 PASS.
- No secrets, user session DB, network calls or external AI tools touched.

**Review and limitations:** Team Lead examined the test-only source changes, bounded 12-second ready deadline, child PID ownership and cleanup on timeout, unique temp directories, idempotence assertions and no behavior change to production application. Cross-platform CI exact SHA and independent human review pending. Eight real providers running for two hours, 50 crash/cancel/resume repetitions per OS, installed native app GUI/PTY/provider and full R1 phase exit remain NO-GO.

**Team Lead verdict:** SOURCE REVIEW GO for PR only; protected merge and R1 phase exit NO-GO.
