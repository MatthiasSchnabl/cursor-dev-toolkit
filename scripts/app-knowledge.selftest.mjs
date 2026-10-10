#!/usr/bin/env node
// Offline, deterministic smoke tests. Run: bun scripts/app-knowledge.selftest.mjs
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import {fileURLToPath} from "node:url";
import crypto from "node:crypto";
import child from "node:child_process";
import assert from "node:assert/strict";

const script=path.join(path.dirname(fileURLToPath(import.meta.url)),"app-knowledge.mjs");
const sha=s=>crypto.createHash("sha256").update(s).digest("hex");
const tmp=fs.mkdtempSync(path.join(os.tmpdir(),"app-knowledge-test-"));
const repo=path.join(tmp,"sample");
fs.mkdirSync(repo,{recursive:true});
function write(p,content) { const target=path.join(repo,p);fs.mkdirSync(path.dirname(target),{recursive:true});fs.writeFileSync(target,content); }
function git(...args) { const r=child.spawnSync("git",["-C",repo,...args],{encoding:"utf8",env:{...process.env,GIT_AUTHOR_NAME:"test",GIT_AUTHOR_EMAIL:"test@example.invalid",GIT_COMMITTER_NAME:"test",GIT_COMMITTER_EMAIL:"test@example.invalid"}});assert.equal(r.status,0,r.stderr); }
function cli(cmd) {return child.spawnSync(process.execPath,[script,cmd,"--root",repo],{encoding:"utf8"});}
try {
  const missing=cli("preflight");
  assert.notEqual(missing.status,0);
  assert.match(missing.stderr,/OPENAPI_REQUIRED/);
  git("init","-q");
  write("README.md","Reference README\n");
  const oas={openapi:"3.1.0",info:{title:"Demo",version:"1.2.3"},paths:{"/v1/items":{get:{operationId:"listItems",responses:{"200":{description:"ok"}}}}}};
  const api=JSON.stringify(oas,null,2)+"\n";
  write("api/openapi.json",api);
  write("app-knowledge.config.json",JSON.stringify({schema_version:1,application_id:"demo",openapi_path:"api/openapi.json",knowledge_dir:"docs/app-knowledge",release_space_prefix:"demo.release.",watch_paths:["api/","src/"]},null,2));
  const kinds=["product","domain","workflows","ui","permissions","api","agent-capabilities"];
  const documents=kinds.map(kind=>{
    const p="docs/app-knowledge/"+kind+"/guide.md",text="# "+kind+"\n\nVerified sample.\n";
    write(p,text);
    return {id:kind,kind,path:p,sha256:sha(text),verification:"verified",sources:["README.md","api/openapi.json"],operation_ids:kind==="api"?["listItems"]:[]};
  });
  write("docs/app-knowledge/evals/golden.json",JSON.stringify({cases:[{question:"How do I list items?"}]}));
  const manifest={schema_version:1,application_id:"demo",app_version:"1.2.3",openapi_sha256:sha(api),documents,gaps:[]};
  write("docs/app-knowledge/manifest.json",JSON.stringify(manifest,null,2)+"\n");
  assert.equal(cli("preflight").status,0);
  assert.equal(cli("validate").status,0);
  git("add","-A");git("commit","-q","-m","init");
  assert.equal(cli("publish-plan").status,0);
  assert.notEqual(cli("stage").status,0,"staging must require explicit fingerprint approval");
  write("docs/app-knowledge/api/guide.md","# changed content\n");
  assert.match(cli("validate").stderr,/DOCUMENT_DRIFT/);
  write("docs/app-knowledge/api/guide.md","# api\n\nVerified sample.\n\n");
  assert.notEqual(cli("validate").status,0,"hash must match exact bytes");
  write("docs/app-knowledge/api/guide.md","# api\n\nVerified sample.\n");
  manifest.documents.find(x=>x.kind==="api").sha256=sha("# api\n\nVerified sample.\n");
  write("docs/app-knowledge/manifest.json",JSON.stringify(manifest,null,2)+"\n");
  assert.equal(cli("validate").status,0);
  assert.equal(cli("install-hook").status,0,"opt-in local hook installation should work");
  assert.notEqual(cli("install-hook").status,0,"existing hook must not be overwritten");
  write("src/main.rs","fn changed() {}\n");git("add","src/main.rs");
  assert.match(cli("diff-check").stderr,/KNOWLEDGE_DRIFT/);
  manifest.maintenance={no_behavior_change:{reason:"Internal-only test fixture change",reviewed_files:["src/main.rs"]}};
  write("docs/app-knowledge/manifest.json",JSON.stringify(manifest,null,2)+"\n");
  git("add","docs/app-knowledge/manifest.json");
  assert.equal(cli("diff-check").status,0);
  console.log("app-knowledge self-test PASS");
} finally {
  fs.rmSync(tmp,{recursive:true,force:true});
}
