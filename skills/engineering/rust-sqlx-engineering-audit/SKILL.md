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
disable-model-invocation: true
---

# Rust SQLx + PostgreSQL Engineering Audit

This is a compact orchestration layer. Detailed process and engineering guidance use progressive disclosure.

## Non-negotiable contract

- This operation is **AUDIT only**.
- Existing source/tests/migrations/schema/configuration remain read-only except for the persistent audit report allowed by the procedure.
- Never remediate findings from this skill.
- Findings require evidence, concrete impact, confidence, and root-cause deduplication.

## References

- **Audit procedure:** [references/audit-procedure.md](references/audit-procedure.md)
  Read its **Contents** first, then load only sections required for the current phase.
- **Canonical engineering standard:** [../_standards/rust-sqlx-postgresql-engineering-standard.md](../_standards/rust-sqlx-postgresql-engineering-standard.md)
  Read its **Contents** first and load only sections relevant to current evidence/remediation.

Do not preload long references wholesale unless the task genuinely requires them.

## Execution

1. Establish repository/worktree, SQLx version/features, and PostgreSQL context required by the procedure.
2. Follow the procedure phase by phase.
3. Load matching standard sections for the persistence behavior currently being evaluated.
4. Scope specialist subagents to only relevant references.
5. Validate evidence independently in the parent agent.
6. Persist/update the audit ledger exactly as defined.
7. Run required SQLx/PostgreSQL and repository verification before completion.

## Context discipline

- Keep authoritative persistence rules in the canonical standard.
- Keep workflow mechanics in the procedure reference.
- Long references must expose `## Contents` near the top.
- Keep this `SKILL.md` below 500 lines.
