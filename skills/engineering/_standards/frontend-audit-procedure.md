# Frontend Engineering Audit Procedure v1

## Contents

1. Scope and mandatory version gate
2. Evidence inventory
3. Assessment and severity
4. Persistent ledger
5. Verification and closeout

## 1. Scope and mandatory version gate

- Determine the frontend package root from its own package.json and lockfile. In monorepos, do not assume repository root is frontend package root.
- Before source modifications **or substantive audit work**, run the global read-only command:
  node "$CURSOR_DEV_TOOLKIT_ROOT/scripts/frontend-preflight.mjs" --root "$FRONTEND_ROOT" --framework nextjs|sveltekit
- On any failure: STOP. Report versions, incompatibilities and migration docs; ask explicit user approval for a controlled upgrade. Do not install packages, edit manifest, run codemods, or apply incompatible framework guidance. Any future migration is a separate operator-approved task.
- Select Next.js or SvelteKit canonical standard plus Frontend Core; do not mix framework APIs. Pin cited versions from the preflight.
- Audit is read-only for application code, tests and configuration. Only a persistent audit ledger may be created/updated.

## 2. Evidence inventory

- Establish repo revision, framework/library versions, package scripts, node version, router mode and config flags.
- Inspect route layouts, server/client load boundaries, mutation paths, domain/API interfaces, auth and caching.
- Use focused structural search; do not traverse generated folders or dump huge code files into context.
- Evaluate all categories of the matching standard using representative call traces and app-critical flows.
- Missing evidence is recorded as NOT_VERIFIED, not an automatically confirmed failure.

## 3. Assessment and severity

- Findings must describe observed behavior, affected paths with line evidence, reproducibility, impact, severity, confidence and exact manufacturer rule.
- P0: cross-user leak, unauthorized data/action, embedded secret, destructive unsafe path.
- P1: broken state/SSR/action/cache or major misuse demonstrably affecting correctness.
- P2: test, resource, maintainability or performance concern supported by concrete evidence.
- Deduplicate common root causes. Mark fixed historical issues if clearly absent; do not fabricate issues merely to fill a scorecard.

## 4. Persistent ledger

- Write an audit report at docs/engineering-audits/<frontend|sveltekit|nextjs>/<audit-id>.md. The audit id is a unique versioned date + git short SHA.
- Header fields: audit_id, status (open|partial|resolved), framework, exact versions, source_revision, standard, reviewed_scope, verification_results.
- Findings: unique stable ID (FCORE-###, SKIT-### or NEXT-###), severity, status (OPEN|PARTIAL|RESOLVED|NOT_VERIFIED), evidence paths/lines, expected vs actual, impact, fix guidance and test requirements.
- Never overwrite earlier reports or mark RESOLVED without execution evidence.
- A report with no verified issues should have status resolved and explicitly state coverage limitations.

## 5. Verification and closeout

- Read-only local checks (typecheck, lint, test, build) may be run when safe and relevant; do not auto-install missing dependencies.
- Report verified findings, gaps, severity count and exact tests executed/not executed; avoid treating static review as production proof.
- Do not call a Fix workflow from an Audit workflow without a separate explicit request.
