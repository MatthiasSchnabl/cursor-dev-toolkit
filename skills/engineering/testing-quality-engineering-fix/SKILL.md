---
name: testing-quality-engineering-fix
description: >
  Remediate only findings from the newest eligible open Testing & Quality
  engineering audit. Never silently re-audit or invent findings. Resolves
  tracked TEST-* findings against the canonical Testing & Quality Engineering
  Standard.
disable-model-invocation: true
---

# Testing & Quality Engineering Fix

This is a compact orchestration layer. Detailed process and engineering guidance use progressive disclosure.

## Non-negotiable contract

- This operation is **FIX only**.
- A completed open/partial audit must already exist.
- Never invent findings or silently re-audit.
- Revalidate every recorded finding before editing.
- Mark RESOLVED only after required verification passes.

## References

- **Fix procedure:** [references/fix-procedure.md](references/fix-procedure.md)
  Read its **Contents** first, then load only sections required for the current phase.
- **Canonical engineering standard:** [../_standards/testing-quality-engineering-standard.md](../_standards/testing-quality-engineering-standard.md)
  Read its **Contents** first and load only sections relevant to current evidence/remediation.

Do not preload long references wholesale unless the task genuinely requires them.

## Execution

1. Establish the baseline required by the procedure.
2. Follow the procedure phase by phase.
3. Load matching standard sections for the behavior currently being evaluated.
4. Scope specialist subagents to relevant references only.
5. Validate evidence independently in the parent agent.
6. Persist/update the audit ledger exactly as defined.
7. Run required verification before completion.

## Context discipline

- Keep authoritative rules in the canonical standard.
- Keep workflow mechanics in the procedure reference.
- Long references must expose `## Contents` near the top.
- Keep this `SKILL.md` below 500 lines.
