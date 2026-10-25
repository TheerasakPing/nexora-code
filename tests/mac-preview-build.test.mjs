import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
const read = path => readFileSync(new URL('../' + path, import.meta.url), 'utf8');

test('Mac release remains stripped without stripping host proc-macros', () => {
  const cargo = read('src-tauri/Cargo.toml');
  assert.match(cargo, /\[profile\.release\][\s\S]*?strip\s*=\s*true/);
  assert.match(cargo, /\[profile\.release\.build-override\]\s*\nstrip\s*=\s*false/);
});

test('Mac preview uses a no-Finder DMG and rejects production signing keys', () => {
  const build = read('scripts/nexora/build-mac-preview-dmg.sh');
  const verify = read('scripts/nexora/verify-mac-preview-dmg.sh');
  assert.match(build, /tauri -- build --bundles app/);
  assert.match(build, /codesign --verify --deep --strict/);
  assert.match(build, /hdiutil attach -quiet -readonly/);
  assert.match(build, /TAURI_SIGNING_PRIVATE_KEY/);
  assert.match(build, /PREVIEW-UNNOTARIZED\.dmg/);
  assert.doesNotMatch(build, /osascript|\/Applications\/Nexora Code\.app/);
  assert.match(verify, /shasum -a 256 -c/);
  assert.match(verify, /lipo -archs/);
});

test('test-only Mac workflow archives preview, never publishes a release', () => {
  const action = read('.github/workflows/nexora-r6-alpha.yml');
  assert.match(action, /scripts\/nexora\/build-mac-preview-dmg\.sh/);
  assert.match(action, /scripts\/nexora\/verify-mac-preview-dmg\.sh/);
  assert.match(action, /actions\/upload-artifact@v4/);
  assert.match(action, /contents: read/);
  assert.doesNotMatch(action, /gh release create|softprops\/action-gh-release/);
});

test('Mac installer discloses Gatekeeper rejection and embeds visible instructions', () => {
  const build = read('scripts/nexora/build-mac-preview-dmg.sh');
  const verify = read('scripts/nexora/verify-mac-preview-dmg.sh');
  const instructions = read('scripts/nexora/MAC-PREVIEW-INSTALL.txt');
  assert.match(build, /MAC-PREVIEW-INSTALL\.txt/);
  assert.match(build, /READ ME - INSTALLATION\.txt/);
  assert.match(verify, /spctl --assess --type execute/);
  assert.match(verify, /GATEKEEPER_ASSESSMENT=REJECTED/);
  assert.match(verify, /missing Gatekeeper instructions/);
  assert.match(instructions, /System Settings > Privacy & Security/);
  assert.match(instructions, /Open Anyway/);
  assert.match(instructions, /AD-HOC SIGNED, NOT APPLE-NOTARIZED/);
  assert.match(instructions, /ลาก/);
  // Never bypass macOS security via an install script.
  for (const script of [build, verify]) {
    assert.doesNotMatch(script, /spctl\s+--(master|global)-disable/);
    assert.doesNotMatch(script, /xattr\s+-[cdr]+/);
    assert.doesNotMatch(script, /sudo\s+/);
  }
});

test('alpha packaging re-runs when app source changes and passes same-SHA quality gates before publishing', () => {
  const action = read('.github/workflows/nexora-r6-alpha.yml');
  for (const path of ['src/**', 'src-tauri/src/**', 'src-tauri/capabilities/**', 'src-tauri/Cargo.lock']) {
    assert.ok(action.includes(`- '${path}'`), `missing source trigger ${path}`);
  }
  for (const gate of ['npm run build', 'npm test', 'npm run test:release', 'cargo test --locked', 'cargo clippy --locked']) {
    assert.ok(action.includes(gate), `missing installer source gate ${gate}`);
    assert.ok(action.indexOf(gate) < action.indexOf('actions/upload-artifact@v4'), `artifact uploaded before ${gate}`);
  }
  assert.match(action, /components: clippy/);
  assert.match(action, /TAURI_SIGNING_PRIVATE_KEY: ''/);
  assert.match(action, /APPLE_SIGNING_IDENTITY: '-'/);
});

test('Mac DMG staging and independent verifier reject non-ad-hoc signatures', () => {
  const build = read('scripts/nexora/build-mac-preview-dmg.sh');
  const verify = read('scripts/nexora/verify-mac-preview-dmg.sh');
  assert.match(build, /codesign --force --deep --sign -/);
  for (const script of [build, verify]) {
    assert.match(script, /codesign -dv --verbose=4/);
    assert.match(script, /Signature=adhoc/);
    assert.match(script, /Authority=/);
    assert.match(script, /codesign --verify --deep --strict/);
  }
});
