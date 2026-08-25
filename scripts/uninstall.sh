#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOLKIT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
# shellcheck source=lib/common.sh
source "$SCRIPT_DIR/lib/common.sh"

GSTACK_DIR="${GSTACK_DIR:-$HOME/.gstack/repos/gstack}"
SUPERPOWERS_DIR="${SUPERPOWERS_DIR:-$HOME/.cursor/plugins/local/superpowers}"
GBRAIN_DIR="${GBRAIN_DIR:-$HOME/.gbrain/repos/gbrain}"
PLUGIN_LINK="$(cursor_plugins_local_dir)/cursor-dev-toolkit"

log "removing toolkit plugin link at $PLUGIN_LINK"
if [[ -L "$PLUGIN_LINK" ]]; then
  rm -f "$PLUGIN_LINK"
elif [[ -d "$PLUGIN_LINK" ]] && [[ "$(cd "$PLUGIN_LINK" && pwd)" == "$(cd "$TOOLKIT_ROOT" && pwd)" ]]; then
  rm -rf "$PLUGIN_LINK"
fi

log "removing gbrain launcher (repo checkout preserved at $GBRAIN_DIR)"
rm -f "$HOME/.local/bin/gbrain"

log "uninstall complete — gstack, superpowers, graphify runtimes preserved"
log "to remove runtimes manually: delete $GSTACK_DIR, $SUPERPOWERS_DIR, uv tool uninstall graphifyy"
