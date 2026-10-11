# SvelteKit Engineering Standard — 3.0.0

Tested baseline: SvelteKit 3.0.0; Svelte >=5.57.1; Node >=22.17; TypeScript >=6; Vite >=8.0.12; vite-plugin-svelte >=7. Run mandatory read-only frontend preflight before applying these rules.

## Contents

1. Version 3 migration and runtime
2. Routing and server/client boundaries
3. Data loading and invalidation
4. Forms and mutations
5. Svelte 5 reactivity and state
6. Authentication and safety
7. Error handling and navigation
8. Rendering, performance and testing
9. Manufacturer sources

## 1. Version 3 migration and runtime

- These rules must not be applied to unupgraded SvelteKit 2 projects. Stop and ask about planned migration.
- SvelteKit configuration belongs to the sveltekit(...) Vite plugin in vite.config.*, not legacy svelte.config.js.
- Prefer Node package imports alias #lib rather than legacy $lib for new code; follow actual package imports config.
- Use current $app/env, not old $app/environment. Consult the v3 migration guide before changing v2 sources.
- Remote functions remain experimental in 3.0; prohibit adoption as unreviewed default. Require opt-in and dedicated tests.

## 2. Routing and server/client boundaries

- Use +page.svelte, +layout.svelte, +page.ts, +page.server.ts, +layout.server.ts, +server.ts and +error.svelte according to their routing responsibilities.
- Server secrets, cookies and database code must remain in server-only modules, server load and server actions.
- Store request- and user-specific data in event.locals or other request-local ownership; not mutable shared module globals.
- Prefer framework routing/layouts over custom routers or transporting server results through self-HTTP endpoints.

## 3. Data loading and invalidation

- Use server-only load for credentials/private APIs; universal load only when browser/server parity and serialization are intended.
- Derive route types from generated ./\$types. Validate params and avoid leaking more data to client than necessary.
- Use depends/invalidate appropriately and avoid avoidable chained awaits; do not confuse client navigation with full SSR lifecycle.
- Avoid external side effects in load when they can be replayed due to navigation/invalidation.

## 4. Forms and mutations

- Prefer +page.server.ts actions plus fail(status, safe field errors) and progressive use:enhance for ordinary HTML forms.
- Validate and authorize inside the action, not only browser controls. Honor no-JS fallback and redirect rules.
- Use +server.ts for genuine HTTP endpoints and webhooks. Do not create duplicate mutation surfaces without a concrete need.
- Account for SvelteKit 3 enhanced action failure status codes, idempotency and retry boundaries.

## 5. Svelte 5 reactivity and state

- Prefer $state, $derived and $effect according to semantics; compute derived values with $derived, not side-effect loops.
- Keep state scoped to component/request/context as needed. Never share mutable per-user state across SSR requests.
- Gate browser APIs to client runtime and avoid hydration mismatches.

## 6. Authentication and safety

- Use hooks.server.ts/event.locals where appropriate, with authorization on every sensitive data read and mutation.
- Do not serialize tokens/cookies to browser. Use SvelteKit built-in CSRF origin checks, v3 trustedOrigins configuration, and safe cookie attributes.
- Validate request shape, upload size and error output; avoid raw unsanitized HTML.

## 7. Error handling and navigation

- Prefer native error, fail, redirect, +error.svelte and link navigation mechanics.
- Differentiate invalid input, authentication, authorization, not-found and infrastructure failure with accurate statuses.

## 8. Rendering, performance and testing

- Choose SSR, prerender and client behavior explicitly; do not disable SSR to hide state problems.
- Validate adapter/deploy/runtime compatibility. Avoid expensive parent loads and unbounded streaming.
- Local gates: Node/version preflight, svelte-check, lint, relevant tests, build; E2E for critical navigation and forms.
- Audit especially SSR session isolation, action permissions, form failure behavior, route invalidation and migration syntax.

## 9. Manufacturer sources

- https://svelte.dev/blog/sveltekit-3-is-here
- https://svelte.dev/docs/kit/migrating-to-sveltekit-3
- https://svelte.dev/docs/kit/routing
- https://svelte.dev/docs/kit/load
- https://svelte.dev/docs/kit/form-actions
- https://svelte.dev/docs/kit/state-management
