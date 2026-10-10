---
name: app-knowledge-inventory
description: Inventory an application's versioned product, domain, UI, workflow, authorization and OpenAPI knowledge for a private in-app assistant using RAGGW. Hard-stop if OpenAPI is absent or structurally invalid. Never publish or invent business semantics.
disable-model-invocation: true
---

# Application Knowledge Inventory

Use the globally installed toolkit; work in the current **application repository**, never clone the toolkit into it.

## Mandatory preflight (before edits)

1. Identify the application's root using git. Inspect existing app-knowledge.config.json if present.
2. Execute: `bun "$CURSOR_DEV_TOOLKIT_ROOT/scripts/app-knowledge.mjs" preflight --root "$APP_ROOT"`.
3. If the command fails, **STOP without writing inventory files**. Report missing OpenAPI, invalid operationIds/responses/refs, and the need for a bundled OpenAPI JSON document.
4. The preflight checks essential structure and references, not complete OAS conformance. Require a pinned OpenAPI linter for full spec compliance before production adoption.

## Direct references

- [Application Knowledge Contract](../../../docs/application-knowledge-contract.md) — read its **Contents** and relevant sections.
- [Configuration example](../../../docs/examples/app-knowledge.config.example.json).
- [Manifest example](../../../docs/examples/app-knowledge.manifest.example.json).
- [Deterministic validator](../../../scripts/app-knowledge.mjs).

## Repository investigation

Map the entire application, not only routes:

- Product functions, screens, navigation, user-visible behavior.
- Domain objects, field meaning, enums, validation, status transitions, invariants.
- End-to-end workflows and realistic user tasks.
- Roles, permission boundaries and per-operation authorization.
- HTTP API operationIds, request/response fields, errors, preconditions, idempotency and effects.
- For Rust Rig assistant integration: typed tool capabilities, forbidden/bulk actions, confirmation policy and backend-authenticated caller context.

Use OpenAPI as API contract authority, code/tests as behavioral evidence, and business docs for intent. Cross-check conflicting evidence; record gaps rather than hallucinating.

## Produce application-owned artifacts

After successful preflight, create/update `app-knowledge.config.json`, `docs/app-knowledge/manifest.json` and Markdown in the seven categories defined by the contract. Use stable document IDs, SHA-256 digests, sources[], explicit operation_ids[] coverage and verification statuses. Keep knowledge concise, task-oriented, searchable and version-specific; never dump the entire codebase into RAGGW.

- Generate initial inventory in coherent slices; preserve existing verified facts unless contradicted by stronger evidence.
- For uncertain semantics set `needs_review` or add a blocking gap. Do not silently mark inferred facts verified.
- Never include production data, personal data, API tokens or direct database extracts.
- Keep Git source commit separate from mutable manifest hashes; publish records a commit SHA later.

## Closure

Run `bun "$CURSOR_DEV_TOOLKIT_ROOT/scripts/app-knowledge.mjs" validate --root "$APP_ROOT"`.

Report: app version, OpenAPI digest, number of operations, covered operations, each knowledge category, verified vs needs_review, blocking gaps, and whether the package is publish-ready. A failing validation is an honest **partial inventory**, never a success claim. Do not invoke RAGGW APIs and do not publish.
