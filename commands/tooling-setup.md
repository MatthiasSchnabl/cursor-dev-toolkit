# Tooling Setup

Install or repair missing cursor-dev-toolkit runtime components idempotently.

## Steps

1. Find toolkit root at `~/.cursor/plugins/local/cursor-dev-toolkit` or set `CURSOR_DEV_TOOLKIT_ROOT`.
2. Run:

```bash
"$TOOLKIT_ROOT/scripts/bootstrap.sh"
```

3. Optionally build project graph:

```bash
"$TOOLKIT_ROOT/scripts/graphify-ensure-project.sh"
```

4. Run `/tooling-doctor` to confirm.

Do not write secrets. Do not run `gbrain init`.
