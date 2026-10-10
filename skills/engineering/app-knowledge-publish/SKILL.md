---
name: app-knowledge-publish
description: Explicit operator-controlled publishing of a verified application knowledge release into an isolated pre-registered RAGGW release space. No automatic production activation and no unsafe partial activation.
disable-model-invocation: true
---

# Application Knowledge Publish — staged release only

This is a deliberate manual operation. Do not run it from a commit, push, CI hook or another skill without explicit user instruction.

## Direct references

- [Application Knowledge Contract](../../../docs/application-knowledge-contract.md) — especially release identity, staging, activation.
- [RAGGW integration requirements](../../../docs/application-knowledge-raggw.md).
- [Validator and staged publisher](../../../scripts/app-knowledge.mjs).

## Gate 1: immutable input

1. Identify the application root. Require a clean Git worktree and a known immutable source commit; never publish dirty/uncommitted artifacts.
2. Run the strict OpenAPI preflight and full inventory validation.
3. Check every document is `verified`, there are no blocking gaps, every OpenAPI operation is covered, and required document digests match current files.
4. Check golden retrieval/tool-selection eval plan exists. No claim of quality without executing those evals against the candidate release.
5. Confirm RAGGW configuration/environment, exact target space and grants. Never display secrets.

## Gate 2: operator plan

Execute `bun "$CURSOR_DEV_TOOLKIT_ROOT/scripts/app-knowledge.mjs" publish-plan --root "$APP_ROOT"`.

Present source Git SHA, application version, document count, OpenAPI digest, release fingerprint and immutable candidate space. If any prerequisite fails, stop; no RAGGW mutation.

## Gate 3: isolated staging (explicit confirmation)

Only after the user approves **that exact fingerprint and target release space**:

Set `RAGGW_URL` and `RAGGW_TOKEN` through the runtime environment (never write credentials to source), and execute:

`bun "$CURSOR_DEV_TOOLKIT_ROOT/scripts/app-knowledge.mjs" stage --root "$APP_ROOT" --space "$SPACE" --confirm "$RELEASE_ID"`

The publisher writes versioned documents only into a **pre-provisioned release-specific Knowledge Space**, waits for each RAGGW job to succeed and emits a staging receipt. On failure, the release stays unactivated. Never retry a dead_letter job silently.

## Gate 4: independent consumer activation

**Do not activate production from this skill.** The current RAGGW API offers single-document ingestion, not a proven transactional whole-release activation primitive. Require held-out retrieval evals, authorization review, deployment compatibility and a separate app/operator cutover of the active release space.

Report `STAGED_NOT_ACTIVATED`; never say "published to production" merely because jobs succeeded.
