# Planner — composer selection regression atomicity (2026-10-26)

- [x] Lead: inspect hosted macOS E2E shard 4 failure `40246401525` (185 pass, one fail)
- [x] QA: rerun selected test five times on exact PR #43 head; 5/5 pass locally
- [x] Lead: locate race between separate Playwright evaluate calls during asynchronous session render
- [ ] QA: keep setSelectionRange and immediate read in one browser task; preserve the post-save assertion unchanged
- [ ] QA: run focused regression, unit and build; submit stacked PR if green
- [ ] Lead: require final full four-shard suite and independent reviewer before develop merge

The reported failure occurs **before the delayed reasoning save is released**, between browser-driver operations that can allow an unrelated session render. Establishing and confirming the selection atomically avoids measuring that unrelated browser scheduling window; the independent post-save preservation assertion is retained without timeout increase, retries, or skips.
