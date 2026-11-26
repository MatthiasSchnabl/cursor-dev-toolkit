---
description: Check or regenerate the configured Git documentation inventory (requires accepted docs-guard binary)
---

# Documentation Guard

Read [the canonical contract](../docs/documentation-guard.md) from the installed toolkit. This command is staged for later rollout; do not install an unverified executable.

Check the repository's AGENTS.md and docs-guard.toml. Do not initialize existing repositories or alter their governance without approval.

For an opted-in project, use the pinned installed docs-guard executable. Run doctor, plan, generate only after staging the intended curated inputs, stage only generated outputs, then check --index. For a committed snapshot use check --commit HEAD. Never substitute a generation exit code for a check PASS. Never hide a gate failure or fabricate a release approval.
