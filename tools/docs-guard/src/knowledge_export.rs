//! Read-only, explicit knowledge candidate export. Reuses the Guard's own
//! snapshot, policy and Git subprocess implementation; no network or publishing.
use super::*;

const MAX_DOC: usize = 256 * 1024;
const MAX_TOTAL: usize = 1024 * 1024;
const ROLES: &[&str] = &[
    "scope",
    "architecture",
    "decisions",
    "verification",
    "status",
    "handoff",
    "history",
    "operations",
    "scientific_status",
];

fn exact_commit(root: &Path, value: &str) -> Result<String> {
    if !matches!(value.len(), 40 | 64)
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        || resolve_commit(root, value)? != value
    {
        return Err(GuardError::new(
            Code::InvalidSnapshot,
            "full immutable commit required",
        ));
    }
    Ok(value.to_owned())
}

pub(super) fn export(
    root: &Path,
    commit: &str,
    base: Option<&str>,
    repository: &str,
    roles: &[String],
) -> Result<serde_json::Value> {
    let sha = exact_commit(root, commit)?;
    let base = base.map(|value| exact_commit(root, value)).transpose()?;
    let selected: BTreeSet<_> = roles.iter().map(String::as_str).collect();
    if selected.is_empty()
        || selected.len() != roles.len()
        || selected.iter().any(|role| !ROLES.contains(role))
    {
        return Err(GuardError::new(
            Code::ConfigInvalid,
            "select distinct supported curated roles",
        ));
    }
    let inv = inventory(root, Some(&sha))?;
    let config = load_config(root, &inv)?;
    if config.repository != repository {
        return Err(GuardError::new(
            Code::ConfigInvalid,
            "repository identity mismatch",
        ));
    }
    let report = evaluate(
        root,
        Some(&sha),
        base.as_deref(),
        false,
        false,
        Adoption::default(),
    )?;
    let guard = serde_json::to_value(&report).map_err(|e| {
        GuardError::new(Code::Serialization, "could not serialize check report").caused_by(e)
    })?;
    if report.outcome != "PASS" {
        return Ok(serde_json::json!({
            "schema_version": 1, "outcome": "GUARD_FAILED", "guard_report": guard,
            "documents": [], "publish_authorized": false,
        }));
    }
    let mut paths = BTreeMap::<String, Vec<&str>>::new();
    for role in selected {
        let path = config.roles.get(role).ok_or_else(|| {
            GuardError::new(Code::ConfigInvalid, format!("role not configured: {role}"))
        })?;
        if !matches!(
            Path::new(path).extension().and_then(|s| s.to_str()),
            Some("md" | "rst" | "adoc")
        ) {
            return Err(GuardError::new(
                Code::InvalidPath,
                "only curated text documents may be exported",
            ));
        }
        paths.entry(path.clone()).or_default().push(role);
    }
    let mut documents = Vec::new();
    let mut total = 0usize;
    for (path, roles) in paths {
        let entry = inv.get(&path).filter(|e| e.regular()).ok_or_else(|| {
            GuardError::new(Code::InvalidSnapshot, "expected regular tracked document").at(&path)
        })?;
        let size = utf8(git(root, &["cat-file", "-s", &entry.oid], None)?)?
            .trim()
            .parse::<usize>()
            .map_err(|_| GuardError::new(Code::GitProtocol, "invalid blob size"))?;
        if size > MAX_DOC || total.saturating_add(size) > MAX_TOTAL {
            return Err(GuardError::new(
                Code::ResourceLimit,
                "knowledge export exceeds content bound",
            ));
        }
        let bytes = blob(root, &inv, &path)?;
        if bytes.len() != size {
            return Err(GuardError::new(Code::GitProtocol, "blob length mismatch"));
        }
        total += size;
        documents.push(serde_json::json!({
            "path": path, "roles": roles, "git_blob": entry.oid, "content": utf8(bytes)?,
        }));
    }
    Ok(serde_json::json!({
        "schema_version": 1, "outcome": "CANDIDATES_READY", "repository": repository,
        "commit": sha, "guard_report": guard, "documents": documents,
        "semantic_validation": "not_performed", "publish_authorized": false,
    }))
}
