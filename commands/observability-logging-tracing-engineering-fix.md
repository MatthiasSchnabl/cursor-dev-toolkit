---
name: observability-logging-tracing-engineering-fix
description: Remediate only findings from the newest eligible open Observability, Logging & Tracing engineering audit. Never silently re-audit or invent findings. Never reproduce secrets.
---

# Observability, Logging & Tracing Engineering Fix

Read and follow the plugin skill `skills/engineering/observability-logging-tracing-engineering-fix/SKILL.md` completely, including `skills/engineering/_standards/observability-logging-tracing-engineering-standard.md`.

## Operation

This command is **fix** only.

- Locate the newest eligible open Observability, Logging & Tracing audit and remediate only findings recorded there.
- Never silently start an audit as part of a fix.
- If no eligible open audit exists, STOP and report that `/observability-logging-tracing-engineering-audit` must run first.
- Never reproduce secret values; use `[REDACTED]`.

Write audit updates under `docs/engineering-audits/observability/` in the **current repository**.
