---
name: rust-axum-engineering-audit
description: >
  Perform a repository-wide evidence-based engineering audit of Rust Axum,
  Tokio, Tower and HTTP/API code against the canonical Rust Axum Engineering
  Standard. Produces a persistent audit ledger with verified findings but
  never modifies production source code.
---

# Rust Axum Engineering Audit

Perform a complete evidence-based audit of the repository's Rust Axum HTTP/API implementation.

This skill is READ-ONLY with respect to production source code.

It may create exactly one persistent audit report under:

`docs/engineering-audits/axum/`

It MUST NOT fix findings.

The canonical engineering standard is:

`../_standards/rust-axum-engineering-standard.md`

Read that standard completely before evaluating the repository.

---

# 1. Core Contract

This skill performs:

```text
REPOSITORY DISCOVERY
        ↓
BASELINE VERIFICATION
        ↓
AXUM-SPECIFIC ANALYSIS
        ↓
SUBAGENT ANALYSIS
        ↓
EVIDENCE VALIDATION
        ↓
DEDUPLICATION
        ↓
PERSISTENT AUDIT REPORT
```

It MUST NOT perform:

```text
AUDIT
→ FIX
```

Fixing is exclusively the responsibility of:

```text
/rust-axum-engineering-fix
```

---

# 2. Audit Output

Create:

```text
docs/engineering-audits/axum/
AXUM-AUDIT-YYYYMMDD-HHMMSS-<short-git-sha>.md
```

Example:

```text
AXUM-AUDIT-20260919-171500-a92d326.md
```

The audit file is the persistent ledger used by the corresponding fix skill.

Do not create a temporary audit that exists only in chat.

---

# 3. Audit Frontmatter

Every report MUST begin with:

```yaml
---
schema_version: 1
audit_id: AXUM-AUDIT-20260919-171500-a92d326
domain: rust-axum
standard: rust-axum-engineering-standard
standard_version: 1
baseline_commit: <full-git-sha>
baseline_branch: <branch>
baseline_worktree: clean
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

Never manufacture findings merely to keep an audit open.

---

# 4. Working Tree Baseline

Before analysis inspect:

```bash
git rev-parse HEAD
git branch --show-current
git status --short
rustc --version
cargo --version
```

Record the results.

If the working tree contains material uncommitted changes:

```yaml
baseline_worktree: dirty
```

and state that the audit includes uncommitted repository state.

Do not discard, modify or reset user changes.

---

# 5. Repository Instructions

Before evaluating code, inspect applicable repository instructions:

```text
AGENTS.md
nested AGENTS.md
.cursor/rules/
CONTRIBUTING.md
README.md
architecture documentation
ADRs
Cargo.toml
workspace Cargo.toml
rust-toolchain.toml
OpenAPI/API specifications
```

Repository-specific architecture may legitimately specialize the general standard.

Do not report a violation before determining whether such specialization exists.

---

# 6. Discover Axum Architecture

Build a repository inventory before judging individual files.

Identify:

```text
Axum crates
server binaries
Router construction
nested routers
handlers
extractors
custom extractors
request DTOs
response DTOs
application state
substates
middleware
Tower layers
Tower HTTP layers
Tokio tasks
background tasks
HTTP clients
authentication integration
authorization integration
error types
IntoResponse implementations
Problem Details
OpenAPI integration
health endpoints
shutdown handling
tracing
metrics
integration/router tests
```

Identify relevant dependency versions from Cargo metadata rather than assuming current versions.

Do not infer architecture from filenames alone.

---

# 7. Scope Boundary

This audit owns Axum/Tokio/Tower/HTTP engineering concerns.

It MAY inspect neighboring application, domain and persistence code when needed to understand an Axum finding.

It should NOT duplicate deep audits owned by:

```text
rust-engineering
rust-sqlx-postgresql
testing-quality
security-supply-chain
```

Examples:

A raw SQL injection defect belongs primarily to Security/SQLx.

A handler directly performing SQL is an Axum architectural finding because the HTTP boundary is violated.

A missing authorization check may be noted when clearly visible, but systematic authorization analysis belongs to Security.

---

# 8. Read-Only Rule

AUDIT MUST NOT modify production code.

Do not run:

```text
cargo fmt
cargo fix
cargo update
automatic code rewrites
migration commands
code generators that alter tracked source
```

Allowed read-only operations include:

```text
cargo fmt --check
cargo check
cargo clippy
cargo test
cargo metadata
cargo tree
git grep
rg
```

The only file this skill may intentionally create or update is its own audit report.

---

# 9. Verification Baseline

Use repository-defined verification commands when available.

Otherwise run appropriate equivalents of:

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
```

For workspaces use appropriate workspace and target settings.

Do NOT blindly enable:

```text
--all-features
```

when features are mutually exclusive, target-specific or unsupported together.

Record each executed command and its result.

A failed command is evidence to investigate.

It is not automatically a confirmed Axum finding.

---

# 10. Candidate Discovery

Search broadly for relevant Axum patterns.

Candidate searches may include:

```text
Router::
.route(
.route_layer(
.layer(
ServiceBuilder
State<
Extension<
FromRequest
FromRequestParts
Json<
Path<
Query<
IntoResponse
StatusCode
tokio::spawn
spawn_blocking
Mutex
RwLock
.await
Timeout
TraceLayer
CorsLayer
RequestBodyLimitLayer
ConcurrencyLimitLayer
BufferLayer
LoadShedLayer
reqwest::Client
axum::serve
with_graceful_shutdown
```

Pattern matches are only discovery hints.

Never create a finding solely because a construct exists.

---

# 11. Mandatory Analysis Domains

Systematically evaluate all applicable areas below.

## A. Handler Architecture

Verify whether handlers remain transport adapters.

Look for handlers containing:

```text
business rules
SQL
large workflows
repository orchestration
authorization policy implementation
complex transformations
retry logic
external-system orchestration
```

Determine whether application/domain responsibilities are leaking into HTTP handlers.

Do not report handler size alone.

Report responsibility violations.

---

# 12. HTTP / Domain Separation

Check whether:

```text
axum::Json
StatusCode
HeaderMap
Path
Query
Response
HTTP-specific error types
```

leak unnecessarily into:

```text
domain
application
core business logic
```

Evaluate actual dependency direction.

Do not demand artificial layering when the repository intentionally uses a simpler architecture and no meaningful coupling problem exists.

---

# 13. DTO Separation

Inspect whether external API DTOs are improperly reused as:

```text
domain models
database records
internal commands
```

Look specifically for:

```text
mass assignment risk
domain invariants weakened for Serde
persistence fields accidentally exposed
HTTP compatibility driving domain design
```

Only create a finding when coupling creates concrete engineering risk.

---

# 14. Extractors

Review:

```text
standard extractors
custom extractors
body-consuming extractors
extractor ordering
rejection handling
```

Check custom extractors for excessive hidden behavior.

Extractors should normally extract and validate request context, not become hidden business-service execution mechanisms.

---

# 15. Application State

Inspect all `State<T>` and related state composition.

Look for:

```text
service-locator AppState
request-specific state stored globally
Arc<Mutex<AppState>>
unnecessary shared ownership
mutable global state
large unstructured dependency bags
```

Check whether state contains cheap-to-clone handles and cohesive dependencies.

Do not flag a large AppState by field count alone.

Evaluate architectural cohesion.

---

# 16. Request Context

Inspect handling of:

```text
authenticated identity
tenant context
request IDs
locale
trace context
client metadata
```

Check whether critical request context is explicit and typed rather than hidden in globals or repeatedly reconstructed.

---

# 17. Error Architecture

Trace representative error flows:

```text
infrastructure error
→ application error
→ API error
→ HTTP response
```

Check for:

```text
raw internal errors returned externally
HTTP status codes inside domain code
handler-local duplicated error mappings
200 responses containing errors
all failures collapsed to 500
```

Determine whether error semantics are centralized enough to remain consistent.

---

# 18. RFC 9457

Where the API has adopted RFC 9457, verify:

```text
application/problem+json
stable problem type
HTTP status
machine-readable semantics
sanitized detail
consistent error schema
```

Do not require RFC 9457 when the repository explicitly defines another valid external contract.

If RFC 9457 is the repository standard, deviations are findings.

---

# 19. HTTP Semantics

Review representative routes for correct use of:

```text
GET
POST
PUT
PATCH
DELETE

200
201
204
400
401
403
404
409
422
429
5xx
```

Do not enforce REST aesthetics.

Report concrete semantic contract errors.

---

# 20. Router Composition

Inspect router structure.

Evaluate:

```text
capability-level composition
route ownership
nesting
middleware scope
fallback handling
method handling
```

Look for oversized central router modules and unclear capability boundaries only when they materially reduce maintainability.

---

# 21. Middleware

Inventory all middleware and Tower layers.

For each significant layer determine:

```text
purpose
scope
ordering
request path
response path
error behavior
resource implications
```

Middleware ordering is behavior, not formatting.

Axum integrates directly with Tower middleware, so layer composition must be evaluated as a service stack.

---

# 22. Middleware Scope

Check whether middleware is applied at the narrowest correct scope.

Examples:

```text
global tracing
protected-route authentication
admin authorization
endpoint-specific timeout
endpoint-specific body limit
```

Look for global middleware that creates unsafe or confusing exceptions.

---

# 23. Async Correctness

Inspect async request paths for:

```text
blocking filesystem calls
blocking SDKs
CPU-heavy work
long synchronous calculations
inappropriate cryptographic/compression work
```

Do not assume every synchronous operation is problematic.

Evaluate whether it can materially block Tokio executor threads.

---

# 24. `spawn_blocking`

Inspect uses of:

```rust
tokio::task::spawn_blocking
```

for:

```text
legitimate blocking work
bounded concurrency
error propagation
cancellation expectations
```

Do not recommend `spawn_blocking` for trivial work.

---

# 25. Spawned Tasks

Inspect:

```rust
tokio::spawn(...)
```

and equivalent task creation.

Determine:

```text
task owner
lifetime
failure handling
shutdown behavior
cancellation
duplicate execution semantics
durability requirements
```

Fire-and-forget business-critical work is a high-risk pattern.

---

# 26. Locks and `.await`

Review shared synchronization:

```text
Mutex
RwLock
Semaphore
DashMap or equivalent
```

Check whether guards are held across `.await`.

Determine whether lock scope is necessary.

Do not report every async mutex as a defect.

---

# 27. Shared Mutable State

Determine whether mutable in-process state creates:

```text
scaling inconsistency
concurrency risk
hidden global behavior
unnecessary synchronization
```

Consider whether ownership, channels or durable persistence would be more appropriate.

Do not prescribe actors or channels without concrete benefit.

---

# 28. Cancellation Safety

Inspect workflows involving:

```text
database write
external side effect
event publication
multi-step mutation
long-running request
```

Determine what happens if the request future is cancelled.

Do not assume timeout means the underlying side effect did not occur.

Report only concrete consistency risks.

---

# 29. Timeout Model

Determine whether meaningful request/dependency paths have intentional timeouts.

Inspect:

```text
HTTP request timeout
DB acquisition/query timeout where visible
external HTTP timeout
body handling timeout
background operation timeout
```

Check that timeout middleware maps correctly into HTTP semantics.

---

# 30. Backpressure

Evaluate protection of constrained resources:

```text
database pool
external API
CPU
blocking workers
memory
uploads
queues
expensive endpoints
```

Look for unbounded work creation.

Do not invent arbitrary concurrency numbers.

Report missing control where resource exhaustion is plausible.

---

# 31. Request Body Limits

Inspect endpoints accepting:

```text
JSON
multipart
uploads
bulk data
```

Determine whether body sizes are bounded appropriately.

Do not require the same limit globally for every endpoint.

---

# 32. Collection Bounds

Inspect HTTP endpoints returning or accepting potentially large collections.

Check:

```text
pagination
maximum page size
bulk record limits
streaming where justified
```

Unbounded `Vec<T>` APIs over potentially large datasets are candidates for investigation.

---

# 33. HTTP Clients

Inspect outbound client construction.

Check for:

```text
new client per request
missing pooling
missing connect/request timeout
unsafe redirect behavior
unbounded response handling
```

Deep SSRF/security analysis belongs to Security.

Axum audit focuses on lifecycle and resilience.

---

# 34. Graceful Shutdown

Inspect process startup and shutdown.

Determine whether production server startup provides:

```text
shutdown signal
cessation of new work
bounded in-flight completion
background-task shutdown
resource cleanup
```

Do not require elaborate shutdown logic for binaries that are clearly not long-running services.

---

# 35. Health Endpoints

If health endpoints exist, distinguish where relevant:

```text
liveness
readiness
```

Check whether liveness incorrectly depends on every external dependency and could create restart loops.

Deep information-disclosure analysis belongs to Security.

---

# 36. Observability

Inspect:

```text
tracing
request IDs
span construction
route templates
latency
error recording
downstream correlation
```

Determine whether production request flows are diagnosable.

Do not demand logging at every function.

---

# 37. Metric Cardinality

Where metrics exist, inspect labels for accidental high cardinality:

```text
request IDs
user IDs
resource IDs
raw URLs
```

Prefer route templates where applicable.

Only create a finding if telemetry implementation actually exhibits the issue.

---

# 38. OpenAPI / Contract Synchronization

Where OpenAPI is canonical or client generation depends on it, inspect whether implementation and contract remain synchronized.

Consider:

```text
DTOs
status codes
errors
authentication
pagination
enums
nullability
```

Do not require OpenAPI if the project does not use it.

---

# 39. Tests as Evidence

Inspect tests to determine whether apparent architectural or behavioral concerns are intentionally covered.

Axum audit may identify missing transport-level verification directly related to a finding.

Broad testing-strategy analysis belongs to the Testing skill.

---

# 40. Subagent Strategy

For repository-wide audits, use up to three parallel focused subagents.

Cursor subagents provide separate context windows and are appropriate for independent parallel workstreams.

Subagents MUST NOT modify repository files.

They return candidate findings only.

---

# 41. Subagent A — HTTP/API Architecture

Analyze:

```text
handlers
routers
extractors
DTOs
HTTP/domain separation
error mapping
RFC 9457
HTTP semantics
OpenAPI contracts
```

Return:

```text
candidate finding
locations
evidence
suspected standard violation
confidence
```

Do not produce final findings.

---

# 42. Subagent B — Async / State / Concurrency

Analyze:

```text
State<T>
shared ownership
Mutex/RwLock
locks across await
tokio::spawn
spawn_blocking
blocking work
cancellation
task lifecycle
graceful shutdown
```

Return candidate findings only.

---

# 43. Subagent C — Middleware / Resilience / Observability

Analyze:

```text
Tower layers
middleware ordering
middleware scope
timeouts
backpressure
body limits
HTTP clients
tracing
request IDs
metrics
health endpoints
```

Return candidate findings only.

---

# 44. Parent Validation

Subagent output is NOT authoritative.

The parent agent MUST validate every candidate by inspecting:

```text
implementation
callers
router composition
tests
configuration
architecture documentation
```

where relevant.

Reject false positives.

Merge duplicate symptoms into root-cause findings.

---

# 45. Evidence Requirement

A confirmed finding requires concrete repository evidence.

Normally include:

```text
file path
symbol or line
relevant implementation
caller/router/middleware context
test/config evidence where material
```

Never create a finding because something merely "looks suspicious."

---

# 46. Needs Investigation

If evidence is insufficient, place the concern under:

```text
Needs Investigation
```

Examples:

```text
performance requires measurement
production proxy behavior unknown
deployment timeout behavior unknown
external infrastructure owns body limits
runtime task behavior cannot be established
```

Do not present hypotheses as confirmed violations.

---

# 47. Finding IDs

Use:

```text
AXUM-001
AXUM-002
AXUM-003
...
```

IDs are immutable throughout remediation.

Never renumber existing findings.

---

# 48. Finding Status

Every confirmed finding starts as:

```text
OPEN
```

Allowed lifecycle states:

```text
OPEN
IN_PROGRESS
RESOLVED
NOT_APPLICABLE
ACCEPTED_RISK
BLOCKED
STALE
```

The audit skill normally creates only `OPEN`.

It MUST NOT autonomously set `ACCEPTED_RISK`.

---

# 49. Severity

Use:

```text
CRITICAL
HIGH
MEDIUM
LOW
INFO
```

Examples:

## CRITICAL

```text
credible systemic data corruption through cancellation/concurrency
catastrophic service-wide resource exhaustion under ordinary hostile input
```

## HIGH

```text
business-critical fire-and-forget side effect
serious async/concurrency defect
major HTTP contract violation
likely executor starvation
systemic missing resource bounds
```

## MEDIUM

```text
architectural coupling with concrete maintenance risk
inconsistent error contract
meaningful state/service-locator problem
missing timeout on material dependency path
```

## LOW

```text
localized idiomatic/architectural weakness
minor middleware scoping issue
small observability deficiency
```

Do not convert style preferences into severity.

---

# 50. Confidence

Each finding MUST have:

```text
HIGH
MEDIUM
LOW
```

confidence.

Use `Needs Investigation` instead of a confirmed finding when confidence is too low to support a violation.

---

# 51. Finding Format

Use exactly:

```markdown
## AXUM-001 — Short precise title

**Status:** OPEN
**Severity:** HIGH
**Confidence:** HIGH
**Standard:** §<section> <rule>
**Locations:**
- `src/api/articles.rs:...`
- `src/app.rs:...`

### Evidence

Concrete repository behavior.

### Why this violates the standard

Explain the engineering invariant.

### Impact

Concrete consequence.

### Required remediation

Describe the required outcome without over-prescribing implementation.

### Verification

Define how successful remediation must be proven.

### Resolution

Not yet implemented.
```

---

# 52. Root-Cause Deduplication

Do not create separate findings for every occurrence of the same systemic problem.

Example:

```text
8 handlers each contain authorization/business orchestration
```

may be one architectural finding with representative locations.

Prefer root causes over symptom counts.

---

# 53. Cross-Layer Findings

Axum issues may span multiple files/layers.

Example:

```text
handler accepts raw String ID
→ application accepts raw String
→ error mapping becomes inconsistent
```

Create one Axum finding when this forms one coherent root cause.

Do not split findings merely by file.

---

# 54. Verified Strengths

Explicitly record areas checked and found sound.

Examples:

```text
handlers consistently thin
application layer independent from Axum
middleware ordering intentionally composed
no locks held across await
graceful shutdown implemented
RFC 9457 errors consistent
request IDs propagated correctly
```

Only state strengths actually verified.

---

# 55. Audit Report Structure

Produce:

```markdown
# Rust Axum Engineering Audit

## Executive Summary

## Repository Baseline

## Scope

## Axum Architecture Overview

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

Empty severity sections may be omitted.

---

# 56. Executive Summary

State concisely:

```text
number of confirmed findings
highest severity
main systemic concern(s)
quality-gate status
audit completeness
```

Do not exaggerate.

---

# 57. Finding Summary Table

Use:

```markdown
| ID | Severity | Confidence | Status | Title |
|---|---|---|---|---|
| AXUM-001 | HIGH | HIGH | OPEN | ... |
```

Detailed finding sections remain authoritative.

---

# 58. Recommended Remediation Order

Order by risk and dependency.

Typical sequence:

```text
correctness / concurrency
→ HTTP contract correctness
→ resource protection
→ architecture/state boundaries
→ observability
→ maintainability
```

Group findings when one coherent implementation should address them together.

Do not create artificially tiny remediation slices.

---

# 59. Audit Completeness

An audit is complete only when:

1. canonical Axum standard was read
2. repository-specific instructions were read
3. Axum architecture was mapped
4. baseline commit/worktree was recorded
5. relevant quality gates were executed or limitations documented
6. all applicable mandatory analysis domains were reviewed
7. repository-wide audit used focused subagents where useful
8. every candidate finding was independently validated
9. duplicate symptoms were consolidated
10. uncertainty was separated from findings
11. verified strengths were recorded
12. report was persisted
13. production source was not modified

---

# 60. Final Rule

The audit must answer:

```text
What is demonstrably wrong?
Why is it wrong?
Where is the evidence?
What risk does it create?
What outcome is required?
How will a later fix prove resolution?
```

It must NOT answer by changing the implementation.

That belongs exclusively to:

```text
/rust-axum-engineering-fix
```
