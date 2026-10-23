import assert from 'node:assert/strict';
import { existsSync, readFileSync, statSync } from 'node:fs';
import { createHash } from 'node:crypto';
import test from 'node:test';

const text = path => readFileSync(new URL('../' + path, import.meta.url), 'utf8');
const bytes = path => readFileSync(new URL('../' + path, import.meta.url));
const hash = content => createHash('sha256').update(content).digest('hex');

test('Nexora source brand is consistent across native metadata, browser and launcher', () => {
  const config = JSON.parse(text('src-tauri/tauri.conf.json'));
  assert.equal(config.productName, 'Nexora Code');
  assert.equal(config.identifier, 'com.theerasakping.nexoracode');
  assert.match(text('index.html'), /<title>Nexora Code<\/title>/);
  assert.doesNotMatch(text('index.html'), /PandaCode/);
  assert.match(text('src/main.ts'), /class="brand-logo" src="\/nexora-mark\.svg"/);
  assert.match(text('src/main.ts'), /class="brand-name"[^>]*>Nexora <strong>Code<\/strong>/);
  assert.match(text('src/overlay.ts'), /class="launcher-brand-logo"/);
  assert.match(text('src/overlay.ts'), /Nexora Code quick launcher/);
  assert.match(text('src/about-settings.ts'), /class="settings-about-brand"/);
});

test('vector master and exported favicon/logo variants are original and identical', () => {
  const master = text('brand/nexora-mark.svg');
  assert.match(master, /Nexora Code/);
  assert.match(master, /45E5E1/);
  assert.match(master, /A986FF/);
  assert.doesNotMatch(master, /data:image|PandaCode|bamboo/);
  assert.equal(hash(master), hash(text('public/icon.svg')));
  assert.equal(hash(master), hash(text('public/nexora-mark.svg')));
  for (const icon of [
    'src-tauri/icons/icon.icns',
    'src-tauri/icons/icon.ico',
    'src-tauri/icons/32x32.png',
    'src-tauri/icons/128x128.png',
    'src-tauri/icons/128x128@2x.png',
  ]) {
    assert.ok(existsSync(new URL('../' + icon, import.meta.url)), icon);
    assert.ok(statSync(new URL('../' + icon, import.meta.url)).size > 200, icon);
  }
  assert.equal(bytes('src-tauri/icons/icon.icns').subarray(0, 4).toString(), 'icns');
  assert.equal(bytes('src-tauri/icons/icon.ico').readUInt16LE(2), 1);
});

test('branding does not delete required fork notices or rename stable internal contracts', () => {
  assert.match(text('src/about-settings.ts'), /This is a fork of PandaCode/);
  assert.match(text('README.md'), /This is a fork of PandaCode/);
  assert.match(text('src/token-saver.ts'), /Response style \(PandaCode token saver/);
  assert.match(text('src/main.ts'), /pandacode\.provider/);
  assert.match(text('src/bridge.ts'), /pandacode|Nexora Code/);
  assert.match(text('src/styles.css'), /nexora-watermark\.svg/);
  assert.doesNotMatch(text('src/main.ts'), /icon\('bamboo'\)/);
});
