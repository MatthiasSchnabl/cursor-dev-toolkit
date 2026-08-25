---
name: graphify
description: Use for structural codebase questions, dependencies, and change-impact analysis when graphify-out/ exists or can be built.
---

# Graphify

Use Graphify for architecture discovery, dependency paths, and change-impact analysis.

## Commands

```bash
graphify query "<question>"
graphify path "<symbol-a>" "<symbol-b>"
graphify explain "<concept>"
graphify extract . --code-only --no-cluster   # build graph if missing
graphify update . --no-cluster                # refresh after structural changes
```

## When to use

- Unfamiliar subsystems
- Cross-module relationships
- Impact analysis before refactors

## When NOT to use

- Known exact string matches (use Grep)
- Durable project history (use GBrain)
- Trivial one-file edits when the file is already known

Graph output lives in `graphify-out/` per repository. Verify important conclusions against source code.
