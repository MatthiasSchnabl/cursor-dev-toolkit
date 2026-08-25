# Tooling Doctor

Run the cursor-dev-toolkit verification script and report PASS/WARN/FAIL for each component.

## Steps

1. Locate the toolkit root (installed plugin at `~/.cursor/plugins/local/cursor-dev-toolkit` or `CURSOR_DEV_TOOLKIT_ROOT`).
2. Run:

```bash
"$TOOLKIT_ROOT/scripts/verify.sh" --check-project-graph
```

3. Report results compactly: toolkit plugin, bun, graphify, gstack, Superpowers, GBrain, secrets (configured/missing only — never print values).

If failures occur, suggest `/tooling-setup`.
