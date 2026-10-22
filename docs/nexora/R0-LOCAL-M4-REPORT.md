# R0 local engineering verification — 22 October 2026

Host: Apple M4 / arm64, macOS 27.2; Node.js 26.8.2, cargo 1.95.0.
Base: PandaCode v0.3.16 SHA 30f9c823e3c38f472f89e8f8405575ecfe7173ee.
Branch: feat/r0-foundation (unpublished local branch).

Verified on the connected Mac:
- npm ci --no-audit --no-fund: PASS (90 packages)
- npm run build: PASS (TypeScript and Vite production frontend)
- npm test: PASS (104/104 Node unit tests)
- npm run test:release: PASS (31/31 release script fixture tests)
- npm run test:nexora:r0: PASS (fork identity, attribution and updater safety)

Not yet a production sign-off:
- Rust test suite: 982 passed, 0 failed, 33 ignored; additional CLI integration target 1/1 passed on Mac M4.
- Mac native app / Finder launch and native GUI approval not yet verified.
- Windows 11 x64 and Linux x64 native acceptance not yet run.
- Fork updater disabled: new signing keys and release verification are required.
- CI target Node 22, whereas this Mac check used Node 26.8.2.
- No GitHub PR / remote CI until GitHub CLI is authenticated as the repository owner.

## Mac-specific Provider Discovery regression found and fixed

Initial Rust run: 981 passed, 1 failed, 33 ignored. The test fixture named its dummy executable
'dsh', but the real Mac also had Homebrew dsh in /opt/homebrew/bin, which correctly took precedence.
The fixture now uses a UUID-suffixed executable name while production provider lookup stays unchanged.
Focused test: PASS. Full Rust retest: 982 passed, 0 failed, 33 ignored; CLI target 1 passed.
These are local tests, not proof of signed desktop installer or cross-platform GUI acceptance.
