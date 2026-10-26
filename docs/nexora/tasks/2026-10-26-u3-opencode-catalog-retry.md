# Nexora Code U3 — stale OpenCode catalog recovery (2026-10-26)

## A. Intake and Lead assignment
- Request: continue selective PandaCode U2/U3 backports and hardening.
- Scope: on an OpenCode catalog **partial error** with saved IDs, keep the saved choices visible but do not cache incomplete metadata for five minutes.
- Branch: `fix/20261026-model-discovery-u3` based on `origin/develop` at f0e18e8, isolated worktree.
- Team Lead: ChatGPT. Available delegated agents: NONE; `TEAM_LEAD_SERIAL` owns code, browser regression, review. No independent reviewer claimed.
- Boundaries: no CLI authentication, provider execution, Keychain/DB changes or cloud calls.
- Acceptance: successful discovery retains 5-minute cache; partial failure preserves saved IDs and refreshes on subsequent request after short backoff; failed catalog with no saved choices continues to report error.

## B. Work allocation
| ID | Deliverable | Owner | Status |
| --- | --- | --- | --- |
| U3-01 | Fix degraded-catalog TTL without stale-result races | TEAM_LEAD_SERIAL | [x] local browser scope |
| U3-02 | Browser regression for temporary catalog partial error and retry | TEAM_LEAD_SERIAL | [x] local browser scope |
| U3-QA | Diff inspection, unit, browser E2E, smoke, report risks | TEAM_LEAD_SERIAL | [ ] independent/native review pending |

## G. NC-TEAM-003 evidence (update after runs)
| ID | Unit/Contract | E2E | Smoke | Lead review | Evidence |
| --- | --- | --- | --- | --- | --- |
| U3-01 | PASS: TS build + 211 Node unit tests | PASS: OpenCode Browser E2E 8/8 | PASS: R0 startup/identity smoke | PASS: Lead reviewed `src/main.ts` change | Degraded metadata cache retries after 5s; normal catalog retains 5 min |
| U3-02 | PASS: Playwright typed source check | PASS: 8/8 including temporary catalog failure/recovery | PASS: preview picker with selected model ID | PASS: Lead inspected regression fixture | `tests/opencode-models.spec.ts`, Mac Apple Silicon |
| U3-QA | PASS: release tests 43/44 and 1 skip | PENDING: native installed CLI | PENDING: real provider login | PENDING: independent review | No CLI credentials, no end-user data modified |

## E. Release Gate
- Changes are safe local UI metadata; any native CLI/model-provider tests remain independently PENDING.
- Do not merge into protected branches or release until CI, Lead review and relevant native tests.
