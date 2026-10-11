#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const args = process.argv.slice(2);
const opt = (name) => { const i = args.indexOf(name); return i < 0 ? undefined : args[i + 1]; };
const root = path.resolve(opt('--root') ?? process.cwd());
const policyPath = path.join(path.dirname(fileURLToPath(import.meta.url)), 'frontend-versions.json');
const policy = JSON.parse(fs.readFileSync(policyPath, 'utf8'));
const errors = [];
const info = [];
const fail = (s) => errors.push(s);
const readJSON = (f) => { try { return JSON.parse(fs.readFileSync(f, 'utf8')); } catch { return null; } };
const pkg = readJSON(path.join(root, 'package.json'));
if (!pkg) fail('PACKAGE_REQUIRED: no readable package.json at --root');
const dependencies = { ...(pkg?.dependencies ?? {}), ...(pkg?.devDependencies ?? {}) };
const hasNext = Boolean(dependencies.next);
const hasKit = Boolean(dependencies['@sveltejs/kit']);
const requested = opt('--framework');
if (requested && !['nextjs', 'sveltekit'].includes(requested)) fail('invalid --framework; use nextjs or sveltekit');
if (hasNext && hasKit) fail('AMBIGUOUS_FRAMEWORK: choose an individual package root; no mixed gate');
const framework = requested ?? (hasNext ? 'nextjs' : hasKit ? 'sveltekit' : null);
if (!framework) fail('FRAMEWORK_NOT_FOUND: expected next or @sveltejs/kit in this package.json');
if (framework === 'nextjs' && !hasNext) fail('FRAMEWORK_NOT_FOUND: next is not a direct dependency');
if (framework === 'sveltekit' && !hasKit) fail('FRAMEWORK_NOT_FOUND: @sveltejs/kit is not a direct dependency');
const packageVersion = (name) => {
  const f = path.join(root, 'node_modules', ...name.split('/'), 'package.json');
  const manifest = readJSON(f);
  if (!manifest || typeof manifest.version !== 'string') { fail('DEPENDENCY_NOT_INSTALLED: ' + name + ' not resolvable at ' + f); return null; }
  return manifest.version;
};
const stable = (v) => typeof v === 'string' && /^\d+\.\d+\.\d+$/.test(v);
const parts = (v) => v.split('.').map(Number);
const minimum = (v, floor) => {
  if (!stable(v) || !stable(floor)) return false;
  const a = parts(v), b = parts(floor);
  return a[0] > b[0] || (a[0] === b[0] && (a[1] > b[1] || (a[1] === b[1] && a[2] >= b[2])));
};
const declaredConsistent = (name, actual) => {
  const declared = dependencies[name];
  if (typeof declared !== 'string') {
    fail('DEPENDENCY_DECLARATION_REQUIRED: missing package.json dependency '+name);
    return;
  }
  const normalized = declared.replace(/^[~^]/, '');
  if (!stable(normalized) || !stable(actual)) {
    fail('DEPENDENCY_DECLARATION_UNVERIFIED: '+name+' declared '+declared+'; use a verifiable semver range for this strict baseline');
    return;
  }
  const [major, minor, patch] = parts(normalized);
  const [installedMajor, installedMinor, installedPatch] = parts(actual);
  const allows = declared.startsWith('^') ?
    installedMajor===major && minimum(actual, normalized) :
    declared.startsWith('~') ?
      installedMajor===major && installedMinor===minor && installedPatch>=patch :
      actual===normalized;
  if (!allows) fail('PACKAGE_LOCK_DRIFT: '+name+' declared '+declared+' but installed '+actual+'; update manifest and lock only with approval');
};
const checkExact = (name, target) => {
  const v = packageVersion(name);
  if (v) declaredConsistent(name, v);
  if (v && v !== target) fail('UPGRADE_APPROVAL_REQUIRED: ' + name + ' installed ' + v + ', toolkit tested ' + target + '. Stop and ask user before any upgrade.');
  else if (v) info.push(name + ' ' + v);
};
const checkMin = (name, target) => {
  const v = packageVersion(name);
  if (v) declaredConsistent(name, v);
  if (v && !minimum(v, target)) fail('UPGRADE_APPROVAL_REQUIRED: ' + name + ' installed ' + v + ', minimum ' + target + '. Stop and ask user before any upgrade.');
  else if (v) info.push(name + ' ' + v + ' (>= ' + target + ')');
};
const nodeVersion = process.versions.node;
const selected = policy.frameworks[framework];
if (selected) {
  info.push('framework=' + framework + ' tested=' + selected.primary.version);
  if (!minimum(nodeVersion, selected.node_min)) fail('NODE_RUNTIME_INCOMPATIBLE: Node ' + nodeVersion + '; ' + framework + ' requires >= ' + selected.node_min);
  checkExact(selected.primary.package, selected.primary.version);
  for (const [name, floor] of Object.entries(selected.minimum_packages ?? {})) checkMin(name, floor);
}
if (framework === 'nextjs') {
  if (!['app', 'src/app'].some(x => fs.existsSync(path.join(root,x)))) fail('APP_ROUTER_REQUIRED: no app/ or src/app/ found. Pages Router requires separate migration.');
}
if (framework === 'sveltekit') {
  if (['svelte.config.js', 'svelte.config.ts', 'svelte.config.mjs'].some(x => fs.existsSync(path.join(root,x)))) fail('SVELTEKIT3_CONFIG_MIGRATION_REQUIRED: legacy svelte.config.* is not supported in SvelteKit 3');
  const viteConfig = ['vite.config.ts', 'vite.config.js', 'vite.config.mjs'].find(x => fs.existsSync(path.join(root,x)));
  if (!viteConfig || !fs.readFileSync(path.join(root,viteConfig),'utf8').includes('@sveltejs/kit/vite')) fail('SVELTEKIT3_VITE_CONFIG_REQUIRED: configure the kit plugin in vite.config.*');
}
const knownLocks = ['package-lock.json', 'pnpm-lock.yaml', 'yarn.lock', 'bun.lock', 'bun.lockb'];
let lockDir = root, lockfiles = [];
for (let depth=0; depth<6; depth++) {
  lockfiles=knownLocks.filter(x=>fs.existsSync(path.join(lockDir,x)));
  if (lockfiles.length) break;
  if (fs.existsSync(path.join(lockDir,'.git'))) break;
  const parent=path.dirname(lockDir);
  if (parent===lockDir) break;
  lockDir=parent;
}
if (lockfiles.length === 0) fail('LOCKFILE_REQUIRED: no package/workspace lockfile found');
if (lockfiles.length > 1) fail('LOCKFILE_AMBIGUOUS: multiple lockfiles at '+lockDir);
if (lockfiles.length===1) {
  info.push('lockfile='+path.relative(root,path.join(lockDir,lockfiles[0])));
  const manager=typeof pkg?.packageManager==='string' ? pkg.packageManager.split('@')[0] : null;
  const expected={npm:'package-lock.json',pnpm:'pnpm-lock.yaml',yarn:'yarn.lock',bun:'bun.lock'};
  if (manager && expected[manager] && ![expected[manager],manager==='bun'?'bun.lockb':''].includes(lockfiles[0])) {
    fail('PACKAGE_MANAGER_MISMATCH: packageManager='+manager+' but lockfile='+lockfiles[0]);
  }
}
if (!pkg?.scripts || !pkg.scripts.build) fail('BUILD_SCRIPT_REQUIRED: package.json scripts.build');
for (const s of info) console.log('[INFO] '+s);
if (errors.length) {
  for (const s of errors) console.error('[FAIL] '+s);
  if (framework && selected) {
    console.error('[STOP] No code changes, dependency installs, codemods or tests against the new standards.');
    console.error('[ACTION] Ask: "The project does not match the tested ' + framework + ' version (' + selected?.primary.version + '). May I plan and perform a controlled upgrade?"');
    console.error('[ACTION] On approval: inspect breaking changes and peer requirements; prepare migration on a dedicated branch; then install, test and rerun this gate. Do not auto-accept codemod defaults.');
    console.error('[DOCS] '+selected?.migration_url);
  }
  process.exitCode = 2;
} else {
  console.log('[PASS] FRONTEND_FRAMEWORK_BASELINE_VERIFIED '+framework+' '+selected.primary.version);
}
