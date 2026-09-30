---
name: testing-quality-engineering-fix
description: Remediate only findings from the newest eligible open Testing & Quality engineering audit. Never silently re-audit or invent findings.
---

# Testing & Quality Engineering Fix

Read and follow the plugin skill `skills/engineering/testing-quality-engineering-fix/SKILL.md` completely, including `skills/engineering/_standards/testing-quality-engineering-standard.md`.

## Operation

This command is **fix** only.

- Locate the newest eligible open Testing & Quality audit and remediate only findings recorded there.
- Never silently start an audit as part of a fix.
- If no eligible open audit exists, STOP and report that `/testing-quality-engineering-audit` must run first.

Write audit updates under `docs/engineering-audits/testing/` in the **current repository**.
