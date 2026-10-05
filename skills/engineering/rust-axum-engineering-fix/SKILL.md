---
name: rust-axum-engineering-fix
description: >
  Remediate only findings from the newest eligible open Rust Axum engineering
  audit. Never silently re-audit or invent findings. Resolves tracked AXUM-*
  findings against the canonical Rust Axum Engineering Standard.
disable-model-invocation: true
---

# Rust Axum Engineering Fix

This is a compact orchestration layer. Detailed process and engineering guidance use progressive disclosure.

## Non-negotiable contract

- This operation is **FIX only**.
- A completed open/partial audit must already exist.
- Never invent findings and never silently perform a fresh audit.
- Revalidate each recorded finding before editing.
- Mark RESOLVED only after required verification passes.

## References

- **Fix procedure:** [references/fix-procedure.md](references/fix-procedure.md)
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
