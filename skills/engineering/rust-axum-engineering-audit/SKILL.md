---
name: rust-axum-engineering-audit
description: >
  Perform a repository-wide evidence-based engineering audit of Rust Axum,
  Tokio, Tower and HTTP/API code against the canonical Rust Axum Engineering
  Standard. Produces a persistent audit ledger with verified findings but
  never modifies production source code.
disable-model-invocation: true
---

# Rust Axum Engineering Audit

This is a compact orchestration layer. Detailed process and engineering guidance use progressive disclosure.

## Non-negotiable contract

- This operation is **AUDIT only**.
- Existing source/tests/configuration remain read-only except for the persistent audit report explicitly allowed by the procedure.
- Never remediate findings from this skill.
- Findings require evidence, concrete impact, confidence, and root-cause deduplication.

## References

- **Audit procedure:** [references/audit-procedure.md](references/audit-procedure.md)
  Read its **Contents** first, then load only sections required for the current phase.
- **Canonical engineering standard:** [../_standards/rust-axum-engineering-standard.md](../_standards/rust-axum-engineering-standard.md)
  Read its **Contents** first and load only sections relevant to current evidence/remediation.

Do not preload long references wholesale unless the task genuinely requires them.

## Execution

1. Establish the repository/worktree and framework-version baseline required by the procedure.
2. Follow the procedure phase by phase.
3. Load matching standard sections for the code/behavior currently being evaluated.
4. Scope specialist subagents to only the references relevant to their responsibility.
5. Validate evidence independently in the parent agent.
6. Persist/update the audit ledger exactly as defined by the procedure.
7. Run required verification before claiming completion.

## Context discipline

- Keep authoritative rules in the canonical standard.
- Keep workflow mechanics in the procedure reference.
- Long references must expose `## Contents` near the top.
- Keep this `SKILL.md` below 500 lines.
