# Planner — native macOS GUI permission evidence

- [x] Lead intake: installed Nexora Code app opens and SQLite integrity passes; GUI Accessibility from Nexora Studio execution context is denied (`AXIsProcessTrusted=false`, System Events `UI elements enabled=false`), screenshot capture fails. Do not claim GUI E2E success.
- [x] Scope: add a *read-only* check of GUI automation permission to the existing installed macOS acceptance verifier, distinguished from actual GUI interaction acceptance.
- [x] QA role (serial, no independent agent connected): unit tests for allowed/denied/unknown permission, smoke on installed macOS, E2E token-saver browser and build.
- [x] Lead self-review: inspect changed file diff, data/privacy safeguards and no false release-ready state.
- [ ] Open PR into develop, CI and independent review before production acceptance. No user data edits or installer changes.

Never toggle TCC or Accessibility settings programmatically. A successful preflight is not proof that the GUI was exercised, or that Screen Recording is authorized.

## Verified result

- Mac installed runner: `localSmokePass=true`, `runnerAccessibility=BLOCKED_ACCESSIBILITY`, interactive GUI stays `PENDING_MANUAL_ACCEPTANCE`, signing/notarization blocked.
- Unit 211/211; release contract 43 pass, 1 existing skip; frontend build pass; Token Saver E2E 3/3.
- No OS permissions toggled, no screenshots captured, no database or session modified.
