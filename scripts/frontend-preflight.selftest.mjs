#!/usr/bin/env node
// Offline version-gate regression tests; deliberately no downloads or project writes.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import child from 'node:child_process';
import assert from 'node:assert/strict';

const script=path.join(path.dirname(fileURLToPath(import.meta.url)),'frontend-preflight.mjs');
const root=fs.mkdtempSync(path.join(os.tmpdir(),'toolkit-frontend-test-'));
const write=(relative,value)=>{
  const file=path.join(root,relative);fs.mkdirSync(path.dirname(file),{recursive:true});
  fs.writeFileSync(file,typeof value==='string'?value:JSON.stringify(value));
};
const installed=(name,version)=>write(path.join('node_modules',...name.split('/'),'package.json'),{name,version});
const run=(...opts)=>child.spawnSync(process.execPath,[script,'--root',root,...opts],{encoding:'utf8'});
try {
  const missing=run();
  assert.notEqual(missing.status,0);
  assert.match(missing.stderr,/PACKAGE_REQUIRED/);
  write('package-lock.json','{}');
  write('src/app/page.tsx','export default function Page() { return null; }');
  write('package.json',{scripts:{build:'next build'},dependencies:{next:'^16.4.0',react:'^19.0.0','react-dom':'^19.0.0'},devDependencies:{typescript:'^5.1.0'}});
  installed('next','16.4.0');
  installed('react','19.3.0');
  installed('react-dom','19.3.0');
  installed('typescript','5.9.0');
  const good=run();
  assert.equal(good.status,0,good.stderr);
  assert.match(good.stdout,/FRONTEND_FRAMEWORK_BASELINE_VERIFIED/);
  installed('next','16.3.0');
  const old=run();
  assert.notEqual(old.status,0);
  assert.match(old.stderr,/UPGRADE_APPROVAL_REQUIRED/);
  installed('next','16.4.0');
  write('yarn.lock','synthetic extra lock');
  assert.match(run().stderr,/LOCKFILE_AMBIGUOUS/);
  fs.unlinkSync(path.join(root,'yarn.lock'));
  fs.rmSync(path.join(root,'src'),{recursive:true,force:true});
  assert.match(run().stderr,/APP_ROUTER_REQUIRED/);
  write('package.json',{scripts:{build:'vite build'},devDependencies:{'@sveltejs/kit':'^3.0.0',svelte:'^5.57.1',typescript:'^6.0.0',vite:'^8.0.12','@sveltejs/vite-plugin-svelte':'^7.0.0'}});
  installed('@sveltejs/kit','2.70.3');
  installed('svelte','5.57.1');
  installed('typescript','6.0.0');
  installed('vite','8.0.12');
  installed('@sveltejs/vite-plugin-svelte','7.0.0');
  write('vite.config.ts',"import { sveltekit } from '@sveltejs/kit/vite';\n");
  assert.match(run().stderr,/UPGRADE_APPROVAL_REQUIRED/);
  installed('@sveltejs/kit','3.0.0');
  write('svelte.config.js','export default {}');
  assert.match(run().stderr,/SVELTEKIT3_CONFIG_MIGRATION_REQUIRED/);
  fs.unlinkSync(path.join(root,'svelte.config.js'));
  const v3=run();
  if (Number(process.versions.node.split('.')[0])<22 || Number(process.versions.node.split('.')[0])===22 && Number(process.versions.node.split('.')[1])<17) {
    assert.match(v3.stderr,/NODE_RUNTIME_INCOMPATIBLE/);
  } else {
    assert.equal(v3.status,0,v3.stderr);
  }
  console.log('frontend preflight regression PASS');
} finally {
  fs.rmSync(root,{recursive:true,force:true});
}
