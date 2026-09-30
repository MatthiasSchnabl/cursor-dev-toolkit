---
name: rust-sqlx-engineering-audit
description: >
  Perform a repository-wide evidence-based engineering audit of Rust SQLx and
  PostgreSQL persistence code against the canonical Rust SQLx + PostgreSQL
  Engineering Standard. Evaluates query correctness, compile-time verification,
  dynamic SQL, repositories, transactions, concurrency, constraints, migrations,
  connection pools, indexes, query performance, bulk operations, pagination,
  error mapping, observability and database integration testing. Produces a
  persistent audit ledger and never fixes findings.
---

# Rust SQLx + PostgreSQL Engineering Audit

Perform a complete evidence-based audit of SQLx/PostgreSQL persistence engineering.

This skill is READ-ONLY with respect to existing source, tests, migrations and database schema.

It may create exactly one persistent audit report under:

`docs/engineering-audits/sqlx/`

It MUST NOT fix findings.

The canonical standard is:

`../_standards/rust-sqlx-postgresql-engineering-standard.md`

Read that standard completely before evaluating the repository.

---

# 1. Core Contract

The workflow is:

```text
REPOSITORY DISCOVERY
        ↓
SQLX / POSTGRES VERSION DISCOVERY
        ↓
PERSISTENCE ARCHITECTURE DISCOVERY
        ↓
SCHEMA / MIGRATION DISCOVERY
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
/rust-sqlx-engineering-fix
```

---

# 2. Audit Output

Create:

```text
docs/engineering-audits/sqlx/
SQLX-AUDIT-YYYYMMDD-HHMMSS-<short-git-sha>.md
```

Example:

```text
SQLX-AUDIT-20260919-220000-a92d326.md
```

The report is the persistent ledger used by the fix skill.

---

# 3. Audit Frontmatter

Every audit MUST begin with:

```yaml
---
schema_version: 1
audit_id: SQLX-AUDIT-20260919-220000-a92d326
domain: rust-sqlx-postgresql
standard: rust-sqlx-postgresql-engineering-standard
standard_version: 1
baseline_commit: <full-git-sha>
baseline_branch: <branch>
baseline_worktree: clean
sqlx_version: <resolved-version>
postgresql_version: <known-version-or-unknown>
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

Never manufacture findings.

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

Record the worktree state.

Never reset or discard user changes.

---

# 5. Determine Actual SQLx Version

Inspect:

```text
Cargo.toml
workspace Cargo.toml
Cargo.lock
cargo metadata
cargo tree
```

Identify:

```text
sqlx version
sqlx-core version
sqlx-postgres version
sqlx-cli expectations
enabled SQLx features
runtime/TLS features
macros feature
migrate feature
json/uuid/time/chrono/decimal features
```

Do not assume the repository uses the newest SQLx release.

Version-specific API conclusions must match the resolved dependency.

---

# 6. Determine PostgreSQL Context

Where possible identify:

```text
PostgreSQL major version
managed/self-hosted
extensions
connection proxy/pooler
migration mechanism
schema ownership
runtime database identity
```

Sources may include:

```text
Docker Compose
deployment configuration
CI
README
migration setup
infrastructure documentation
```

If unknown, record the limitation.

Do not guess production PostgreSQL configuration from local defaults.

---

# 7. Repository Instructions

Read applicable:

```text
AGENTS.md
nested AGENTS.md
.cursor/rules/
README.md
CONTRIBUTING.md
architecture docs
ADRs
database documentation
Cargo.toml
migrations
SQL files
CI workflows
```

Repository-specific persistence decisions may specialize the general standard.

---

# 8. Read-Only Rule

AUDIT MUST NOT modify:

```text
Rust source
SQL source
migrations
schema
database data
Cargo.toml
Cargo.lock
.sqlx metadata
tests
CI
```

Do not run:

```text
cargo sqlx migrate run
cargo sqlx migrate revert
cargo sqlx prepare
cargo update
cargo fix
DDL/DML against production or shared DB
```

unless the operation is explicitly known to be against an isolated disposable audit environment and does not alter repository state.

Prefer read-only verification.

---

# 9. Persistence Architecture Discovery

Map:

```text
PgPool construction
repository adapters
repository traits/ports
SQLx row types
domain mappings
transactions
query modules
dynamic query builders
migrations
database configuration
read models
bulk/import logic
pagination
filtering
locking/concurrency
external database writers
```

Trace representative flows:

```text
Application Use Case
        ↓
Repository Port
        ↓
SQLx Adapter
        ↓
PgPool / Transaction
        ↓
PostgreSQL
```

---

# 10. Establish Quality Baseline

Use repository-defined commands.

Where applicable run read-only equivalents of:

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
```

If the repository intentionally uses SQLx offline metadata, run:

```bash
cargo sqlx prepare --check
```

or workspace equivalent where the tool is available.

Do not regenerate `.sqlx` during audit.

Record unavailable tooling as a limitation.

---

# 11. SQLx Compile-Time Verification

Inventory use of:

```text
query!
query_as!
query_scalar!
query_file!
query_file_as!
```

versus runtime:

```text
query
query_as
query_scalar
QueryBuilder
raw_sql
```

Static queries should normally prefer compile-time verification when practical.

Do not report runtime queries merely because macros exist.

Dynamic SQL and other justified cases legitimately require runtime construction.

---

# 12. `.sqlx` Offline Metadata

If SQLx macros are used without a live build database, inspect:

```text
.sqlx/
CI prepare --check
schema used to generate metadata
feature/target coverage
```

Look for stale or missing metadata.

Do not require offline mode if CI/build intentionally uses a schema-equivalent live database.

---

# 13. Compile-Time Schema Consistency

Determine whether the schema used for query verification corresponds to repository migrations.

Look for situations where:

```text
query macros validate against unrelated developer DB
```

while migrations represent a different schema.

Compile-time checking is only useful when the checked schema is authoritative.

---

# 14. Static vs Dynamic SQL

Prefer static SQL where query structure is static.

Use dynamic query building when structure genuinely varies.

Look for unnecessary string-built SQL where a static query is possible.

Do not force static macros onto genuinely dynamic filtering/sorting.

---

# 15. Dynamic SQL

Inspect every material `QueryBuilder`.

Search for:

```text
.push(
.push_bind(
format!(
ORDER BY
WHERE
JOIN
LIMIT
OFFSET
```

External values must be bound.

Dynamic identifiers/operators must come from trusted enumerations or mappings.

---

# 16. SQL Injection

A `QueryBuilder::push()` containing user-controlled values is a major finding candidate.

Values should normally use:

```text
.push_bind(...)
```

Do not accept manual quoting/escaping as equivalent to bind parameters.

---

# 17. Dynamic Identifiers

PostgreSQL bind parameters cannot represent identifiers.

Where dynamic:

```text
column
sort field
table
operator
direction
```

is required, map externally supplied choices through trusted Rust enums/mappings.

Never insert arbitrary request strings into SQL syntax.

---

# 18. Filter Architecture

For non-trivial dynamic filtering inspect whether the application uses a validated representation such as:

```text
Filter AST
typed predicates
validated sort enum
bounded operators
```

rather than passing fragments of SQL through layers.

---

# 19. Column Selection

Look for production:

```sql
SELECT *
```

where explicit projections would improve contract stability, performance or data exposure.

Do not flag trivial/admin/internal queries without consequence.

---

# 20. Fetch Semantics

Review use of:

```text
fetch_one
fetch_optional
fetch_all
fetch
execute
```

against expected cardinality.

Examples:

```text
exactly one
zero or one
many bounded
streaming
mutation
```

Cardinality mismatches are correctness issues.

---

# 21. Unbounded Fetches

Inspect:

```text
fetch_all
Vec<T>
```

on potentially large or user-controlled result sets.

Determine whether:

```text
pagination
streaming
aggregation
hard bounds
```

exist.

Do not flag naturally tiny lookup tables.

---

# 22. Persistence Models

Determine whether SQL row representations are improperly reused as:

```text
domain entities
API DTOs
commands
```

when responsibilities differ materially.

Do not require separate structs without actual semantic benefit.

---

# 23. Domain Mapping

Inspect conversions between persistence and domain models.

Look for:

```text
invalid domain states created from DB rows
unchecked enums
sentinel values
silent truncation
lost precision
```

Database decoding does not replace domain validation.

---

# 24. Domain IDs

Where important identifiers exist, evaluate whether raw:

```text
String
Uuid
i64
```

across unrelated concepts creates realistic mix-up risk.

Do not demand newtypes mechanically.

---

# 25. NULL Semantics

Inspect:

```text
Option<T>
NULL
NOT NULL
defaults
```

for semantic alignment.

Do not use sentinel values such as:

```text
0
""
1970-01-01
```

to represent absence unless the domain explicitly defines them.

---

# 26. Numeric Semantics

For monetary/exact decimal values, inspect use of:

```text
NUMERIC / DECIMAL
Decimal-compatible Rust types
```

rather than floating-point where exactness is required.

---

# 27. Timestamp Semantics

Inspect:

```text
TIMESTAMPTZ
TIMESTAMP
DATE
TIME
```

against domain meaning.

Real-world instants should normally have explicit timezone semantics.

Do not rewrite valid local-calendar concepts into timestamps unnecessarily.

---

# 28. Constraints

Review whether universal data invariants are enforced using suitable:

```text
PRIMARY KEY
FOREIGN KEY
UNIQUE
NOT NULL
CHECK
EXCLUDE
```

constraints.

Application-only enforcement is insufficient for invariants requiring global database correctness.

---

# 29. Uniqueness Races

Search patterns conceptually equivalent to:

```text
SELECT whether value exists
→ INSERT if not
```

without a database uniqueness constraint.

Concurrent correctness must rely on PostgreSQL constraints/atomic operations.

---

# 30. Foreign Keys

Inspect relationship integrity and delete/update semantics.

Review:

```text
ON DELETE
ON UPDATE
```

choices where material.

Do not assume every relationship requires a physical FK if architecture deliberately allows loose coupling, but require a concrete reason for critical relational invariants.

---

# 31. Foreign-Key Indexing

Remember PostgreSQL does not automatically create an index on the referencing side of every foreign key.

Evaluate workload before recommending indexes.

Do not create findings solely because an FK column lacks an index.

---

# 32. JSONB

Inspect JSONB use.

Good candidates:

```text
sparse attributes
semi-structured extension data
external payload preservation
```

Potential misuse:

```text
core relational entities hidden inside JSON
frequently queried strongly structured fields
constraints impossible to enforce
```

Do not reject JSONB categorically.

---

# 33. Arrays

Inspect PostgreSQL arrays where used.

Do not use arrays merely to avoid modeling relationships.

Arrays are appropriate for genuine atomic multi-valued attributes.

---

# 34. PgPool Lifecycle

There should normally be one shared pool per database role/target, created at application startup and cheaply cloned.

Look for:

```text
new pool per request
new pool per repository invocation
repeated connect
```

---

# 35. Pool Sizing

Inspect explicit:

```text
max_connections
min_connections
acquire_timeout
idle_timeout
max_lifetime
```

where configured.

Do not judge a numeric pool size without deployment context.

Evaluate the global connection budget across replicas and other consumers.

---

# 36. Connection Acquisition

Look for connections acquired much earlier than needed or held through unrelated work.

Preferred principle:

```text
acquire late
release early
```

---

# 37. External Calls While Holding Connection

Trace code that holds:

```text
PoolConnection
Transaction
```

while awaiting:

```text
HTTP
LLM
message broker
filesystem
slow external service
```

This is a major pool/resource risk unless transaction semantics truly require it.

---

# 38. Transactions

Inventory explicit transactions.

Determine whether each represents a real atomicity boundary.

Do not use transactions mechanically for every single query.

Do not split logically atomic multi-write operations across independent connections.

---

# 39. Transaction Ownership

Application/use-case layer should normally own business transaction scope.

Repository methods may participate in a transaction but should not arbitrarily define broader business atomicity.

Look for nested/hidden transaction semantics.

---

# 40. Commit / Rollback

Inspect transaction completion.

Successful workflows should explicitly commit.

Dropped transactions may roll back as a safety mechanism, but do not use implicit drop rollback as ordinary success-path control flow.

---

# 41. External Calls Inside Transactions

Avoid long external operations inside database transactions.

Risks include:

```text
locks held
connection held
deadlocks
transaction timeout
poor throughput
```

Use outbox/jobs/compensation patterns where cross-system atomicity requires architecture beyond one DB transaction.

---

# 42. Isolation Levels

Where correctness depends on transaction isolation, determine whether assumptions are explicit.

Do not assume PostgreSQL `READ COMMITTED` provides a stable transaction-wide snapshot.

If stronger semantics are required, verify actual transaction configuration.

---

# 43. Serializable Transactions

If `SERIALIZABLE` is used, inspect bounded whole-transaction retry behavior for serialization failures where required.

Do not retry only the final SQL statement from a failed transaction.

---

# 44. Deadlocks

Inspect multi-row/multi-table locking for inconsistent ordering.

Where deadlocks are realistically possible, check whether:

```text
lock ordering
bounded retry
short transactions
```

address them.

Do not invent deadlock findings without a plausible competing path.

---

# 45. Row Locks

Inspect:

```sql
FOR UPDATE
FOR NO KEY UPDATE
FOR SHARE
FOR KEY SHARE
```

where used.

Use the weakest lock that preserves semantics.

Do not replace optimistic concurrency with pessimistic locks without evidence.

---

# 46. Optimistic Concurrency

For concurrent mutable resources consider:

```text
version column
updated_at token
WHERE version = ?
rows_affected
```

where lost updates are a real risk.

Do not require versioning for every table.

---

# 47. Read-Then-Write Races

Look for:

```text
read value
modify in Rust
write value
```

where an atomic SQL update could avoid races.

Examples:

```sql
UPDATE ... SET count = count + 1
```

Use PostgreSQL atomically where appropriate.

---

# 48. `RETURNING`

Inspect unnecessary write-then-read round trips.

PostgreSQL `RETURNING` may improve correctness and reduce races/round trips.

Do not require it where the returned data is not needed.

---

# 49. Upserts

Inspect:

```sql
ON CONFLICT
```

semantics.

Do not use:

```sql
ON CONFLICT DO NOTHING
```

to silently hide data-integrity errors unless duplicate-ignore behavior is actually desired.

---

# 50. `rows_affected`

For mutations whose semantics require:

```text
exactly one row changed
at least one row changed
nothing changed = conflict/not-found
```

inspect `rows_affected`.

Do not discard cardinality information when correctness depends on it.

---

# 51. N+1 Queries

Inspect loops performing one DB query per item.

Look for:

```text
N+1
repeated lookups
repeated existence checks
```

Prefer set-based SQL/batching when the data volume justifies it.

---

# 52. Bulk Operations

Review bulk inserts/updates.

Possible mechanisms include:

```text
multi-row INSERT
QueryBuilder
arrays + UNNEST
COPY
temporary staging
```

Choose by actual volume and semantics.

Do not prescribe one mechanism for all batch sizes.

---

# 53. Bind Limits

Dynamic/bulk query construction must respect PostgreSQL/driver parameter constraints.

Batch large workloads deliberately.

Do not construct arbitrarily large `IN (...)` or multi-value queries.

---

# 54. Round Trips

Identify unnecessary sequences such as:

```text
EXISTS
→ SELECT
→ UPDATE
```

when one SQL statement could provide equivalent semantics.

Do not optimize round trips at the expense of clarity without material benefit.

---

# 55. Existence Queries

When only existence matters, prefer appropriate:

```sql
EXISTS
```

semantics over fetching complete records/counts.

Do not require pre-existence checks before writes when the write itself can establish the outcome safely.

---

# 56. Count Queries

Exact `COUNT(*)` can be expensive over large filtered datasets.

Inspect API designs requiring exact totals where data scale matters.

Do not label all counts problematic.

---

# 57. Pagination

Inspect:

```text
LIMIT
OFFSET
cursor
keyset
```

for potentially large datasets.

Requirements:

```text
bounded page size
stable deterministic ordering
```

Large OFFSET pagination is a performance concern where deep navigation is realistic.

---

# 58. Cursor Pagination

Where cursor/keyset pagination is used verify:

```text
stable ordering
unique tie-breaker
cursor validation
matching filter/sort context
```

Do not require cursor pagination for small datasets.

---

# 59. Sorting

Dynamic sort columns/directions must be validated through trusted mappings.

Stable pagination requires deterministic sorting.

---

# 60. Indexes

Evaluate indexes based on workload.

Do not recommend:

```text
index every WHERE column
```

Inspect:

```text
filter predicates
join keys
ordering
selectivity
write cost
composite order
```

---

# 61. Specialized Indexes

Use:

```text
partial
covering
GIN
GiST
BRIN
trigram
```

only where query workload supports them.

Do not create architecture findings merely because a theoretically useful index could exist.

---

# 62. Query Performance Evidence

Performance findings require evidence.

Prefer:

```text
EXPLAIN
EXPLAIN ANALYZE
BUFFERS
query statistics
realistic data volume
```

where safe and available.

Do not claim a query is slow merely by visual inspection unless complexity is structurally obvious.

---

# 63. `EXPLAIN ANALYZE` Safety

Remember `EXPLAIN ANALYZE` executes the query.

Do not run it against destructive statements or sensitive production workloads during an audit without explicit controlled conditions.

---

# 64. Planner Behavior

Do not force PostgreSQL index assumptions from Rust code.

The planner chooses execution plans based on statistics and costs.

Investigate estimate-vs-actual mismatches where relevant.

---

# 65. Text Search

Inspect broad:

```sql
ILIKE '%term%'
```

on large datasets.

Where search is material, evaluate:

```text
trigram
full-text search
specialized indexing
```

based on requirements.

---

# 66. Timeouts

Where configured, inspect:

```text
statement_timeout
lock_timeout
pool acquire timeout
```

and transaction duration.

Do not invent timeout values without operational context.

---

# 67. Idle Transactions

Long-lived idle transactions can retain locks/snapshots/resources.

Look for application patterns making this plausible.

---

# 68. Error Classification

Inspect whether persistence errors preserve useful distinctions such as:

```text
unique violation
foreign-key violation
serialization failure
deadlock
timeout
connection failure
not found
```

before mapping to application/API errors.

---

# 69. Constraint Identification

Prefer stable:

```text
SQLSTATE
named constraint
typed database error information
```

over parsing human-readable PostgreSQL error messages.

---

# 70. Error Leakage

Raw SQLx/PostgreSQL errors must not cross external API boundaries.

Preserve internal diagnostics separately.

---

# 71. Retry Semantics

Retry only genuinely transient failures.

Where a transaction must be retried, retry the whole transaction unit.

Avoid multi-layer retry amplification.

---

# 72. Migration Architecture

Inventory:

```text
migration files
execution mechanism
CI validation
deployment application
rollback policy
```

Treat migrations as production code.

---

# 73. Applied Migration Immutability

Already-applied production migrations should normally remain immutable.

Fix forward with a new migration.

Do not rewrite migration history casually.

---

# 74. Rolling Deployment Compatibility

For schema changes used across deployments evaluate:

```text
expand
compatible application deployment
backfill
contract
```

where zero/low-downtime rolling deployments matter.

Do not require expand-contract for single-instance maintenance deployments without need.

---

# 75. Destructive Migrations

Review:

```text
DROP COLUMN
DROP TABLE
type changes
NOT NULL additions
large rewrites
```

for data-loss and deployment compatibility risk.

---

# 76. Large Backfills

Large backfills require explicit operational planning.

Do not assume one transaction/update is safe at production scale.

---

# 77. Index Creation in Production

For large production tables, inspect whether index creation may require non-blocking/concurrent strategy.

Account for migration tool transaction behavior.

Do not prescribe `CONCURRENTLY` without confirming deployment and migration semantics.

---

# 78. Migration Privileges

Where observable, prefer separation between:

```text
runtime DB identity
migration identity
admin identity
```

Deep privilege/security analysis belongs to Security.

SQLx audit focuses on persistence architecture.

---

# 79. Prepared Statements / Poolers

If PgBouncer/proxies are used, inspect compatibility with SQLx prepared-statement behavior and pooler mode.

Do not create findings when no external pooler exists.

---

# 80. `raw_sql`

Inspect use of SQLx raw SQL APIs.

They may be appropriate for:

```text
DDL
migrations/admin
multi-statement SQL
```

Do not use raw multi-statement SQL as normal application DML without a concrete reason.

---

# 81. Database Functions / Triggers

Functions, triggers and generated columns may be appropriate.

Evaluate whether important hidden business behavior creates maintainability or correctness problems.

Do not require all logic to live in Rust.

---

# 82. Database Defaults

Inspect defaults for semantic alignment.

Determine intentionally whether timestamps/IDs/status defaults are generated in:

```text
application
database
```

Avoid multiple competing sources of truth.

---

# 83. Repository APIs

Repository interfaces should express domain capabilities.

Look for over-generic APIs such as:

```text
GenericRepository<T>
CRUD<T>
find_by_any_field(...)
```

when they obscure invariants.

Do not reject reusable repository helpers that remain semantically clear.

---

# 84. Cardinality in Repository Contracts

Method return types should communicate:

```text
must exist
may not exist
many
mutation outcome
```

Use:

```text
T
Option<T>
Vec<T>
result/count types
```

intentionally.

---

# 85. Hidden Database Calls

Inspect abstractions for surprising implicit queries.

Database access should remain understandable from use-case/repository code.

Avoid domain getters that secretly perform I/O.

---

# 86. Observability

Inspect whether material DB behavior can be diagnosed.

Useful signals:

```text
operation name
query duration
pool acquisition wait
transaction duration
rows affected
error category
```

Do not log bind values indiscriminately.

---

# 87. Query Logging

Avoid logging sensitive SQL parameters.

Prefer operation-level structural telemetry.

Raw SQL may itself contain sensitive literals when queries are built incorrectly.

---

# 88. Pool Metrics

For production services where DB saturation matters, evaluate observability of:

```text
connections used
idle
acquire wait
timeouts
```

Do not require metrics infrastructure in small/non-production tools without need.

---

# 89. Integration Tests

SQLx/PostgreSQL behavior should be tested against real PostgreSQL where semantics matter.

Inspect tests for:

```text
constraints
transactions
locking
migrations
PostgreSQL types
dynamic queries
pagination
```

Do not accept SQLite as proof of PostgreSQL-specific behavior.

---

# 90. Concurrency Tests

Where findings involve:

```text
unique races
optimistic locking
job claiming
deadlocks
idempotency
```

inspect whether deterministic concurrency tests exist.

---

# 91. Migration Tests

Important migrations should be tested against representative prior-state data where risk justifies it.

Deep test architecture remains owned by Testing.

---

# 92. Candidate Finding Rule

A suspicious SQL pattern is not automatically a finding.

Before reporting:

1. inspect SQL
2. inspect callers
3. inspect schema/constraints
4. inspect transaction context
5. inspect expected data volume
6. inspect tests
7. inspect repository architecture
8. determine concrete consequence

Only then create a confirmed finding.

---

# 93. Parallel Subagents

For repository-wide audits use up to three specialist subagents.

They MUST NOT modify source/schema.

Return candidate findings only.

---

# 94. Subagent A — Queries & Persistence Architecture

Analyze:

```text
query macros
runtime queries
QueryBuilder
SQL injection
DTO/row/domain mapping
repository contracts
fetch semantics
N+1
pagination
bulk operations
```

Return candidate findings with evidence.

---

# 95. Subagent B — Transactions & Data Integrity

Analyze:

```text
constraints
transactions
isolation
locks
optimistic concurrency
upserts
rows_affected
uniqueness races
connection lifetime
external calls in transactions
```

Return candidate findings only.

---

# 96. Subagent C — Schema, Performance & Operations

Analyze:

```text
migrations
indexes
query plans where evidence exists
pool configuration
timeouts
prepared statements
PostgreSQL-specific design
observability
integration testing
```

Return candidate findings only.

---

# 97. Parent Validation

Subagent output is NOT authoritative.

The parent MUST validate every candidate.

Reject false positives.

Merge symptoms sharing one root cause.

---

# 98. Finding IDs

Use:

```text
SQLX-001
SQLX-002
SQLX-003
...
```

IDs remain stable throughout remediation.

---

# 99. Finding Status

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

`ACCEPTED_RISK` requires explicit human/team decision.

---

# 100. Severity

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
credible systemic data corruption
unbound SQL injection with high-impact access
catastrophic concurrency/integrity failure
```

## HIGH

```text
lost-update race
broken transaction boundary
missing database invariant permitting invalid critical data
serious connection/lock exhaustion issue
```

## MEDIUM

```text
N+1 at material scale
unbounded result path
stale SQLx metadata undermining CI
weak persistence/domain boundary
important migration risk
```

## LOW

```text
localized unnecessary round trip
minor projection inefficiency
small repository maintainability issue
```

Severity follows actual risk.

---

# 101. Confidence

Every finding MUST have:

```text
HIGH
MEDIUM
LOW
```

If evidence is insufficient, use:

```text
Needs Investigation
```

instead of overstating certainty.

---

# 102. Finding Structure

Use:

```markdown
## SQLX-001 — Short precise title

**Status:** OPEN
**Severity:** HIGH
**Confidence:** HIGH
**Category:** <query / transaction / schema / performance / ...>
**Standard:** §<section> <rule>
**Locations:**
- `src/...`
- `migrations/...`

### Evidence

Concrete repository evidence.

### Failure Scenario

Explain the concrete persistence/concurrency/performance failure.

### Impact

Describe correctness, integrity, latency, resource or maintenance impact.

### Existing Controls

Controls already present, if relevant.

### Why this violates the standard

Explain the persistence engineering invariant.

### Required remediation

Describe the required outcome.

### Verification

Define tests, SQL checks, query-plan evidence or other proof required.

### Resolution

Not yet implemented.
```

---

# 103. Performance Findings

Do not label a query slow solely from syntax.

Classify unmeasured concerns under:

```text
Needs Investigation
```

unless structural complexity or obvious repeated work provides sufficient evidence.

When measurement is required, specify:

```text
representative dataset
EXPLAIN ANALYZE where safe
BUFFERS where relevant
latency/query-count metric
```

---

# 104. Root-Cause Deduplication

Prefer systemic findings.

Example:

```text
SQLX-004 — Article repository performs per-row enrichment queries
```

with multiple call sites is better than 30 identical N+1 findings.

---

# 105. Verified Strengths

Record actually verified strengths such as:

```text
all static queries compile-time checked
.sqlx metadata verified in CI
no unbound dynamic values found
transactions are short and explicit
critical invariants have database constraints
repository APIs encode cardinality clearly
PostgreSQL integration tests exercise migrations
```

Do not add generic praise.

---

# 106. Needs Investigation

Examples:

```text
index usefulness requires production data distribution
pool size depends on deployment replica count
query performance requires realistic volume
production PostgreSQL version unavailable
pooler mode unknown
```

Define a concrete investigation method.

---

# 107. Audit Report Structure

Produce:

```markdown
# Rust SQLx + PostgreSQL Engineering Audit

## Executive Summary

## Repository Baseline

## SQLx / PostgreSQL Baseline

## Persistence Architecture Overview

## Schema & Migration Overview

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

---

# 108. Summary Table

Use:

```markdown
| ID | Severity | Confidence | Status | Category | Title |
|---|---|---|---|---|---|
| SQLX-001 | HIGH | HIGH | OPEN | Transaction | ... |
```

---

# 109. Recommended Remediation Order

Normally prioritize:

```text
data corruption / injection
→ transactional correctness
→ constraints / concurrency
→ migration safety
→ resource exhaustion
→ query cardinality / N+1
→ measured performance
→ architecture / maintainability
```

Respect implementation dependencies.

---

# 110. Audit Completeness

Audit is complete only when:

1. canonical SQLx standard was read
2. actual SQLx version/features were established
3. PostgreSQL context was established or limitations recorded
4. repository instructions were read
5. persistence architecture was mapped
6. schema/migrations were inspected
7. quality/SQLx gates were run where available
8. queries were systematically reviewed
9. transactions/concurrency were reviewed
10. performance claims remained evidence-based
11. specialist subagents were used where useful
12. candidates were independently validated
13. duplicate symptoms were correlated
14. verified strengths were recorded
15. report was persisted
16. existing source/schema was not modified

---

# 111. Final Rule

The audit must answer:

```text
Does this persistence layer preserve data correctness and concurrency
semantics while using PostgreSQL and SQLx predictably, safely,
efficiently and observably?
```

It must not change the implementation.

Remediation belongs exclusively to:

```text
/rust-sqlx-engineering-fix
```
