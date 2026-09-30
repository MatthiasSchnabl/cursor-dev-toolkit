---
name: rust-rig-agentic-engineering-fix
description: Remediate only findings from the newest eligible open Rust Rig agentic engineering audit. Never silently re-audit or invent findings.
---

# Rust Rig Agentic Engineering Fix

Read and follow the plugin skill `skills/engineering/rust-rig-agentic-engineering-fix/SKILL.md` completely, including `skills/engineering/_standards/rust-rig-agentic-engineering-standard.md`.

## Operation

This command is **fix** only.

- Locate the newest eligible open Rust Rig agentic audit and remediate only findings recorded there.
- Never silently start an audit as part of a fix.
- If no eligible open audit exists, STOP and report that `/rust-rig-agentic-engineering-audit` must run first.

Write audit updates under `docs/engineering-audits/rig/` in the **current repository**.
