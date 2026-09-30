---
name: security-supply-chain-engineering-fix
description: Remediate only findings from the newest eligible open Security & Supply-Chain engineering audit. Never silently re-audit or invent findings. Never reproduce secrets.
---

# Security & Supply-Chain Engineering Fix

Read and follow the plugin skill `skills/engineering/security-supply-chain-engineering-fix/SKILL.md` completely, including `skills/engineering/_standards/security-supply-chain-engineering-standard.md`.

## Operation

This command is **fix** only.

- Locate the newest eligible open Security & Supply-Chain audit and remediate only findings recorded there.
- Never silently start an audit as part of a fix.
- If no eligible open audit exists, STOP and report that `/security-supply-chain-engineering-audit` must run first.
- Never reproduce secret values; use `[REDACTED]`.

Write audit updates under `docs/engineering-audits/security/` in the **current repository**.
