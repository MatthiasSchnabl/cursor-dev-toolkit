#!/usr/bin/env bash
# Shared helpers for cursor-dev-toolkit scripts.
set -euo pipefail

toolkit_root() {
  local src="${BASH_SOURCE[1]:-${BASH_SOURCE[0]}}"
  cd "$(dirname "$src")/.." && pwd
}

cursor_plugins_local_dir() {
  if [[ -n "${CURSOR_PLUGINS_LOCAL:-}" ]]; then
    printf '%s' "$CURSOR_PLUGINS_LOCAL"
    return 0
  fi
  printf '%s/.cursor/plugins/local' "${HOME}"
}

load_versions() {
  local root="$1"
  local env_file="${root}/versions.env"
  [[ -f "$env_file" ]] || { echo "cursor-dev-toolkit: missing $env_file" >&2; return 1; }
  # shellcheck disable=SC1090
  source "$env_file"
  : "${TOOLKIT_VERSION:?}"
  : "${BUN_VERSION:?}"
  : "${GSTACK_REF:?}"
  : "${GRAPHIFY_VERSION:?}"
  : "${SUPERPOWERS_REF:?}"
  : "${GBRAIN_REF:?}"
}

log() { printf 'cursor-dev-toolkit: %s\n' "$*"; }
warn() { printf 'cursor-dev-toolkit: WARN: %s\n' "$*" >&2; }
die() { printf 'cursor-dev-toolkit: ERROR: %s\n' "$*" >&2; exit 1; }

ensure_path_entry() {
  local dir="$1"
  mkdir -p "$dir"
  case ":$PATH:" in
    *:"$dir":*) ;;
    *) export PATH="$dir:$PATH" ;;
  esac
}

persist_path_dirs() {
  local marker_begin="# >>> cursor-dev-toolkit PATH >>>"
  local marker_end="# <<< cursor-dev-toolkit PATH <<<"
  local block
  block=$(cat <<EOF
$marker_begin
export PATH="\$HOME/.local/bin:\$HOME/.bun/bin:\$PATH"
$marker_end
EOF
)
  for rc in "$HOME/.profile" "$HOME/.bashrc"; do
    if [[ -f "$rc" ]] && grep -Fq "$marker_begin" "$rc"; then
      continue
    fi
    printf '\n%s\n' "$block" >>"$rc"
  done
}

install_linux_prereqs() {
  if [[ "$(uname -s)" != "Linux" ]] || ! command -v apt-get >/dev/null 2>&1; then
    return 0
  fi
  log "ensuring Linux prerequisites via apt"
  export DEBIAN_FRONTEND=noninteractive
  sudo apt-get update -qq
  sudo apt-get install -y -qq git curl ca-certificates jq python3 build-essential
}

require_cmd() {
  local cmd="$1"
  command -v "$cmd" >/dev/null 2>&1 || die "required command not found: $cmd"
}
