# Observability, Logging & Tracing Engineering Audit

## Contents

- §1: Core Contract
- §2: Audit Output
- §3: Audit Frontmatter
- §4: Baseline
- §5: Resolve Crate Versions
- §6: Read-Only Rule
- §7: Scope
- §8: Evidence Requirement
- §9: Parallel Subagents
- §10: Parent Validation
- §11: Finding IDs
- §12: Finding Status
- §13: Severity and Confidence
- §14: Finding Structure
- §15: Verified Strengths
- §16: Audit Report Structure
- §17: Audit Completeness
- §18: Final Rule

> Navigation: use this map to load only the sections required for the current phase. Do not preload the whole file unless the task genuinely requires it.


Perform a complete evidence-based audit of logging, tracing, and observability.

This skill is READ-ONLY with respect to existing source, tests, and telemetry configuration.

It may create exactly one persistent audit report under:

`docs/engineering-audits/observability/`

It MUST NOT fix findings.

The canonical standard is:

`../_standards/observability-logging-tracing-engineering-standard.md`

Use the standard's **Contents** map first and load the sections relevant to the current audit phase. Before closeout, verify that every applicable standard domain was covered.

Never reproduce secret values, prompts, tokens, or personal data. Use `[REDACTED]`.

---

# 1. Core Contract

The workflow is:

```text
REPOSITORY DISCOVERY
        ↓
CRATE VERSION DISCOVERY
        ↓
TELEMETRY ARCHITECTURE DISCOVERY
        ↓
QUALITY BASELINE
        ↓
PARALLEL SPECIALIST ANALYSIS
        ↓
PARENT EVIDENCE VALIDATION
        ↓
ROOT-CAUSE CORRELATION
        ↓
PERSISTENT AUDIT
```

Never perform:

```text
AUDIT
→ FIX
```

Remediation belongs exclusively to:

```text
/observability-logging-tracing-engineering-fix
```

---

# 2. Audit Output

Create:

```text
docs/engineering-audits/observability/
OBS-AUDIT-YYYYMMDD-HHMMSS-<short-git-sha>.md
```

The report is the persistent ledger used by the fix skill.

---

# 3. Audit Frontmatter

Every report MUST begin with:

```yaml
---
schema_version: 1
audit_id: OBS-AUDIT-YYYYMMDD-HHMMSS-<short-git-sha>
domain: observability-logging-tracing
standard: observability-logging-tracing-engineering-standard
standard_version: 1
baseline_commit: <full-git-sha>
baseline_branch: <branch>
baseline_worktree: clean
tracing_version: <resolved-version-or-absent>
sqlx_version: <resolved-version-or-absent>
tower_http_version: <resolved-version-or-absent>
rig_version: <resolved-version-or-absent>
opentelemetry_version: <resolved-version-or-absent>
created_at: <ISO-8601>
updated_at: <ISO-8601>
audit_status: complete
implementation_status: open
---
```

Allowed:

```text
audit_status:
- complete
- incomplete

implementation_status:
- open
- partial
- resolved
- requires-reaudit
```

If no confirmed actionable findings exist:

```yaml
audit_status: complete
implementation_status: resolved
```

Do not manufacture findings.

---

# 4. Baseline

Before analysis inspect:

```bash
git rev-parse HEAD
git branch --show-current
git status --short
rustc --version
cargo --version
```

Record the worktree state. Never reset or discard user changes.

---

# 5. Resolve Crate Versions

Read `Cargo.lock` or `cargo metadata` in the repository under audit.

Identify resolved versions and features for:

```text
tracing
tracing-subscriber
tower-http
sqlx
rig
opentelemetry
tracing-opentelemetry
```

Version-specific claims must match those versions. Absence of OpenTelemetry or Rig is not a finding by itself.

---

# 6. Read-Only Rule

AUDIT MUST NOT modify:

```text
Rust source
tests
Cargo.toml
Cargo.lock
subscriber configuration
prompt or fixture files
CI
```

The only intentional write is the audit report.

---

# 7. Scope

Systematically evaluate, where the application actually has the surface:

```text
tracing/subscriber initialization
span hierarchy
span naming
structured fields
log levels
EnvFilter/runtime filtering
async instrumentation correctness
request IDs
correlation
HTTP tracing
error chains
duplicate error logging
SQLx tracing
DB semantic spans
external integrations
Rig agent/run/tool tracing
GenAI telemetry
sensitive content
OpenTelemetry integration where present
telemetry tests
developer-debuggability
```

Axum handler architecture, SQL correctness, agent authorization, and secret storage policy stay with their own standards. Report the observability root cause here.

---

# 8. Evidence Requirement

A missing log is not a finding.

Before reporting:

1. inspect the execution path
2. inspect subscriber and filter setup
3. inspect the resolved crate version
4. determine the concrete debugging or operational impact

Legitimate findings include:

```text
a critical async workflow has no correlation
Span::enter is held across .await
an error loses its source chain
a tenant or user request cannot be followed
sqlx::query output makes development unusable
agent tool calls are not correlated to the run
secrets are captured through Debug fields
production logs are unstructured strings
```

These alone are not findings:

```text
a function has no #[instrument]
OpenTelemetry is absent
JSON logging is absent
a trivial helper has no span
```

---

# 9. Parallel Subagents

For repository-wide audits use up to three specialist subagents.

They MUST NOT modify files. They return candidate findings only.

## Subagent A — Application and Async Instrumentation

Analyze:

```text
span hierarchy
#[instrument]
Future::instrument
Span::enter misuse
use-case tracing
errors
log levels
structured fields
```

## Subagent B — HTTP, SQLx, and External Dependencies

Analyze:

```text
request IDs
TraceLayer
middleware ordering
HTTP correlation
SQLx query tracing
repository spans
external HTTP
latency
```

For `tower-http` 0.6, request id must be set before `TraceLayer` creates the span, and propagated onto the response before that layer records it. `Router::layer` applies in the opposite order from `ServiceBuilder`.

For `sqlx-core` 0.8, query events use target `sqlx::query` and include `db.statement` plus row counts and elapsed time. Bind values are not in `QueryLogger::finish`. Re-check the resolved version before claiming argument logging.

## Subagent C — Rig, Telemetry, Security, and Developer Experience

Analyze:

```text
agent run IDs
model/tool spans
token/turn telemetry
GenAI semantic conventions
prompt/content logging
redaction
filters
local debugging ergonomics
OpenTelemetry
```

Do not require prompt or response capture. Default content telemetry is a finding when it is on, not when it is off.

---

# 10. Parent Validation

Subagent output is NOT authoritative.

The parent MUST validate every candidate against repository evidence.

Reject false positives.

Merge symptoms that share one root cause.

---

# 11. Finding IDs

Use:

```text
OBS-001
OBS-002
OBS-003
```

IDs remain stable throughout remediation.

---

# 12. Finding Status

Initial status:

```text
OPEN
```

Allowed:

```text
OPEN
IN_PROGRESS
RESOLVED
NOT_APPLICABLE
ACCEPTED_RISK
BLOCKED
STALE
```

The audit MUST NOT autonomously set `ACCEPTED_RISK`.

---

# 13. Severity and Confidence

Severity:

```text
CRITICAL
HIGH
MEDIUM
LOW
INFO
```

Confidence:

```text
HIGH
MEDIUM
LOW
```

Severity follows diagnostic or operational impact, not the mere absence of a span.

Use `Needs Investigation` instead of a weakly supported confirmed finding.

---

# 14. Finding Structure

Use:

```markdown
## OBS-001 — Precise title

**Status:** OPEN
**Severity:** HIGH
**Confidence:** HIGH
**Category:** <spans / http / sqlx / errors / rig / redaction / ...>
**Standard:** §<section> <rule>
**Locations:**
- `src/...`

### Evidence

Concrete repository behavior. Redact secrets.

### Debugging / Operational Impact

What a developer or operator cannot answer, or what leaks.

### Why this violates the standard

The violated observability invariant.

### Required remediation

The required outcome, without an over-prescribed patch.

### Verification

The check that proves the outcome.

### Resolution

Not yet implemented.
```

---

# 15. Verified Strengths

Record controls actually verified.

Examples:

```text
request IDs consistently propagated
no async Span::enter misuse found
errors preserve source chains
SQL query logging safely filterable
agent runs have end-to-end run correlation
sensitive GenAI content disabled by default
```

Do not add generic praise.

---

# 16. Audit Report Structure

Produce:

```markdown
# Observability, Logging & Tracing Engineering Audit

## Executive Summary

## Repository Baseline

## Telemetry Crate Baseline

## Instrumentation Overview

## Verification Results

## Finding Summary

## Critical Findings

## High Findings

## Medium Findings

## Low Findings

## Verified Strengths

## Needs Investigation

## Audit Limitations

## Recommended Remediation Order
```

Summary table:

```markdown
| ID | Severity | Confidence | Status | Category | Title |
|---|---|---|---|---|---|
| OBS-001 | HIGH | HIGH | OPEN | Async | ... |
```

Normally remediate in this order:

```text
secret or personal-data leakage
→ correlation loss on critical paths
→ async span misuse
→ error-chain loss
→ filter and production-format failures
→ missing semantic spans where diagnosis is blocked
→ noise and duplication
```

---

# 17. Audit Completeness

Audit is complete only when:

1. the canonical observability standard was read
2. resolved telemetry crate versions were recorded
3. subscriber initialization and filters were inspected
4. representative request, job, or agent paths were traced
5. async `Span::enter` use was searched
6. HTTP, SQLx, and external calls were reviewed where present
7. agent and GenAI telemetry were reviewed where present
8. redaction of secrets and prompts was reviewed
9. candidates were independently validated
10. verified strengths were recorded
11. the report was persisted
12. source and configuration were not modified

---

# 18. Final Rule

The audit must answer:

```text
Can a developer follow a failed request, job, or agent run
to the failing operation, the error chain, and the duration,
without secrets in the telemetry?
```

The audit does not improve the system.

Remediation belongs exclusively to:

```text
/observability-logging-tracing-engineering-fix
```
