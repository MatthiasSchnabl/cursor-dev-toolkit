# Frontend Engineering Core

Baseline: Strict TypeScript applications using SvelteKit or Next.js; visual design and animation are out of scope.

## Contents

1. Architecture and ownership
2. TypeScript and runtime contracts
3. Server state, client state and data flow
4. Security and authorization
5. Errors and accessibility
6. Performance and reliability
7. Local quality gates
8. Audit severity

## 1. Architecture and ownership

- Prefer framework-native router, loader, mutation and error conventions over generic bespoke infrastructure.
- Keep UI rendering, HTTP transport, domain rules and persistence at explicit boundaries; avoid duplicate domain rules in frontend/backends.
- Organize by coherent feature responsibility rather than speculative abstraction layers. Share code only when actually reused.
- Maintain a single package manager and a committed lockfile; never silently change either.
- Keep server-only modules out of browser bundles; treat serialization as a public boundary.

## 2. TypeScript and runtime contracts

- Use strict TypeScript and precise types, avoid any or unchecked casts without a documented reason.
- TypeScript types do not validate untrusted runtime input; validate form/API data and configuration at boundaries.
- Generate API clients/types from OpenAPI where the backend has an authoritative contract. Do not edit generated sources.
- Define success/errors/timeouts/cancellation semantics rather than returning ambiguous success-shaped defaults.

## 3. Server state, client state and data flow

- Use framework-native server loaders and mutations before adding independent global-fetching abstractions.
- Distinguish server-authoritative state, URL/navigation state, transient component state and genuinely shared UI state.
- Avoid duplicating server records as independent client sources of truth. Reconcile optimistic changes with the backend.
- Avoid sequential request waterfalls where calls are independent; consider cancellation and race conditions.

## 4. Security and authorization

- Enforce authorization at the server read/action boundary, not only in components or route visibility.
- Never serialize secrets, tokens or other confidential env settings into client bundles, props or logs.
- Handle CSRF/XSS, cookies, request IDs, payload bounds and upload restrictions with native framework mechanisms.
- Cache data according to its visibility and user scope; do not create cross-session state leaks.

## 5. Errors and accessibility

- Explicitly cover loading, empty, 401/403, missing, validation failure, timeout and retry states.
- Use framework error/redirect primitives. Report safe structured failures; do not reveal internals.
- Semantic HTML, keyboard operability, focus and associated form errors are engineering fundamentals, not optional design polish.

## 6. Performance and reliability

- Minimize client JavaScript and interactive islands; use SSR/streaming only with correct runtime semantics.
- Bound lists, requests and concurrent work. Measure before adding caching, memoization or third-party state packages.
- Preserve SSR/client hydration consistency and correctly handle browser-only APIs.

## 7. Local quality gates

- Framework-version preflight is mandatory before audit, fix or material edits; mismatches require approval, never automatic upgrades.
- Run existing typecheck, lint, relevant unit/integration tests, production build and risk-driven E2E checks locally first.
- Do not trigger costly GitHub CI merely to discover local formatting, type or build errors.
- Commit coherent tests and ensure peer versions plus lockfile are consistent after any explicitly approved migration.

## 8. Audit severity

- P0: secret or cross-user data leak, server authorization bypass, dangerous unaudited mutation.
- P1: incorrect data/state, broken mutation/SSR/hydration, major framework misuse or unbounded workload.
- P2: proven maintainability, test coverage or performance gap with meaningful effect.
- Findings require exact path and evidence, consequence, confidence and actionable minimal repair. Avoid cosmetic mandates.
