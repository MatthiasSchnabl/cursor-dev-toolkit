---
name: rust-engineering
description: >
  Shared process for /rust-engineering-audit and /rust-engineering-fix.
  Evidence-based Rust engineering audits and controlled remediation against the
  canonical Rust Engineering Standard, covering correctness, type safety,
  ownership, borrowing, allocations, error handling, unsafe code, concurrency,
  idiomatic Rust, maintainability and performance. Audit never remediates.
  Fix is allowed only when a completed open audit already exists and must
  resolve tracked findings from that audit.
disable-model-invocation: true
---

# Rust Engineering Audit & Remediation

This is the compact orchestration layer for the established Rust audit/fix workflow. Detailed procedure and engineering guidance use progressive disclosure.

## Non-negotiable contract

This internal process skill supports exactly:

- `/rust-engineering-audit` — fresh read-only audit.
- `/rust-engineering-fix` — remediation only against a completed open/partial Rust audit.

There is no combined workflow. Audit never fixes; fix never invents findings or silently re-audits.

## References

- **Process procedure:** [references/process.md](references/process.md)  
  Read its **Contents** first, then load only the sections needed for the current operation and phase.
- **Canonical Rust standard:** [references/rust-engineering-standard.md](references/rust-engineering-standard.md)  
  Read its **Contents** first and load only the Rust topics relevant to current evidence or remediation.

Do not preload either long reference wholesale unless the task genuinely requires the complete document.

## Execution

1. Determine whether the explicit command selected audit or fix.
2. Establish repository/worktree baseline.
3. Follow the matching process sections in `references/process.md`.
4. Load matching canonical-standard sections as evidence requires.
5. Use specialist subagents only where the process requires them; scope each subagent to relevant references.
6. Validate candidate findings/resolution evidence independently in the parent.
7. Persist/update the audit ledger exactly as defined.
8. Run required verification before claiming completion.

## Context discipline

- Keep engineering rules in the canonical standard.
- Keep workflow mechanics in the process reference.
- Long references must expose `## Contents` near the top.
- Keep this `SKILL.md` below 500 lines.
