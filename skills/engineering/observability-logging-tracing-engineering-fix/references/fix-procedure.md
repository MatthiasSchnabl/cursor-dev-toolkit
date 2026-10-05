# Observability, Logging & Tracing Engineering Fix

## Contents

- §1: Operation
- §2: Fundamental Workflow Rule
- §3: Eligible Audit Selection
- §4: Freshness Check Before Fix
- §5: Fix Scope
- §6: Fix Order
- §7: Revalidate Before Editing
- §8: Implementation Quality
- §9: Fix Verification
- §10: Regression Tests
- §11: Resolution Record
- §12: Blocked and Accepted Risk
- §13: Updating Overall Audit State
- §14: No Hidden Work
- §15: Completion Criteria
- §16: Core Invariant

> Navigation: use this map to load only the sections required for the current phase. Do not preload the whole file unless the task genuinely requires it.


This skill remediates findings recorded in a completed open Observability, Logging & Tracing engineering audit.

It MUST NOT perform a new audit.

The canonical engineering standard is:

`../_standards/observability-logging-tracing-engineering-standard.md`

Read that standard completely before modifying source. Do not resolve an observability finding by introducing a known Rust, Axum, SQLx, testing, security, or Rig violation.

Never reproduce secret values, prompts, tokens, or personal data. Use `[REDACTED]`.

Do not introduce a new OpenTelemetry collector or backend just to close a finding. Test the integration the repository already has.

---

# 1. Operation

This skill supports exactly one operation:

```text
/observability-logging-tracing-engineering-fix  →  fix
```

```text
locate the newest eligible open Observability, Logging & Tracing audit
and remediate only findings recorded in that audit
```

Never silently start an audit as part of a fix.

Auditing is exclusively the responsibility of:

```text
/observability-logging-tracing-engineering-audit
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

If no eligible open audit exists:

```text
STOP
```

and report:

```text
No open Observability, Logging & Tracing Engineering Audit is available.
Run /observability-logging-tracing-engineering-audit first.
```

Do not invent findings in fix mode.

---

# 3. Eligible Audit Selection

FIX mode MUST first locate audit files:

```text
docs/engineering-audits/observability/OBS-AUDIT-*.md
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

If several open audits exist, use the newest applicable audit by `created_at` unless the user explicitly selects an audit ID.

Do not silently combine separate audit histories.

If none exists, STOP. Do not audit automatically.

---

# 4. Freshness Check Before Fix

Before modifying source:

1. read `baseline_commit`
2. inspect current `HEAD`
3. inspect repository changes since baseline
4. compare changed paths with open finding evidence
5. determine whether each finding remains valid

If evidence has been invalidated, set the finding `STALE` and:

```yaml
implementation_status: requires-reaudit
```

Do not rediscover a replacement finding in FIX mode.

A fresh `/observability-logging-tracing-engineering-audit` is required.

---

# 5. Fix Scope

FIX mode may modify code only to remediate `OBS-*` findings recorded in the selected audit.

Do not perform unrelated refactoring, cleanup, dependency updates, or architecture redesign.

Do not start other engineering audits or fixes.

Neighboring application code may change only when a recorded observability finding requires it.

---

# 6. Fix Order

Follow the selected audit's **Recommended Remediation Order** when present.

Otherwise:

```text
CRITICAL
HIGH
MEDIUM
LOW
```

---

# 7. Revalidate Before Editing

Before implementing each finding:

1. reread its evidence
2. confirm the issue still exists
3. inspect the subscriber, spans, and tests around it
4. identify the smallest coherent remediation

If evidence no longer applies, mark `STALE` or `NOT_APPLICABLE` with justification.

---

# 8. Implementation Quality

A fix must comply with the full Observability, Logging & Tracing Engineering Standard.

Do not resolve one finding by introducing another known violation.

Do not silence compiler warnings, Clippy, tests, or lints to make the implementation appear successful.

---

# 9. Fix Verification

Every finding must define its verification plan before implementation.

After implementation, run targeted tests first, then the relevant repository quality gates. Where the repository defines them, that includes:

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
```

Also verify the specific contract the finding names, such as:

```text
request ID propagation
structured span fields
redaction
error chain
async span correctness
SQLx filterability
agent run/tool correlation
```

A finding may become `RESOLVED` only after required verification succeeds.

---

# 10. Regression Tests

If a finding represents incorrect diagnostic behavior, add a regression test where practical.

Prefer assertions on structured fields over exact formatted log lines.

A fix that cannot be protected by a meaningful test must explain why.

---

# 11. Resolution Record

When resolved, replace `Not yet implemented.` with:

```markdown
### Resolution

**Result:** RESOLVED
**Implemented:** <short description>
**Verification:**
- `<command>` — PASS

**Evidence:**
- `src/example.rs:...`

**Resolved at:** <timestamp>
```

Do not claim PASS if a command was not run successfully. Do not paste secret values.

---

# 12. Blocked and Accepted Risk

If remediation cannot proceed, set `BLOCKED` and document the blocker.

Do not convert blocked findings to resolved.

The agent MUST NOT independently set `ACCEPTED_RISK`. Only record it after an explicit user decision, with decision, reason, and date.

Accepted-risk findings count as closed for overall implementation state.

---

# 13. Updating Overall Audit State

If actionable findings remain:

```yaml
implementation_status: partial
```

If every finding is `RESOLVED`, `NOT_APPLICABLE`, or `ACCEPTED_RISK`:

```yaml
implementation_status: resolved
```

If evidence became materially stale:

```yaml
implementation_status: requires-reaudit
```

Always update `updated_at`.

A later fix must not select a resolved audit. Do not add newly noticed concerns as findings. Record them only as an observation for a future audit.

---

# 14. No Hidden Work

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

# 15. Completion Criteria

FIX is complete only when:

1. an eligible open observability audit existed
2. audit freshness was checked
3. only `OBS-*` findings from that audit defined implementation scope
4. each attempted finding was revalidated
5. fixes complied with the Observability, Logging & Tracing Engineering Standard
6. relevant regression tests were added where appropriate
7. targeted verification passed for resolved findings
8. relevant repository quality gates were run
9. finding statuses were updated
10. audit implementation status was recalculated
11. unresolved findings remain visible
12. no unrelated cleanup was silently included

---

# 16. Core Invariant

Never collapse audit and fix into one uncontrolled activity.

The audit file under `docs/engineering-audits/observability/` is the persistent ledger.

A future fix must only act on unresolved findings in the newest applicable observability audit.

Once the audit is resolved, no later fix may silently continue working from it.
