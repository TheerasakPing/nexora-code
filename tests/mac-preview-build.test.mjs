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
