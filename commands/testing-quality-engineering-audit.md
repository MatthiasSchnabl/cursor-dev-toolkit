---
name: testing-quality-engineering-audit
description: Perform a fresh evidence-based testing and quality engineering audit against the canonical Testing & Quality Engineering Standard. Do not modify production or test source. Do not remediate.
---

# Testing & Quality Engineering Audit

Read and follow the plugin skill `skills/engineering/testing-quality-engineering-audit/SKILL.md` completely, including `skills/engineering/_standards/testing-quality-engineering-standard.md`.

## Operation

This command is **audit** only.

- Perform a fresh Testing & Quality engineering audit.
- Never start a fix from this command.
- Write audits under `docs/engineering-audits/testing/` in the **current repository**.
- Do not modify production source, tests, fixtures, snapshots, or CI during audit.
