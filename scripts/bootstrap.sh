#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOLKIT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
# shellcheck source=lib/common.sh
source "$SCRIPT_DIR/lib/common.sh"
load_versions "$TOOLKIT_ROOT"

GSTACK_DIR="${GSTACK_DIR:-$HOME/.gstack/repos/gstack}"
SUPERPOWERS_DIR="${SUPERPOWERS_DIR:-$HOME/.cursor/plugins/local/superpowers}"
GBRAIN_DIR="${GBRAIN_DIR:-$HOME/.gbrain/repos/gbrain}"
WITH_PROJECT_GRAPH=0
PROJECT_ROOT="${PROJECT_ROOT:-$(pwd)}"

usage() {
  cat <<'EOF'
Usage: scripts/bootstrap.sh [--with-project-graph] [--project-root DIR]

Installs pinned runtime tools (gstack, Graphify, Superpowers, GBrain).
Idempotent. Does not write secrets or run gbrain init.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --with-project-graph) WITH_PROJECT_GRAPH=1; shift ;;
    --project-root) PROJECT_ROOT="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown argument: $1" ;;
  esac
done

install_bun() {
  ensure_path_entry "$HOME/.bun/bin"
  if command -v bun >/dev/null 2>&1; then
    local current
    current="$(bun --version 2>/dev/null || true)"
    if [[ "$current" == "$BUN_VERSION" ]]; then
      log "bun $BUN_VERSION already installed"
      return 0
    fi
    log "upgrading bun from ${current:-unknown} to $BUN_VERSION"
  else
    log "installing bun $BUN_VERSION"
  fi
  curl -fsSL "https://bun.sh/install" | bash -s -- "bun-v${BUN_VERSION}"
  ensure_path_entry "$HOME/.bun/bin"
  command -v bun >/dev/null 2>&1 || die "bun install failed"
  [[ "$(bun --version)" == "$BUN_VERSION" ]] || die "bun version mismatch"
}

install_uv() {
  ensure_path_entry "$HOME/.local/bin"
  if command -v uv >/dev/null 2>&1; then
    log "uv already installed"
    return 0
  fi
  log "installing uv"
  local installer="$HOME/.local/bin/uv-installer.sh"
  curl -fsSL "https://astral.sh/uv/install.sh" -o "$installer"
  bash "$installer"
  rm -f "$installer"
  ensure_path_entry "$HOME/.local/bin"
  command -v uv >/dev/null 2>&1 || die "uv install failed"
}

install_graphify() {
  ensure_path_entry "$HOME/.local/bin"
  log "installing graphifyy==${GRAPHIFY_VERSION}"
  uv tool install --force "graphifyy==${GRAPHIFY_VERSION}"
  command -v graphify >/dev/null 2>&1 || die "graphify not on PATH"
  local installed
  installed="$(graphify --version 2>/dev/null | awk '{print $NF}')"
  [[ "$installed" == "$GRAPHIFY_VERSION" ]] || die "graphify version mismatch"
}

install_gstack() {
  mkdir -p "$(dirname "$GSTACK_DIR")"
  if [[ ! -d "$GSTACK_DIR/.git" ]]; then
    log "cloning gstack into $GSTACK_DIR"
    git clone --quiet https://github.com/garrytan/gstack.git "$GSTACK_DIR"
  fi
  cd "$GSTACK_DIR"
  git fetch --quiet origin
  local current
  current="$(git rev-parse HEAD)"
  if [[ "$current" != "$GSTACK_REF" ]]; then
    log "checking out gstack ref $GSTACK_REF"
    git checkout --quiet "$GSTACK_REF"
  else
    log "gstack already at $GSTACK_REF"
  fi
  [[ "$(git rev-parse HEAD)" == "$GSTACK_REF" ]] || die "gstack checkout failed"
  log "running gstack setup for Cursor"
  GSTACK_SKIP_COREUTILS="${GSTACK_SKIP_COREUTILS:-1}" ./setup --host cursor --prefix --quiet
}

install_superpowers() {
  mkdir -p "$(dirname "$SUPERPOWERS_DIR")"
  if [[ ! -d "$SUPERPOWERS_DIR/.git" ]]; then
    log "cloning superpowers into $SUPERPOWERS_DIR"
    git clone --quiet https://github.com/obra/superpowers.git "$SUPERPOWERS_DIR"
  fi
  cd "$SUPERPOWERS_DIR"
  git fetch --quiet origin
  local current
  current="$(git rev-parse HEAD)"
  if [[ "$current" != "$SUPERPOWERS_REF" ]]; then
    log "checking out superpowers ref $SUPERPOWERS_REF"
    git checkout --quiet "$SUPERPOWERS_REF"
  else
    log "superpowers already at $SUPERPOWERS_REF"
  fi
  [[ "$(git rev-parse HEAD)" == "$SUPERPOWERS_REF" ]] || die "superpowers checkout failed"
  [[ -f "$SUPERPOWERS_DIR/.cursor-plugin/plugin.json" ]] || die "superpowers manifest missing"
}

install_gbrain() {
  ensure_path_entry "$HOME/.local/bin"
  mkdir -p "$(dirname "$GBRAIN_DIR")"
  if [[ ! -d "$GBRAIN_DIR/.git" ]]; then
    log "cloning gbrain into $GBRAIN_DIR"
    git clone --quiet https://github.com/garrytan/gbrain.git "$GBRAIN_DIR"
  fi
  cd "$GBRAIN_DIR"
  git fetch --quiet origin
  local current
  current="$(git rev-parse HEAD)"
  if [[ "$current" != "$GBRAIN_REF" ]]; then
    log "checking out gbrain ref $GBRAIN_REF"
    git checkout --quiet "$GBRAIN_REF"
  else
    log "gbrain already at $GBRAIN_REF"
  fi
  [[ "$(git rev-parse HEAD)" == "$GBRAIN_REF" ]] || die "gbrain checkout failed"
  log "installing gbrain dependencies with bun"
  bun install --frozen-lockfile 2>/dev/null || bun install
  local launcher="$HOME/.local/bin/gbrain"
  cat >"$launcher" <<LAUNCHER
#!/usr/bin/env bash
exec bun "$GBRAIN_DIR/src/cli.ts" "\$@"
LAUNCHER
  chmod +x "$launcher"
  command -v gbrain >/dev/null 2>&1 || die "gbrain launcher failed"
  log "gbrain installed ($(gbrain --version 2>/dev/null || echo unknown))"
}

build_project_graph() {
  local graph_file="$PROJECT_ROOT/graphify-out/graph.json"
  if [[ -f "$graph_file" ]]; then
    log "graphify graph already present at $graph_file"
    return 0
  fi
  log "building code-only graph for $PROJECT_ROOT"
  (cd "$PROJECT_ROOT" && graphify extract . --code-only --no-cluster)
  [[ -f "$graph_file" ]] || die "graphify extract did not produce graph.json"
}

main() {
  install_linux_prereqs
  require_cmd git
  require_cmd curl
  persist_path_dirs
  install_bun
  install_uv
  install_graphify
  install_gstack
  install_superpowers
  install_gbrain
  if [[ "$WITH_PROJECT_GRAPH" -eq 1 ]]; then
    build_project_graph
  fi
  log "bootstrap complete (toolkit $TOOLKIT_VERSION)"
}

main "$@"
