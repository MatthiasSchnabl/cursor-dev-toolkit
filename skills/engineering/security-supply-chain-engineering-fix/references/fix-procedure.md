# Security & Supply-Chain Engineering Fix

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
- §12: Blocked Findings
- §13: Accepted Risk
- §14: Updating Overall Audit State
- §15: Meaning of Resolved Audit
- §16: Fix Must Not Become Audit
- §17: No Hidden Work
- §18: Completion Criteria
- §19: Core Invariant

> Navigation: use this map to load only the sections required for the current phase. Do not preload the whole file unless the task genuinely requires it.


This skill remediates findings recorded in a completed open Security & Supply-Chain engineering audit.

It MUST NOT perform a new audit.

The canonical engineering standard is:

`../../_standards/security-supply-chain-engineering-standard.md`

Use the standard's **Contents** map first and load the sections relevant to each recorded finding before modifying its scope. Do not resolve a security finding by introducing a known Rust-, Axum-, SQLx- or testing-engineering violation.

Never reproduce secret values in chat, logs or Resolution records. Use `[REDACTED]`.

---

# 1. Operation

This skill supports exactly one operation:

```text
/security-supply-chain-engineering-fix  →  fix
```

```text
locate the newest eligible open Security & Supply-Chain audit
and remediate only findings recorded in that audit
```

Never silently start an audit as part of a fix.

Auditing is exclusively the responsibility of:

```text
/security-supply-chain-engineering-audit
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
No open Security & Supply-Chain engineering audit is available.
Run /security-supply-chain-engineering-audit first.
```

Do not invent findings in fix mode.

---

# 3. Eligible Audit Selection

FIX mode MUST first locate audit files:

```text
docs/engineering-audits/security/SECURITY-AUDIT-*.md
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

If several open audits exist, use the newest applicable audit by:

```text
created_at
```

unless the user explicitly selects an audit ID.

Do not silently combine separate audit histories.

If none exists:

```text
STOP
```

Do not audit automatically.

---

# 4. Freshness Check Before Fix

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

A fresh `/security-supply-chain-engineering-audit` is required.

---

# 5. Fix Scope

FIX mode may modify code only to remediate `SEC-*` findings recorded in the selected audit.

Source, tests, CI, Dockerfiles, deployment manifests, Cargo.toml and Cargo.lock may change only when a recorded finding requires it.

Do not perform unrelated:

```text
refactoring
cleanup
broad dependency upgrades
format rewrites
architecture redesign
secret rotation without an explicit finding
```

unless required to implement a recorded finding correctly.

Do not start:

```text
/rust-engineering-audit
/rust-engineering-fix
/rust-axum-engineering-audit
/rust-axum-engineering-fix
/testing-quality-engineering-audit
/testing-quality-engineering-fix
/rust-sqlx-engineering-audit
/rust-sqlx-engineering-fix
```

Avoid scope creep.

---

# 6. Fix Order

Follow the selected audit's **Recommended Remediation Order** when present.

Otherwise resolve findings in dependency-aware order.

Normally prioritize:

```text
CRITICAL
HIGH
MEDIUM
LOW
```

Architecture dependencies may require a different coherent order.

Do not fix findings in arbitrary file order.

---

# 7. Revalidate Before Editing

Before implementing each finding:

1. reread its evidence
2. confirm the issue still exists
3. inspect the cited code, config, CI and trust boundaries
4. understand attack/failure preconditions and impact
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

# 8. Implementation Quality

A fix must comply with the full Security & Supply-Chain Engineering Standard.

Do not resolve one finding by introducing another known violation.

Prefer root-cause remediation over symptom suppression.

Never reproduce secret values.

Do not silence:

```text
compiler warnings
Clippy
tests
lints
```

to make implementation appear successful.

---

# 9. Fix Verification

Every finding must define its verification plan before implementation.

After implementation, run:

```text
targeted tests/checks
```

first.

Then run the relevant broader repository quality gates, including security tooling already used by the repository when applicable.

A finding may only become:

```text
RESOLVED
```

after required verification succeeds.

---

# 10. Regression Tests

If a finding represents a missing or incorrect security control, add or repair negative regression coverage where practical.

A fix that cannot be protected by a meaningful test must explain why.

Do not add meaningless tests merely to satisfy this rule.

---

# 11. Resolution Record

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
- `.github/workflows/example.yml:...`

**Resolved at:** <timestamp>
```

Do not claim PASS if a command was not run successfully.

Do not paste secrets into the Resolution record.

---

# 12. Blocked Findings

If remediation cannot proceed:

```text
Status: BLOCKED
```

and document the exact blocker.

Examples:

```text
missing architectural decision
production secret store unavailable
required identity-provider contract unknown
CI environment cannot be reproduced
```

Do not convert blocked findings to resolved.

---

# 13. Accepted Risk

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

# 14. Updating Overall Audit State

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

# 15. Meaning of Resolved Audit

A resolved audit is immutable as an implementation baseline except for administrative metadata.

A later `/security-supply-chain-engineering-fix` must NOT select it.

If future repository changes reintroduce the same problem, a new `/security-supply-chain-engineering-audit` must discover it again.

Do not reopen historical resolved audits automatically.

---

# 16. Fix Must Not Become Audit

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

# 17. No Hidden Work

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

# 18. Completion Criteria

FIX is complete only when:

1. an eligible open Security & Supply-Chain audit existed
2. audit freshness was checked
3. only `SEC-*` findings from that audit defined implementation scope
4. each attempted finding was revalidated
5. fixes complied with the Security & Supply-Chain Engineering Standard
6. relevant regression tests were added or repaired where appropriate
7. targeted verification passed for resolved findings
8. relevant repository quality gates were run
9. finding statuses were updated
10. audit implementation status was recalculated
11. unresolved findings remain visible
12. no unrelated cleanup was silently included
13. secrets were not reproduced

---

# 19. Core Invariant

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

The audit file under `docs/engineering-audits/security/` is the persistent ledger connecting both operations.

A future fix must only act on unresolved findings in the newest applicable Security & Supply-Chain audit.

Once the audit is resolved, no later fix may silently continue working from it.
