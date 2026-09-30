---
name: rust-axum-engineering-fix
description: Remediate only findings from the newest eligible open Rust Axum engineering audit. Never silently re-audit or invent findings.
---

# Rust Axum Engineering Fix

Read and follow the plugin skill `skills/engineering/rust-axum-engineering-fix/SKILL.md` completely, including `skills/engineering/_standards/rust-axum-engineering-standard.md`.

## Operation

This command is **fix** only.

- Locate the newest eligible open Rust Axum audit and remediate only findings recorded there.
- Never silently start an audit as part of a fix.
- If no eligible open audit exists, STOP and report that `/rust-axum-engineering-audit` must run first.

Write audit updates under `docs/engineering-audits/axum/` in the **current repository**.
