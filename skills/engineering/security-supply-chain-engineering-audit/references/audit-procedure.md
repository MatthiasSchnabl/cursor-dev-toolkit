# Security & Supply-Chain Engineering Audit

## Contents

- §§1–10: Core Contract → Existing Security Tooling
- §§11–20: Mandatory Analysis Domains → Secrets
- §§21–30: Secret-Bearing Types → SSRF
- §§31–40: Outbound HTTP → Password Handling
- §§41–50: Randomness → Cargo.lock
- §§51–60: RustSec / Advisories → Third-Party CI Actions
- §§61–70: Release Pipeline → Subagent A — Application & API Security
- §§71–80: Subagent B — Injection & Integration Security → MEDIUM
- §§81–90: LOW → Executive Summary
- §§91–94: Summary Table → Final Rule

> Navigation: use this map to load only the sections required for the current phase. Do not preload the whole file unless the task genuinely requires it.


Perform a complete evidence-based security and software supply-chain audit.

This skill is READ-ONLY with respect to existing repository source and configuration.

It may create exactly one persistent audit report under:

`docs/engineering-audits/security/`

It MUST NOT fix findings.

The canonical standard is:

`../_standards/security-supply-chain-engineering-standard.md`

Read that standard completely before evaluating the repository.

---

# 1. Core Contract

The workflow is:

```text
REPOSITORY DISCOVERY
        ↓
TRUST-BOUNDARY DISCOVERY
        ↓
SECURITY BASELINE
        ↓
DEPENDENCY / CI EVIDENCE
        ↓
PARALLEL SPECIALIST ANALYSIS
        ↓
PARENT VALIDATION
        ↓
ROOT-CAUSE CORRELATION
        ↓
PERSISTENT SECURITY AUDIT
```

Never perform:

```text
AUDIT
→ FIX
```

Remediation belongs exclusively to:

```text
/security-supply-chain-engineering-fix
```

---

# 2. Audit Output

Create:

```text
docs/engineering-audits/security/
SECURITY-AUDIT-YYYYMMDD-HHMMSS-<short-git-sha>.md
```

Example:

```text
SECURITY-AUDIT-20260919-220000-a92d326.md
```

This file is the persistent remediation ledger.

---

# 3. Audit Frontmatter

Every report MUST begin with:

```yaml
---
schema_version: 1
audit_id: SECURITY-AUDIT-20260919-220000-a92d326
domain: security-supply-chain
standard: security-supply-chain-engineering-standard
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

Never create artificial security findings merely to populate an audit.

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

Record whether the worktree is clean.

Never reset, discard or modify user changes.

---

# 5. Repository Security Context

Read applicable:

```text
AGENTS.md
nested AGENTS.md
.cursor/rules/
README.md
CONTRIBUTING.md
architecture documentation
ADRs
Cargo.toml
Cargo.lock
rust-toolchain.toml
Dockerfiles
compose files
deployment manifests
CI workflows
OpenAPI/API specifications
authentication documentation
authorization documentation
environment/configuration templates
```

Do not evaluate individual code fragments without first understanding the repository's trust model.

---

# 6. Read-Only Rule

Do not modify:

```text
source
tests
Cargo.toml
Cargo.lock
CI
Dockerfiles
deployment manifests
configuration
secrets
snapshots
```

during the audit.

Do not run:

```text
cargo update
cargo fix
automatic dependency upgrades
secret rotation
migration commands
container mutation
deployment actions
```

The only intended repository write is the audit report.

---

# 7. Security Architecture Discovery

Map:

```text
public entry points
HTTP/API endpoints
authentication mechanisms
authorization model
roles/scopes/permissions
tenant boundaries
service-to-service authentication
session/cookie usage
webhooks
file uploads
outbound HTTP
database identities
secret sources
logging/tracing
admin/debug endpoints
background jobs
CI/CD
release artifacts
containers/runtime
external integrations
```

Identify which data and capabilities are security-sensitive.

---

# 8. Trust Boundaries

Explicitly identify trust transitions such as:

```text
Internet → reverse proxy
proxy → Axum
authenticated user → resource
tenant → tenant data
application → PostgreSQL
application → external API
external webhook → application
uploaded file → processing
source repository → CI
CI → release artifact
artifact → runtime environment
```

A security audit without trust-boundary analysis is incomplete.

---

# 9. Security Baseline Commands

Use repository-defined security commands when available.

Where applicable and already supported by the repository/tooling, inspect results from:

```bash
cargo check
cargo clippy -- -D warnings
cargo test
cargo tree
cargo tree -d
cargo audit
cargo deny check
```

Do not install or modify dependencies merely to run an audit unless explicitly permitted.

If tooling is unavailable, record the limitation.

---

# 10. Existing Security Tooling

Identify whether the repository uses:

```text
cargo-audit
cargo-deny
Dependabot
Renovate
CodeQL
secret scanning
SAST
container scanning
SBOM generation
artifact signing
provenance/attestation
```

Presence alone is not proof of security.

Absence alone is not automatically a finding.

Evaluate risk and repository requirements.

---

# 11. Mandatory Analysis Domains

Systematically evaluate all applicable areas below.

---

# 12. Authentication

Inspect how identities are authenticated.

Evaluate:

```text
token verification
issuer
audience
expiration
not-before
accepted algorithms
required claims
session validation
service identities
```

Do not assume a JWT is trusted because it can be decoded.

---

# 13. Authentication Boundary

Determine whether protected business operations can execute before authentication is established.

Look for:

```text
unprotected routes
optional identity where mandatory
authentication bypasses
development bypasses
duplicated inconsistent auth logic
```

---

# 14. Authorization

Trace representative protected operations.

Evaluate:

```text
who may perform the operation
on which object
under which tenant
with which role/scope/policy
```

Authentication is not authorization.

---

# 15. Object-Level Authorization

Search for resource access patterns based on externally supplied IDs.

Check whether:

```text
GET /resource/{id}
PATCH /resource/{id}
DELETE /resource/{id}
```

verify access to the specific object.

Do not rely on unpredictable UUIDs as access control.

---

# 16. Function-Level Authorization

Inspect privileged functionality:

```text
admin endpoints
management operations
imports
exports
configuration changes
bulk operations
user/role management
```

Frontend visibility does not count as authorization.

---

# 17. Property-Level Authorization

Inspect request DTOs and update models.

Look for mass-assignment risks where clients can set fields such as:

```text
role
tenant_id
is_admin
owner_id
status
approval_state
internal flags
```

merely because those fields exist in a deserializable struct.

---

# 18. Tenant Isolation

Where multi-tenancy exists, inspect:

```text
request context
application policies
repository interfaces
SQL predicates
cache keys
jobs
exports
background processing
```

for cross-tenant access risk.

Do not trust a client-supplied tenant ID unless tenant selection is explicitly authorized.

---

# 19. Fail-Closed Behavior

Review security-dependent failure behavior.

Examples:

```text
authorization service unavailable
token verification configuration missing
policy resolution failure
tenant context missing
```

Determine whether failure results in denial rather than unintended access.

---

# 20. Secrets

Search for potential secrets in owned repository content.

Inspect:

```text
source
configuration
examples
tests
fixtures
Dockerfiles
CI workflows
migration files
scripts
```

Do not print full discovered secrets into the audit.

If a real credential is found:

* redact it
* identify location
* treat exposure as potentially high severity
* recommend rotation in remediation

Never reproduce secret values.

---

# 21. Secret-Bearing Types

Inspect whether secrets can leak through:

```text
Debug
Display
Serialize
errors
logs
traces
```

Do not create findings based purely on a type name.

Verify actual exposure path.

---

# 22. Logging and Tracing

Inspect:

```text
Authorization headers
cookies
tokens
request payloads
response payloads
personal data
financial data
confidential domain fields
```

for accidental logging.

Review middleware as well as explicit log calls.

---

# 23. Error Disclosure

Inspect client-facing error paths.

Look for exposure of:

```text
SQL errors
stack traces
filesystem paths
internal hostnames
source locations
constraint internals
secret values
```

Distinguish internal observability from external API output.

---

# 24. Input Validation

Review externally controlled input for:

```text
type
length
range
format
cardinality
semantic constraints
resource cost
```

Deserialization alone is not domain validation.

---

# 25. Resource Limits

Inspect potentially attacker-controlled resource consumption:

```text
request bodies
uploads
page size
bulk records
filter depth
IN lists
regexes
CPU-heavy operations
database scans
external fan-out
queues
background tasks
```

Valid input can still be abusive.

---

# 26. SQL Injection

Inspect all dynamic SQL paths.

Check:

```text
format!
string concatenation
QueryBuilder::push
dynamic identifiers
ORDER BY
operators
table/column selection
```

Untrusted values must be bound.

Dynamic identifiers must derive from trusted mappings/enums.

---

# 27. Command Injection

Search process execution.

Inspect:

```text
Command
sh -c
bash -c
cmd /c
PowerShell
external utilities
```

Determine whether untrusted data becomes command syntax.

Prefer structured argument passing.

---

# 28. Path Traversal

Inspect filesystem operations influenced by external data.

Review:

```text
filenames
relative paths
archive extraction
exports
imports
temporary files
```

for traversal or symlink escape risk.

---

# 29. File Upload Security

Where uploads exist inspect:

```text
size
filename handling
storage path
authorization
content assumptions
processing
retention
```

Do not trust MIME types or extensions alone when content type is security-relevant.

---

# 30. SSRF

Identify caller-influenced outbound URLs.

Evaluate:

```text
scheme validation
destination restrictions
localhost
loopback
private networks
link-local
metadata services
redirects
DNS behavior
IPv4/IPv6
```

Do not declare SSRF merely because the application uses HTTP clients.

There must be attacker influence over destination selection.

---

# 31. Outbound HTTP

Inspect clients for:

```text
TLS verification
timeouts
redirect policy
response-size behavior
authentication
destination validation
```

Third-party responses remain untrusted input.

---

# 32. CORS

Inspect production CORS configuration.

Look for:

```text
wildcard origins
credential combinations
unnecessarily broad methods
unnecessarily broad headers
```

Do not treat CORS as authorization.

---

# 33. CSRF

Where browser cookie/session authentication exists, evaluate CSRF protections.

Do not require CSRF mechanisms for architectures where the threat does not apply.

---

# 34. Cookies and Sessions

Where cookies are security-relevant inspect:

```text
Secure
HttpOnly
SameSite
scope
lifetime
session rotation
```

Do not assume bearer-token API rules apply to cookie-based sessions.

---

# 35. Redirects

Inspect caller-controlled redirect destinations.

Check for open-redirect behavior where relevant.

---

# 36. Forwarded Headers

Inspect use of:

```text
Forwarded
X-Forwarded-For
X-Forwarded-Proto
X-Forwarded-Host
```

Determine whether the application trusts them only behind controlled infrastructure.

---

# 37. Webhooks

Where incoming webhooks exist inspect:

```text
signature validation
timestamp validation
replay protection
raw payload verification
authentication
```

Do not rely on source IP alone unless explicitly part of the provider's security contract.

---

# 38. Replay / Idempotency

Inspect security-sensitive or economic operations for replay risk.

Examples:

```text
payments
orders
webhooks
bulk actions
external commands
```

Determine whether idempotency keys/timestamps/nonces are required.

---

# 39. Cryptography

Search for cryptographic functionality.

Flag custom implementations of:

```text
encryption
hashing protocols
signatures
password hashing
token generation
```

where established libraries should be used.

Do not report ordinary use of mature cryptographic libraries as a risk merely because cryptography exists.

---

# 40. Password Handling

Where passwords exist inspect:

```text
storage algorithm
verification
logging
serialization
reset tokens
```

Fast general-purpose hashes are not suitable password storage.

---

# 41. Randomness

Inspect security tokens and identifiers requiring unpredictability.

Do not use timestamps, counters or predictable PRNGs for:

```text
sessions
reset tokens
API credentials
nonces
```

where unpredictability is required.

---

# 42. Unsafe Rust and FFI

Security audit should inspect security-sensitive `unsafe` and FFI boundaries.

Deep memory-safety correctness belongs primarily to Rust Engineering.

Security scope asks whether unsafe/FFI materially expands attack surface or processes attacker-controlled data.

---

# 43. Serialization / Deserialization

Inspect externally deserialized structures for:

```text
unbounded collections
deep nesting
unknown-field behavior
mass assignment
internal fields accidentally exposed
```

Successful Serde parsing does not make data trusted.

---

# 44. Response Data Minimization

Inspect API responses for unnecessary exposure.

Do not return complete internal/database models when only a subset is intended.

Look specifically for sensitive internal fields.

---

# 45. Admin / Debug Surfaces

Identify:

```text
debug routes
profilers
internal diagnostics
configuration dumps
test endpoints
auth bypasses
admin APIs
metrics
```

Determine whether they can be exposed unintentionally in production.

---

# 46. Health / Metrics Information Disclosure

Inspect public operational endpoints for unnecessary information such as:

```text
dependency URLs
internal hostnames
configuration
versions
credentials
topology
```

Do not require all health information to be hidden if infrastructure legitimately needs it.

---

# 47. Database Privileges

Where repository/deployment configuration reveals database identities, inspect whether runtime access is excessive.

Look for runtime dependence on:

```text
superuser
schema owner
migration privileges
DDL access
```

Do not infer production privileges solely from local development defaults.

---

# 48. Dependency Inventory

Inspect:

```bash
cargo tree
cargo tree -d
```

where applicable.

Understand:

```text
direct dependencies
large transitive graphs
duplicate versions
Git dependencies
path dependencies
native dependencies
proc macros
build scripts
```

Do not report dependency count by itself as a defect.

---

# 49. Dependency Necessity

Identify dependencies that materially expand attack surface without obvious need.

Do not recommend custom reimplementation of mature security-sensitive functionality merely to remove a dependency.

---

# 50. Cargo.lock

For deployable applications, inspect whether dependency resolution is pinned appropriately.

Do not modify the lock file during audit.

---

# 51. RustSec / Advisories

Where `cargo audit` can be executed, inspect all advisories.

For each relevant advisory determine:

```text
affected package
affected version
whether vulnerable functionality is reachable/relevant
fixed version
existing mitigation
```

Do not treat every advisory as equally exploitable.

---

# 52. cargo-deny

Where configured, inspect:

```text
advisories
bans
licenses
sources
```

Do not create duplicate findings for one root cause reported by multiple tools.

---

# 53. Advisory Exceptions

Inspect ignored advisories for:

```text
reason
owner
expiry/review
mitigation
```

Permanent unexplained ignores are candidates for findings.

---

# 54. Dependency Sources

Review:

```text
Git dependencies
mutable branches
untrusted registries
uncontrolled forks
external path dependencies
```

for reproducibility and supply-chain risk.

---

# 55. Crate Features

Inspect security-relevant features such as:

```text
dangerous modes
legacy protocol support
native TLS options
debug/test helpers
broad default features
```

Only report when enabled features create material attack surface or unsafe behavior.

---

# 56. Build Scripts / Proc Macros

Remember that:

```text
build.rs
procedural macros
```

execute during build.

Treat them as supply-chain executable code.

Do not automatically mark all proc macros as risky.

---

# 57. CI Trust Boundary

Read actual CI workflows.

Inspect:

```text
workflow triggers
pull_request
pull_request_target
fork behavior
secret availability
token permissions
third-party actions
artifact upload
deployment jobs
```

Do not rely on intended CI architecture described only in documentation.

---

# 58. CI Secrets

Check whether privileged secrets may be exposed to untrusted PR code.

Remember repository code may execute via:

```text
tests
build.rs
proc macros
scripts
build tools
```

---

# 59. CI Permissions

Inspect workflow token permissions.

Prefer least privilege.

Look for broad repository/package/write privileges where unnecessary.

---

# 60. Third-Party CI Actions

Inspect external workflow actions.

Consider:

```text
maintainer trust
mutable tags
immutable revisions
permissions
secret access
```

Do not demand commit-SHA pinning unless repository policy or threat model justifies it, but record material uncontrolled execution risk.

---

# 61. Release Pipeline

Where repository contains release workflows inspect:

```text
artifact source
branch/tag restrictions
build reproducibility
artifact publication permissions
signing/attestation
environment approvals
```

Do not invent release requirements absent from the project.

---

# 62. Containers

Where containers are used inspect:

```text
runtime user
base image
multi-stage build
installed tooling
embedded secrets
exposed ports
filesystem permissions
```

Do not flag a non-root absence automatically without considering runtime requirements and deployment controls.

---

# 63. Runtime Debug Material

Check whether production images include unnecessary:

```text
source
compiler
Cargo
build cache
debug tools
credentials
```

when a minimal runtime image is feasible.

---

# 64. Configuration Defaults

Inspect defaults for insecure fallback behavior such as:

```text
auth disabled
CORS wildcard
TLS disabled
unlimited bodies
debug logging
default credentials
```

Production security should fail closed.

---

# 65. Development Bypasses

Search for:

```text
dev_mode
skip_auth
disable_security
allow_all
test_auth
```

and equivalent.

Determine whether such behavior can accidentally reach production.

---

# 66. Security Testing

Inspect whether critical security requirements have negative tests.

Examples:

```text
invalid auth rejected
forbidden resource rejected
cross-tenant access rejected
mass assignment prevented
oversized request rejected
secret not exposed
invalid signature rejected
```

Deep test-strategy quality belongs to Testing.

Security audit evaluates whether major controls have evidence.

---

# 67. Incident Readiness

Where relevant inspect whether security events are diagnosable through:

```text
request IDs
principal identifiers
audit events
deployment version
dependency inventory
```

without excessive sensitive logging.

Do not demand compliance-grade audit infrastructure for low-risk systems without requirement.

---

# 68. Audit Events

Where privileged actions exist, inspect whether necessary security audit events are recorded.

Examples:

```text
role changes
credential operations
administrative mutations
sensitive exports
security configuration changes
```

Only create findings where auditability is actually required by risk/product context.

---

# 69. Parallel Subagents

For repository-wide audits use up to three specialist subagents.

They MUST NOT modify repository files.

They return candidate findings only.

---

# 70. Subagent A — Application & API Security

Analyze:

```text
authentication
authorization
object-level authorization
tenant isolation
mass assignment
input validation
resource limits
CORS/CSRF
cookies
webhooks
error disclosure
logging
```

Return candidate findings with concrete evidence.

---

# 71. Subagent B — Injection & Integration Security

Analyze:

```text
SQL injection
command injection
path traversal
file upload
SSRF
outbound HTTP
redirects
deserialization
cryptography
randomness
unsafe/FFI exposure
```

Return candidate findings only.

---

# 72. Subagent C — Supply Chain & Delivery Security

Analyze:

```text
Cargo dependencies
advisories
cargo-deny policy
Cargo.lock
Git dependencies
build scripts
proc macros
CI trust
CI secrets
workflow permissions
containers
release artifacts
runtime privileges
```

Return candidate findings only.

---

# 73. Parent Validation

Subagent output is NOT authoritative.

For every candidate the parent MUST:

1. inspect the cited code/config
2. inspect relevant callers/flows
3. inspect tests where useful
4. inspect deployment assumptions where available
5. determine actual trust boundary
6. assess exploit preconditions
7. reject false positives
8. merge duplicates

Only validated issues become findings.

---

# 74. No Speculative Vulnerabilities

Never describe a theoretical pattern as an exploitable vulnerability without evidence.

Distinguish:

```text
confirmed vulnerability
confirmed weakness
hardening opportunity
needs investigation
```

Do not sensationalize.

---

# 75. Finding IDs

Use:

```text
SEC-001
SEC-002
SEC-003
...
```

IDs remain stable throughout remediation.

---

# 76. Finding Status

Initial status:

```text
OPEN
```

Allowed lifecycle:

```text
OPEN
IN_PROGRESS
RESOLVED
NOT_APPLICABLE
ACCEPTED_RISK
BLOCKED
STALE
```

The audit agent MUST NOT autonomously assign `ACCEPTED_RISK`.

---

# 77. Severity

Use:

```text
CRITICAL
HIGH
MEDIUM
LOW
INFO
```

Severity MUST reflect:

```text
impact
exploitability
required access
affected data/capability
scope
existing mitigations
```

Do not base severity solely on tool-provided CVSS.

---

# 78. CRITICAL

Examples may include:

```text
credible unauthenticated remote compromise
broad authentication bypass
cross-tenant unrestricted data access
committed active production credential with major privilege
remote code execution path
```

Use CRITICAL sparingly.

---

# 79. HIGH

Examples may include:

```text
object-level authorization bypass
significant tenant-isolation failure
reachable SQL/command injection
serious SSRF into trusted network
high-impact secret exposure
unsafe CI secret exposure to untrusted code
relevant exploitable dependency vulnerability
```

---

# 80. MEDIUM

Examples may include:

```text
meaningful information disclosure
insufficient resource bounds
weak token claim validation
overly broad privileges
missing negative security regression protection
unsafe dependency/source policy with credible risk
```

---

# 81. LOW

Examples may include:

```text
localized hardening deficiency
minor information exposure
low-impact excessive permissions
security-maintainability concern
```

INFO should be used sparingly for material observations that are not violations.

---

# 82. Confidence

Every finding MUST use:

```text
HIGH
MEDIUM
LOW
```

Low-confidence security concerns should usually become:

```text
Needs Investigation
```

rather than vulnerabilities.

---

# 83. Required Finding Structure

Use:

```markdown
## SEC-001 — Short precise title

**Status:** OPEN
**Severity:** HIGH
**Confidence:** HIGH
**Category:** <authentication / authorization / injection / supply-chain / ...>
**Standard:** §<section> <rule>
**Locations:**
- `src/...`
- `.github/workflows/...`

### Evidence

Concrete repository evidence without reproducing secrets.

### Attack / Failure Preconditions

What must be true for exploitation or security failure.

### Impact

What confidentiality, integrity, availability or privilege is affected.

### Existing Mitigations

Controls already present, if any.

### Why this violates the standard

Explain the security invariant.

### Required remediation

Describe the required security outcome.

### Verification

Define how remediation must be proven.

### Resolution

Not yet implemented.
```

---

# 84. Redaction Rule

Never place actual discovered secrets into the audit.

Use:

```text
[REDACTED]
```

and enough metadata to identify the affected location.

---

# 85. Root-Cause Deduplication

Do not create separate findings for every endpoint affected by one authorization architecture defect.

Prefer:

```text
SEC-003 — Repository lookup contract lacks tenant scoping
```

with all affected paths listed.

Root cause matters more than symptom count.

---

# 86. Cross-Domain Security Findings

Correlate issues across:

```text
Axum
application
SQLx
tests
CI
deployment
```

when they form one coherent vulnerability.

Do not split one exploit chain artificially into several low-value findings.

---

# 87. Needs Investigation

Use when material evidence is missing.

Examples:

```text
production proxy trust configuration unavailable
actual DB runtime privileges unknown
cloud metadata network reachability unknown
token issuer configuration external to repository
production secret-management behavior unknown
```

Define exactly what must be checked.

---

# 88. Verified Strengths

Record security controls actually verified.

Examples:

```text
OIDC audience and issuer validated
tenant scope included in all relevant repository queries
no raw SQL concatenation found
no secrets committed
runtime DB account separated from migration identity
cargo audit clean
CI PR workflows do not expose privileged secrets
```

Do not add generic praise.

---

# 89. Audit Report Structure

Produce:

```markdown
# Security & Supply-Chain Engineering Audit

## Executive Summary

## Repository Baseline

## Security Architecture & Trust Boundaries

## Security Tooling / Verification Results

## Finding Summary

## Critical Findings

## High Findings

## Medium Findings

## Low Findings

## Supply-Chain Findings

## Verified Strengths

## Needs Investigation

## Audit Limitations

## Recommended Remediation Order
```

Avoid duplicating one finding under several sections.

---

# 90. Executive Summary

State:

```text
number of confirmed findings
highest severity
main systemic risks
whether known-vulnerability checks passed
whether audit coverage was complete
```

Do not claim the repository is "secure."

An audit provides evidence, not proof of absence of vulnerabilities.

---

# 91. Summary Table

Use:

```markdown
| ID | Severity | Confidence | Status | Category | Title |
|---|---|---|---|---|---|
| SEC-001 | HIGH | HIGH | OPEN | Authorization | ... |
```

---

# 92. Remediation Order

Normally prioritize:

```text
active secret / direct compromise
→ authentication / authorization
→ tenant isolation
→ injection / SSRF / code execution
→ data exposure
→ CI/supply-chain compromise
→ resource abuse
→ hardening / least privilege
```

Account for dependencies between fixes.

---

# 93. Audit Completeness

Audit is complete only when:

1. canonical Security Standard was read
2. repository instructions were read
3. security architecture/trust boundaries were mapped
4. baseline revision/worktree was recorded
5. applicable security tooling was executed or limitations documented
6. all applicable mandatory security domains were reviewed
7. dependency/supply-chain configuration was inspected
8. CI trust boundaries were inspected
9. specialist subagents were used where useful
10. all candidates were independently validated
11. exploitability was not overstated
12. duplicates were correlated by root cause
13. secrets were redacted
14. verified strengths were recorded
15. report was persisted
16. source/configuration was not modified

---

# 94. Final Rule

The audit must distinguish:

```text
confirmed vulnerability
confirmed security weakness
hardening opportunity
needs investigation
```

Never inflate one category into another.

Security findings require evidence, attack/failure preconditions, impact and a verifiable remediation outcome.

Remediation belongs exclusively to:

```text
/security-supply-chain-engineering-fix
```
