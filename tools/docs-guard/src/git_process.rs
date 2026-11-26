//! Bounded direct Git process execution. No shell or project code execution.
use crate::Result;
use std::{
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
    sync::OnceLock,
    thread,
    time::{Duration, Instant},
};

const MAX_STDOUT: usize = 32 * 1024 * 1024;
const MAX_STDERR: usize = 64 * 1024;
const MAX_INPUT: usize = 32 * 1024 * 1024;
const CALL_TIMEOUT: Duration = Duration::from_secs(20);
const INVOCATION_TIMEOUT: Duration = Duration::from_secs(180);
static START: OnceLock<Instant> = OnceLock::new();

fn read_bounded(reader: impl Read, max: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader.take((max + 1) as u64).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    if bytes.len() > max { return Err(format!("Git output exceeded {max} bytes")); }
    Ok(bytes)
}

pub(crate) fn git(root: &Path, args: &[&str], input: Option<&[u8]>) -> Result<Vec<u8>> {
    let start = *START.get_or_init(Instant::now);
    let remaining = INVOCATION_TIMEOUT.checked_sub(start.elapsed()).ok_or("Git invocation budget exhausted")?;
    let deadline = Instant::now() + remaining.min(CALL_TIMEOUT);
    if input.is_some_and(|data| data.len() > MAX_INPUT) { return Err("Git input exceeds size limit".into()); }
    let mut command = Command::new("git");
    for key in ["GIT_DIR","GIT_WORK_TREE","GIT_INDEX_FILE","GIT_COMMON_DIR","GIT_OBJECT_DIRECTORY","GIT_ALTERNATE_OBJECT_DIRECTORIES","GIT_NAMESPACE","GIT_CONFIG_COUNT","GIT_CONFIG_PARAMETERS","GIT_CONFIG","GIT_REPLACE_REF_BASE","GIT_CONFIG_GLOBAL","GIT_CONFIG_SYSTEM","GIT_GRAFT_FILE","GIT_SHALLOW_FILE","GIT_ATTR_SOURCE"] {
        if std::env::var_os(key).is_some() { return Err(format!("unsupported Git environment override: {key}")); }
    }
    command.current_dir(root)
        .env("GIT_NO_REPLACE_OBJECTS","1")
        .env("GIT_NO_LAZY_FETCH","1")
        .env("GIT_OPTIONAL_LOCKS","0")
        .env("GIT_TERMINAL_PROMPT","0")
        .env("GIT_ALLOW_PROTOCOL","")
        .env("GIT_PAGER","cat")
        .args(args).stdout(Stdio::piped()).stderr(Stdio::piped())
        .stdin(if input.is_some() { Stdio::piped() } else { Stdio::null() });
    let mut child = command.spawn().map_err(|e| format!("cannot start git: {e}"))?;
    let outcome = (|| {
        let stdout = child.stdout.take().ok_or("missing Git stdout")?;
        let stderr = child.stderr.take().ok_or("missing Git stderr")?;
        let out = thread::Builder::new().spawn(move || read_bounded(stdout,MAX_STDOUT)).map_err(|e| e.to_string())?;
        let err = thread::Builder::new().spawn(move || read_bounded(stderr,MAX_STDERR)).map_err(|e| e.to_string())?;
        let writer = if let Some(data) = input {
            let mut stdin = child.stdin.take().ok_or("missing Git stdin")?;
            let bytes = data.to_vec();
            Some(thread::Builder::new().spawn(move || stdin.write_all(&bytes)).map_err(|e| e.to_string())?)
        } else { None };
        // Drain both pipes concurrently, including when writing stdin. Never wait
        // indefinitely for pipe closure by a descendant; only direct Git is killed.
        let mut status = None;
        loop {
            if status.is_none() { status = child.try_wait().map_err(|e| e.to_string())?; }
            if let Some(exit) = status {
                if out.is_finished() && err.is_finished() && writer.as_ref().is_none_or(|w| w.is_finished()) {
                    let stdout = out.join().map_err(|_| "Git stdout reader panicked")??;
                    let _stderr = err.join().map_err(|_| "Git stderr reader panicked")??;
                    if !exit.success() {
                        return Err(format!("Git {} failed (status {}); stderr omitted to protect repository configuration",args.first().copied().unwrap_or("command"),exit));
                    }
                    if let Some(w) = writer {
                        w.join().map_err(|_| "Git stdin writer panicked")?.map_err(|e| e.to_string())?;
                    }
                    return Ok(stdout);
                }
            }
            if Instant::now() >= deadline { return Err(format!("Git timeout: {}",args.first().copied().unwrap_or("command"))); }
            thread::sleep(Duration::from_millis(5));
        }
    })();
    if outcome.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_output_rejects_instead_of_truncating() {
        assert_eq!(read_bounded(&b"abc"[..],3).unwrap(),b"abc");
        assert!(read_bounded(&b"abcd"[..],3).is_err());
    }
}
