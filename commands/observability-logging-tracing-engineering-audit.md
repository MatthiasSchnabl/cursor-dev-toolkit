---
name: observability-logging-tracing-engineering-audit
description: Perform a fresh evidence-based observability, logging, and tracing audit against the canonical Observability, Logging & Tracing Engineering Standard. Do not modify production source. Do not remediate.
---

# Observability, Logging & Tracing Engineering Audit

Read and follow the plugin skill `skills/engineering/observability-logging-tracing-engineering-audit/SKILL.md` completely, including `skills/engineering/_standards/observability-logging-tracing-engineering-standard.md`.

## Operation

This command is **audit** only.

- Perform a fresh Observability, Logging & Tracing engineering audit.
- Never start a fix from this command.
- Write audits under `docs/engineering-audits/observability/` in the **current repository**.
- Do not modify production source, tests, or telemetry configuration during audit.
- Never reproduce secret values; use `[REDACTED]`.
