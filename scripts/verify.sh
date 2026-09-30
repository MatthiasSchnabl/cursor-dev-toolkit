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

RUST_ENG_SKILL="$PLUGIN_DIR/skills/engineering/rust-engineering"
[[ -f "$RUST_ENG_SKILL/SKILL.md" ]] || fail "missing rust-engineering skill"
[[ -f "$RUST_ENG_SKILL/references/rust-engineering-standard.md" ]] || fail "missing rust-engineering standard reference"
[[ -f "$PLUGIN_DIR/commands/rust-engineering-audit.md" ]] || fail "missing rust-engineering-audit command"
[[ -f "$PLUGIN_DIR/commands/rust-engineering-fix.md" ]] || fail "missing rust-engineering-fix command"
[[ ! -f "$PLUGIN_DIR/commands/rust-engineering.md" ]] || fail "obsolete combined rust-engineering command still present"
pass "rust-engineering skill and commands"

AXUM_STD="$PLUGIN_DIR/skills/engineering/_standards/rust-axum-engineering-standard.md"
AXUM_AUDIT_SKILL="$PLUGIN_DIR/skills/engineering/rust-axum-engineering-audit"
AXUM_FIX_SKILL="$PLUGIN_DIR/skills/engineering/rust-axum-engineering-fix"
[[ -f "$AXUM_STD" ]] || fail "missing rust-axum engineering standard"
[[ -f "$AXUM_AUDIT_SKILL/SKILL.md" ]] || fail "missing rust-axum-engineering-audit skill"
[[ -f "$AXUM_FIX_SKILL/SKILL.md" ]] || fail "missing rust-axum-engineering-fix skill"
[[ -f "$PLUGIN_DIR/commands/rust-axum-engineering-audit.md" ]] || fail "missing rust-axum-engineering-audit command"
[[ -f "$PLUGIN_DIR/commands/rust-axum-engineering-fix.md" ]] || fail "missing rust-axum-engineering-fix command"
pass "rust-axum-engineering skills and commands"

TEST_STD="$PLUGIN_DIR/skills/engineering/_standards/testing-quality-engineering-standard.md"
TEST_AUDIT_SKILL="$PLUGIN_DIR/skills/engineering/testing-quality-engineering-audit"
TEST_FIX_SKILL="$PLUGIN_DIR/skills/engineering/testing-quality-engineering-fix"
[[ -f "$TEST_STD" ]] || fail "missing testing-quality engineering standard"
[[ -f "$TEST_AUDIT_SKILL/SKILL.md" ]] || fail "missing testing-quality-engineering-audit skill"
[[ -f "$TEST_FIX_SKILL/SKILL.md" ]] || fail "missing testing-quality-engineering-fix skill"
[[ -f "$PLUGIN_DIR/commands/testing-quality-engineering-audit.md" ]] || fail "missing testing-quality-engineering-audit command"
[[ -f "$PLUGIN_DIR/commands/testing-quality-engineering-fix.md" ]] || fail "missing testing-quality-engineering-fix command"
pass "testing-quality-engineering skills and commands"

SEC_STD="$PLUGIN_DIR/skills/engineering/_standards/security-supply-chain-engineering-standard.md"
SEC_AUDIT_SKILL="$PLUGIN_DIR/skills/engineering/security-supply-chain-engineering-audit"
SEC_FIX_SKILL="$PLUGIN_DIR/skills/engineering/security-supply-chain-engineering-fix"
[[ -f "$SEC_STD" ]] || fail "missing security-supply-chain engineering standard"
[[ -f "$SEC_AUDIT_SKILL/SKILL.md" ]] || fail "missing security-supply-chain-engineering-audit skill"
[[ -f "$SEC_FIX_SKILL/SKILL.md" ]] || fail "missing security-supply-chain-engineering-fix skill"
[[ -f "$PLUGIN_DIR/commands/security-supply-chain-engineering-audit.md" ]] || fail "missing security-supply-chain-engineering-audit command"
[[ -f "$PLUGIN_DIR/commands/security-supply-chain-engineering-fix.md" ]] || fail "missing security-supply-chain-engineering-fix command"
pass "security-supply-chain-engineering skills and commands"

[[ -f "$PLUGIN_DIR/rules/rust-rig-agentic-engineering-standard.mdc" ]] || fail "missing rust-rig-agentic engineering standard rule"
pass "rust-rig-agentic engineering standard rule"

RIG_STD="$PLUGIN_DIR/skills/engineering/_standards/rust-rig-agentic-engineering-standard.md"
RIG_AUDIT_SKILL="$PLUGIN_DIR/skills/engineering/rust-rig-agentic-engineering-audit"
RIG_FIX_SKILL="$PLUGIN_DIR/skills/engineering/rust-rig-agentic-engineering-fix"
[[ -f "$RIG_STD" ]] || fail "missing rust-rig-agentic engineering standard"
[[ -f "$RIG_AUDIT_SKILL/SKILL.md" ]] || fail "missing rust-rig-agentic-engineering-audit skill"
[[ -f "$RIG_FIX_SKILL/SKILL.md" ]] || fail "missing rust-rig-agentic-engineering-fix skill"
[[ -f "$PLUGIN_DIR/commands/rust-rig-agentic-engineering-audit.md" ]] || fail "missing rust-rig-agentic-engineering-audit command"
[[ -f "$PLUGIN_DIR/commands/rust-rig-agentic-engineering-fix.md" ]] || fail "missing rust-rig-agentic-engineering-fix command"
pass "rust-rig-agentic-engineering skills and commands"

SQLX_STD="$PLUGIN_DIR/skills/engineering/_standards/rust-sqlx-postgresql-engineering-standard.md"
SQLX_AUDIT_SKILL="$PLUGIN_DIR/skills/engineering/rust-sqlx-engineering-audit"
SQLX_FIX_SKILL="$PLUGIN_DIR/skills/engineering/rust-sqlx-engineering-fix"
[[ -f "$SQLX_STD" ]] || fail "missing rust-sqlx-postgresql engineering standard"
[[ -f "$SQLX_AUDIT_SKILL/SKILL.md" ]] || fail "missing rust-sqlx-engineering-audit skill"
[[ -f "$SQLX_FIX_SKILL/SKILL.md" ]] || fail "missing rust-sqlx-engineering-fix skill"
[[ -f "$PLUGIN_DIR/commands/rust-sqlx-engineering-audit.md" ]] || fail "missing rust-sqlx-engineering-audit command"
[[ -f "$PLUGIN_DIR/commands/rust-sqlx-engineering-fix.md" ]] || fail "missing rust-sqlx-engineering-fix command"
pass "rust-sqlx-engineering skills and commands"

[[ -f "$PLUGIN_DIR/rules/observability-logging-tracing-engineering-standard.mdc" ]] || fail "missing observability-logging-tracing engineering standard rule"
pass "observability-logging-tracing engineering standard rule"

OBS_STD="$PLUGIN_DIR/skills/engineering/_standards/observability-logging-tracing-engineering-standard.md"
OBS_AUDIT_SKILL="$PLUGIN_DIR/skills/engineering/observability-logging-tracing-engineering-audit"
OBS_FIX_SKILL="$PLUGIN_DIR/skills/engineering/observability-logging-tracing-engineering-fix"
[[ -f "$OBS_STD" ]] || fail "missing observability-logging-tracing engineering standard"
[[ -f "$OBS_AUDIT_SKILL/SKILL.md" ]] || fail "missing observability-logging-tracing-engineering-audit skill"
[[ -f "$OBS_FIX_SKILL/SKILL.md" ]] || fail "missing observability-logging-tracing-engineering-fix skill"
[[ -f "$PLUGIN_DIR/commands/observability-logging-tracing-engineering-audit.md" ]] || fail "missing observability-logging-tracing-engineering-audit command"
[[ -f "$PLUGIN_DIR/commands/observability-logging-tracing-engineering-fix.md" ]] || fail "missing observability-logging-tracing-engineering-fix command"
pass "observability-logging-tracing-engineering skills and commands"

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
