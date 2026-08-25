---
name: gbrain
description: Use GBrain for durable project knowledge, prior decisions, and historical context before guessing.
---

# GBrain

Use GBrain when prior project knowledge may materially affect the task.

## Use cases

- Previous architecture decisions and implementation choices
- Historical constraints, incidents, learnings
- Project conventions not fully represented in current code

Query before guessing. Treat GBrain as contextual evidence, not source-code truth.

## When NOT to use GBrain

- Source-code search (use Graphify or Grep)
- Code structure / dependency analysis (use Graphify)
- Build, test, or deployment operations

## Commands

```bash
gbrain search "<terms>"
gbrain query "<question>"
gbrain code-def <symbol>
gbrain code-refs <symbol>
gbrain code-callers <symbol>
gbrain code-callees <symbol>
```

Use `--source <source-id>` when querying a specific indexed corpus.

## Verification discipline

Verify facts against source code, schemas, tests, and current docs. Never write secrets to GBrain.
