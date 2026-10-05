---
name: rust-sqlx-engineering-fix
description: >
  Remediate only findings from the newest eligible open Rust SQLx/PostgreSQL
  engineering audit. Never silently re-audit or invent findings. Resolves
  tracked SQLX-* findings against the canonical Rust SQLx + PostgreSQL
  Engineering Standard.
disable-model-invocation: true
---

# Rust SQLx + PostgreSQL Engineering Fix

This is a compact orchestration layer. Detailed process and engineering guidance use progressive disclosure.

## Non-negotiable contract

- This operation is **FIX only**.
- A completed open/partial audit must already exist.
- Never invent findings and never silently perform a fresh audit.
- Revalidate each recorded finding against current code/schema before editing.
- Mark RESOLVED only after the finding-specific database/application verification passes.

## References

- **Fix procedure:** [references/fix-procedure.md](references/fix-procedure.md)
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
