//! Stable public errors; internal sources are retained but never serialized.
use std::{error::Error, fmt, io, path::Path};

pub(crate) type Result<T> = std::result::Result<T, GuardError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Code {
    ConfigMissing, ConfigInvalid, ConfigExists, InvalidPath, OutputScope,
    GitEnvironment, GitUnavailable, GitFailed, GitTimeout, GitProtocol,
    InvalidSnapshot, ComparisonBase, SnapshotChanged, OutputChanged,
    UnsupportedAttribute, ResourceLimit, InvalidEncoding, InvalidLink,
    FileRead, FileWrite, OutputStream, Serialization, Internal,
}

impl Code {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ConfigMissing => "CONFIG_MISSING",
            Self::ConfigInvalid => "CONFIG_INVALID",
            Self::ConfigExists => "CONFIG_EXISTS",
            Self::InvalidPath => "INVALID_PATH",
            Self::OutputScope => "OUTPUT_SCOPE",
            Self::GitEnvironment => "GIT_ENVIRONMENT",
            Self::GitUnavailable => "GIT_UNAVAILABLE",
            Self::GitFailed => "GIT_FAILED",
            Self::GitTimeout => "GIT_TIMEOUT",
            Self::GitProtocol => "GIT_PROTOCOL",
            Self::InvalidSnapshot => "INVALID_SNAPSHOT",
            Self::ComparisonBase => "COMPARISON_BASE",
            Self::SnapshotChanged => "SNAPSHOT_CHANGED",
            Self::OutputChanged => "OUTPUT_CHANGED",
            Self::UnsupportedAttribute => "UNSUPPORTED_ATTRIBUTE",
            Self::ResourceLimit => "RESOURCE_LIMIT",
            Self::InvalidEncoding => "INVALID_ENCODING",
            Self::InvalidLink => "INVALID_LINK",
            Self::FileRead => "FILE_READ",
            Self::FileWrite => "FILE_WRITE",
            Self::OutputStream => "OUTPUT_STREAM",
            Self::Serialization => "SERIALIZATION",
            Self::Internal => "INTERNAL_ERROR",
        }
    }
    fn category(self) -> &'static str {
        match self {
            Self::ConfigMissing | Self::ConfigInvalid | Self::ConfigExists | Self::ComparisonBase => "configuration",
            Self::InvalidPath | Self::OutputScope | Self::GitEnvironment | Self::UnsupportedAttribute => "security",
            Self::GitUnavailable | Self::GitFailed | Self::GitTimeout | Self::GitProtocol => "git",
            Self::InvalidSnapshot | Self::SnapshotChanged | Self::OutputChanged => "snapshot",
            Self::ResourceLimit => "resource",
            Self::InvalidEncoding | Self::InvalidLink => "input",
            Self::FileRead | Self::FileWrite | Self::OutputStream => "io",
            Self::Serialization | Self::Internal => "internal",
        }
    }
    fn recovery(self) -> &'static str {
        match self {
            Self::ConfigMissing | Self::ConfigInvalid | Self::ConfigExists => "Review the staged configuration; initialize only when absent.",
            Self::InvalidPath | Self::OutputScope | Self::UnsupportedAttribute => "Correct the path or attributes; do not bypass the safety check.",
            Self::GitEnvironment => "Remove the unsupported Git environment override in a trusted shell.",
            Self::GitUnavailable | Self::GitFailed | Self::GitProtocol => "Check the trusted Git installation and repository locally.",
            Self::GitTimeout | Self::ResourceLimit => "Check repository size and runner capacity; retry only after addressing the limit.",
            Self::InvalidSnapshot | Self::ComparisonBase => "Resolve the Git snapshot or supply the trusted ancestor.",
            Self::SnapshotChanged | Self::OutputChanged => "Review changes and retry on a stable index and worktree.",
            Self::InvalidEncoding | Self::InvalidLink => "Correct the unsupported input.",
            Self::FileRead | Self::FileWrite => "Check permissions, storage and the target path before retrying.",
            Self::OutputStream => "Restore the output stream and rerun; a failed response does not prove no writes occurred.",
            Self::Serialization | Self::Internal => "Report the failure with version and operation; do not treat it as a PASS.",
        }
    }
}

#[derive(Debug)]
pub(crate) struct GuardError {
    pub(crate) code: Code,
    detail: String,
    operation: Option<String>,
    path: Option<String>,
    source: Option<Box<dyn Error + Send + Sync>>,
    written_outputs: Vec<String>,
    write_attempted: Option<String>,
    cleanup_failed: bool,
}

impl GuardError {
    pub(crate) fn new(code: Code, detail: impl Into<String>) -> Self {
        Self { code, detail: detail.into(), operation: None, path: None, source: None,
            written_outputs: Vec::new(), write_attempted: None, cleanup_failed: false }
    }
    pub(crate) fn caused_by(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }
    pub(crate) fn operation(mut self, operation: impl Into<String>) -> Self {
        if self.operation.is_none() { self.operation = Some(operation.into()); }
        self
    }
    pub(crate) fn at(mut self, path: impl Into<String>) -> Self {
        if self.path.is_none() { self.path = Some(path.into()); }
        self
    }
    pub(crate) fn io(code: Code, operation: &str, path: Option<&Path>, source: io::Error) -> Self {
        let mut err = Self::new(code,format!("I/O failure ({:?})",source.kind())).operation(operation);
        if let Some(path) = path { err = err.at(path.to_string_lossy().into_owned()); }
        err.caused_by(source)
    }
    pub(crate) fn writes(mut self, completed: &[String], attempted: Option<&str>) -> Self {
        self.written_outputs = completed.to_vec();
        self.write_attempted = attempted.map(str::to_owned);
        self
    }
    pub(crate) fn cleanup_failed(mut self) -> Self { self.cleanup_failed = true; self }
    pub(crate) fn public_json(&self) -> serde_json::Value {
        serde_json::json!({
            "schema_version": 1, "version": env!("CARGO_PKG_VERSION"), "outcome": "ERROR",
            "code": self.code.name(), "category": self.code.category(),
            "operation": self.operation, "path": self.path,
            "detail": self.detail, "recovery": self.code.recovery(),
            "written_outputs": self.written_outputs, "write_attempted": self.write_attempted,
            "partial_write_possible": !self.written_outputs.is_empty() || self.write_attempted.is_some(),
            "cleanup_failed": self.cleanup_failed,
        })
    }
}
impl fmt::Display for GuardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}: {}",self.code.name(),self.detail.escape_debug())?;
        if let Some(op) = &self.operation { write!(f," [operation={}]",op.escape_debug())?; }
        if let Some(path) = &self.path { write!(f," [path={}]",path.escape_debug())?; }
        if !self.written_outputs.is_empty() || self.write_attempted.is_some() {
            write!(f," [completed={:?}; attempted={:?}; inspect outputs before staging]",self.written_outputs,self.write_attempted)?;
        }
        if self.cleanup_failed { write!(f," [process cleanup failed; inspect runner]")?; }
        write!(f,"; {}",self.code.recovery())
    }
}
impl Error for GuardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_source_without_serializing_it() {
        let e = GuardError::io(Code::FileWrite,"persist",None,io::Error::new(io::ErrorKind::PermissionDenied,"secret-canary"));
        assert!(e.source().is_some());
        assert_eq!(e.public_json()["code"],"FILE_WRITE");
        assert!(!e.public_json().to_string().contains("secret-canary"));
        assert!(!e.to_string().contains("secret-canary"));
    }
    #[test]
    fn partial_write_metadata_is_explicit() {
        let e = GuardError::new(Code::FileWrite,"write failed").writes(&["register.md".into()],Some("manifest.tsv"));
        let json = e.public_json();
        assert_eq!(json["partial_write_possible"],true);
        assert_eq!(json["written_outputs"][0],"register.md");
        assert_eq!(json["write_attempted"],"manifest.tsv");
    }
}
