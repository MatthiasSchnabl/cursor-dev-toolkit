---
name: rust-sqlx-engineering-fix
description: Remediate only findings from the newest eligible open Rust SQLx/PostgreSQL engineering audit. Never silently re-audit or invent findings.
---

# Rust SQLx + PostgreSQL Engineering Fix

Read and follow the plugin skill `skills/engineering/rust-sqlx-engineering-fix/SKILL.md` completely, including `skills/engineering/_standards/rust-sqlx-postgresql-engineering-standard.md`.

## Operation

This command is **fix** only.

- Locate the newest eligible open Rust SQLx/PostgreSQL audit and remediate only findings recorded there.
- Never silently start an audit as part of a fix.
- If no eligible open audit exists, STOP and report that `/rust-sqlx-engineering-audit` must run first.

Write audit updates under `docs/engineering-audits/sqlx/` in the **current repository**.
