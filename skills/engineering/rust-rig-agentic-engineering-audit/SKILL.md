---
name: rust-rig-agentic-engineering-audit
description: >
  Perform a repository-wide evidence-based engineering audit of agentic systems
  implemented with Rust and Rig against the canonical Rust Rig Agentic
  Engineering Standard. Evaluates agent architecture, harnesses, models, tools,
  ToolContext, hooks, run budgets, structured output, prompts, context, memory,
  retrieval/RAG, MCP, multi-agent orchestration, guardrails, observability,
  deterministic tests and agent evaluations. Produces a persistent audit
  ledger and never fixes findings.
disable-model-invocation: true
---

# Rust Rig Agentic Engineering Audit

This is a compact orchestration layer. Detailed process and engineering guidance use progressive disclosure.

## Non-negotiable contract

- This operation is **AUDIT only**.
- Existing source/test/configuration remains read-only except for the persistent audit report allowed by the procedure.
- Never remediate from this skill.
- Findings require evidence, impact, confidence, and root-cause deduplication.

## References

- **Audit procedure:** [references/audit-procedure.md](references/audit-procedure.md)
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
