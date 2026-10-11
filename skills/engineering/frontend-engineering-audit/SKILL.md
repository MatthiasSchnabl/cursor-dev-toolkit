---
name: frontend-engineering-audit
description: Audit the shared Frontend Core rules using the documented read-only evidence and ledger lifecycle; strictly gate framework versions before work.
disable-model-invocation: true
---

# FRONTEND Engineering Audit

Explicit invocation only. The globally installed toolkit owns these standards; individual app projects do not need a copy.

## Mandatory fail-closed framework gate

1. Locate the actual frontend package root, including nested frontend/ directories in monorepos.
2. Read [tested version policy](../../../scripts/frontend-versions.json) and run [read-only preflight](../../../scripts/frontend-preflight.mjs):
   - node "$CURSOR_DEV_TOOLKIT_ROOT/scripts/frontend-preflight.mjs" --root "$FRONTEND_ROOT"
3. If it reports a mismatch, missing modules, wrong router, or unsupported SvelteKit configuration: **STOP** and ask the user whether a planned framework migration/installation is approved. Never run install/codemods automatically.
4. If user declines, do not apply the newer standard to the old project. Report the blocker; an older-version standard would require separate maintenance.

## Canonical references

- [Shared Frontend Core](../_standards/frontend-engineering-core.md)
- [Framework selection](../_standards/frontend-engineering-core.md)
- [Audit procedure](../_standards/frontend-audit-procedure.md)

Read each reference's Contents and only required sections. Select Next.js or SvelteKit specific standard after detection; never apply both.

## Execution

- Read-only for application code; investigate the shared Frontend Core rules and relevant type, security, state and testing boundaries.
- Persist a separate evidence-ledger under docs/engineering-audits/frontend/; do not modify code or silently fix.
- Report verified findings, impact, confidence, unverified gaps and local checks executed.

## Constraints

No unauthorised framework/dependency upgrade, no design or animation requirements, no global mutable user state or cross-user cache, and no false claim of validated behavior. Keep SKILL.md compact and references progressive.
