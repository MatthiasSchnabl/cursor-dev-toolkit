#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_DEST="${HOME}/.cursor/plugins/local/cursor-dev-toolkit"
# shellcheck source=scripts/lib/common.sh
source "$ROOT/scripts/lib/common.sh"
load_versions "$ROOT"

log() { printf 'install: %s\n' "$*"; }
die() { printf 'install: ERROR: %s\n' "$*" >&2; exit 1; }

mkdir -p "$(dirname "$PLUGIN_DEST")"
root_resolved="$(cd "$ROOT" && pwd)"
dest_resolved=""
if [[ -d "$PLUGIN_DEST" ]]; then dest_resolved="$(cd "$PLUGIN_DEST" && pwd)"; fi

copy_plugin_tree() {
  local source="$1" destination="$2"
  if [[ -e "$destination" || -L "$destination" ]]; then rm -rf "$destination"; fi
  mkdir -p "$destination"
  (cd "$source" && tar --exclude='.git' --exclude='.cursor-toolkit-install' -cf - .) | (cd "$destination" && tar -xf -)
  local commit="unknown"
  git -C "$source" rev-parse HEAD >/dev/null 2>&1 && commit="$(git -C "$source" rev-parse HEAD)"
  {
    printf '%s\n' "$source"
    printf '%s\n' "$commit"
    printf '%s\n' "$TOOLKIT_VERSION"
  } >"$destination/.cursor-toolkit-install"
  log "copied $source -> $destination"
}

if [[ -L "$PLUGIN_DEST" ]]; then
  log "replacing symlink with a real local plugin copy"
  copy_plugin_tree "$ROOT" "$PLUGIN_DEST"
elif [[ -n "$dest_resolved" && "$dest_resolved" == "$root_resolved" ]]; then
  log "checkout already lives at $PLUGIN_DEST"
elif [[ -e "$PLUGIN_DEST" ]]; then
  if [[ -f "$PLUGIN_DEST/.cursor-plugin/plugin.json" ]]; then
    log "refreshing local plugin copy"
    copy_plugin_tree "$ROOT" "$PLUGIN_DEST"
  else
    die "refusing to overwrite unrelated $PLUGIN_DEST"
  fi
else
  copy_plugin_tree "$ROOT" "$PLUGIN_DEST"
fi

chmod +x "$ROOT/scripts/"*.sh "$ROOT/install.sh" 2>/dev/null || true

log "running bootstrap..."
"$ROOT/scripts/bootstrap.sh"

cat <<EOF

cursor-dev-toolkit installed.

Next steps:
  1. Cursor -> Developer: Reload Window
  2. Customize -> confirm cursor-dev-toolkit under User scope
  3. Run: $ROOT/scripts/verify.sh
  4. In Cursor, run /engineering-context-doctor if engineering rules/skills look stale.

EOF
