#!/usr/bin/env node
// Application Knowledge Contract v1. No external dependencies, no credentials on disk.
import fs from "node:fs";
import path from "node:path";
import crypto from "node:crypto";
import child from "node:child_process";

const methods = new Set(["get","put","post","delete","patch","options","head","trace"]);
const kinds = new Set(["product","domain","workflows","ui","permissions","api","agent-capabilities"]);
const args = process.argv.slice(2);
const command = args[0] ?? "help";
const option = name => { const i=args.indexOf(name); return i<0 ? undefined : args[i+1]; };
const root=path.resolve(option("--root") ?? process.cwd());
function fatal(message) { throw new Error(message); }
function requireText(s, name) { if (typeof s!=="string" || !s.trim()) fatal(name+" must be a non-empty string"); return s; }
function safePath(relative, mustExist=true) {
  requireText(relative,"path");
  if(path.isAbsolute(relative) || relative.includes("\\") || relative.split("/").some(p=>p===".."||p==="."||p==="")) fatal("unsafe repository path: "+relative);
  const full=path.resolve(root,relative);
  if(!full.startsWith(root+path.sep)) fatal("path escapes repository: "+relative);
  let check=root;
  for(const piece of relative.split("/")) {
    check=path.join(check,piece);
    if(fs.existsSync(check) && fs.lstatSync(check).isSymbolicLink()) fatal("symlink prohibited in knowledge path: "+relative);
  }
  if(mustExist && !fs.existsSync(full)) fatal("missing file: "+relative);
  return full;
}
function loadJson(relative) {
  let data;
  try { data=JSON.parse(fs.readFileSync(safePath(relative),"utf8")); } catch(e) { fatal("invalid JSON "+relative+": "+e.message); }
  if(!data || Array.isArray(data) || typeof data!=="object") fatal("JSON object required: "+relative);
  return data;
}
function digest(bytes) { return crypto.createHash("sha256").update(bytes).digest("hex"); }
function hashFile(relative) { return digest(fs.readFileSync(safePath(relative))); }
const configFile="app-knowledge.config.json";
function readConfig(required) {
  if(!fs.existsSync(path.join(root,configFile))) {
    if(required) fatal("missing "+configFile+" (run /app-knowledge-inventory after OpenAPI preflight)");
    return null;
  }
  const cfg=loadJson(configFile);
  if(cfg.schema_version!==1) fatal("unsupported config schema_version");
  requireText(cfg.application_id,"application_id");
  requireText(cfg.openapi_path,"openapi_path");
  if(cfg.knowledge_dir!=="docs/app-knowledge") fatal("knowledge_dir must be docs/app-knowledge in contract v1");
  return cfg;
}
function openapiFile(cfg) {
  if(cfg) return cfg.openapi_path;
  const guesses=["api/openapi.json","api/openapi/openapi.json","openapi.json","docs/openapi.json","openapi/openapi.json"];
  const found=guesses.filter(p=>fs.existsSync(path.join(root,p)));
  if(found.length!==1) fatal("OPENAPI_REQUIRED: expected one OpenAPI JSON file or app-knowledge.config.json with openapi_path; found "+found.length+". Abort inventory; do not synthesize a contract.");
  return found[0];
}
function openapiCheck(spec) {
  if(!/^3\.(0|1)\.\d+/.test(String(spec.openapi??""))) fatal("OpenAPI 3.0/3.1 required");
  requireText(spec.info?.title,"OpenAPI info.title");
  requireText(spec.info?.version,"OpenAPI info.version");
  if(!spec.paths || typeof spec.paths!=="object") fatal("OpenAPI paths object required");
  const operations=[],seen=new Set();
  for(const [route,values] of Object.entries(spec.paths)) {
    if(!route.startsWith("/")) fatal("invalid OpenAPI route: "+route);
    for(const [method,value] of Object.entries(values??{})) {
      if(!methods.has(method)) continue;
      const id=requireText(value?.operationId,method.toUpperCase()+" "+route+" operationId");
      if(seen.has(id)) fatal("duplicate operationId: "+id);
      if(!value.responses || !Object.keys(value.responses).length) fatal("responses missing: "+id);
      seen.add(id); operations.push({id,method,route});
    }
  }
  if(!operations.length) fatal("OpenAPI has no operations");
  const refs=[];
  function collect(node) {
    if(!node || typeof node!=="object") return;
    if(typeof node.$ref==="string") refs.push(node.$ref);
    for(const value of Object.values(node)) if(value && typeof value==="object") collect(value);
  }
  collect(spec);
  for(const ref of refs) {
    if(!ref.startsWith("#/")) fatal("external OpenAPI $ref needs resolved/bundled JSON: "+ref);
    const parts=ref.slice(2).split("/").map(x=>x.replace(/~1/g,"/").replace(/~0/g,"~"));
    let item=spec;
    for(const p of parts) item=item?.[p];
    if(item===undefined) fatal("unresolved OpenAPI $ref: "+ref);
  }
  return operations;
}
function preflight() {
  const cfg=readConfig(false), specPath=openapiFile(cfg);
  const spec=loadJson(specPath),operations=openapiCheck(spec);
  console.log("[PASS] OpenAPI preflight: "+specPath+" ("+operations.length+" operations, version "+spec.info.version+")");
  return {cfg,specPath,spec,operations};
}
function inventory() {
  const base=preflight();
  if(!base.cfg) fatal("Create "+configFile+" using the Application Knowledge Contract; no files were generated.");
  const mPath=base.cfg.knowledge_dir+"/manifest.json",m=loadJson(mPath);
  if(m.schema_version!==1 || m.application_id!==base.cfg.application_id) fatal("manifest identity/schema mismatch");
  if(m.app_version!==base.spec.info.version) fatal("app_version must equal OpenAPI info.version");
  if(m.openapi_sha256!==hashFile(base.specPath)) fatal("OPENAPI_DRIFT: sha256 mismatch; regenerate inventory");
  if(!Array.isArray(m.documents) || !m.documents.length) fatal("documents inventory empty");
  const ids=new Set(),covered=new Set(),presentKinds=new Set();
  for(const d of m.documents) {
    requireText(d.id,"document.id");
    if(ids.has(d.id)) fatal("duplicate document id "+d.id);
    ids.add(d.id);
    if(!kinds.has(d.kind)) fatal("invalid document kind for "+d.id);
    presentKinds.add(d.kind);
    const p=requireText(d.path,"document.path");
    if(!p.startsWith(base.cfg.knowledge_dir+"/") || !p.endsWith(".md")) fatal("document must be markdown inside knowledge_dir: "+p);
    if(d.sha256!==hashFile(p)) fatal("DOCUMENT_DRIFT: "+p);
    if(d.verification!=="verified" && d.verification!=="needs_review") fatal("verification status missing: "+d.id);
    if(!Array.isArray(d.sources)||!d.sources.length) fatal("evidence sources missing: "+d.id);
    for(const s of d.sources) safePath(s);
    if(!Array.isArray(d.operation_ids)) fatal("operation_ids array required: "+d.id);
    for(const op of d.operation_ids) covered.add(op);
  }
  for(const kind of kinds) if(!presentKinds.has(kind)) fatal("missing required knowledge category: "+kind);
  const known=new Set(base.operations.map(x=>x.id));
  for(const op of covered) if(!known.has(op)) fatal("inventory references unknown OpenAPI operationId: "+op);
  const missing=base.operations.filter(x=>!covered.has(x.id));
  if(missing.length) fatal("API_COVERAGE_GAP: uncovered operationIds: "+missing.map(x=>x.id).join(", "));
  if(!Array.isArray(m.gaps)) fatal("gaps array required (use [] if none)");
  for(const gap of m.gaps) {
    requireText(gap.id,"gap.id"); requireText(gap.description,"gap.description");
    if(typeof gap.blocks_publish!=="boolean") fatal("gap.blocks_publish must be boolean");
  }
  const blocked=m.gaps.filter(x=>x.blocks_publish);
  const unreviewed=m.documents.filter(x=>x.verification!=="verified");
  const fingerprint=digest(JSON.stringify({
    application_id:m.application_id,app_version:m.app_version,
    openapi_sha256:m.openapi_sha256,
    documents:m.documents.map(d=>[d.id,d.sha256]).sort((a,b)=>a[0].localeCompare(b[0]))
  })).slice(0,16);
  console.log("[PASS] manifest consistency: "+m.documents.length+" docs; "+covered.size+"/"+base.operations.length+" API operations");
  if(blocked.length) console.log("[BLOCKED] "+blocked.length+" release-blocking knowledge gaps");
  if(unreviewed.length) console.log("[BLOCKED] "+unreviewed.length+" unverified knowledge documents");
  return {base,manifest:m,fingerprint,releaseReady:!blocked.length&&!unreviewed.length};
}
function publishPlan() {
  const result=inventory(), cfg=result.base.cfg;
  if(!result.releaseReady) fatal("PUBLISH_BLOCKED: unresolved gaps or unverified documents");
  const prefix=requireText(cfg.release_space_prefix,"release_space_prefix");
  if(!prefix.startsWith(cfg.application_id+".release.") || !/^[a-z0-9.-]+$/.test(prefix)) fatal("release_space_prefix must start with application_id+'.release.'");
  const space=prefix+result.fingerprint;
  const plan={
    status:"READY_TO_STAGE_ONLY",application_id:cfg.application_id,app_version:result.manifest.app_version,
    release_id:result.fingerprint,release_space_key:space,document_count:result.manifest.documents.length,
    source_git_sha:child.execFileSync("git",["-C",root,"rev-parse","HEAD"],{encoding:"utf8"}).trim(),
    operation_count:result.base.operations.length,
    note:"RAGGW knowledge release atomic activation is not available. No production activation or writes occur here."
  };
  console.log(JSON.stringify(plan,null,2));
}
async function stage() {
  const result=inventory(),cfg=result.base.cfg;
  if(!result.releaseReady) fatal("PUBLISH_BLOCKED: unverified or missing knowledge");
  const prefix=requireText(cfg.release_space_prefix,"release_space_prefix");
  if(!prefix.startsWith(cfg.application_id+".release.") || !/^[a-z0-9.-]+$/.test(prefix)) fatal("unsafe release_space_prefix");
  const space=prefix+result.fingerprint;
  if(option("--confirm")!==result.fingerprint) fatal("STAGE_DENIED: run publish-plan and pass --confirm <release_id>");
  if(option("--space")!==space) fatal("STAGE_DENIED: --space must be the exact immutable release space "+space);
  const base=requireText(process.env.RAGGW_URL,"RAGGW_URL");
  const token=requireText(process.env.RAGGW_TOKEN,"RAGGW_TOKEN");
  const url=new URL(base);
  if(url.protocol!=="https:" && !(url.protocol==="http:" && ["localhost","127.0.0.1","[::1]"].includes(url.hostname))) fatal("HTTPS required for non-local RAGGW");
  const origin=url.href.replace(/\/+$/,"");
  async function request(endpoint,init={}) {
    const res=await fetch(origin+endpoint,{...init,headers:{"Authorization":"Bearer "+token,...init.headers},signal:AbortSignal.timeout(15000)});
    if(!res.ok) fatal("RAGGW "+endpoint+" HTTP "+res.status+" (no secret/body logged)");
    return res.json();
  }
  const discovery=await request("/v1/discovery");
  if(!Array.isArray(discovery.knowledge_spaces)||!discovery.knowledge_spaces.some(x=>(x.key===space||x.knowledge_space_key===space)&&x.can_write===true)) fatal("STAGE_DENIED: release space is not explicitly registered with write grant");
  let maxRevision=0;
  for(const doc of result.manifest.documents) {
    const content=fs.readFileSync(safePath(doc.path),"utf8");
    const sourceId=result.fingerprint+"/"+doc.id;
    const body=JSON.stringify({knowledge_space_key:space,source_type:"app_knowledge",source_id:sourceId,version_key:doc.sha256,media_type:"text/markdown",content});
    const idempotencyKey=digest(space+"/"+sourceId+"/"+doc.sha256);
    const job=await request("/v1/documents",{method:"POST",headers:{"Content-Type":"application/json","Idempotency-Key":idempotencyKey},body});
    if(!job.job_id) fatal("RAGGW ingest returned no job_id");
    let done=false;
    for(let n=0;n<35;n++) {
      const j=await request("/v1/jobs/"+encodeURIComponent(job.job_id));
      if(j.state==="succeeded") {
        maxRevision=Math.max(maxRevision,Number(j.result?.knowledge_revision)||0);
        done=true; break;
      }
      if(j.state==="failed"||j.state==="dead_letter") fatal("RAGGW ingest failed for "+doc.id+" ("+(j.error?.error_code||"unknown")+")");
      await new Promise(resolve=>setTimeout(resolve,500));
    }
    if(!done) fatal("RAGGW ingest polling expired for "+doc.id);
    console.log("[STAGED] "+doc.id);
  }
  console.log(JSON.stringify({status:"STAGED_NOT_ACTIVATED",release_id:result.fingerprint,space,knowledge_revision:maxRevision,documents:result.manifest.documents.length,note:"Requires app-specific retrieval evals and independent activation gate."},null,2));
}
async function main(){
  if(!fs.existsSync(root)||!fs.statSync(root).isDirectory()) fatal("invalid root "+root);
  if(command==="preflight") preflight();
  else if(command==="validate") {const r=inventory(); if(!r.releaseReady) fatal("NOT_RELEASE_READY");}
  else if(command==="publish-plan") publishPlan();
  else if(command==="stage") await stage();
  else {
    console.log("Usage: bun scripts/app-knowledge.mjs <preflight|validate|publish-plan|stage> [--root REPO] [--space RELEASE_SPACE --confirm RELEASE_ID]");
    process.exitCode=command==="help"?0:2;
  }
}
main().catch(e=>{console.error("[FAIL] "+e.message);process.exitCode=1;});
