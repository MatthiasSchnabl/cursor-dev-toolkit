---
name: rust-engineering
description: >
  Audit and remediate Rust code against the repository's canonical Rust Engineering Standard.
  Use for repository-wide or scoped Rust engineering audits covering correctness, type safety,
  ownership, borrowing, allocations, error handling, unsafe code, concurrency, idiomatic Rust,
  maintainability and performance. Audit is the default operation. Fix operations are allowed
  only when a completed open audit already exists and must resolve tracked findings from that audit.
---

# Rust Engineering Audit & Remediation

This skill performs evidence-based Rust engineering audits and controlled remediation.

The canonical standard is:

`references/rust-engineering-standard.md`

The standard defines **what good Rust engineering means**.

This skill defines **how the repository is inspected, findings are recorded, and fixes are controlled**.

Do not duplicate or reinterpret the standard unless necessary to apply it to concrete repository evidence.

---

# 1. Operations

This skill supports exactly two operations:

```text
audit
fix
```

## Default

If no operation is explicitly requested:

```text
audit
```

is mandatory.

Examples:

```text
/rust-engineering
/rust-engineering audit
```

both mean:

```text
perform a fresh Rust engineering audit
```

A request such as:

```text
/rust-engineering fix
```

means:

```text
locate the newest eligible open Rust audit
and remediate only findings recorded in that audit
```

---

# 2. Fundamental Workflow Rule

The mandatory lifecycle is:

```text
AUDIT
  ↓
record findings
  ↓
FIX
  ↓
verify implementation
  ↓
update finding statuses
  ↓
mark audit resolved when no actionable findings remain
```

A fix MUST always be based on a previously completed audit.

Never perform:

```text
FIX
→ discover arbitrary additional issues
→ fix those too
```

Never silently perform a new audit as part of a fix.

If no eligible open audit exists:

```text
STOP
```

and report:

```text
No open Rust engineering audit is available.
Run /rust-engineering audit first.
```

Do not invent findings in fix mode.

---

# 3. Audit State

Audit reports are persistent repository artifacts.

Store them under:

```text
docs/engineering-audits/rust/
```

Use filenames:

```text
RUST-AUDIT-YYYYMMDD-HHMMSS-<short-git-sha>.md
```

Example:

```text
RUST-AUDIT-20260918-214500-a92d326.md
```

Audit state must remain machine-readable and human-readable.

---

# 4. Audit Frontmatter

Every audit MUST begin with YAML frontmatter:

```yaml
---
schema_version: 1
audit_id: RUST-AUDIT-20260918-214500-a92d326
domain: rust
standard: rust-engineering-standard
standard_version: 1
baseline_commit: a92d326d758bf5ca754bbe0e6e829b2da9151849
baseline_branch: main
baseline_worktree: clean
created_at: 2026-09-18T21:45:00Z
updated_at: 2026-09-18T21:45:00Z
audit_status: complete
implementation_status: open
---
```

Allowed `audit_status` values:

```text
complete
incomplete
```

Allowed `implementation_status` values:

```text
open
partial
resolved
requires-reaudit
```

Meaning:

### `open`

Audit is complete and contains one or more unresolved findings.

### `partial`

At least one finding has been resolved, but actionable findings remain.

### `resolved`

No actionable findings remain.

### `requires-reaudit`

Repository evolution invalidated material audit evidence.

A fix MUST NOT operate on an audit marked:

```text
resolved
requires-reaudit
```

---

# 5. Audit With No Findings

If the audit finds no confirmed violations:

```yaml
audit_status: complete
implementation_status: resolved
```

Do not manufacture low-value findings merely to populate the report.

A clean audit is a valid result.

---

# 6. Dirty Working Tree

Before auditing run:

```bash
git status --short
```

Record whether the working tree is clean.

An audit MAY inspect a dirty working tree.

However:

```yaml
baseline_worktree: dirty
```

must be recorded.

If the repository is materially dirty, clearly state that findings include uncommitted state.

Do not modify source files during audit.

A fix based on a dirty-baseline audit requires additional freshness validation before modification.

---

# 7. Canonical Standard

Before analysis, read completely:

```text
references/rust-engineering-standard.md
```

Also inspect repository-specific instructions that may override or specialize the general standard:

```text
AGENTS.md
nested AGENTS.md
.cursor/rules/
repository architecture documentation
ADRs
CONTRIBUTING.md
README.md
Cargo.toml workspace configuration
rust-toolchain / rust-toolchain.toml
```

Repository-specific rules may make a general recommendation intentionally inapplicable.

Do not report a violation until this has been considered.

---

# 8. Audit Scope Discovery

Before reviewing code, build a repository inventory.

Identify:

```text
workspace root
workspace members
Rust crates
libraries
binaries
examples
build scripts
procedural macros
FFI modules
unsafe code
feature flags
target-specific code
generated code
vendored code
tests
benchmarks
```

Read:

```text
Cargo.toml
Cargo.lock where present
rust-toolchain.toml where present
```

Do not assume a single-crate repository.

---

# 9. Exclusions

Do not perform normal source-quality review on:

```text
target/
vendored third-party source
generated source
build artifacts
external submodules not owned by the repository
```

unless repository-owned logic materially depends on generated or vendored behavior being reviewed.

Clearly distinguish:

```text
owned source
generated source
third-party source
```

---

# 10. Establish Baseline

Record:

```bash
git rev-parse HEAD
git branch --show-current
git status --short
rustc --version
cargo --version
```

Inspect the repository's canonical verification commands before inventing new ones.

Prefer repository-defined commands when available.

---

# 11. Read-Only Audit Rule

AUDIT mode MUST NOT modify production source.

Do not run modifying commands such as:

```text
cargo fmt
cargo fix
cargo update
automatic refactoring
migration commands
code generators that modify tracked source
```

Use read-only equivalents.

Allowed examples include:

```bash
cargo fmt --check
cargo check
cargo clippy
cargo test
cargo metadata
cargo tree
```

Use `--locked` when a valid committed lock file exists and doing so matches repository policy.

Do not mutate `Cargo.lock` merely to complete an audit.

---

# 12. Verification Baseline

Run the strongest relevant repository-defined verification.

If no stronger repository-specific procedure exists, evaluate approximately:

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
```

For workspaces, use appropriate workspace/target options.

Do NOT blindly use:

```text
--all-features
```

when features may be mutually exclusive, platform-specific, or intentionally unsupported together.

Record:

```text
command
result
exit status
relevant output
```

in the audit.

A failing command is evidence.

It is not automatically a finding until the cause has been understood.

---

# 13. Candidate Discovery

Search broadly for potential risk patterns.

Examples include:

```text
.clone()
.to_owned()
.to_string()
.collect()
unwrap()
expect()
panic!()
todo!()
unimplemented!()
unsafe
#[allow(...)]
Arc<Mutex<_>>
Arc<RwLock<_>>
Box<dyn Any>
String-based domain states
large public APIs
very large functions/modules
blocking code in async contexts
locks across await
unchecked conversions
casts
duplicate business logic
```

Candidate discovery is NOT finding creation.

A pattern match is only a reason to inspect context.

Never report:

```text
".clone() exists"
```

as a finding by itself.

---

# 14. Parallel Subagent Analysis

For repository-wide audits, delegate independent analysis passes to subagents.

Use up to three focused analysis workstreams.

They may run in parallel.

## Subagent A — Type Safety & Correctness

Investigate:

```text
domain modelling
invalid representable states
primitive obsession
Option/Result semantics
conversion traits
error handling
panic paths
visibility
public APIs
invariants
type-driven design
```

Return candidate findings only.

Do not modify files.

---

## Subagent B — Ownership, Memory & Concurrency

Investigate:

```text
unnecessary cloning
unnecessary allocations
ownership transfers
intermediate collections
shared ownership
Arc/Rc use
Mutex/RwLock use
locks across await
unsafe
FFI
task ownership
concurrency hazards
blocking behavior
```

Return candidate findings only.

Do not modify files.

---

## Subagent C — Maintainability & Performance

Investigate:

```text
idiomatic Rust
DRY violations
premature abstractions
module boundaries
function responsibilities
dependency use
API design
comments/rustdoc
algorithmic complexity
obvious hot-path inefficiencies
avoidable materialization
resource handling
```

Return candidate findings only.

Do not modify files.

---

# 15. Parent Agent Owns Findings

Subagents produce:

```text
candidate findings
```

only.

The parent agent MUST independently validate every candidate before placing it in the audit.

Subagent output is not authoritative evidence.

For every candidate:

1. inspect the cited code
2. inspect relevant callers
3. inspect relevant types
4. inspect relevant tests
5. inspect configuration when relevant
6. determine whether the standard applies
7. determine actual runtime/domain effect
8. reject false positives
9. merge duplicates

Only then create a finding.

---

# 16. Evidence Requirement

Every confirmed finding MUST contain concrete repository evidence.

Evidence should normally include:

```text
file path
line or symbol
relevant implementation
relevant caller/context when needed
relevant test/configuration evidence
```

Do not report a violation based solely on intuition.

If evidence is insufficient:

```text
do not create a confirmed finding
```

Instead place it under:

```text
Needs Investigation
```

---

# 17. Findings vs Needs Investigation

Use:

```text
Confirmed Finding
```

only when the repository contains sufficient evidence.

Use:

```text
Needs Investigation
```

when:

```text
required runtime behavior cannot be proven
external constraints are unknown
performance impact requires measurement
architectural intent is unclear
the relevant environment is unavailable
```

Never inflate uncertainty into severity.

---

# 18. Finding IDs

Use stable sequential IDs inside one audit:

```text
RUST-001
RUST-002
RUST-003
```

IDs MUST NOT change during subsequent fixes.

Do not renumber findings after resolving them.

---

# 19. Finding Status

Each finding MUST contain one status.

Allowed values:

```text
OPEN
IN_PROGRESS
RESOLVED
NOT_APPLICABLE
ACCEPTED_RISK
BLOCKED
STALE
```

Initial status for confirmed findings:

```text
OPEN
```

Only FIX mode may normally transition `OPEN` to another implementation state.

---

# 20. Status Semantics

## OPEN

Confirmed and not yet implemented.

## IN_PROGRESS

Implementation has begun but verification is incomplete.

Do not leave a finding in this state at the end of a normal fix run unless work genuinely remains incomplete.

## RESOLVED

Remediation has been implemented and verification succeeded.

## NOT_APPLICABLE

Later examination demonstrated that the finding does not apply.

Must include evidence.

## ACCEPTED_RISK

Risk intentionally remains.

The agent MUST NOT assign this status autonomously.

It requires explicit user/team decision.

## BLOCKED

Valid finding cannot currently be implemented because a concrete dependency or prerequisite is missing.

## STALE

The repository changed enough that the original evidence can no longer be relied upon.

Requires a new audit.

---

# 21. Severity

Use:

```text
CRITICAL
HIGH
MEDIUM
LOW
INFO
```

## CRITICAL

Examples:

```text
credible memory unsafety / undefined behavior
severe concurrent correctness defect
likely production data corruption
systemic catastrophic failure in normal operation
```

## HIGH

Examples:

```text
likely production correctness defect
significant panic/crash path
major resource/concurrency problem
serious invariant violation
```

## MEDIUM

Examples:

```text
maintainability defect with concrete risk
material unnecessary allocations
weak type modelling causing plausible defects
non-critical error-handling weakness
```

## LOW

Examples:

```text
localized idiomatic weakness
minor maintainability concern
small but concrete inefficiency
```

## INFO

Use sparingly for non-violating observations that materially help engineering decisions.

Do not downgrade real problems merely because fixing them is difficult.

Do not upgrade stylistic preferences into risk findings.

---

# 22. Confidence

Every finding MUST have:

```text
HIGH
MEDIUM
LOW
```

confidence.

## HIGH

Direct repository evidence demonstrates the violation and impact.

## MEDIUM

Violation is demonstrated but some impact assumptions remain.

## LOW

Evidence indicates a likely problem but important runtime/context information is unavailable.

Low-confidence items should frequently become:

```text
Needs Investigation
```

rather than confirmed findings.

---

# 23. Required Finding Structure

Every confirmed finding must use this structure:

```markdown
## RUST-001 — Short precise title

**Status:** OPEN
**Severity:** HIGH
**Confidence:** HIGH
**Standard:** §<section> <rule>
**Locations:**
- `src/example.rs:120-148`
- `src/caller.rs:44-62`

### Evidence

Concrete description of what the repository does.

### Why this violates the standard

Explain the violated engineering invariant.

### Impact

Describe the concrete correctness, safety, memory, performance,
maintenance, or architectural consequence.

### Required remediation

Describe the expected engineering outcome.

Do not prescribe an unnecessarily specific implementation when
multiple valid implementations exist.

### Verification

Define how the remediation must be proven.

### Resolution

Not yet implemented.
```

---

# 24. No Duplicate Findings

Do not create multiple findings for symptoms of one root cause.

Example:

```text
clone in handler
clone in service
clone in repository
```

may represent one ownership-design problem.

Prefer one root-cause finding with multiple locations when appropriate.

---

# 25. Systemic Findings

If the same defect pattern exists broadly, create one systemic finding when that is more useful than dozens of identical findings.

List representative and affected locations.

Example:

```text
RUST-004 — Domain identifiers are represented as raw String across application boundaries
```

Do not create 40 separate "use a newtype" findings.

---

# 26. Performance Findings

Do not claim a performance defect solely because code looks inefficient.

Distinguish:

```text
obvious structural waste
```

from:

```text
performance hypothesis
```

Examples of sufficiently direct structural evidence may include:

```text
known O(n²) implementation where O(n) is straightforward
repeated materialization of very large collections
N repeated full scans
blocking CPU-heavy operation on executor path
```

Otherwise classify performance concerns under:

```text
Needs Investigation
```

with a measurement plan.

---

# 27. Unsafe Review

Every `unsafe` block encountered must be intentionally reviewed.

Check:

```text
scope
SAFETY comment
documented invariants
aliasing
lifetimes
pointer validity
thread safety
FFI contracts
tests
```

Absence of a problem is recorded as a verified strength, not a fabricated finding.

---

# 28. Clone Review

Do not ban `clone()`.

For meaningful clones determine:

```text
why ownership duplication is required
size/cost of value
frequency
hot-path relevance
whether borrowing or ownership transfer is clearer
whether avoiding clone would create disproportionate complexity
```

Only unnecessary or materially harmful cloning becomes a finding.

---

# 29. Panic Review

Review:

```text
unwrap()
expect()
panic!()
unreachable!()
```

in context.

Acceptable invariant-enforcing panics must not be reported merely because they exist.

Expected runtime failures must not panic.

Tests/examples may use different conventions from production code.

---

# 30. Abstraction Review

Do not enforce DRY mechanically.

A small amount of duplication is preferable to a false abstraction.

Create abstraction findings only when duplicated knowledge or business rules create real divergence risk.

---

# 31. Dependency Review

Rust audit may identify:

```text
unnecessary dependency
duplicate capability
excessive generic dependency use
inappropriate abstraction dependency
```

but vulnerability/license/supply-chain analysis belongs primarily to the Security skill.

Do not duplicate Security audit scope unnecessarily.

---

# 32. Testing Scope

Rust audit may inspect tests to understand behavior and validate findings.

Deep testing-strategy evaluation belongs to the Testing & Quality skill.

Do not create broad testing findings here unless missing tests directly prevent verification of a Rust engineering invariant.

---

# 33. Audit Report Structure

The final audit file MUST contain:

```markdown
# Rust Engineering Audit

## Executive Summary

## Repository Baseline

## Scope

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

Omit empty severity sections if useful.

---

# 34. Executive Summary

Keep the executive summary concise.

State:

```text
number of confirmed findings
highest severity
most important systemic risks
whether baseline quality gates passed
whether audit coverage was complete
```

Do not exaggerate.

---

# 35. Finding Summary

Include a compact table:

```markdown
| ID | Severity | Confidence | Status | Title |
|---|---|---|---|---|
| RUST-001 | HIGH | HIGH | OPEN | ... |
```

This table is a summary only.

Detailed sections remain authoritative for evidence and remediation.

---

# 36. Verified Strengths

Explicitly document areas that were checked and found sound.

Examples:

```text
no unsafe code present
error handling consistently typed
domain identifiers use newtypes
no locks held across await
public API surface appropriately narrow
```

Only state strengths actually verified.

Do not use generic praise.

---

# 37. Audit Limitations

Record unavailable evidence.

Examples:

```text
platform-specific target unavailable
integration tests require inaccessible infrastructure
benchmark environment unavailable
feature combination not runnable
```

An audit with limitations may still be complete if the limitation is explicit and does not invalidate the reviewed findings.

---

# 38. Recommended Remediation Order

Order remediation based on dependencies and risk.

Normally:

```text
correctness / safety
→ architectural invariants
→ ownership/resource issues
→ maintainability
→ measured performance
```

Do not create artificial task fragmentation.

Group coherent findings when one implementation should resolve them together.

---

# 39. FIX Operation — Preconditions

FIX mode MUST first locate audit files:

```text
docs/engineering-audits/rust/RUST-AUDIT-*.md
```

Select the newest audit satisfying:

```text
audit_status: complete
implementation_status: open
```

or:

```text
implementation_status: partial
```

Do not use:

```text
resolved
requires-reaudit
```

audits.

If none exists:

```text
STOP
```

Do not audit automatically.

---

# 40. Audit Selection

If several open audits exist, use the newest applicable audit by:

```text
created_at
```

unless the user explicitly selects an audit ID.

Older overlapping audits may subsequently be marked superseded only when that relationship has been explicitly established.

Do not silently combine separate audit histories.

---

# 41. Freshness Check Before Fix

Before modifying source:

1. read `baseline_commit`
2. inspect current `HEAD`
3. inspect repository changes since baseline
4. compare changed paths with open finding evidence
5. determine whether each finding remains valid

If changes do not materially affect an open finding:

```text
continue
```

If evidence has been invalidated:

```text
Status: STALE
```

and update overall audit:

```yaml
implementation_status: requires-reaudit
```

Do not rediscover a replacement finding in FIX mode.

A fresh AUDIT is required.

---

# 42. Fix Scope

FIX mode may modify code only to remediate findings recorded in the selected audit.

Do not perform unrelated:

```text
refactoring
cleanup
dependency updates
format rewrites
architecture redesign
```

unless required to implement a recorded finding correctly.

Avoid scope creep.

---

# 43. Fix Order

Resolve findings in dependency-aware order.

Normally prioritize:

```text
CRITICAL
HIGH
MEDIUM
LOW
```

but architecture dependencies may require a different coherent order.

Do not fix findings in arbitrary file order.

---

# 44. Revalidate Before Editing

Before implementing each finding:

1. reread its evidence
2. confirm the issue still exists
3. inspect directly affected callers/tests
4. understand expected behavior
5. identify the smallest coherent remediation

If evidence no longer applies:

```text
STALE
```

or:

```text
NOT_APPLICABLE
```

with justification.

Do not force a fix merely because an audit once contained the finding.

---

# 45. Implementation Quality

A fix must comply with the full Rust Engineering Standard.

Do not resolve one finding by introducing another known violation.

Prefer root-cause remediation over symptom suppression.

Do not silence:

```text
compiler warnings
Clippy
tests
lints
```

to make implementation appear successful.

---

# 46. Fix Verification

Every finding must define its verification plan before implementation.

After implementation, run:

```text
targeted tests/checks
```

first.

Then run the relevant broader repository quality gates.

A finding may only become:

```text
RESOLVED
```

after required verification succeeds.

---

# 47. Regression Tests

If a finding represents incorrect behavior or a bug, add a regression test where practical.

A fix that cannot be protected by a meaningful regression test must explain why.

Do not add meaningless tests merely to satisfy this rule.

---

# 48. Resolution Record

When resolved, replace:

```text
### Resolution

Not yet implemented.
```

with:

```markdown
### Resolution

**Result:** RESOLVED
**Implemented:** <short description>
**Verification:**
- `<command>` — PASS
- `<test>` — PASS

**Evidence:**
- `src/example.rs:...`
- `tests/example.rs:...`

**Resolved at:** <timestamp>
```

Do not claim PASS if a command was not run successfully.

---

# 49. Blocked Findings

If remediation cannot proceed:

```text
Status: BLOCKED
```

and document the exact blocker.

Examples:

```text
missing architectural decision
unavailable external dependency
required API contract unknown
incompatible upstream version
```

Do not convert blocked findings to resolved.

---

# 50. Accepted Risk

The agent MUST NOT independently decide:

```text
ACCEPTED_RISK
```

Only use it after an explicit user/team decision.

Record:

```text
decision
reason
date
```

when available.

Accepted-risk findings count as closed for overall implementation state.

---

# 51. Updating Overall Audit State

After each fix run, recalculate audit state.

If actionable findings remain:

```yaml
implementation_status: partial
```

If all findings are one of:

```text
RESOLVED
NOT_APPLICABLE
ACCEPTED_RISK
```

set:

```yaml
implementation_status: resolved
```

If evidence became materially stale:

```yaml
implementation_status: requires-reaudit
```

Always update:

```yaml
updated_at:
```

---

# 52. Meaning of Resolved Audit

A resolved audit is immutable as an implementation baseline except for administrative metadata.

A later `/rust-engineering fix` must NOT select it.

If future repository changes reintroduce the same problem, a new audit must discover it again.

Do not reopen historical resolved audits automatically.

---

# 53. Fix Must Not Become Audit

While fixing, the agent may notice unrelated concerns.

Do not add them to the current audit as new findings.

Record them only as:

```text
Observation requiring future audit
```

if materially important.

The next audit determines whether they become findings.

This preserves:

```text
Audit defines scope
Fix implements scope
```

---

# 54. No Hidden Work

At the end of FIX mode report:

```text
audit ID used
findings attempted
findings resolved
findings remaining
findings blocked
findings stale
verification result
new implementation_status
```

Do not claim the audit is resolved if open actionable findings remain.

---

# 55. Completion Criteria — AUDIT

AUDIT is complete only when:

1. canonical Rust standard was read
2. repository-specific instructions were read
3. repository topology was established
4. baseline revision/state was recorded
5. relevant verification commands were executed or limitations recorded
6. source was systematically inspected
7. appropriate parallel subagent analysis was performed for a repository-wide audit
8. candidate findings were independently validated
9. duplicates were consolidated
10. uncertainty was separated from confirmed findings
11. verified strengths were recorded
12. audit report was written
13. no production source was modified

---

# 56. Completion Criteria — FIX

FIX is complete only when:

1. an eligible open audit existed
2. audit freshness was checked
3. only audit findings defined implementation scope
4. each attempted finding was revalidated
5. fixes complied with the Rust Engineering Standard
6. relevant regression tests were added where appropriate
7. targeted verification passed for resolved findings
8. relevant repository quality gates were run
9. finding statuses were updated
10. audit implementation status was recalculated
11. unresolved findings remain visible
12. no unrelated cleanup was silently included

---

# 57. Core Invariant

Never collapse:

```text
AUDIT
```

and:

```text
FIX
```

into one uncontrolled activity.

The audit is the immutable decision baseline.

The fix is an implementation against that baseline.

The audit file is the persistent ledger connecting both operations.

A future fix must only act on unresolved findings in the newest applicable audit.

Once the audit is resolved, no later fix may silently continue working from it.
