# Documentation Guard — scoped hardening audit

audit_status: complete
implementation_status: open
baseline: 883a33a4ddd9dfdc2e51a3acc1c6ac71e4b4a7ea
date: 2026-11-26
scope: tools/docs-guard, docs-guard installer, associated regression tests

Static source audit, not an independent full-repository audit. No execution on Contabo or GitHub Actions. Standards: Rust §§6, 14, 25–30; Security §§28–42, 50–51, 68–69, 125, 130, 145–150; Testing §§20–25, 31, 71–73, 79, 85, 101, 106–107; CLI §§2–5. GBrain/Graphify/gstack unavailable in this GitHub-only workflow. AGENTS.md not present at root. Follow audit → persisted findings → scoped fix → verification, without claiming RESOLVED before real QA.

| ID | Severity | Confidence | Status | Evidence and impact |
|---|---|---|---|---|
| DG-H01 | HIGH | HIGH | OPEN | main.rs Config::validate and safe_output (lines 199–260, 418–435): lexical case-sensitive comparisons allow output aliases on case-insensitive filesystems; generated paths can lie below a tracked gitlink or nested repository. Potential write outside intended project scope. |
| DG-H02 | HIGH | HIGH | OPEN | main.rs evaluate lines 566–608: initial output preflight is not revalidated against exact observed bytes immediately before each replacement. Intervening edits or new files can be overwritten. Not a full concurrent-writer sandbox. |
| DG-H03 | MEDIUM | HIGH | OPEN | main.rs check_output_attributes lines 460–474: only staged attributes are evaluated; a conflicting worktree filter/encoding configuration can escape preflight. |
| DG-H04 | MEDIUM | HIGH | OPEN | main.rs load_config lines 479–482: large configuration accepted up to generic Git stdout limit; unconstrained rule/role lists multiply evaluation and allocation cost. TOML parser diagnostics may reproduce input. |
| DG-H05 | MEDIUM | HIGH | OPEN | git_process.rs: raw Git stderr returned in public error report, potentially including repository URLs or sensitive configuration. Inherited environment overrides incomplete. |
| DG-H06 | MEDIUM | HIGH | OPEN | main.rs audit_change_rules lines 484–525: old rule can be satisfied by reclassifying its curated target as a candidate-generated artifact; compare-base policy needs explicit separation. Mode-only regression still missing. |
| DG-H07 | MEDIUM | HIGH | OPEN | main.rs audit_links line 383: full inventory scan for every directory link; avoid avoidable quadratic work. Generated Markdown labels/headings escape only selected markup characters. |
| DG-H08 | MEDIUM | HIGH | OPEN | scripts/docs-guard-install.sh: lock parsing accepts extra empty columns through shell whitespace splitting; doctor/existing install path checks digest but not executable mode. |
| DG-H09 | HIGH | HIGH | OPEN | Cargo.lock missing; all compilation, Clippy, RustSec, platform/consumer tests NOT_RUN. Cannot resolve by static patch; real QA mandatory. |

Fix scope: existing behavior and security boundaries only. Regression tests precede or accompany changes. No new dependencies, no additional runtime features. Retain source and runtime verification statuses separately.

## Remediation pass 1 (source only)

DG-H01–H08: source corrections authored, with regression cases for case alias, nested repository, attribute drift, configuration size, mode-only rule and malformed release lock. All remain OPEN pending actual compiler/runtime evidence. DG-H02 is a narrower optimistic recheck, not atomic compare-and-swap; concurrent malicious writers require isolation. DG-H06 excludes current generated targets from prior rule satisfaction; externally trusted policy selection remains mandatory. DG-H09 remains OPEN, deferred as agreed. No findings are marked RESOLVED. No new crates. See code commits after audit baseline.

## Error-handling follow-up audit (2026-11-26)

Baseline: c8233f4e4c2609256099c9a6beb7015e9563b2c7. Scope limited to error boundaries and failure recovery; Rust standard §§6–7, CLI standard §2, Security §§29–39 and Testing §§24, 105–106. Source review only; all findings OPEN.

| ID | Severity | Evidence / required correction |
|---|---|---|
| DG-E01 | HIGH | main.rs Result<T,String> and generic EXECUTION_ERROR: no stable classification for config, Git, I/O, security, limits, conflicts or internal errors; preserve typed sources and safe context. |
| DG-E02 | HIGH | main.rs evaluate: second output failure after first replacement does not report completed writes or recovery instructions. |
| DG-E03 | MEDIUM | main.rs emit/run: several output failures silently return exit 2; report stream failure on stderr without attempting another JSON document. |
| DG-E04 | MEDIUM | git_process.rs: start, exit, timeout, pipe/worker failures collapse into strings; cleanup errors discarded. Preserve distinctions and do not expose raw stderr. |
| DG-E05 | MEDIUM | tests/snapshots.rs: generic execution error assertion and limited failure-contract coverage. Add code/category/operation, partial-write and redaction tests. |
| DG-E06 | MEDIUM | audit_links: raw link destinations in diagnostics can leak query strings or credentials. Do not echo raw destination; retain file, line and resolved safe path. |

No scope expansion, no new dependencies, no runtime verification until authorized QA.

### Error-handling remediation (source only)

DG-E01–E06: typed error model with 23 named codes, safe public JSON, context/source preservation, Git exit and I/O kind metadata, process cleanup indicator, partial-write recovery, output-stream fallback and raw link destination redaction authored. Added deterministic write-sequence failure test and CLI classification/redaction cases. All findings remain OPEN until actual runtime verification, including missing Git, timeout/overflow, OS permissions, broken pipe and real partial-write recovery. No new crates. No runtime tests or Contabo work.
