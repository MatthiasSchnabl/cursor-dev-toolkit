#!/usr/bin/env bash
set -euo pipefail

export PATH="$HOME/.local/bin:$HOME/.bun/bin:$PATH"

TOOLKIT_ROOT="${CURSOR_DEV_TOOLKIT_ROOT:-/tmp/cursor-dev-toolkit}"
SCRIPT_DIR="$TOOLKIT_ROOT/scripts"

log() { printf 'cursor-dev-toolkit-start: %s\n' "$*"; }
warn() { printf 'cursor-dev-toolkit-start: WARN: %s\n' "$*" >&2; }

if [[ -z "${GBRAIN_DATABASE_URL:-}" ]]; then
  warn "GBRAIN_DATABASE_URL not configured"
  warn "Configure in Cursor Dashboard -> Cloud Agents -> Secrets"
  exit 0
fi

command -v gbrain >/dev/null 2>&1 || { warn "gbrain not on PATH"; exit 1; }

doctor_json="$(mktemp)"
gbrain doctor --json >"$doctor_json" 2>/dev/null || true
if [[ ! -s "$doctor_json" ]]; then
  warn "gbrain doctor produced no output"
  rm -f "$doctor_json"
  exit 1
fi

if command -v jq >/dev/null 2>&1; then
  conn="$(jq -r '.checks[] | select(.name=="connection") | .status' "$doctor_json" | head -1)"
  [[ "$conn" == "ok" ]] || { warn "gbrain connection not ok"; rm -f "$doctor_json"; exit 1; }
fi
rm -f "$doctor_json"
log "gbrain runtime ok"
