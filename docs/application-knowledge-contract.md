# Application Knowledge Contract v1

## Contents

- 1. Goal and authority
- 2. Prerequisite: OpenAPI
- 3. Configuration and files
- 4. Inventory evidence and completeness
- 5. Release identity and lifecycle
- 6. RAGGW staging and activation
- 7. Engineering maintenance and gates
- 8. Security and assistant integration
- 9. Acceptance checks

## 1. Goal and authority

One globally installed cursor-dev-toolkit serves multiple independent app repositories. Each app operates its own assistant (for example Rust Rig). RAGGW is the app's **versioned product knowledge plane** only. Runtime user/business data stays in the application's authorized services and database. RAGGW is not an action-authorization authority.

Knowledge facts carry source evidence (relative code/test/OpenAPI/document paths), verification status, and the app release version. A model must not assert undocumented business semantics as facts. Record unresolved facts in manifest.gaps. Never automatically ingest secrets, user data, production dumps, prompts, customer records, or live database results.

## 2. Prerequisite: OpenAPI

Run the deterministic preflight **before writing inventory files**. Missing/invalid OpenAPI means hard stop and an actionable report. A machine-readable **JSON** OpenAPI 3.0/3.1 document is required for contract v1 (a YAML source can be exported/bundled to JSON first). Every operation needs a unique operationId and responses. All local references must resolve and external references must be bundled. This is a strict structural preflight, **not a complete official OAS conformance test**; integrate a pinned spec linter for full conformance before operational rollout.

OpenAPI is the authority for HTTP operation names and schemas; do not infer or invent operations when absent. Domain rules, UI workflows, authorization semantics, and tool side effects require separate evidence.

## 3. Configuration and files

The app owns these files; the plugin is **not** cloned into the app:

- app-knowledge.config.json: schema_version=1, application_id, openapi_path, knowledge_dir="docs/app-knowledge", release_space_prefix="<application_id>.release.", watch_paths (prefixes)
- docs/app-knowledge/manifest.json: schema_version=1, application_id, app_version (equals OpenAPI info.version), openapi_sha256, documents[], gaps[]
- docs/app-knowledge/{product,domain,workflows,ui,permissions,api,agent-capabilities}/*.md: curated versioned knowledge
- docs/app-knowledge/evals/: app-specific golden questions and execution evidence (not automatically treated as passing)

Each manifest.documents entry: id (stable within app), kind (one of seven), path (relative Markdown), sha256 (raw bytes), verification ("verified" or "needs_review"), sources[] (existing relative repo paths; human-readable source locations also belong inside Markdown), operation_ids[] (a subset of actual OpenAPI operations). Each API operationId must be covered by at least one document. Empty operation_ids is valid for non-API documents. All seven categories must exist.

The manifest must not store its own hash or a mutable HEAD SHA (self-reference). A release fingerprint is computed deterministically from application_id, app_version, OpenAPI digest, and sorted document digests. At publish time, the immutable **source Git SHA** is recorded in the plan/receipt. Do not confuse Git SHA, application version, and RAGGW knowledge_revision.

## 4. Inventory evidence and completeness

Inventory UI navigation, page actions, domain entities and field semantics, status machines, use cases, workflows, roles/permissions, HTTP errors, side effects, authorization boundaries, and agent tool capabilities. Derive from code + tests + schemas + explicit business documentation. A generated API catalogue alone is not sufficient.

Cross-check: every OpenAPI operation represented; every important UI workflow mapped; required and conditional fields understood; permission and state preconditions stated; API errors and idempotency explained; unsafe/destructive operations marked as not exposed by default.

Gaps use {"id":"...","description":"...","blocks_publish":true|false}. Evidence of unsupported meaning must remain a gap. Publication is blocked by any blocks_publish gap or any needs_review document. A passing structural validator is necessary but **not evidence of complete semantic correctness**.

## 5. Release identity and lifecycle

PROPOSED -> INVENTORIED -> REVIEWED -> STAGED -> EVALUATED -> ACTIVATED -> RETIRED.

- PROPOSED/INVENTORIED: generated knowledge may be incomplete.
- REVIEWED: manifest validates and humans reviewed inference-heavy semantics.
- STAGED: every RAGGW ingestion job succeeded into an **isolated, pre-registered release-specific space**.
- EVALUATED: app-specific golden question and tool-selection evals passed in that exact space.
- ACTIVATED: the consuming app intentionally changes to that release space **together with a compatible application deployment**.
- RETIRED: old space/access revoked after consumer cutover and retention review.

The plugin may stage but **must never claim ACTIVATED**. There is no validated native atomic activation API in RAGGW v1. Changes to app runtime configuration/authorization are external and require their own deployment gate. A partially ingested candidate space must never be used by consumers.

## 6. RAGGW staging and activation

Use RAGGW's existing Application API, not direct Qdrant writes:

- GET /v1/discovery verifies the exact release space has an explicit write grant.
- POST /v1/documents with Idempotency-Key and stable release-prefixed source_id.
- GET /v1/jobs/{job_id} until succeeded, failed or dead_letter.
- Track knowledge_revision as ingestion watermark, **not** a whole-release transaction.

Space key is release_space_prefix + computed release fingerprint; it must already exist with correct ApplicationRegistration grants. A distinct knowledge space is required for each release. The publisher fails closed if staging configuration, grants, approvals, or ingestion are incomplete. Production consumers must not read candidate release spaces.

Manual operator explicitly runs /app-knowledge-publish and confirms the plan/fingerprint. Environment secrets RAGGW_URL and RAGGW_TOKEN are supplied at runtime only. No write from an ordinary commit or CI event.

## 7. Engineering maintenance and gates

Cursor's global maintenance rule applies only when the app opts in via app-knowledge.config.json. After a material change, update the affected knowledge docs + manifest SHA hashes. A pure internal refactor can record a reviewed no-behavior-change decision in the manifest; never claim a non-change without checking API/UI/domain/authorization impact.

Run the deterministic local validator before committing. Optional local hook runs staged diff checking; existing hooks may not be overwritten. CI is an optional backstop, not an expensive LLM regeneration loop. No GitHub Action is required to publish.

## 8. Security and assistant integration

The app assistant is internal to that app. Reads of dynamic data and all actions go to typed application tools/services enforcing the current user's actual permissions; do not execute SQL suggested by the model. Destructive/bulk actions are denied by default unless an explicit application policy allows them. Tool descriptions and request schemas follow OpenAPI where compatible, but tool authorization is enforced server-side. Tool invocation must be auditable and may require user confirmation.

Knowledge retrieval, model output, and tool input remain untrusted until validated. Redact secrets and sensitive business data from knowledge files. Classification and external-egress policy must be set on the RAGGW space.

## 9. Acceptance checks

- Strict OpenAPI preflight fails before inventory begins when source missing.
- One plugin installed globally; no app-local copy required.
- Full operationId coverage and seven knowledge categories.
- SHA-256 drift detection and direct evidence refs.
- Independent release fingerprint, clean worktree and explicit staging confirmation.
- Isolated release space; no automatic activation.
- Human-reviewed semantics and held-out retrieval/tool evals before consumer cutover.
