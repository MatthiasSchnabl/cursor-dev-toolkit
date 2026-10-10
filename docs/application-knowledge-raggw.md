# RAGGW Application Knowledge Release Integration

## Contract

The current RAGGW v1 application surface provides `/v1/discovery`, `/v1/documents`, and `/v1/jobs/{job_id}`. The agent surface provides read-only retrieval tools. The ApplicationRegistration grants a consuming app access to explicitly registered Knowledge Spaces.

**No proof of whole-release atomic activation exists in this contract.** The global publisher supports isolated candidate-space *staging only*. No publish operation may reconfigure a running assistant, enable a candidate in production, or silently replace the currently active release.

## Provisioning

Before staging a knowledge release, an operator must provision an isolated Knowledge Space named `<application_id>.release.<release_fingerprint>`, with correct classification, retention and egress settings, then explicitly grant write permission to the publisher registration. The consumer must not be granted access until evaluation and cutover. The publisher must never create spaces or self-grant permissions.

Use OAuth credentials via the runtime environment: `RAGGW_URL` and `RAGGW_TOKEN`. Never store tokens in an app repo.

## Publishing

1. `publish-plan` validates OpenAPI, all seven knowledge categories, all operationIds, SHA-256 digests, reviewed documents and blocking gaps.
2. Operator confirms exact fingerprint and target Knowledge Space.
3. `stage` calls discovery and requires an explicit `can_write` grant on the exact release space.
4. Each versioned Markdown document is ingested with a stable `source_id` and idempotency key, and the job is polled until `succeeded`.
5. The output is `STAGED_NOT_ACTIVATED`. On any failure, leave the candidate isolated; report exact document ID and reason without leaking secrets.
6. Run separate retrieval and tool-selection golden evaluations. Check precise API IDs as well as task-based questions.
7. A distinct app deployment/cutover mechanism must activate the release-specific Knowledge Space only after successful evaluation.

The app (e.g. Rust Rig assistant) handles live per-user data and all actions through authorized app services. RAGGW never executes app operations. Backend authorization, approval for sensitive actions, audit logging and least privilege are always the app's responsibility.

## Preproduction gaps

- Full official OpenAPI conformance linting beyond the current deterministic structural validator.
- Machine-verified held-out retrieval/tool-selection quality gate (app-specific).
- A RAGGW-native atomic Knowledge Release activation primitive or proven deployment-controlled cutover.
- A release-space lifecycle and tombstone/retention cleanup policy.

These are explicit operational gates, not implicit promises from a successful ingestion.
