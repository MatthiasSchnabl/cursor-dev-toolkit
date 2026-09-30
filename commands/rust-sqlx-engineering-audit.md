---
name: rust-sqlx-engineering-audit
description: Perform a fresh evidence-based Rust SQLx/PostgreSQL engineering audit against the canonical Rust SQLx + PostgreSQL Engineering Standard. Do not modify production source, tests, migrations or schema. Do not remediate.
---

# Rust SQLx + PostgreSQL Engineering Audit

Read and follow the plugin skill `skills/engineering/rust-sqlx-engineering-audit/SKILL.md` completely, including `skills/engineering/_standards/rust-sqlx-postgresql-engineering-standard.md`.

## Operation

This command is **audit** only.

- Perform a fresh Rust SQLx + PostgreSQL engineering audit.
- Never start a fix from this command.
- Write audits under `docs/engineering-audits/sqlx/` in the **current repository**.
- Do not modify production source, tests, migrations, schema, Cargo.toml, Cargo.lock, or `.sqlx` metadata during audit.
