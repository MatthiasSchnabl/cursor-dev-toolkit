---
name: nextjs-engineering-fix
description: Fix confirmed findings in Next.js 16.4 App Router, RSC, Server Functions and Cache Components using the documented open-audit and verification lifecycle; strictly gate framework versions before work.
disable-model-invocation: true
---

# NEXTJS Engineering Fix

Explicit invocation only. The globally installed toolkit owns these standards; individual app projects do not need a copy.

## Mandatory fail-closed framework gate

1. Locate the actual frontend package root, including nested frontend/ directories in monorepos.
2. Read [tested version policy](../../../scripts/frontend-versions.json) and run [read-only preflight](../../../scripts/frontend-preflight.mjs):
   - node "$CURSOR_DEV_TOOLKIT_ROOT/scripts/frontend-preflight.mjs" --root "$FRONTEND_ROOT" --framework nextjs
3. If it reports a mismatch, missing modules, wrong router, or unsupported SvelteKit configuration: **STOP** and ask the user whether a planned framework migration/installation is approved. Never run install/codemods automatically.
4. If user declines, do not apply the newer standard to the old project. Report the blocker; an older-version standard would require separate maintenance.

## Canonical references

- [Shared Frontend Core](../_standards/frontend-engineering-core.md)
- [Specific framework standard](../_standards/nextjs-engineering-standard.md)
- [Fix procedure](../_standards/frontend-fix-procedure.md)

Read each reference's Contents and only required sections. Confirm project exactly matches this framework before the procedure starts.

## Execution

- Require the newest eligible OPEN/PARTIAL docs/engineering-audits/nextjs/ audit for matching framework/version. Without it, STOP.
- Revalidate and fix **only** recorded findings; never invent findings or silently re-audit.
- Verify with existing project scripts (typecheck, lint, tests, build, targeted E2E) before marking RESOLVED.

## Constraints

No unauthorised framework/dependency upgrade, no design or animation requirements, no global mutable user state or cross-user cache, and no false claim of validated behavior. Keep SKILL.md compact and references progressive.
