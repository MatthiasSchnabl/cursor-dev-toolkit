# Tooling Doctor

Run the cursor-dev-toolkit verification and report PASS/INFO/FAIL compactly.

## Steps

1. Locate the toolkit root (installed plugin at `~/.cursor/plugins/local/cursor-dev-toolkit` or `CURSOR_DEV_TOOLKIT_ROOT`).
2. Run:

```bash
"$TOOLKIT_ROOT/scripts/verify.sh" --check-project-graph
```

3. The verification includes the engineering-context doctor:
   - installed plugin/source freshness,
   - manifest/version consistency,
   - compact engineering rule anchors,
   - SKILL.md size limits,
   - Contents maps for long references,
   - SessionStart context injection.
4. Report toolkit, engineering context, bun, graphify, gstack, Superpowers, GBrain and project graph.
5. Never print secret values.

If installation/context checks fail, refresh with `install.ps1` / `install.sh`, reload Cursor, and rerun `/engineering-context-doctor`.
