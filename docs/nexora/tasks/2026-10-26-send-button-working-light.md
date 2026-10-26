# Nexora Code — Running send/stop button orbit light

- [x] Lead: use existing `composer-running` activity state and `stop-button` action, no JS state duplication or click interception.
- [x] UI: subtle animated light travels slowly around the button border while agent works; original stop action, focus, color themes and size remain unchanged.
- [x] Accessibility: `prefers-reduced-motion` disables rotation and shows static accent border.
- [ ] W1: run frontend build, unit tests, smoke; validate hover + pointer events.
- [ ] Independent code review and cross-platform CI.
- [ ] Native GUI E2E and release acceptance pending.
