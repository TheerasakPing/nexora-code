import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const read=path=>readFileSync(new URL('../'+path,import.meta.url),'utf8');
test('R2 registry refuses common inline auth/secret arguments',()=>{
 const source=read('src-tauri/src/mcp_registry.rs');
 for(const term of ['--api-key','--token','--password','--secret','--header','--env','client_secret=','authorization:']) {
   assert.ok(source.includes(term), 'missing secret-argument denial: '+term);
 }
 assert.match(source,/valid_credential_ref/);
 assert.match(source,/credential_ref/);
});
test('R2 credential reads require trusted native consent, no renderer IPC bridge',()=>{
 const source=read('src-tauri/src/mcp_vault.rs');
 assert.match(source,/VerifiedOperatorConsent/);
 assert.doesNotMatch(source,/#\[tauri::command\]/);
 assert.match(source,/secret_store::set/);
 assert.match(source,/secret_store::get/);
 assert.match(read('src-tauri/src/secret_store.rs'),/#\[cfg\(test\)\][\s\S]*?fn memory\(/);
});
test('R2 unapproved registry entries stay fail closed',()=>{
 const src=read('src-tauri/src/mcp_permissions.rs');
 assert.match(src,/approval_system_pending/);
 assert.match(src,/allowed: false/);
 assert.doesNotMatch(src,/pub fn evaluate[\s\S]*?PolicyDecision \{\s*allowed: true/);
});
