---
name: rust-rig-agentic-engineering-fix
description: >
  Remediate only findings from the newest eligible open Rust Rig agentic
  engineering audit. Never silently re-audit or invent findings. Resolves
  tracked RIG-* findings against the canonical Rust Rig Agentic
  Engineering Standard.
disable-model-invocation: true
---

# Rust Rig Agentic Engineering Fix

This is a compact orchestration layer. Detailed process and engineering guidance use progressive disclosure.

## Non-negotiable contract

- This operation is **FIX only**.
- A completed open/partial audit must already exist.
- Never invent findings or silently re-audit.
- Revalidate every recorded finding before editing.
- Mark RESOLVED only after required deterministic and behavioral verification passes.

## References

- **Fix procedure:** [references/fix-procedure.md](references/fix-procedure.md)
  Read its **Contents** first, then load only sections required for the current phase.
- **Canonical engineering standard:** [../_standards/rust-rig-agentic-engineering-standard.md](../_standards/rust-rig-agentic-engineering-standard.md)
  Read its **Contents** first and load only sections relevant to current evidence/remediation.

Do not preload long references wholesale unless the task genuinely requires them.

## Execution

1. Establish repository/worktree and framework/runtime baseline required by the procedure.
2. Follow the procedure phase by phase.
3. Load matching standard sections for the behavior currently being evaluated.
4. Scope specialist subagents to relevant references only.
5. Validate evidence independently in the parent agent.
6. Persist/update the audit ledger exactly as defined.
7. Run deterministic verification and relevant evals where the finding concerns probabilistic behavior.

## Context discipline

- Keep authoritative rules in the canonical standard.
- Keep workflow mechanics in the procedure reference.
- Long references must expose `## Contents` near the top.
- Keep this `SKILL.md` below 500 lines.
