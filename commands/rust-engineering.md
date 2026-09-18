---
name: rust-engineering
description: Audit or remediate Rust code against the canonical Rust Engineering Standard. Default is audit; fix requires an existing open audit.
---

# Rust Engineering

Read and follow the plugin skill `skills/engineering/rust-engineering/SKILL.md` completely, including `references/rust-engineering-standard.md`.

## Operation

- Default: **audit**. `/rust-engineering` and `/rust-engineering audit` both mean a fresh audit.
- **fix** only when the user explicitly asked for fix: locate the newest eligible open Rust audit and remediate only findings recorded there.
- Never silently start an audit as part of a fix.
- If no eligible open audit exists for fix, STOP and report that `/rust-engineering audit` must run first.

Write audits under `docs/engineering-audits/rust/` in the **current repository**. Do not modify production source during audit.
