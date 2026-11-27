---
description: Check or regenerate the configured Git documentation inventory (requires accepted docs-guard binary)
---

# Documentation Guard

Read [the canonical contract](../docs/documentation-guard.md) from the installed toolkit. Central Linux developer QA is accepted. Use the [verified central image and hash](../docs/documentation-guard-central-linux.md) through the existing test runner; do not install an unverified executable.

Check the repository's AGENTS.md and docs-guard.toml. Do not initialize existing repositories or alter their governance without approval.

For an opted-in Contabo project, use the approved central image through `test_run` with the consumer's authorized workspace and verify the executable hash first. For local installation, use only the pinned executable returned by `scripts/docs-guard-install.sh verify`; if no approved local release is available, stop rather than substitute a PATH executable. Run doctor, plan, generate only after staging the intended curated inputs, stage only generated outputs, then check --index. For a committed snapshot use check --commit HEAD. Never substitute a generation exit code for a check PASS. Never hide a gate failure or fabricate a release approval.
