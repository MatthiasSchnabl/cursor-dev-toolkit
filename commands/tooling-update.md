# Tooling Update

Update runtime tools to versions pinned in `versions.env` after the user has bumped refs intentionally.

## Steps

1. Confirm `versions.env` was updated in the toolkit repository.
2. Run:

```bash
"$TOOLKIT_ROOT/scripts/bootstrap.sh"
```

3. Run `/tooling-doctor`.

Do not auto-upgrade to `latest` or `main`. Only install pinned refs from `versions.env`.
