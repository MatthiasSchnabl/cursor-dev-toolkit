use clap::{Parser, Subcommand};
use percent_encoding::percent_decode_str;
use pulldown_cmark::{Event, Parser as Markdown, Tag};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io::Write,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};

type Result<T> = std::result::Result<T, String>;
const CONFIG: &str = "docs-guard.toml";

#[derive(Parser)]
#[command(version, about = "Offline Git-snapshot documentation governance")]
struct Cli {
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Action,
}
#[derive(Subcommand)]
enum Action {
    Init,
    Generate,
    Plan,
    Check {
        #[arg(long, conflicts_with = "commit")]
        index: bool,
        #[arg(long)]
        commit: Option<String>,
    },
    Status {
        #[arg(long, conflicts_with = "commit")]
        index: bool,
        #[arg(long)]
        commit: Option<String>,
    },
    Doctor,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    schema_version: u32,
    repository: String,
    profile: String,
    register: String,
    manifest: String,
    #[serde(default)]
    document_roots: Vec<String>,
    #[serde(default)]
    document_extensions: Vec<String>,
    #[serde(default)]
    exclude_document_roots: Vec<String>,
    roles: BTreeMap<String, String>,
    #[serde(default)]
    extra_required: Vec<String>,
    #[serde(default)]
    link_exclusions: Vec<String>,
}

#[derive(Clone, Debug)]
struct Entry {
    mode: String,
    oid: String,
}
impl Entry {
    fn regular(&self) -> bool {
        matches!(self.mode.as_str(), "100644" | "100755")
    }
}
type Inventory = BTreeMap<String, Entry>;
#[derive(Serialize, Debug)]
struct Finding {
    code: &'static str,
    path: String,
    detail: String,
}
#[derive(Serialize)]
struct Report {
    version: &'static str,
    snapshot: String,
    repository: String,
    tracked_files: usize,
    documents: usize,
    findings: Vec<Finding>,
    outcome: &'static str,
}
fn finding(code: &'static str, path: &str, detail: impl Into<String>) -> Finding {
    Finding { code, path: path.into(), detail: detail.into() }
}

fn git(root: &Path, args: &[&str], input: Option<&[u8]>) -> Result<Vec<u8>> {
    let mut command = Command::new("git");
    command.current_dir(root).args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    command.stdin(if input.is_some() { Stdio::piped() } else { Stdio::null() });
    let mut child = command.spawn().map_err(|e| format!("cannot start git: {e}"))?;
    if let Some(bytes) = input {
        let mut stdin = child.stdin.take().ok_or("missing git stdin")?;
        stdin.write_all(bytes).map_err(|e| e.to_string())?;
    }
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!("git {}: {}", args.join(" "), String::from_utf8_lossy(&out.stderr).trim()));
    }
    Ok(out.stdout)
}
fn utf8(bytes: Vec<u8>) -> Result<String> {
    String::from_utf8(bytes).map_err(|e| format!("non-UTF-8 input: {e}"))
}
fn repo_root(path: &Path) -> Result<PathBuf> {
    let s = utf8(git(path, &["rev-parse", "--show-toplevel"], None)?)?;
    fs::canonicalize(s.trim_end_matches(['\r', '\n'])).map_err(|e| e.to_string())
}
fn valid_path(path: &str) -> bool {
    !path.is_empty() && !path.contains(['\\', '\r', '\n', '\t', '\0', ':'])
        && path.split('/').all(|s| !s.is_empty() && s != "." && s != ".." && !s.eq_ignore_ascii_case(".git"))
        && Path::new(path).components().all(|c| matches!(c, Component::Normal(_)))
}
fn validate_path(path: &str) -> Result<()> {
    if valid_path(path) { Ok(()) } else { Err(format!("unsupported or unsafe repository path: {path:?}")) }
}
fn inventory(root: &Path, commit: Option<&str>) -> Result<Inventory> {
    let raw = if let Some(sha) = commit {
        git(root, &["ls-tree", "-r", "-z", sha], None)?
    } else {
        git(root, &["ls-files", "--stage", "-z"], None)?
    };
    let mut entries = BTreeMap::new();
    for record in raw.split(|b| *b == 0).filter(|b| !b.is_empty()) {
        let record = std::str::from_utf8(record).map_err(|_| "non-UTF-8 Git path")?;
        let (meta, path) = record.split_once('\t').ok_or("malformed Git entry")?;
        validate_path(path)?;
        let fields: Vec<_> = meta.split(' ').collect();
        if fields.len() != 3 { return Err("malformed Git metadata".into()); }
        let oid = if commit.is_some() { fields[2] } else {
            if fields[2] != "0" { return Err(format!("unmerged index entry: {path}")); }
            fields[1]
        };
        if !matches!(oid.len(), 40 | 64) || !oid.bytes().all(|b| b.is_ascii_hexdigit()) || oid.bytes().all(|b| b == b'0') {
            return Err(format!("invalid object ID: {path}"));
        }
        if entries.insert(path.into(), Entry { mode: fields[0].into(), oid: oid.into() }).is_some() {
            return Err(format!("duplicate entry: {path}"));
        }
    }
    Ok(entries)
}
fn blob(root: &Path, inv: &Inventory, path: &str) -> Result<Vec<u8>> {
    let entry = inv.get(path).filter(|e| e.regular()).ok_or_else(|| format!("missing regular tracked file: {path}"))?;
    git(root, &["cat-file", "blob", &entry.oid], None)
}
fn within(path: &str, prefix: &str) -> bool {
    path == prefix || path.strip_prefix(prefix).is_some_and(|rest| rest.starts_with('/'))
}
impl Config {
    fn required_roles(&self) -> Result<Vec<&'static str>> {
        let mut roles = vec!["scope", "agent_rules", "index", "status"];
        match self.profile.as_str() {
            "minimal" => (),
            "standard" | "production" | "scientific" => roles.extend(["architecture", "decisions", "verification", "handoff", "history"]),
            _ => return Err(format!("unknown profile: {}", self.profile)),
        }
        if self.profile == "production" { roles.push("operations"); }
        if self.profile == "scientific" { roles.push("scientific_status"); }
        Ok(roles)
    }
    fn validate(&self) -> Result<()> {
        if self.schema_version != 1 || self.repository.trim().is_empty() { return Err("unsupported schema or missing repository identity".into()); }
        if self.register == self.manifest || self.register == CONFIG || self.manifest == CONFIG {
            return Err("generated paths must be distinct from one another and config".into());
        }
        for p in [&self.register, &self.manifest].into_iter().chain(self.document_roots.iter()).chain(self.exclude_document_roots.iter()).chain(self.roles.values()).chain(self.extra_required.iter()).chain(self.link_exclusions.iter()) {
            validate_path(p)?;
        }
        if self.document_roots.is_empty() || self.document_extensions.is_empty() ||
            self.document_extensions.iter().any(|e| e.is_empty() || !e.bytes().all(|b| b.is_ascii_alphanumeric())) {
            return Err("document roots and alphanumeric extensions required".into());
        }
        for role in self.required_roles()? {
            if !self.roles.contains_key(role) { return Err(format!("profile requires role: {role}")); }
        }
        if self.roles.values().chain(self.extra_required.iter()).any(|p| p == &self.register || p == &self.manifest) {
            return Err("generated files cannot fulfill curated documentation requirements".into());
        }
        Ok(())
    }
    fn documents(&self, inv: &Inventory) -> Vec<String> {
        inv.iter().filter(|(p,e)| {
            e.regular() && *p != &self.register && *p != &self.manifest
                && !self.exclude_document_roots.iter().any(|r| within(p,r))
                && (self.roles.values().any(|r| r == *p) || self.extra_required.contains(p)
                || self.document_roots.iter().any(|r| within(p,r)) && p.rsplit_once('.').is_some_and(|(_, ext)| self.document_extensions.iter().any(|e| e.eq_ignore_ascii_case(ext))))
        }).map(|(p,_)| p.clone()).collect()
    }
}

fn relative(from: &str, to: &str) -> String {
    let mut a: Vec<_> = from.split('/').collect();
    a.pop();
    let b: Vec<_> = to.split('/').collect();
    let common = a.iter().zip(&b).take_while(|(x,y)| x == y).count();
    let mut parts = vec![".."; a.len() - common];
    parts.extend_from_slice(&b[common..]);
    parts.join("/")
}
fn href(path: &str) -> String {
    let mut result = String::new();
    for b in path.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~/".contains(&b) { result.push(char::from(b)); }
        else { result.push_str(&format!("%{b:02X}")); }
    }
    result
}
fn register(config: &Config, docs: &[String]) -> Vec<u8> {
    let mut groups: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for p in docs {
        let key = p.rsplit_once('/').map(|(dir,_)| dir).unwrap_or("(root)");
        groups.entry(key.into()).or_default().push(p);
    }
    let mut out = format!("# Document register\n\nGenerated from Git. Presence is not semantic approval.\n\nDocuments: {}\n\n", docs.len());
    for (group, paths) in groups {
        out.push_str(&format!("## {}\n\n", group.replace(['<','>'], "_")));
        for p in paths {
            let label = p.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('[', "\\[").replace(']', "\\]");
            out.push_str(&format!("- [{label}]({})\n", href(&relative(&config.register,p))));
        }
        out.push('\n');
    }
    out.into_bytes()
}
fn manifest(config: &Config, inv: &Inventory) -> Vec<u8> {
    let mut out = String::from("# docs-guard Git inventory v1; manifest itself excluded\n# path\tmode\tobject_id\n");
    for (p,e) in inv {
        if p != &config.manifest { out.push_str(&format!("{p}\t{}\t{}\n", e.mode,e.oid)); }
    }
    out.into_bytes()
}
fn resolve_link(source: &str, target: &str) -> Result<Option<String>> {
    if target.is_empty() || target.starts_with('#') || target.starts_with("//") { return Ok(None); }
    let before_fragment = target.split(['#','?']).next().unwrap_or("");
    if before_fragment.is_empty() { return Ok(None); }
    if before_fragment.split('/').next().is_some_and(|s| s.contains(':')) {
        if before_fragment.starts_with("file:") { return Err("file URI is not repository-relative".into()); }
        return Ok(None);
    }
    let decoded = percent_decode_str(before_fragment).decode_utf8().map_err(|_| "invalid link encoding")?;
    if decoded.contains(['\\', '\0', ':']) { return Err("unsafe link path".into()); }
    let mut parts: Vec<&str> = if decoded.starts_with('/') { Vec::new() } else {
        source.split('/').collect::<Vec<_>>()[..source.split('/').count()-1].to_vec()
    };
    for segment in decoded.split('/') {
        match segment {
            "" | "." => (),
            ".." => { if parts.pop().is_none() { return Err("link escapes repository".into()); } },
            _ => parts.push(segment),
        }
    }
    let path = parts.join("/");
    if path.is_empty() { return Ok(Some(path)); }
    validate_path(&path)?;
    Ok(Some(path))
}
fn audit_links(root: &Path, inv: &Inventory, config: &Config, docs: &[String], findings: &mut Vec<Finding>) -> Result<()> {
    for path in docs {
        if !path.to_ascii_lowercase().ends_with(".md") || config.link_exclusions.iter().any(|p| within(path,p)) { continue; }
        let content = utf8(blob(root,inv,path)?)?;
        for (event, range) in Markdown::new(&content).into_offset_iter() {
            if let Event::Start(Tag::Link {dest_url,..} | Tag::Image {dest_url,..}) = event {
                match resolve_link(path,&dest_url) {
                    Ok(None) => (),
                    Ok(Some(p)) if p.is_empty() || inv.contains_key(&p) || inv.keys().any(|key| within(key,&p)) => (),
                    Ok(Some(p)) => findings.push(finding("BROKEN_LINK",path,format!("byte {}: {dest_url} -> {p}",range.start))),
                    Err(e) => findings.push(finding("UNSAFE_LINK",path,format!("byte {}: {dest_url}: {e}",range.start))),
                }
            }
        }
    }
    Ok(())
}
fn safe_output(root: &Path, relative: &str) -> Result<PathBuf> {
    validate_path(relative)?;
    let mut path = root.to_path_buf();
    let pieces: Vec<_> = relative.split('/').collect();
    for (i, part) in pieces.iter().enumerate() {
        path.push(part);
        match fs::symlink_metadata(&path) {
            Ok(m) if m.file_type().is_symlink() => return Err(format!("symlink output refused: {}",path.display())),
            Ok(m) if i+1 < pieces.len() && !m.is_dir() => return Err(format!("not a directory: {}",path.display())),
            Ok(m) if i+1 == pieces.len() && !m.is_file() => return Err(format!("not a regular file: {}",path.display())),
            Ok(_) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(path)
}
fn write_output(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::create_dir_all(path.parent().ok_or("missing parent")?).map_err(|e| e.to_string())?;
    fs::write(path,bytes).map_err(|e| e.to_string())
}

fn evaluate(root: &Path, commit: Option<&str>, generate: bool, plan: bool) -> Result<Report> {
    let mut inv = inventory(root,commit)?;
    let config: Config = toml::from_str(&utf8(blob(root,&inv,CONFIG)?)?).map_err(|e| format!("invalid config: {e}"))?;
    config.validate()?;
    let mut findings = Vec::new();
    let required: BTreeSet<_> = config.roles.values().chain(config.extra_required.iter()).collect();
    for p in required {
        if !inv.get(p).is_some_and(Entry::regular) {
            findings.push(finding("MISSING_REQUIRED",p,"missing regular tracked file"));
        }
    }
    let docs = config.documents(&inv);
    let reg = register(&config,&docs);
    let reg_oid = utf8(git(root,&["hash-object","--stdin"],Some(&reg))?)?.trim().to_string();
    let reg_mode = inv.get(&config.register).map(|e| e.mode.clone()).unwrap_or_else(|| "100644".into());
    if reg_mode != "100644" { findings.push(finding("OUTPUT_MODE",&config.register,"generated register must be mode 100644")); }
    if inv.get(&config.manifest).is_some_and(|e| e.mode != "100644") {
        findings.push(finding("OUTPUT_MODE",&config.manifest,"generated manifest must be mode 100644"));
    }
    let mut expected = inv.clone();
    expected.insert(config.register.clone(),Entry {mode:"100644".into(),oid:reg_oid});
    let man = manifest(&config,&expected);
    for (p, bytes) in [(&config.register,&reg),(&config.manifest,&man)] {
        if !generate && !plan && !inv.get(p).is_some_and(Entry::regular) {
            findings.push(finding("MISSING_GENERATED",p,"run generate and stage output"));
        } else if !generate && !plan && blob(root,&inv,p)? != *bytes {
            findings.push(finding("STALE_GENERATED",p,"run generate and stage output"));
        }
    }
    // New generated outputs count as valid link targets in generation planning.
    if generate || plan {
        inv.entry(config.register.clone()).or_insert(Entry {mode:"100644".into(),oid:String::new()});
        inv.entry(config.manifest.clone()).or_insert(Entry {mode:"100644".into(),oid:String::new()});
    }
    audit_links(root,&inv,&config,&docs,&mut findings)?;
    if generate || plan {
        let targets = [(&config.register,&reg),(&config.manifest,&man)];
        for (p, bytes) in &targets {
            let target = safe_output(root,p)?;
            if target.exists() {
                let existing = fs::read(&target).map_err(|e| e.to_string())?;
                // Refuse to overwrite edits or pre-existing untracked files.
                if existing != **bytes && (!inv.get(*p).is_some_and(|e| e.regular() && !e.oid.is_empty()) || blob(root,&inv,p)? != existing) {
                    findings.push(finding("OUTPUT_DIRTY",p,"stage/resolve existing output edits before regeneration"));
                }
            }
        }
        if generate && findings.is_empty() {
            for (p,bytes) in targets { write_output(&safe_output(root,p)?,bytes)?; }
        }
    }
    Ok(Report {
        version: env!("CARGO_PKG_VERSION"),
        snapshot: commit.unwrap_or("INDEX").into(),
        repository: config.repository,
        tracked_files: inventory(root,commit)?.len(),
        documents: docs.len(),
        outcome: if !findings.is_empty() {"FAIL"} else if generate {"GENERATED_NOT_CHECKED"} else if plan {"PLAN_READY"} else {"PASS"},
        findings,
    })
}
fn run(cli: &Cli) -> Result<i32> {
    let root = repo_root(&cli.root)?;
    let report = match &cli.command {
        Action::Init => {
            let path = safe_output(&root,CONFIG)?;
            if path.exists() { return Err("configuration already exists; refusing overwrite".into()); }
            let mut file = fs::OpenOptions::new().write(true).create_new(true).open(path).map_err(|e| e.to_string())?;
            file.write_all(include_bytes!("../examples/docs-guard.toml")).map_err(|e| e.to_string())?;
            if cli.json { println!("{}",serde_json::json!({"outcome":"CONFIG_CREATED_NOT_CHECKED","path":CONFIG})); }
            else { println!("Created {CONFIG}; adapt identity and roles, create missing curated docs, stage inputs."); }
            return Ok(0);
        },
        Action::Doctor => {
            let version = utf8(git(&root,&["--version"],None)?)?;
            if cli.json { println!("{}",serde_json::json!({"outcome":"DIAGNOSTIC","git":version.trim(),"root":root,"version":env!("CARGO_PKG_VERSION")})); }
            else { println!("docs-guard {} | {} | {}",env!("CARGO_PKG_VERSION"),version.trim(),root.display()); }
            return Ok(0);
        },
        Action::Generate => evaluate(&root,None,true,false)?,
        Action::Plan => evaluate(&root,None,false,true)?,
        Action::Check {index,commit} | Action::Status {index,commit} => {
            let sha = if *index { None } else {
                let spec = format!("{}^{{commit}}",commit.as_deref().unwrap_or("HEAD"));
                Some(utf8(git(&root,&["rev-parse","--verify","--end-of-options",&spec],None)?)?.trim().to_owned())
            };
            evaluate(&root,sha.as_deref(),false,false)?
        }
    };
    if cli.json { println!("{}",serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?); }
    else {
        println!("DOCS_GUARD_{} snapshot={} tracked={} documents={}",report.outcome,report.snapshot,report.tracked_files,report.documents);
        for f in &report.findings { println!("{} {}: {}",f.code,f.path,f.detail); }
    }
    Ok(if report.findings.is_empty() {0} else {1})
}
fn main() {
    let cli = Cli::parse();
    let code = match run(&cli) {
        Ok(code) => code,
        Err(e) => {
            if cli.json { println!("{}",serde_json::json!({"outcome":"ERROR","detail":e})); }
            else { eprintln!("DOCS_GUARD_ERROR: {e}"); }
            2
        }
    };
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relative_paths() {
        assert_eq!(relative("docs/inventory/register.md","README.md"),"../../README.md");
        assert_eq!(href("../a b(1).md"),"../a%20b%281%29.md");
        assert_eq!(resolve_link("docs/a.md","../README.md#heading").unwrap(),Some("README.md".into()));
        assert!(resolve_link("README.md","../outside").is_err());
        assert!(resolve_link("README.md","%2e%2e/outside").is_err());
        assert_eq!(resolve_link("README.md","https://example.org").unwrap(),None);
    }
    #[test]
    fn unsafe_paths() {
        for p in ["", "../x","a//b","a/./b",".git/config","a\\b","a\tb","C:/x"] { assert!(!valid_path(p)); }
        assert!(valid_path("docs/space and ü.md"));
        assert!(!within("docs-old/a.md","docs"));
    }
}
