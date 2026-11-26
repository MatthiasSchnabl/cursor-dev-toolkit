use clap::{Parser, Subcommand};
use percent_encoding::percent_decode_str;
use pulldown_cmark::{Event, Parser as Markdown, Tag};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io::{Read, Write},
    path::{Component, Path, PathBuf},
    process::ExitCode,
};

mod error;
mod git_process;
use error::{Code, GuardError, Result};
use git_process::git;
const CONFIG: &str = "docs-guard.toml";
const MAX_CONFIG: usize = 256 * 1024;
const MAX_POLICY_ITEMS: usize = 2048;
const MAX_MARKDOWN: usize = 4 * 1024 * 1024;
const MAX_FINDINGS: usize = 10_000;
const MAX_OUTPUT_FILE: usize = 32 * 1024 * 1024;

#[derive(Parser)]
#[command(version, about = "Offline Git-snapshot documentation governance")]
struct Cli {
    /// Repository root or a directory inside it.
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,
    /// Emit a single versioned JSON result instead of human output.
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Action,
}
#[derive(Subcommand)]
enum Action {
    /// Create a configuration template without overwriting existing files.
    Init,
    /// Regenerate derived files from the staged Git index; never stage them.
    Generate {
        /// Adopt a reviewed legacy register at this exact staged Git object ID.
        #[arg(long, value_name = "OID")]
        adopt_register: Option<String>,
        /// Adopt a reviewed legacy manifest at this exact staged Git object ID.
        #[arg(long, value_name = "OID")]
        adopt_manifest: Option<String>,
    },
    /// Preview generation without writing files.
    Plan {
        #[arg(long, value_name = "OID")]
        adopt_register: Option<String>,
        #[arg(long, value_name = "OID")]
        adopt_manifest: Option<String>,
    },
    /// Check an immutable commit (HEAD by default) or the staged index.
    Check {
        /// Check staged content rather than a commit.
        #[arg(long, conflicts_with = "commit")]
        index: bool,
        /// Resolve and check this commit or ref.
        #[arg(long)]
        commit: Option<String>,
        /// Trusted comparison base (required for configured change rules).
        #[arg(long)]
        base: Option<String>,
    },
    /// Report documentation findings for the selected snapshot.
    Status {
        #[arg(long, conflicts_with = "commit")]
        index: bool,
        #[arg(long)]
        commit: Option<String>,
        #[arg(long)]
        base: Option<String>,
    },
    /// Report the installed CLI and Git environment.
    Doctor,
}
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Profile { Minimal, Standard, Production, Scientific }

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    schema_version: u32,
    repository: String,
    profile: Profile,
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
    optional_documents: Vec<String>,
    #[serde(default)]
    link_exclusions: Vec<String>,
    #[serde(default)]
    change_rules: Vec<ChangeRule>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChangeRule {
    id: String,
    watch: Vec<String>,
    require: Vec<String>,
    review_document: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    schema_version: u32,
    version: &'static str,
    snapshot: String,
    comparison_base: Option<String>,
    change_rules_checked: bool,
    repository: String,
    tracked_files: usize,
    documents: usize,
    findings: Vec<Finding>,
    changed_outputs: Vec<String>,
    legacy_adoptions: Vec<String>,
    outcome: &'static str,
}
fn finding(code: &'static str, path: &str, detail: impl Into<String>) -> Finding {
    let detail = detail.into();
    // Bound human and machine diagnostic size even for hostile Markdown URLs.
    let mut chars = detail.chars();
    let mut bounded: String = chars.by_ref().take(2048).collect();
    if chars.next().is_some() { bounded.push_str(" [diagnostic shortened]"); }
    Finding { code, path: path.into(), detail: bounded }
}

fn utf8(bytes: Vec<u8>) -> Result<String> {
    String::from_utf8(bytes).map_err(|e| GuardError::new(Code::InvalidEncoding,"non-UTF-8 input").caused_by(e))
}
fn repo_root(path: &Path) -> Result<PathBuf> {
    let s = utf8(git(path, &["rev-parse", "--show-toplevel"], None)?)?;
    fs::canonicalize(s.trim_end_matches(['\r', '\n'])).map_err(|e| GuardError::io(Code::FileRead,"canonicalize repository",Some(path),e))
}
fn valid_path(path: &str) -> bool {
    !path.is_empty() && !path.contains(['\\', ':']) && !path.chars().any(char::is_control)
        && path.split('/').all(|s| !s.is_empty() && s != "." && s != ".." && !s.eq_ignore_ascii_case(".git"))
        && Path::new(path).components().all(|c| matches!(c, Component::Normal(_)))
}
fn validate_path(path: &str) -> Result<()> {
    if valid_path(path) { Ok(()) } else { Err(GuardError::new(Code::InvalidPath,format!("unsupported or unsafe repository path: {path:?}"))) }
}
fn inventory(root: &Path, commit: Option<&str>) -> Result<Inventory> {
    let raw = if let Some(sha) = commit {
        git(root, &["ls-tree", "-r", "-z", sha], None)?
    } else {
        git(root, &["ls-files", "--stage", "-z"], None)?
    };
    let mut entries = BTreeMap::new();
    for record in raw.split(|b| *b == 0).filter(|b| !b.is_empty()) {
        let record = std::str::from_utf8(record).map_err(|e| GuardError::new(Code::InvalidEncoding,"non-UTF-8 Git path").caused_by(e))?;
        let (meta, path) = record.split_once('\t').ok_or_else(|| GuardError::new(Code::InvalidSnapshot,"malformed Git entry"))?;
        validate_path(path)?;
        let fields: Vec<_> = meta.split(' ').collect();
        if fields.len() != 3 { return Err(GuardError::new(Code::InvalidSnapshot,"malformed Git metadata")); }
        let oid = if commit.is_some() { fields[2] } else {
            if fields[2] != "0" { return Err(GuardError::new(Code::InvalidSnapshot,format!("unmerged index entry: {path}"))); }
            fields[1]
        };
        if !matches!(oid.len(), 40 | 64) || !oid.bytes().all(|b| b.is_ascii_hexdigit()) || oid.bytes().all(|b| b == b'0') {
            return Err(GuardError::new(Code::InvalidSnapshot,format!("invalid object ID: {path}")));
        }
        if !matches!(fields[0],"100644" | "100755" | "120000" | "160000") {
            return Err(GuardError::new(Code::InvalidSnapshot,format!("unsupported Git entry mode: {path}: {}",fields[0])));
        }
        if entries.insert(path.into(), Entry { mode: fields[0].into(), oid: oid.into() }).is_some() {
            return Err(GuardError::new(Code::InvalidSnapshot,format!("duplicate entry: {path}")));
        }
    }
    Ok(entries)
}
fn blob(root: &Path, inv: &Inventory, path: &str) -> Result<Vec<u8>> {
    let entry = inv.get(path).filter(|e| e.regular()).ok_or_else(|| GuardError::new(Code::InvalidSnapshot,format!("missing regular tracked file: {path}")))?;
    git(root, &["cat-file", "blob", &entry.oid], None).map_err(|e| e.at(path))
}
fn within(path: &str, prefix: &str) -> bool {
    prefix == "." || path == prefix || path.strip_prefix(prefix).is_some_and(|rest| rest.starts_with('/'))
}
impl Config {
    fn required_roles(&self) -> Vec<&'static str> {
        let mut roles = vec!["scope", "agent_rules", "index", "status"];
        match self.profile {
            Profile::Minimal => (),
            Profile::Standard | Profile::Production | Profile::Scientific => roles.extend(["architecture", "decisions", "verification", "handoff", "history"]),
        }
        if matches!(self.profile, Profile::Production) { roles.push("operations"); }
        if matches!(self.profile, Profile::Scientific) { roles.push("scientific_status"); }
        roles
    }
    fn validate(&self) -> Result<()> {
        if self.schema_version != 1 || (self.repository.trim().is_empty() || self.repository == "replace-with-project-id") { return Err(GuardError::new(Code::ConfigInvalid,"unsupported schema or missing repository identity")); }
        let reg = self.register.to_lowercase();
        let man = self.manifest.to_lowercase();
        if within(&reg,&man) || within(&man,&reg) || within(&reg,CONFIG) || within(&man,CONFIG) {
            return Err(GuardError::new(Code::ConfigInvalid,"generated paths must be distinct from one another and config"));
        }
        for p in [&self.register, &self.manifest].into_iter().chain(self.exclude_document_roots.iter()).chain(self.roles.values()).chain(self.extra_required.iter()).chain(self.optional_documents.iter()).chain(self.link_exclusions.iter()) {
            validate_path(p)?;
        }
        for p in [&self.register,&self.manifest] {
            if !p.is_ascii() || !p.split('/').all(portable_output_component) {
                return Err(GuardError::new(Code::ConfigInvalid,format!("generated paths require portable ASCII components: {p}")));
            }
        }
        let policy_items = self.document_roots.len() + self.document_extensions.len() + self.exclude_document_roots.len() +
            self.roles.len() + self.extra_required.len() + self.optional_documents.len() + self.link_exclusions.len() +
            self.change_rules.iter().map(|r| 1 + r.watch.len() + r.require.len() + (if r.review_document.is_some() { 1 } else { 0 })).sum::<usize>();
        if policy_items > MAX_POLICY_ITEMS { return Err(GuardError::new(Code::ResourceLimit,"configuration policy item limit exceeded")); }
        for p in &self.document_roots {
            if p != "." { validate_path(p)?; }
        }
        if self.roles.values().chain(self.extra_required.iter()).any(|p| self.exclude_document_roots.iter().any(|r| within(p,r)) || self.link_exclusions.iter().any(|r| within(p,r))) {
            return Err(GuardError::new(Code::ConfigInvalid,"required documentation cannot be excluded from the document register or link audit"));
        }
        if self.document_roots.is_empty() || self.document_extensions.is_empty() ||
            self.document_extensions.iter().any(|e| e.is_empty() || !e.bytes().all(|b| b.is_ascii_alphanumeric())) {
            return Err(GuardError::new(Code::ConfigInvalid,"document roots and alphanumeric extensions required"));
        }
        for role in self.required_roles() {
            if !self.roles.contains_key(role) { return Err(GuardError::new(Code::ConfigInvalid,format!("profile requires role: {role}"))); }
        }
        if self.roles.values().chain(self.extra_required.iter()).chain(self.optional_documents.iter()).any(|p| p.to_lowercase() == reg || p.to_lowercase() == man || p.eq_ignore_ascii_case(CONFIG)) {
            return Err(GuardError::new(Code::ConfigInvalid,"generated files cannot fulfill curated documentation requirements"));
        }
        let mut ids = BTreeSet::new();
        for rule in &self.change_rules {
            if rule.id.trim().is_empty() || !ids.insert(&rule.id) || rule.watch.is_empty() || rule.require.is_empty() {
                return Err(GuardError::new(Code::ConfigInvalid,"change rules require a unique nonempty ID, watch and require paths"));
            }
            for p in &rule.watch {
                validate_path(p)?;
                if within(&self.register,p) || within(&self.manifest,p) || within(CONFIG,p) {
                    return Err(GuardError::new(Code::ConfigInvalid,format!("change rule {} watches generated files or configuration",rule.id)));
                }
            }
            for p in rule.require.iter().chain(rule.review_document.iter()) {
                validate_path(p)?;
                if p == &self.register || p == &self.manifest || p == CONFIG ||
                    rule.watch.iter().any(|prefix| within(p,prefix)) ||
                    self.exclude_document_roots.iter().any(|prefix| within(p,prefix)) ||
                    self.link_exclusions.iter().any(|prefix| within(p,prefix)) ||
                    !self.roles.values().chain(self.extra_required.iter()).chain(self.optional_documents.iter()).any(|d| d == p) {
                    return Err(GuardError::new(Code::ConfigInvalid,format!("change rule {} targets must be declared curated documents outside watched paths",rule.id)));
                }
            }
        }
        Ok(())
    }
    fn documents(&self, inv: &Inventory) -> Vec<String> {
        inv.iter().filter(|(p,e)| {
            e.regular() && *p != &self.register && *p != &self.manifest
                && !self.exclude_document_roots.iter().any(|r| within(p,r))
                && (self.roles.values().any(|r| r == *p) || self.extra_required.contains(p) || self.optional_documents.contains(p)
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
fn markdown_label(value: &str) -> String {
    let mut out = String::new();
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '[' | ']' | '*' | '_' | '~' | '!' | '#' | '\\' => { out.push('\\'); out.push(c); }
            '\u{0060}' => { out.push('\\'); out.push(c); }
            _ => out.push(c),
        }
    }
    out
}
fn register(config: &Config, docs: &[String]) -> Vec<u8> {
    let mut groups: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for p in docs {
        let key = p.rsplit_once('/').map(|(dir,_)| dir).unwrap_or("(root)");
        groups.entry(key.into()).or_default().push(p);
    }
    let mut out = format!("# Document register\n\nGenerated from Git. Presence is not semantic approval.\n\nDocuments: {}\n\n", docs.len());
    for (group, paths) in groups {
        out.push_str(&format!("## {}\n\n", markdown_label(&group)));
        for p in paths {
            let label = markdown_label(p);
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
fn parse_manifest(bytes: &[u8]) -> Option<Inventory> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut lines = text.lines();
    if lines.next()? != "# docs-guard Git inventory v1; manifest itself excluded" ||
        lines.next()? != "# path\tmode\tobject_id" { return None; }
    let mut entries = BTreeMap::new();
    for line in lines {
        let mut columns = line.split('\t');
        let path = columns.next()?;
        let mode = columns.next()?;
        let oid = columns.next()?;
        if columns.next().is_some() || !valid_path(path) ||
            !matches!(mode,"100644" | "100755" | "120000" | "160000") ||
            !matches!(oid.len(),40 | 64) || !oid.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        if entries.insert(path.into(),Entry {mode:mode.into(),oid:oid.into()}).is_some() { return None; }
    }
    Some(entries)
}
fn manifest_drift(previous: &[u8], desired: &[u8]) -> String {
    let (Some(old),Some(new)) = (parse_manifest(previous),parse_manifest(desired)) else {
        return "manifest format differs; review migration or regenerate and stage output".into();
    };
    let added: Vec<_> = new.keys().filter(|p| !old.contains_key(*p)).collect();
    let removed: Vec<_> = old.keys().filter(|p| !new.contains_key(*p)).collect();
    let changed: Vec<_> = new.iter().filter(|(p,e)| old.get(*p).is_some_and(|prior| prior != *e)).map(|(p,_)| p).collect();
    let sample = |paths: &[&String]| paths.iter().take(5).map(|p| p.as_str()).collect::<Vec<_>>().join(", ");
    format!("added={} [{}]; removed={} [{}]; changed={} [{}]; regenerate and stage output",added.len(),sample(&added),removed.len(),sample(&removed),changed.len(),sample(&changed))
}
fn resolve_link(source: &str, target: &str) -> Result<Option<String>> {
    if target.is_empty() || target.starts_with('#') || target.starts_with("//") { return Ok(None); }
    let before_fragment = target.split(['#','?']).next().unwrap_or("");
    if before_fragment.is_empty() { return Ok(None); }
    if before_fragment.split('/').next().is_some_and(|s| s.contains(':')) {
        if before_fragment.to_ascii_lowercase().starts_with("file:") || before_fragment.as_bytes().get(1) == Some(&b':') { return Err(GuardError::new(Code::InvalidLink,"file URI is not repository-relative")); }
        return Ok(None);
    }
    let decoded = percent_decode_str(before_fragment).decode_utf8().map_err(|e| GuardError::new(Code::InvalidLink,"invalid link encoding").caused_by(e))?;
    if decoded.contains(['\\', '\0', ':']) { return Err(GuardError::new(Code::InvalidLink,"unsafe link path")); }
    let mut parts: Vec<&str> = if decoded.starts_with('/') { Vec::new() } else {
        source.rsplit_once('/').map(|(dir,_)| dir.split('/').collect()).unwrap_or_default()
    };
    for segment in decoded.split('/') {
        match segment {
            "" | "." => (),
            ".." => { if parts.pop().is_none() { return Err(GuardError::new(Code::InvalidLink,"link escapes repository")); } },
            _ => parts.push(segment),
        }
    }
    let path = parts.join("/");
    if path.is_empty() { return Ok(Some(path)); }
    validate_path(&path)?;
    Ok(Some(path))
}
fn audit_links(root: &Path, inv: &Inventory, config: &Config, docs: &[String], findings: &mut Vec<Finding>) -> Result<()> {
    let mut directories = BTreeSet::new();
    for key in inv.keys() {
        for (i,_) in key.match_indices('/') { directories.insert(&key[..i]); }
    }
    for path in docs {
        if !path.to_ascii_lowercase().ends_with(".md") || config.link_exclusions.iter().any(|p| within(path,p)) { continue; }
        let bytes = blob(root,inv,path)?;
        if bytes.len() > MAX_MARKDOWN { return Err(GuardError::new(Code::ResourceLimit,format!("Markdown exceeds audit size limit: {path}"))); }
        let content = utf8(bytes)?;
        let newlines: Vec<_> = content.match_indices('\n').map(|(i,_)| i).collect();
        for (event, range) in Markdown::new(&content).into_offset_iter() {
            if let Event::Start(Tag::Link {dest_url,..} | Tag::Image {dest_url,..}) = event {
                let line = newlines.partition_point(|i| *i < range.start) + 1;
                match resolve_link(path,&dest_url) {
                    Ok(None) => (),
                    Ok(Some(p)) if p.is_empty() || inv.get(&p).is_some_and(Entry::regular) || (!inv.contains_key(&p) && directories.contains(p.as_str())) => (),
                    Ok(Some(p)) if inv.contains_key(&p) => findings.push(finding("UNSUPPORTED_LINK_TARGET",path,format!("line {line}: {p} (symlink or submodule)"))),
                    Ok(Some(p)) => findings.push(finding("BROKEN_LINK",path,format!("line {line}: {p}"))),
                    Err(e) => findings.push(finding("UNSAFE_LINK",path,format!("line {line}: {e}"))),
                }
                if findings.len() > MAX_FINDINGS { return Err(GuardError::new(Code::ResourceLimit,"finding limit exceeded; narrow audit scope or resolve existing errors")); }
            }
        }
    }
    Ok(())
}
fn portable_output_component(part: &str) -> bool {
    if part.ends_with([' ','.']) || part.contains(['<','>','"','|','?','*']) { return false; }
    let stem = part.split('.').next().unwrap_or("").to_ascii_uppercase();
    !matches!(stem.as_str(),"CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$") &&
        !((stem.starts_with("COM") || stem.starts_with("LPT")) &&
            stem.chars().count() == 4 && stem.chars().last().is_some_and(|c| matches!(c,'1'..='9' | '¹' | '²' | '³')))
}
fn redirected_file(metadata: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        // Includes junctions and other reparse points, not only symlinks.
        metadata.file_type().is_symlink() || metadata.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    { metadata.file_type().is_symlink() }
}
fn read_worktree_bounded(path: &Path) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    fs::File::open(path).map_err(|e| GuardError::io(Code::FileRead,"open output",Some(path),e))?.take((MAX_OUTPUT_FILE + 1) as u64)
        .read_to_end(&mut bytes).map_err(|e| GuardError::io(Code::FileRead,"read output",Some(path),e))?;
    if bytes.len() > MAX_OUTPUT_FILE { return Err(GuardError::new(Code::ResourceLimit,format!("output exceeds size limit: {}",path.display()))); }
    Ok(bytes)
}
fn safe_output(root: &Path, relative: &str) -> Result<PathBuf> {
    validate_path(relative)?;
    let mut path = root.to_path_buf();
    let pieces: Vec<_> = relative.split('/').collect();
    for (i, part) in pieces.iter().enumerate() {
        if !portable_output_component(part) { return Err(GuardError::new(Code::OutputScope,format!("nonportable output path: {relative}"))); }
        if path.is_dir() {
            for entry in fs::read_dir(&path).map_err(|e| GuardError::io(Code::FileRead,"read output directory",Some(&path),e))? {
                let name = entry.map_err(|e| GuardError::io(Code::FileRead,"read directory entry",Some(&path),e))?.file_name();
                let name = name.to_string_lossy();
                if name.to_lowercase() == part.to_lowercase() && name.as_ref() != *part {
                    return Err(GuardError::new(Code::OutputScope,format!("case-alias output path refused: {relative}")));
                }
            }
        }
        path.push(part);
        match fs::symlink_metadata(&path) {
            Ok(m) if redirected_file(&m) => return Err(GuardError::new(Code::OutputScope,format!("redirected output path refused: {}",path.display()))),
            Ok(m) if i+1 < pieces.len() && !m.is_dir() => return Err(GuardError::new(Code::OutputScope,format!("not a directory: {}",path.display()))),
            Ok(m) if i+1 == pieces.len() && !m.is_file() => return Err(GuardError::new(Code::OutputScope,format!("not a regular file: {}",path.display()))),
            Ok(m) => {
                if m.is_dir() && i+1 < pieces.len() {
                    match fs::symlink_metadata(path.join(".git")) {
                        Ok(_) => return Err(GuardError::new(Code::OutputScope,format!("nested repository output refused: {relative}"))),
                        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                        Err(e) => return Err(GuardError::io(Code::FileRead,"inspect output path",Some(&path),e)),
                    }
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(GuardError::io(Code::FileRead,"inspect output path",Some(&path),e)),
        }
    }
    Ok(path)
}
fn validate_output_scope(config: &Config, inv: &Inventory) -> Result<()> {
    for target in [&config.register,&config.manifest] {
        let folded = target.to_lowercase();
        for (path,entry) in inv {
            let other = path.to_lowercase();
            for (a,b) in target.split('/').zip(path.split('/')) {
                if a.to_lowercase() != b.to_lowercase() { break; }
                if a != b { return Err(GuardError::new(Code::OutputScope,format!("generated path has a case-aliased tracked component: {target} / {path}"))); }
            }
            if path == target {
                if !entry.regular() { return Err(GuardError::new(Code::OutputScope,format!("nonregular tracked output: {target}"))); }
            } else if within(&folded,&other) || within(&other,&folded) {
                return Err(GuardError::new(Code::OutputScope,format!("generated path collides with tracked path: {target} / {path}")));
            }
        }
    }
    Ok(())
}
fn write_output(path: &Path, bytes: &[u8], existing: bool) -> Result<()> {
    let parent = path.parent().ok_or_else(|| GuardError::new(Code::FileWrite,"missing parent"))?;
    fs::create_dir_all(parent).map_err(|e| GuardError::io(Code::FileWrite,"create output directory",Some(parent),e))?;
    // Same-directory temporary file: no truncation of the existing target on write failure.
    let mut file = tempfile::NamedTempFile::new_in(parent).map_err(|e| GuardError::io(Code::FileWrite,"create temporary output",Some(path),e))?;
    file.write_all(bytes).map_err(|e| GuardError::io(Code::FileWrite,"write temporary output",Some(path),e))?;
    if existing {
        file.as_file().set_permissions(fs::metadata(path).map_err(|e| GuardError::io(Code::FileRead,"read output permissions",Some(path),e))?.permissions()).map_err(|e| GuardError::io(Code::FileWrite,"set output permissions",Some(path),e))?;
    }
    file.as_file().sync_all().map_err(|e| GuardError::io(Code::FileWrite,"sync temporary output",Some(path),e))?;
    if existing { file.persist(path).map_err(|e| GuardError::io(Code::FileWrite,"replace output",Some(path),e.error))?; }
    else { file.persist_noclobber(path).map_err(|e| GuardError::io(Code::FileWrite,"publish new output",Some(path),e.error))?; }
    Ok(())
}
fn owned_output(path: &str, config: &Config, content: &[u8]) -> bool {
    if path == config.register { content.starts_with(b"# Document register\n\nGenerated from Git.") }
    else { content.starts_with(b"# docs-guard Git inventory v1; manifest itself excluded\n") }
}

#[derive(Default)]
struct Adoption<'a> {
    register: Option<&'a str>,
    manifest: Option<&'a str>,
}
fn check_output_attributes(root: &Path, path: &str) -> Result<()> {
    let bytes = git(root, &["check-attr","--cached","-z","filter","working-tree-encoding","ident","eol","--",path],None)?;
    let live = git(root, &["check-attr","-z","filter","working-tree-encoding","ident","eol","--",path],None)?;
    if live != bytes { return Err(GuardError::new(Code::UnsupportedAttribute,format!("staged and worktree Git attributes differ: {path}"))); }
    let fields: Vec<_> = bytes.split(|b| *b == 0).collect();
    if fields.len() != 13 || fields[12] != b"" {
        return Err(GuardError::new(Code::UnsupportedAttribute,format!("unexpected Git attribute response: {path}")));
    }
    for triple in fields[..12].chunks_exact(3) {
        let name = std::str::from_utf8(triple[1]).map_err(|e| GuardError::new(Code::GitProtocol,"invalid attribute name").caused_by(e))?;
        let value = std::str::from_utf8(triple[2]).map_err(|e| GuardError::new(Code::GitProtocol,"invalid attribute value").caused_by(e))?;
        if triple[0] != path.as_bytes() || !(matches!(value,"unspecified" | "unset") || (name == "eol" && value == "lf")) {
            return Err(GuardError::new(Code::UnsupportedAttribute,format!("unsupported generated-output attribute: {path}: {name}")));
        }
    }
    Ok(())
}
fn resolve_commit(root: &Path, reference: &str) -> Result<String> {
    let spec = format!("{reference}^{{commit}}");
    let raw = git(root,&["rev-parse","--verify","--end-of-options",&spec],None).map_err(|e| if e.code == Code::GitFailed { GuardError::new(Code::InvalidSnapshot,"cannot resolve requested commit").caused_by(e) } else { e })?;
    Ok(utf8(raw)?.trim().to_owned())
}
fn load_config(root: &Path, inv: &Inventory) -> Result<Config> {
    if !inv.get(CONFIG).is_some_and(Entry::regular) { return Err(GuardError::new(Code::ConfigMissing,"missing regular tracked configuration").at(CONFIG)); }
    let bytes = blob(root,inv,CONFIG)?;
    if bytes.len() > MAX_CONFIG { return Err(GuardError::new(Code::ResourceLimit,"configuration exceeds size limit")); }
    let config: Config = toml::from_str(&utf8(bytes)?).map_err(|e| GuardError::new(Code::ConfigInvalid,"invalid config; inspect local configuration").at(CONFIG).caused_by(e))?;
    config.validate()?;
    Ok(config)
}
fn audit_change_rules(current: &Inventory, base: &Inventory, config: &Config, candidate: &Config, findings: &mut Vec<Finding>) {
    let changed: BTreeSet<&str> = current.keys().chain(base.keys())
        .filter(|p| current.get(*p) != base.get(*p)).map(String::as_str).collect();
    let updated = |p: &str| p != candidate.register.as_str() && p != candidate.manifest.as_str() && current.get(p).is_some_and(|entry| entry.regular() && base.get(p).map(|old| &old.oid) != Some(&entry.oid));
    for rule in &config.change_rules {
        if !changed.iter().any(|path| rule.watch.iter().any(|prefix| within(path,prefix))) { continue; }
        if rule.review_document.as_deref().is_some_and(updated) { continue; }
        for p in &rule.require {
            if !updated(p) {
                findings.push(finding("DOCUMENT_CHANGE_REQUIRED",p,format!("rule {}: update this curated document or its configured review document",rule.id)));
            }
        }
    }
}
fn evaluate(root: &Path, commit: Option<&str>, base: Option<&str>, generate: bool, plan: bool, adoption: Adoption<'_>) -> Result<Report> {
    let mut inv = inventory(root,commit)?;
    let original = inv.clone();
    let tracked_files = inv.len();
    let index_identity = if commit.is_none() {
        let mut bytes = Vec::new();
        for (p,e) in &inv {
            bytes.extend_from_slice(format!("{} {}\t{}\n",e.mode,e.oid,p).as_bytes());
        }
        Some(utf8(git(root,&["hash-object","--stdin"],Some(&bytes))?)?.trim().to_owned())
    } else { None };
    let config = load_config(root,&inv).map_err(|e| e.operation("load configuration").at(CONFIG))?;
    validate_output_scope(&config,&inv)?;
    let mut findings = Vec::new();
    if !generate && !plan && base.is_none() && !config.change_rules.is_empty() {
        return Err(GuardError::new(Code::ComparisonBase,"configured change rules require --base with a trusted comparison commit"));
    }
    if let Some(base_sha) = base {
        let target = match commit { Some(sha) => sha.to_owned(), None => resolve_commit(root,"HEAD")? };
        if commit.is_some() && base_sha == target { return Err(GuardError::new(Code::ComparisonBase,"comparison base must precede the candidate commit")); }
        git(root,&["merge-base","--is-ancestor",base_sha,&target],None)
            .map_err(|e| if e.code == Code::GitFailed && e.git_exit_code == Some(1) { GuardError::new(Code::ComparisonBase,"comparison base must be an ancestor of target HEAD").caused_by(e) } else { e })?;
        let previous = inventory(root,Some(base_sha))?;
        audit_change_rules(&inv,&previous,&config,&config,&mut findings);
        // A changed or deleted rule cannot silently weaken the existing base policy.
        if previous.contains_key(CONFIG) && previous.get(CONFIG) != inv.get(CONFIG) {
            let old_config = load_config(root,&previous)?;
            audit_change_rules(&inv,&previous,&old_config,&config,&mut findings);
        }
    }
    let required: BTreeSet<_> = config.roles.values().chain(config.extra_required.iter()).collect();
    for p in required {
        if !inv.get(p).is_some_and(Entry::regular) {
            findings.push(finding("MISSING_REQUIRED",p,"missing regular tracked file"));
        }
    }
    let docs = config.documents(&inv);
    let reg = register(&config,&docs);
    if reg.len() > MAX_OUTPUT_FILE { return Err(GuardError::new(Code::ResourceLimit,"generated register exceeds size limit")); }
    let reg_oid = utf8(git(root,&["hash-object","--stdin"],Some(&reg))?)?.trim().to_string();
    let reg_mode = inv.get(&config.register).map(|e| e.mode.clone()).unwrap_or_else(|| "100644".into());
    if reg_mode != "100644" { findings.push(finding("OUTPUT_MODE",&config.register,"generated register must be mode 100644")); }
    if inv.get(&config.manifest).is_some_and(|e| e.mode != "100644") {
        findings.push(finding("OUTPUT_MODE",&config.manifest,"generated manifest must be mode 100644"));
    }
    let mut expected = inv.clone();
    expected.insert(config.register.clone(),Entry {mode:"100644".into(),oid:reg_oid});
    let man = manifest(&config,&expected);
    if reg.len() > MAX_OUTPUT_FILE || man.len() > MAX_OUTPUT_FILE { return Err(GuardError::new(Code::ResourceLimit,"generated artifact exceeds size limit")); }
    for (p, bytes) in [(&config.register,&reg),(&config.manifest,&man)] {
        if !generate && !plan && !inv.get(p).is_some_and(Entry::regular) {
            findings.push(finding("MISSING_GENERATED",p,"run generate and stage output"));
        } else if !generate && !plan {
            let actual = blob(root,&inv,p)?;
            if actual != *bytes {
                let detail = if p == &config.manifest { manifest_drift(&actual,bytes) }
                    else { "document register differs; run generate and stage output".into() };
                findings.push(finding("STALE_GENERATED",p,detail));
            }
        }
    }
    // New generated outputs count as valid link targets in generation planning.
    if generate || plan {
        inv.entry(config.register.clone()).or_insert(Entry {mode:"100644".into(),oid:String::new()});
        inv.entry(config.manifest.clone()).or_insert(Entry {mode:"100644".into(),oid:String::new()});
    }
    audit_links(root,&inv,&config,&docs,&mut findings)?;
    let mut changed_outputs = Vec::new();
    let mut legacy_adoptions = Vec::new();
    if generate || plan {
        let targets = [(&config.register,&reg),(&config.manifest,&man)];
        let mut observed = BTreeMap::new();
        for (p, bytes) in &targets {
            check_output_attributes(root,p)?;
            let requested = if *p == &config.register { adoption.register } else { adoption.manifest };
            let entry = original.get(*p).filter(|e| e.regular());
            let unmanaged = match entry {
                Some(_) => !owned_output(p,&config,&blob(root,&original,p)?),
                None => false,
            };
            if let Some(oid) = requested {
                if !unmanaged || !entry.is_some_and(|e| e.oid == oid) {
                    findings.push(finding("ADOPTION_MISMATCH",p,"adoption requires an unmanaged tracked output with the exact reviewed staged object ID"));
                } else {
                    legacy_adoptions.push(format!("{p}={oid}"));
                }
            } else if unmanaged {
                findings.push(finding("OUTPUT_UNMANAGED",p,"tracked output is not owned by docs-guard; use separate outputs or explicitly adopt reviewed legacy object ID"));
            }
            let target = safe_output(root,p)?;
            if !target.exists() {
                observed.insert((*p).clone(),None);
                changed_outputs.push((*p).clone());
                if requested.is_some() {
                    findings.push(finding("ADOPTION_MISSING_WORKTREE",p,"restore the reviewed legacy output before adoption"));
                }
            }
            if target.exists() {
                let existing = read_worktree_bounded(&target)?;
                observed.insert((*p).clone(),Some(existing.clone()));
                if existing != **bytes { changed_outputs.push((*p).clone()); }
                // Refuse to overwrite edits or pre-existing untracked files.
                if (requested.is_some() || existing != **bytes) && (!inv.get(*p).is_some_and(|e| e.regular() && !e.oid.is_empty()) || blob(root,&inv,p)? != existing) {
                    findings.push(finding("OUTPUT_DIRTY",p,"stage/resolve existing output edits before regeneration"));
                }
            }
        }
        if inventory(root,commit)? != original { return Err(GuardError::new(Code::SnapshotChanged,"Git index changed during evaluation; retry on a stable snapshot")); }
        if generate && findings.is_empty() {
            let mut written = Vec::new();
            for (p,bytes) in targets {
                if changed_outputs.contains(p) {
                    let preflight = (|| -> Result<PathBuf> {
                        let path = safe_output(root,p)?;
                        check_output_attributes(root,p)?;
                        if inventory(root,commit)? != original { return Err(GuardError::new(Code::SnapshotChanged,"Git index changed before output replacement")); }
                        let prior = observed.get(p).ok_or_else(|| GuardError::new(Code::Internal,"missing output observation"))?;
                        let now = if path.exists() { Some(read_worktree_bounded(&path)?) } else { None };
                        if &now != prior { return Err(GuardError::new(Code::OutputChanged,"output changed after preflight").at(p)); }
                        Ok(path)
                    })().map_err(|e| e.writes(&written,None))?;
                    write_output(&preflight,bytes,observed.get(p).is_some_and(Option::is_some))
                        .map_err(|e| e.writes(&written,Some(p)))?;
                    written.push(p.clone());
                }
            }
        }
    }
    if !generate && !plan && inventory(root,commit)? != original { return Err(GuardError::new(Code::SnapshotChanged,"Git snapshot changed during check")); }
    if findings.len() > MAX_FINDINGS { return Err(GuardError::new(Code::ResourceLimit,"finding limit exceeded")); }
    findings.sort_by(|a,b| (a.code,&a.path,&a.detail).cmp(&(b.code,&b.path,&b.detail)));
    findings.dedup_by(|a,b| a.code == b.code && a.path == b.path && a.detail == b.detail);
    Ok(Report {
        schema_version: 1,
        version: env!("CARGO_PKG_VERSION"),
        snapshot: commit.map(str::to_owned).unwrap_or_else(|| format!("INDEX:{}", index_identity.unwrap_or_default())),
        comparison_base: base.map(str::to_owned),
        change_rules_checked: base.is_some(),
        repository: config.repository,
        tracked_files,
        documents: docs.len(),
        outcome: if !findings.is_empty() {"FAIL"} else if generate {"GENERATED_NOT_CHECKED"} else if plan {"PLAN_READY"} else {"PASS"},
        findings,
        changed_outputs,
        legacy_adoptions,
    })
}
fn emit(json: bool, value: &serde_json::Value, human: &str) -> Result<()> {
    let mut stdout = std::io::stdout().lock();
    if json { serde_json::to_writer_pretty(&mut stdout,value).map_err(|e| GuardError::new(Code::OutputStream,"could not write JSON response").operation("stdout").caused_by(e))?; }
    else { stdout.write_all(human.as_bytes()).map_err(|e| GuardError::io(Code::OutputStream,"stdout",None,e))?; }
    stdout.write_all(b"\n").map_err(|e| GuardError::io(Code::OutputStream,"stdout",None,e))?;
    stdout.flush().map_err(|e| GuardError::io(Code::OutputStream,"stdout",None,e))
}
fn run(cli: &Cli) -> Result<u8> {
    let root = repo_root(&cli.root)?;
    let report = match &cli.command {
        Action::Init => {
            let path = safe_output(&root,CONFIG)?;
            if path.exists() { return Err(GuardError::new(Code::ConfigExists,"configuration already exists; refusing overwrite")); }
            write_output(&path,include_bytes!("../examples/docs-guard.toml"),false).map_err(|e| e.writes(&[],Some(CONFIG)))?;
            emit(cli.json,&serde_json::json!({"schema_version":1,"version":env!("CARGO_PKG_VERSION"),"outcome":"CONFIG_CREATED_NOT_CHECKED","path":CONFIG}),&format!("Created {CONFIG}; adapt identity and roles, create missing curated docs, stage inputs.")).map_err(|e| e.writes(&[CONFIG.to_owned()],None))?;
            return Ok(0);
        },
        Action::Doctor => {
            let version = utf8(git(&root,&["--version"],None)?)?;
            emit(cli.json,&serde_json::json!({"schema_version":1,"outcome":"DIAGNOSTIC","git":version.trim(),"root":root,"version":env!("CARGO_PKG_VERSION")}),&format!("docs-guard {} | {} | {}",env!("CARGO_PKG_VERSION"),version.trim(),root.display()))?;
            return Ok(0);
        },
        Action::Generate {adopt_register,adopt_manifest} => evaluate(&root,None,None,true,false,Adoption {register:adopt_register.as_deref(),manifest:adopt_manifest.as_deref()})?,
        Action::Plan {adopt_register,adopt_manifest} => evaluate(&root,None,None,false,true,Adoption {register:adopt_register.as_deref(),manifest:adopt_manifest.as_deref()})?,
        Action::Check {index,commit,base} | Action::Status {index,commit,base} => {
            let sha = if *index { None } else { Some(resolve_commit(&root,commit.as_deref().unwrap_or("HEAD"))?) };
            let comparison = base.as_deref().map(|r| resolve_commit(&root,r)).transpose()?;
            evaluate(&root,sha.as_deref(),comparison.as_deref(),false,false,Adoption::default())?
        }
    };
    let completed = if report.outcome == "GENERATED_NOT_CHECKED" { report.changed_outputs.as_slice() } else { &[] };
    if cli.json {
        emit(true,&serde_json::to_value(&report).map_err(|e| GuardError::new(Code::Serialization,"could not serialize report").caused_by(e).writes(completed,None))?,"").map_err(|e| e.writes(completed,None))?;
    } else {
        let mut out = format!("DOCS_GUARD_{} snapshot={} tracked={} documents={}",report.outcome,report.snapshot,report.tracked_files,report.documents);
        if let Some(base) = &report.comparison_base { out.push_str(&format!("\nCOMPARISON_BASE {base}")); }
        for p in &report.changed_outputs { out.push_str(&format!("\nOUTPUT_CHANGE {p}")); }
        for p in &report.legacy_adoptions { out.push_str(&format!("\nLEGACY_ADOPTION {p}")); }
        emit(false,&serde_json::Value::Null,&out).map_err(|e| e.writes(completed,None))?;
        let mut stderr = std::io::stderr().lock();
        for f in &report.findings { writeln!(stderr,"{} {}: {}",f.code,f.path,f.detail.escape_debug()).map_err(|e| GuardError::io(Code::OutputStream,"stderr",None,e))?; }
        stderr.flush().map_err(|e| GuardError::io(Code::OutputStream,"stderr",None,e))?;
    }
    Ok(if report.findings.is_empty() {0} else {1})
}
fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let code = if error.use_stderr() { 2 } else { 0 };
            return ExitCode::from(if error.print().is_ok() { code } else { 2 });
        }
    };
    let code = match run(&cli) {
        Ok(code) => code,
        Err(e) => {
            let action = match &cli.command {
                Action::Init => "init", Action::Doctor => "doctor", Action::Generate { .. } => "generate",
                Action::Plan { .. } => "plan", Action::Check { .. } => "check", Action::Status { .. } => "status",
            };
            let e = e.operation(action);
            // Never append a second JSON object after a failed output write.
            if cli.json && e.code != Code::OutputStream {
                if let Err(stream) = emit(true,&e.public_json(),"") {
                    let _ = writeln!(std::io::stderr().lock(),"DOCS_GUARD_ERROR: {stream}; original={e}");
                }
            } else {
                let _ = writeln!(std::io::stderr().lock(),"DOCS_GUARD_ERROR: {e}");
            }
            2
        }
    };
    ExitCode::from(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn parser_contract_is_valid() {
        Cli::command().debug_assert();
        assert!(Cli::try_parse_from(["docs-guard","check","--index","--commit","HEAD"]).is_err());
        assert!(Cli::try_parse_from(["docs-guard","check","--unknown"]).is_err());
    }
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
