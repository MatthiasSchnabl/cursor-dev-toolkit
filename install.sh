#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_DEST="${HOME}/.cursor/plugins/local/cursor-dev-toolkit"

log() { printf 'install: %s\n' "$*"; }
die() { printf 'install: ERROR: %s\n' "$*" >&2; exit 1; }

mkdir -p "$(dirname "$PLUGIN_DEST")"

if [[ -e "$PLUGIN_DEST" ]]; then
  if [[ -L "$PLUGIN_DEST" ]]; then
    rm -f "$PLUGIN_DEST"
  elif [[ "$(cd "$PLUGIN_DEST" && pwd)" == "$(cd "$ROOT" && pwd)" ]]; then
    log "plugin already linked"
  else
    die "refusing to overwrite existing $PLUGIN_DEST"
  fi
fi

if [[ ! -e "$PLUGIN_DEST" ]]; then
  ln -s "$ROOT" "$PLUGIN_DEST"
  log "linked $PLUGIN_DEST -> $ROOT"
fi

chmod +x "$ROOT/scripts/"*.sh "$ROOT/hooks/"*.sh 2>/dev/null || true
chmod +x "$ROOT/install.sh" 2>/dev/null || true

log "running bootstrap..."
"$ROOT/scripts/bootstrap.sh"

cat <<EOF

cursor-dev-toolkit installed.

Next steps:
  1. Cursor -> Developer: Reload Window
  2. Customize -> confirm cursor-dev-toolkit under User scope
  3. Run: $ROOT/scripts/verify.sh

EOF
