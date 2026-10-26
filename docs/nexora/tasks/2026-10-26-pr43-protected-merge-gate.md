# PR #43 protected branch merge gate — checklist planner

- [x] Lead: verify PR #43 CI 5/5 workflow runs passed at a5cdd1e
- [x] Lead: merge into develop was blocked by GitHub 405: 6/6 required status checks expected
- [x] Integration: verify develop and PR head, merge develop into PR branch without bypassing protection
- [ ] QA: clean Typecheck/Build, Unit, Release Contract, and targeted Playwright smoke
- [ ] CI: verify newly created GitHub workflows and required status contexts
- [ ] Lead: validate independent review/release policies; merge only if protection allows

This refresh is intended to make the pull request up-to-date. It does not change branch protection or fabricate status checks.

## Additional QA finding: simulator frame delivery

- [x] Stress after develop sync reproduced the old spinner check failure 3/28 with 4 workers.
- [x] Test fixture now records delivered frame bytes and waits for the fresh post-pose JPEG, instead of counting only issued requests; UI spinner assertion remains required.
- [x] Under parallel WebGL/OffscreenCanvas load, allow at most 15s for image decode and display. No test retries or skips.
- [x] Updated stress test passed 28/28 with 4 workers.
- [ ] Revalidate full hosted CI on refreshed PR head after push.
