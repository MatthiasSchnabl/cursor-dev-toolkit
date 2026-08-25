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
PLUGIN_DIR="${PLUGIN_DIR:-$(cursor_plugins_local_dir)/cursor-dev-toolkit}"
PROJECT_ROOT="${PROJECT_ROOT:-$(pwd)}"
CHECK_PROJECT_GRAPH=0

export PATH="$HOME/.local/bin:$HOME/.bun/bin:$PATH"

fail() { printf 'tooling-doctor: [FAIL] %s\n' "$*" >&2; exit 1; }
pass() { printf 'tooling-doctor: [PASS] %s\n' "$*"; }
skip() { printf 'tooling-doctor: [SKIP] %s\n' "$*"; }
info() { printf 'tooling-doctor: [INFO] %s\n' "$*"; }

while [[ $# -gt 0 ]]; do
  case "$1" in
    --check-project-graph) CHECK_PROJECT_GRAPH=1; shift ;;
    --project-root) PROJECT_ROOT="$2"; shift 2 ;;
    *) die "unknown argument: $1" ;;
  esac
done

[[ -f "$PLUGIN_DIR/.cursor-plugin/plugin.json" ]] || fail "toolkit plugin missing at $PLUGIN_DIR"
pass "toolkit plugin at $PLUGIN_DIR"

command -v bun >/dev/null 2>&1 || fail "bun not on PATH"
[[ "$(bun --version)" == "$BUN_VERSION" ]] || fail "bun version mismatch"
pass "bun $BUN_VERSION"

command -v graphify >/dev/null 2>&1 || fail "graphify not on PATH"
[[ "$(graphify --version 2>/dev/null | awk '{print $NF}')" == "$GRAPHIFY_VERSION" ]] || fail "graphify version mismatch"
pass "graphify $GRAPHIFY_VERSION"

[[ -d "$GSTACK_DIR/.git" ]] || fail "gstack missing"
[[ "$(git -C "$GSTACK_DIR" rev-parse HEAD)" == "$GSTACK_REF" ]] || fail "gstack ref mismatch"
pass "gstack at $GSTACK_REF"

for skill in gstack-review gstack-qa gstack-investigate; do
  [[ -f "$HOME/.cursor/skills/${skill}/SKILL.md" ]] || fail "missing gstack skill: $skill"
done
pass "gstack skills"

[[ -d "$SUPERPOWERS_DIR/.git" ]] || fail "superpowers missing"
[[ "$(git -C "$SUPERPOWERS_DIR" rev-parse HEAD)" == "$SUPERPOWERS_REF" ]] || fail "superpowers ref mismatch"
pass "Superpowers at $SUPERPOWERS_REF"

command -v gbrain >/dev/null 2>&1 || fail "gbrain not on PATH"
[[ -d "$GBRAIN_DIR/.git" ]] || fail "gbrain repo missing"
[[ "$(git -C "$GBRAIN_DIR" rev-parse HEAD)" == "$GBRAIN_REF" ]] || fail "gbrain ref mismatch"
pass "GBrain at $GBRAIN_REF"

if [[ -n "${GBRAIN_DATABASE_URL:-}" ]]; then
  info "GBRAIN_DATABASE_URL: configured"
  doctor_json="$(mktemp)"
  gbrain doctor --json >"$doctor_json" 2>/dev/null || true
  if [[ -s "$doctor_json" ]] && command -v jq >/dev/null 2>&1; then
    conn="$(jq -r '.checks[] | select(.name=="connection") | .status' "$doctor_json" 2>/dev/null | head -1)"
    [[ "$conn" == "ok" ]] && pass "GBrain connection ok" || fail "GBrain connection failed"
  else
    skip "GBrain doctor parse skipped"
  fi
  rm -f "$doctor_json"
else
  info "GBRAIN_DATABASE_URL: missing"
  skip "GBrain runtime (no secret)"
fi

if [[ "$CHECK_PROJECT_GRAPH" -eq 1 ]]; then
  if [[ -f "$PROJECT_ROOT/graphify-out/graph.json" ]]; then
    pass "project graph present"
  else
    info "project graph missing (run graphify-ensure-project.sh)"
  fi
fi

info "TOOLKIT_VERSION=$TOOLKIT_VERSION"
pass "all checks passed"
