# Nexora Code — Send orbit animation browser E2E acceptance

- [x] Lead: review installed PR #48 visual feature, identified missing behavior-specific E2E.
- [x] W1 UI QA: test controlled in-browser agent stream with real DOM: running composer stop button is visible, border orbit has animation duration `5.5s`, pseudo-element `pointer-events: none`, reduced-motion disables animation, normal motion restores it, end-of-task removes running visual.
- [x] Targeted Playwright smoke on macOS: 2/2 PASS via `npx playwright test tests/streaming.spec.ts --grep 'shows a slow working-button orbit' --repeat-each=2 --workers=1`.
- [ ] Lead: run frontend build/Node Unit, push PR, CI and human review.
- [ ] Native actual user provider Agent session remains not tested (no credentials exposed or imported). Preview/unsigned installer is not Production approval.
