# iPhone Duo held-frame E2E synchronization — planner

- [x] Lead: inspect PR #43 macOS matrix run 40247554436, shard 3, one failure
- [x] QA: map `__simFrameHold`, pose completion and independent frame pump
- [ ] QA: assert a fresh frame request is pending after the pose before releasing the mock hold
- [ ] QA: focused repetitions, unit suite, build, smoke
- [ ] Lead: inspect diff, PR, CI, and only merge with green gates

The test's hold was released without confirming that the post-pose frame request had reached the mock. The pre-pose frames may race with that release, making a test of post-pose spinner completion nondeterministic. Assert the post-pose frame request is made while the hold is active; retain the spinner assertion and zero retries/skips.

## Follow-up after full local matrix

Local four-worker browser matrix passed 744/745, with the same spinner assertion failed. The request-count check was insufficient because it included stale pre-pose requests. QA now records frame `after` cursors and verifies a **post-pose fresh `after=0`** request before releasing the hold. Focused four-worker repetitions: 10/10 pass. Full matrix requires revalidation.
