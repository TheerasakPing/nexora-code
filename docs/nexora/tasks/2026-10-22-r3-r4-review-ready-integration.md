# Task Checklist — R3/R4 Consolidated Review-Ready Integration

**Created before implementation:** 2026-10-22.
**User request:** Combine work now and complete Nexora Code as far as actual CI/review permits, without falsely announcing a production release.
**Team Lead:** acting assistant (TEAM_LEAD_SERIAL); no second independent reviewer/worker has been claimed.
**Scope:** Consolidate Git ancestry of PR #14 + PR #15 + PR #16, preserve their evidence, open a single reviewable PR against develop.
**Branch:** integration/r3-r4-review-ready in separate worktree.
**Priority:** P0 reviewer-gated integration, P1 R3/R4 reliability.
**Safety:** No app installation, real agent spawning, protected branch bypass, signing, main merge or release.
**Dependencies:** #14 CI 6/6 PASS, #15 CI 6/6 PASS, #16 CI 6/6 PASS; protected develop requires six checks and independent approval.
**Definition of Done:** exact source ancestry included; source build/tests/Clippy PASS, no dirty worktree; all six CI checks on this exact new PR head; genuine outside approval before merge.

| Task | Actual assignee | Acceptance | Status |
| --- | --- | --- | --- |
| INT-R3-01 | TEAM_LEAD_SERIAL | Verify ancestry of PR #14/#15/#16 and unmodified feature code | [x] |
| INT-R3-02 | TEAM_LEAD_SERIAL | Full Mac Rust/Clippy and frontend test matrix | [ ] |
| INT-R3-03 | TEAM_LEAD_SERIAL | PR submission, GitHub CI link and Team Lead review evidence | [x] |
| INT-R3-04 | Independent Reviewer PENDING | Genuine separate review before protected develop merge | [ ] |
| INT-R3-05 | Team Lead with verified owner approval | Production GUI/native 3-OS/security/release gates | [ ] |

- [x] Original PR histories retained, no force pushes/branch replacement.
- [ ] CI and all native platform check results validated for new integration head.
- [x] No reviewer identity or permission fabricated.
- [ ] GO / NO-GO stated distinctly for source PR and protected merge.

**Team Lead verdict:** GO for a reviewable integration PR, NO-GO for protected branch merge and production release pending independent approval and exact-head CI.

## Verified handoff (2026-10-22)

- Ancestry included: origin/integration/r3-r4-windows-stabilized (#14), origin/feat/r3-worktree-isolation-audit (#15) and origin/feat/r3-worker-worktree-provisioner (#16); original branches unchanged.
- Original PR #14/#15/#16 had 6/6 required GitHub checks each. These are **not** substituted for the new integration head's CI.
- Source Mac frontend: npm run build PASS; npm test 139 PASS / 0 FAIL; release tests 31 PASS / 0 FAIL; R0 safety PASS.
- Full Mac Rust + Clippy: in progress at first PR submission. Record final result separately before claiming acceptance.
- Independent reviewer: PENDING. Only author TheerasakPing is on collaborators list and self-review cannot satisfy GitHub branch protection.
- No Mac GUI launch, signed installer, native multi-agent E2E, actual operator permission/workflow executor or three-platform real desktop acceptance.
