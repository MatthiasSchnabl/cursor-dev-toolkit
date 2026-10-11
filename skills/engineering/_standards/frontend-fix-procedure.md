# Frontend Engineering Fix Procedure v1

## Contents

1. Preconditions
2. Remediation
3. Verification
4. Ledger closeout

## 1. Preconditions

- Run the read-only framework-version preflight **before any edits**. Stop and ask the user for upgrade approval if incompatible.
- Locate the newest eligible open or partial audit under docs/engineering-audits/<frontend|sveltekit|nextjs>/ for the exact framework/version and current scope.
- No eligible report means STOP. Never invent findings, launch a silent audit or reuse a resolved ledger.
- Reproduce/revalidate each finding. If a finding no longer exists, close it with proof rather than writing speculative code.
- Prefer a focused implementation plan preserving existing public behavior unless finding explicitly addresses a defect.

## 2. Remediation

- Fix only confirmed findings from the selected audit, in a coherent bounded slice.
- Use the matching framework canonical standard plus the shared Frontend Core as guidance; choose native conventions before custom wrappers.
- Do not perform framework upgrades, package manager changes, broad refactors or design changes unless separately approved.
- Keep permissions and sensitive data on the server and avoid accidental cross-request/client-cache exposure.
- Write or update focused tests for original failure path and negative/unauthorized scenarios.

## 3. Verification

- Run preflight again after edits, then project-provided type checking, linting, relevant tests, production build and critical E2E tests as appropriate.
- Use existing package scripts; never silently install or migrate packages during fix.
- Fail closed on unsuccessful relevant gates. For unexecuted E2E/production checks, state their status explicitly.
- Review diff against finding scope, avoid unrelated changes and ensure no regressions.

## 4. Ledger closeout

- Update the **same** audit report with per-finding fix commit/evidence, exact executed tests and implementation status.
- Mark RESOLVED only where verified; leave other findings OPEN or PARTIAL with blockers. Never erase history.
- Report scope, residual risks, verification evidence, and any new unrelated defect as a separate follow-up rather than a fabricated audit finding.
