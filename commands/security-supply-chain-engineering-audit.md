---
name: security-supply-chain-engineering-audit
description: Perform a fresh evidence-based security and supply-chain audit against the canonical Security & Supply-Chain Engineering Standard. Do not modify source or configuration. Do not remediate. Never reproduce secrets.
---

# Security & Supply-Chain Engineering Audit

Read and follow the plugin skill `skills/engineering/security-supply-chain-engineering-audit/SKILL.md` completely, including `skills/engineering/_standards/security-supply-chain-engineering-standard.md`.

## Operation

This command is **audit** only.

- Perform a fresh Security & Supply-Chain engineering audit.
- Never start a fix from this command.
- Write audits under `docs/engineering-audits/security/` in the **current repository**.
- Do not modify source, tests, Cargo.toml, Cargo.lock, CI, Dockerfiles, or configuration during audit.
- Never reproduce secret values; use `[REDACTED]`.
