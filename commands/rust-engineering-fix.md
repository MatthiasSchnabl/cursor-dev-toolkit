---
name: rust-engineering-fix
description: Remediate only findings from the newest eligible open Rust engineering audit. Never silently re-audit or invent findings.
---

# Rust Engineering Fix

Read and follow the plugin skill `skills/engineering/rust-engineering/SKILL.md` completely, including `references/rust-engineering-standard.md`.

## Operation

This command is **fix** only.

- Locate the newest eligible open Rust audit and remediate only findings recorded there.
- Never silently start an audit as part of a fix.
- If no eligible open audit exists, STOP and report that `/rust-engineering-audit` must run first.

Write audit updates under `docs/engineering-audits/rust/` in the **current repository**.
