# Next.js Engineering Standard — 16.4.0 App Router

Tested baseline: Next.js 16.4.0; Node >=20.9; TypeScript >=5.1; compatible React/ReactDOM 19. Verify installed dependencies and router first.

## Contents

1. Version and App Router constraints
2. Server/Client Component boundaries
3. Data access and Server Functions
4. Cache Components and freshness
5. Routing, error handling, streaming
6. Security and authorization
7. Performance and tests
8. Manufacturer sources

## 1. Version and App Router constraints

- The rules apply to the Next.js 16.4.0 App Router, not a Pages Router-only app.
- Keep the default Turbopack model unless verified project constraints require otherwise.
- Next.js 16.4 recommends Cache Components for new apps, but existing apps may not have them enabled. Inspect next.config before applying cache rules.
- Never install a new framework version or run codemods on preflight mismatch without explicit user confirmation.

## 2. Server/Client Component boundaries

- Default layouts/pages to Server Components for server data loading and noninteractive composition.
- Mark use client only at required browser/interactivity boundaries. Avoid turning whole routes into client bundles unnecessarily.
- Prevent server-only imports and secrets from entering Client Components; serialize only safe, minimal data.
- Keep providers deep and route-level composition shallow. Treat hydration mismatches as bugs, not warnings to suppress.

## 3. Data access and Server Functions

- Server Components should call internal server services directly; avoid self-HTTP requests to own Route Handlers.
- Separate validated data access from presentation and place authorization on each sensitive operation.
- Server Functions/Actions can be invoked as network endpoints: validate, authenticate and authorize **inside each function**.
- Use Route Handlers for actual HTTP endpoints/webhooks, not as a substitute for direct server-function calls.
- Handle request data, cookies and dynamic params according to installed Next 16 semantics.

## 4. Cache Components and freshness

- On opted-in Cache Components apps, use explicit use cache boundaries, cacheLife and appropriate invalidation semantics.
- Determine cacheComponents and partialPrefetching from actual configuration; never mix older implicit caching assumptions.
- Keep request-private or user-scoped data out of shared cache entries; validate key/scope/TTL/invalidation.
- Do not automatically enable new config flags in an existing app merely because these rules recommend the model for greenfield.

## 5. Routing, error handling and streaming

- Use page.tsx, layout.tsx, loading.tsx, error.tsx, not-found.tsx and route.ts conventions properly.
- Prefer framework redirect/notFound and nearest error boundary. Do not swallow control flow in blanket catches.
- Validate URL params and query state, avoid request waterfalls, use Suspense with meaningful fallbacks.
- Keep correct separation of cached/static and dynamic content in streamed routes.

## 6. Security and authorization

- A route guard, proxy or hidden button never replaces backend authorization on a read/mutation.
- Do not serialize tokens, secrets or protected server state to client modules.
- Protect Server Actions against unvalidated input, replay or undesired bulk side effects. Do not cache private data publicly.
- Treat cookies, request headers and untrusted API responses as hostile until validated.

## 7. Performance and tests

- Minimize unnecessary client components, dependencies, duplicate client/server fetching and giant providers.
- Respect runtime Node/Edge selection, browser APIs and package version compatibility.
- Local gates: read-only version preflight, TypeScript, ESLint, unit/integration tests, Next build, critical E2E.
- Audit especially auth on Server Functions, Cache Components user boundaries, hydration, routing errors and unnecessary HTTP layers.

## 8. Manufacturer sources

- https://nextjs.org/blog/next-16-4
- https://nextjs.org/docs/app/getting-started/server-and-client-components
- https://nextjs.org/docs/app/getting-started/caching-and-revalidating
- https://nextjs.org/docs/app/getting-started/upgrading
- https://nextjs.org/docs/app/guides/data-security
