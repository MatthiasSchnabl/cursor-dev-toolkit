# Testing & Quality Engineering Audit

## Contents

- §§1–10: Core Contract → Never Hide Existing Failures
- §§11–20: Mandatory Analysis Domains → Randomness
- §§21–30: Test Isolation → Cancellation and Timeout Tests
- §§31–40: Retry Tests → Flaky Tests
- §§41–50: Ignored Tests → CI Quality Gates
- §§51–60: CI Ordering → Assertions
- §§61–70: Test Code Quality → Finding IDs
- §§71–80: Finding Status → Remediation Order
- §§81–82: Audit Completeness → Final Rule

> Navigation: use this map to load only the sections required for the current phase. Do not preload the whole file unless the task genuinely requires it.


Perform a complete evidence-based audit of the repository's testing and quality engineering.

This skill is READ-ONLY with respect to existing production and test source code.

It may create exactly one persistent audit report under:

`docs/engineering-audits/testing/`

It MUST NOT fix findings.

The canonical standard is:

`../_standards/testing-quality-engineering-standard.md`

Use the standard's **Contents** map first and load the sections relevant to the current audit phase. Before closeout, verify that every applicable standard domain was covered.

---

# 1. Core Contract

The workflow is:

```text
REPOSITORY DISCOVERY
        ↓
TEST ARCHITECTURE DISCOVERY
        ↓
QUALITY-GATE BASELINE
        ↓
TEST EXECUTION / EVIDENCE
        ↓
PARALLEL SPECIALIST ANALYSIS
        ↓
PARENT VALIDATION
        ↓
DEDUPLICATION
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
/testing-quality-engineering-fix
```

---

# 2. Audit Output

Create:

```text
docs/engineering-audits/testing/
TEST-AUDIT-YYYYMMDD-HHMMSS-<short-git-sha>.md
```

Example:

```text
TEST-AUDIT-20260919-214500-a92d326.md
```

This file is the persistent ledger used by the fix skill.

---

# 3. Audit Frontmatter

Every report MUST begin with:

```yaml
---
schema_version: 1
audit_id: TEST-AUDIT-20260919-214500-a92d326
domain: testing-quality
standard: testing-quality-engineering-standard
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

If there are no confirmed actionable findings:

```yaml
audit_status: complete
implementation_status: resolved
```

Never manufacture findings merely to produce an audit backlog.

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

Also identify relevant testing tools from the repository.

Examples may include:

```text
cargo test
cargo nextest
cargo llvm-cov
proptest
insta
testcontainers
wiremock
mockall
criterion
cargo-mutants
Playwright
frontend test runners
custom test scripts
CI-specific test commands
```

Do not assume a tool is present.

---

# 5. Repository Instructions

Read applicable:

```text
AGENTS.md
nested AGENTS.md
.cursor/rules/
CONTRIBUTING.md
README.md
architecture docs
ADRs
Cargo.toml
workspace Cargo.toml
rust-toolchain.toml
CI workflows
Makefiles / justfiles / task runners
testing documentation
```

Repository-specific testing decisions may legitimately specialize the general standard.

---

# 6. Read-Only Rule

Do not modify:

```text
production source
tests
fixtures
snapshots
CI workflows
Cargo.toml
Cargo.lock
configuration
```

during the audit.

The only intended repository modification is creation of the audit report.

Do not run:

```text
cargo fix
cargo fmt
snapshot acceptance/update commands
automatic fixture regeneration
coverage mutation
dependency update commands
```

Use read-only equivalents.

---

# 7. Discover the Test Topology

Map the repository's existing test architecture.

Identify:

```text
unit tests
module-local tests
integration tests
router/API tests
database tests
migration tests
contract tests
end-to-end tests
property-based tests
fuzz tests
benchmarks
snapshot/golden tests
security tests
concurrency tests
performance/load tests
doctests
compile-fail tests
frontend tests where repository scope includes them
```

Also identify:

```text
shared fixtures
test builders
test helpers
mock servers
fakes
database setup
container setup
test data
test environment requirements
```

Do not infer test quality from file count.

---

# 8. Establish Quality Gates

Determine canonical repository commands.

Prefer repository-defined commands.

If none exist, establish appropriate Rust baselines such as:

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
```

For workspaces use relevant workspace/target settings.

If the repository uses SQLx, inspect whether:

```bash
cargo sqlx prepare --check
```

or equivalent is part of the expected gate.

Do not add tools or commands merely because the standard mentions them.

---

# 9. Execute Existing Tests

Run the strongest applicable existing test suites that can be executed safely.

Record:

```text
command
scope
duration where useful
result
failed tests
ignored tests
environment limitations
```

A failing test is evidence to investigate.

It is not automatically a Testing Engineering finding until its meaning is understood.

---

# 10. Never Hide Existing Failures

Do not:

```text
delete failing tests
mark tests ignored
update snapshots
relax assertions
retry until green
```

during audit.

Record current behavior exactly.

---

# 11. Mandatory Analysis Domains

Systematically evaluate all applicable areas below.

---

# 12. Test-Layer Architecture

Determine whether the repository has an appropriate balance between:

```text
domain/unit tests
application tests
integration tests
HTTP/router tests
database tests
contract tests
E2E tests
```

Do not enforce an abstract test pyramid mechanically.

Evaluate whether important behavior is being tested at an unnecessarily expensive or unrealistically isolated layer.

---

# 13. Domain Testability

Inspect critical domain/application logic.

Determine whether important:

```text
business invariants
calculations
state transitions
validation
authorization policies
transformations
```

have focused deterministic tests.

Do not require unit tests for trivial mappings/getters.

---

# 14. Behavior vs Implementation

Look for tests tightly coupled to:

```text
private method calls
internal call count
specific helper structure
incidental ordering
internal implementation details
```

when behavior could be tested more robustly.

Do not flag legitimate interaction tests where the interaction itself is contractual.

---

# 15. Regression Protection

Inspect recent bug-fix patterns and critical error handling.

Look for defects fixed without meaningful regression coverage where repository history makes that visible.

Do not assume every historical commit must contain a new test.

Confirm a realistic regression risk before creating a finding.

---

# 16. Failure-Path Coverage

Review critical behavior for tests covering meaningful failures such as:

```text
invalid input
not found
conflict
authentication failure
authorization failure
constraint violation
dependency error
timeout
transaction rollback
concurrent modification
malformed external data
```

Do not demand exhaustive permutations.

Prioritize meaningful risk.

---

# 17. Boundary Values

Check whether important domain and API boundaries are tested.

Examples:

```text
0
1
minimum
maximum
maximum + 1
empty
single value
maximum page size
maximum batch size
precision boundaries
timestamp boundaries
```

Only report missing boundary coverage where a plausible defect class exists.

---

# 18. Test Determinism

Inspect tests for dependence on:

```text
wall-clock time
sleep
thread scheduling
randomness
unordered iteration
shared external state
execution order
fixed ports
fixed filesystem paths
global mutable state
```

Separate actual flaky risk from harmless nondeterminism.

---

# 19. Sleeps

Search for:

```rust
tokio::time::sleep(...)
std::thread::sleep(...)
```

and equivalent.

Determine whether sleep is being used as synchronization rather than as the behavior under test.

Do not report legitimate timer behavior automatically.

---

# 20. Randomness

Inspect tests using random IDs/data.

Determine whether failures are reproducible.

Check for:

```text
deterministic seeds
preserved failing cases
randomness used only for uniqueness
```

Do not prohibit randomness when it does not affect test semantics.

---

# 21. Test Isolation

Determine whether tests depend on:

```text
other tests
shared mutable records
shared files
global process state
mutable environment variables
fixed network resources
```

Look for tests that pass individually but interfere under parallel execution.

---

# 22. Database Testing

Where PostgreSQL/SQLx behavior matters, identify whether tests actually use PostgreSQL.

Inspect coverage of:

```text
query mappings
constraints
transactions
migrations
null handling
PostgreSQL-specific types
locking
ON CONFLICT
RETURNING
JSONB
pagination/filtering
```

Using mocks or SQLite for PostgreSQL-specific semantics is a candidate finding.

---

# 23. Database Test Isolation

Determine how database tests isolate state.

Examples:

```text
transaction-per-test
schema-per-test
database-per-test
container-per-suite
container-per-test
unique namespaces
```

Inspect whether parallel execution can contaminate results.

Do not mandate a specific strategy if the current one is correct and reliable.

---

# 24. Migration Testing

Where migrations are material, inspect whether they are validated against:

```text
realistic prior schema state
existing data
constraints
repository queries
```

Do not assume a migration succeeding on an empty DB proves production safety.

Deep migration design remains primarily SQLx/PostgreSQL scope.

This audit focuses on whether migration behavior is meaningfully tested.

---

# 25. API / Router Tests

For Axum APIs inspect whether transport behavior is tested at the router/service level where appropriate.

Relevant contracts may include:

```text
status codes
headers
JSON schema
RFC 9457 responses
extractor rejection
authentication
authorization
middleware behavior
pagination
```

Do not require a TCP server when in-process router tests provide equivalent confidence.

---

# 26. Contract Tests

Identify externally visible contracts:

```text
HTTP API
OpenAPI
JSON exports
persisted formats
events
external integrations
```

Determine whether critical compatibility semantics are protected.

Do not require contract tests for purely internal unstable representations.

---

# 27. Backward Compatibility

Where persisted/external formats support older versions, inspect for representative compatibility fixtures.

Examples:

```text
historical API payloads
previous exports
older serialized forms
event versions
```

Do not require indefinite compatibility not promised by the product.

---

# 28. Authentication / Authorization Tests

Where applicable inspect negative cases:

```text
missing credentials
invalid credentials
expired credentials
valid credentials
forbidden identity
cross-resource access
cross-tenant access
```

Deep security analysis belongs to Security.

Testing audit asks whether required security behavior is actually protected by tests.

---

# 29. Concurrency Tests

Identify code whose correctness depends on concurrency.

Examples:

```text
optimistic locking
idempotency
job claiming
duplicate creation
row locking
competing updates
shared state
```

Determine whether concurrency is tested using deliberate coordination rather than timing luck.

---

# 30. Cancellation and Timeout Tests

Where cancellation/timeout semantics matter, inspect whether tests cover:

```text
operation before timeout
operation exceeding timeout
correct response
resource cleanup
partial side effects
```

Avoid requiring these tests for ordinary quick synchronous logic.

---

# 31. Retry Tests

Where retry logic exists verify whether tests cover:

```text
transient failure
permanent failure
eventual success
retry exhaustion
maximum attempts
```

Check whether tests avoid real production-scale backoff delays.

---

# 32. Property-Based Testing

Identify logic where property-based testing would provide materially better coverage.

Good candidates include:

```text
parsers
filter ASTs
serializers
normalizers
state transitions
round-trip conversions
complex input validation
```

Do not create a finding merely because `proptest` is absent.

Require concrete value over example-based tests.

---

# 33. Fuzz Testing

Inspect custom parsers/protocol processing for high-risk attacker-controlled input.

Determine whether fuzzing is justified.

Do not demand fuzzing for ordinary application CRUD.

---

# 34. Snapshot / Golden Tests

Inspect snapshot usage for:

```text
meaningful structured contracts
unstable dynamic values
mechanical snapshot acceptance
overly broad snapshots
weak semantic assertions
```

Snapshots are not inherently weak.

Report misuse, not existence.

---

# 35. Test Fixtures

Inspect fixtures for:

```text
unnecessary size
hidden shared state
large production-like dumps in ordinary tests
unclear assumptions
cross-test coupling
sensitive data
```

Minimal fixtures are generally preferable.

---

# 36. Test Builders

Determine whether builders/factories improve or obscure test intent.

Flag builders only when they hide critical state or create misleading defaults.

Do not require builders when direct construction is clearer.

---

# 37. Mocks

Inspect use of mocks.

Look for:

```text
mocking every internal layer
tests that only assert mock interactions
database behavior replaced entirely by mocks
network protocol assumptions never integration-tested
```

Do not ban mocks.

They remain appropriate for meaningful boundaries.

---

# 38. Fakes

Inspect whether fakes diverge materially from production semantics.

A fake that cannot reproduce important production constraints may create false confidence.

Only report this when concrete behavior is affected.

---

# 39. External Dependencies

Check whether ordinary CI tests rely directly on:

```text
production APIs
public internet
shared mutable external services
real customer systems
```

Tests should generally use controlled dependencies.

Dedicated external contract/sandbox suites may legitimately differ.

---

# 40. Flaky Tests

Search repository history/configuration and current suite behavior for evidence of flakiness.

Indicators include:

```text
retries
known flakes
ignored tests
timing sleeps
order dependence
intermittent failures
```

Do not label a test flaky without evidence.

---

# 41. Ignored Tests

Inspect:

```rust
#[ignore]
```

and equivalent disabled tests.

Determine whether each represents:

```text
intentional slow/special suite
or
silenced failure
```

Do not report legitimate intentionally separate suites.

---

# 42. Test Retries

If CI automatically retries tests, determine whether retry behavior masks deterministic defects.

Retry may be valid for infrastructure diagnostics.

It must not become silent acceptance of flaky product tests.

---

# 43. Coverage

If coverage tooling exists, inspect how it is used.

Determine whether:

```text
coverage is diagnostic
coverage regression is tracked
arbitrary percentage targets drive meaningless tests
critical code remains uncovered
```

Do not create a finding solely because total coverage is below a particular percentage.

---

# 44. Critical Coverage Gaps

Where coverage data is available, prioritize gaps in:

```text
domain invariants
error handling
security boundaries
transactions
compatibility logic
complex parsers
```

Do not prioritize trivial boilerplate merely because it is uncovered.

---

# 45. Mutation Testing

If mutation testing is present, inspect whether it is used sensibly.

If absent, recommend it only where:

```text
critical domain logic
mature tests
high correctness requirements
```

make it valuable.

Do not make mutation testing a default requirement.

---

# 46. Performance Tests

Inspect performance-sensitive paths and any benchmarks.

Determine whether benchmarks measure meaningful behavior.

Look for:

```text
unrealistic microbenchmarks
benchmarks disconnected from actual bottleneck
missing regression protection on proven hot paths
```

Do not require performance tests for ordinary non-critical code.

---

# 47. Load Tests

For APIs expected to operate under meaningful concurrency/volume, inspect whether load behavior is tested somewhere appropriate.

Relevant signals include:

```text
p95/p99
throughput
DB saturation
pool saturation
timeouts
backpressure
error rate
```

Do not require load tests in normal pull-request gates unless project requirements justify them.

---

# 48. Test Runtime

Measure or inspect major test-suite runtime where practical.

Determine whether excessive duration damages feedback quality.

Do not classify a suite as too slow without repository context.

---

# 49. Fast Feedback

Inspect whether developers/agents can run focused tests.

Look for monolithic test architecture where every tiny change requires an expensive full-stack environment.

Testing architecture should permit:

```text
focused test
→ affected suite
→ full verification
```

where practical.

---

# 50. CI Quality Gates

Inspect actual CI workflow.

Determine whether normal changes are gated by appropriate:

```text
formatting
compilation
linting
tests
database validation
SQLx metadata
contracts
security checks where delegated
```

Do not assume documentation matches actual CI.

Read workflow definitions.

---

# 51. CI Ordering

Check whether inexpensive deterministic gates run before expensive suites where useful.

This is normally a LOW/MEDIUM efficiency concern, not a correctness issue.

Do not over-prioritize pipeline optimization.

---

# 52. CI Reproducibility

Determine whether CI commands can be reproduced locally.

Look for hidden CI-only behavior, undocumented environment requirements or divergent toolchains.

A failure that developers cannot reproduce materially reduces quality.

---

# 53. Feature Matrix

Where Rust features materially alter behavior, inspect whether supported combinations are tested.

Do not recommend `--all-features` blindly.

Determine actual valid feature combinations.

---

# 54. Platform Matrix

Where platform-specific code exists, determine whether supported platforms receive relevant CI/test coverage.

Do not require multi-platform CI for software explicitly supporting only one target.

---

# 55. Rust Doctests

Inspect whether meaningful public examples exist and run where relevant.

Do not demand doctests for trivial APIs.

---

# 56. Compile-Fail Tests

Where type-level APIs intentionally reject invalid usage, assess whether compile-fail tests would materially protect the contract.

This is specialized and should not become routine advice.

---

# 57. Unsafe Code Tests

Where unsafe Rust exists, inspect whether additional validation exists where applicable:

```text
Miri
sanitizers
fuzzing
focused invariant tests
```

Deep unsafe-code correctness belongs to Rust audit.

Testing audit evaluates verification adequacy.

---

# 58. Test Data Security

Inspect fixtures for:

```text
real credentials
production tokens
personal data
customer exports
confidential data
```

Deep secret-management analysis belongs to Security, but test repositories must not contain inappropriate real data.

---

# 59. Test Naming

Review representative tests for semantic names.

Do not create one finding for occasional weak naming.

Create a systemic maintainability finding only when test failures are broadly difficult to understand because naming is poor.

---

# 60. Assertions

Inspect representative tests for:

```text
weak assertions
overly broad assertions
implementation-coupled assertions
missing semantic assertions
```

Do not demand more assertions simply for quantity.

---

# 61. Test Code Quality

Test code is production maintenance code.

Inspect systemic:

```text
duplication
opaque helpers
overly complex DSLs
giant fixtures
unmaintainable setup
```

Do not apply production abstraction standards mechanically to every small test.

Clarity is the priority.

---

# 62. LLM-Friendly Tests

Determine whether critical tests communicate expected behavior clearly enough for future engineers and agents.

Look for:

```text
descriptive domain values
semantic names
visible expectations
clear fixtures
```

Do not create purely stylistic findings.

---

# 63. Candidate Finding Rule

A test smell is not automatically a finding.

Before reporting:

1. inspect the test
2. inspect the behavior it protects
3. inspect related production code
4. inspect related tests
5. determine actual risk
6. check repository-specific policy
7. determine whether the standard applies

Only then create a finding.

---

# 64. Parallel Subagent Analysis

For repository-wide audits, use up to three parallel specialist subagents.

They MUST NOT modify files.

They return candidate findings only.

---

# 65. Subagent A — Test Architecture & Behavior

Analyze:

```text
unit/domain/application tests
behavior vs implementation
regression protection
failure paths
boundary values
fixtures/builders/mocks
contract clarity
test maintainability
```

Return candidate findings with evidence.

---

# 66. Subagent B — Integration, Database & Concurrency

Analyze:

```text
PostgreSQL tests
SQLx repositories
migration tests
API/router tests
external integrations
test isolation
parallelism
concurrency
timeouts
cancellation
retries
```

Return candidate findings only.

---

# 67. Subagent C — CI, Determinism & Quality Signals

Analyze:

```text
CI workflows
quality gates
flaky tests
ignored tests
sleeps
coverage
snapshots
property/fuzz testing opportunities
performance tests
feature/platform matrix
test runtime
```

Return candidate findings only.

---

# 68. Parent Validation

The parent agent independently validates every subagent candidate.

Inspect:

```text
test source
production source
CI/configuration
repository docs
related callers
related fixtures
```

Subagent output is not final evidence.

Reject false positives.

Merge duplicate symptoms.

---

# 69. Root-Cause Deduplication

Prefer one systemic finding over many symptoms.

Example:

```text
25 HTTP tests recreate the same brittle database fixture
and depend on global records
```

may represent one test-isolation architecture problem.

Do not create 25 individual findings unless remediation genuinely differs.

---

# 70. Finding IDs

Use:

```text
TEST-001
TEST-002
TEST-003
...
```

IDs remain stable throughout remediation.

---

# 71. Finding Status

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

The audit skill normally creates only `OPEN`.

`ACCEPTED_RISK` requires explicit human decision.

---

# 72. Severity

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

Rare. Examples:

```text
critical safety/security behavior completely unverified
while existing evidence indicates tests provide false confidence
```

Do not use CRITICAL merely because tests are absent.

## HIGH

Examples:

```text
critical business workflow has no regression protection
database tests use incompatible fake semantics and mask correctness risk
systemic flaky suite makes CI unreliable
critical concurrency behavior completely untested
```

## MEDIUM

Examples:

```text
meaningful failure paths missing
important contract not protected
test isolation architecture fragile
substantial implementation-coupled tests
important deterministic behavior relies on sleeps
```

## LOW

Examples:

```text
localized test maintainability issue
minor naming/assertion weakness
non-critical CI inefficiency
```

Do not equate "missing test" automatically with HIGH severity.

Severity follows risk.

---

# 73. Confidence

Every finding MUST use:

```text
HIGH
MEDIUM
LOW
```

If evidence is insufficient, prefer:

```text
Needs Investigation
```

instead of a speculative finding.

---

# 74. Finding Structure

Use:

```markdown
## TEST-001 — Short precise title

**Status:** OPEN
**Severity:** HIGH
**Confidence:** HIGH
**Standard:** §<section> <rule>
**Locations:**
- `tests/...`
- `src/...`
- `.github/workflows/...`

### Evidence

Concrete repository evidence.

### Why this violates the standard

Explain the quality invariant.

### Impact

Explain the actual regression, correctness, maintainability,
delivery or confidence risk.

### Required remediation

Describe the expected outcome without over-prescribing implementation.

### Verification

Define how remediation must be proven.

### Resolution

Not yet implemented.
```

---

# 75. Needs Investigation

Use for questions requiring unavailable evidence.

Examples:

```text
suspected performance regression needing realistic data
suspected flaky test without reproducible failure
external sandbox behavior unknown
coverage unavailable
load characteristics unknown
```

Include a concrete investigation method.

---

# 76. Verified Strengths

Record what was actually verified as sound.

Examples:

```text
domain tests are fast and isolated
PostgreSQL integration tests use real PostgreSQL
bug fixes consistently contain regressions
router tests verify RFC 9457 contracts
no test sleeps used for synchronization
CI quality gates are reproducible
```

Do not add generic praise.

---

# 77. Audit Report Structure

Produce:

```markdown
# Testing & Quality Engineering Audit

## Executive Summary

## Repository Baseline

## Test Architecture Overview

## Quality-Gate Results

## Test Execution Results

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

# 78. Executive Summary

State:

```text
number of findings
highest severity
main systemic quality risks
test/CI gate status
whether audit coverage was complete
```

Keep it concise and evidence-based.

---

# 79. Summary Table

Use:

```markdown
| ID | Severity | Confidence | Status | Title |
|---|---|---|---|---|
| TEST-001 | HIGH | HIGH | OPEN | ... |
```

---

# 80. Remediation Order

Prioritize:

```text
false confidence / critical gaps
→ flaky/determinism problems
→ data/integration correctness
→ regression protection
→ contract coverage
→ test architecture
→ CI efficiency
→ lower-value maintainability
```

Group coherent findings.

Do not create unnecessarily granular remediation slices.

---

# 81. Audit Completeness

Audit is complete only when:

1. canonical Testing & Quality Standard was read
2. repository-specific instructions were read
3. test topology was mapped
4. baseline commit/worktree was recorded
5. relevant quality gates were executed or limitations recorded
6. applicable test suites were executed
7. all applicable analysis domains were reviewed
8. repository-wide analysis used specialist subagents where useful
9. candidates were independently validated
10. duplicate symptoms were consolidated
11. findings were separated from hypotheses
12. verified strengths were recorded
13. report was persisted
14. existing production/test source was not modified

---

# 82. Final Rule

This audit evaluates:

```text
How much confidence does the repository's current test and quality
system actually provide?
```

It does not optimize for:

```text
number of tests
coverage percentage
number of assertions
testing framework sophistication
```

The output must identify real quality risks with evidence.

Remediation belongs exclusively to:

```text
/testing-quality-engineering-fix
```
