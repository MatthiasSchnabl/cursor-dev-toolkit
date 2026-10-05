#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOLKIT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
# shellcheck source=lib/common.sh
source "$SCRIPT_DIR/lib/common.sh"
load_versions "$TOOLKIT_ROOT"

PLUGIN_DIR="${PLUGIN_DIR:-$(cursor_plugins_local_dir)/cursor-dev-toolkit}"

fail() { printf 'engineering-context: [FAIL] %s\n' "$*" >&2; exit 1; }
pass() { printf 'engineering-context: [PASS] %s\n' "$*"; }
info() { printf 'engineering-context: [INFO] %s\n' "$*"; }

[[ ! -L "$PLUGIN_DIR" ]] || fail "plugin root is a symlink; install a real directory under plugins/local"
[[ -f "$PLUGIN_DIR/.cursor-plugin/plugin.json" ]] || fail "plugin manifest missing at $PLUGIN_DIR"
pass "plugin root: $PLUGIN_DIR"

manifest_version="$(grep -m1 '"version"' "$PLUGIN_DIR/.cursor-plugin/plugin.json" | sed -E 's/.*"version"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')"
[[ "$manifest_version" == "$TOOLKIT_VERSION" ]] || fail "manifest version $manifest_version != versions.env $TOOLKIT_VERSION"
pass "toolkit version $TOOLKIT_VERSION"

required_rules=(
  development-tool-routing.mdc
  rust-engineering-standard.mdc
  rust-axum-engineering-standard.mdc
  rust-sqlx-engineering-standard.mdc
  testing-quality-engineering-standard.mdc
  security-supply-chain-engineering-standard.mdc
  rust-rig-agentic-engineering-standard.mdc
  observability-logging-tracing-engineering-standard.mdc
)

for rule in "${required_rules[@]}"; do
  path="$PLUGIN_DIR/rules/$rule"
  [[ -f "$path" ]] || fail "missing rule $rule"
  lines="$(wc -l <"$path" | tr -d ' ')"
  if [[ "$rule" == "development-tool-routing.mdc" ]]; then
    [[ "$lines" -le 180 ]] || fail "$rule is too large ($lines lines; max 180)"
  else
    [[ "$lines" -le 120 ]] || fail "$rule is too large ($lines lines; max 120)"
  fi
done
pass "engineering rules are compact and present"

required_skills=(
  rust-engineering
  rust-axum-engineering-audit
  rust-axum-engineering-fix
  rust-sqlx-engineering-audit
  rust-sqlx-engineering-fix
  testing-quality-engineering-audit
  testing-quality-engineering-fix
  security-supply-chain-engineering-audit
  security-supply-chain-engineering-fix
  rust-rig-agentic-engineering-audit
  rust-rig-agentic-engineering-fix
  observability-logging-tracing-engineering-audit
  observability-logging-tracing-engineering-fix
  engineering-context-doctor
)

for skill_name in "${required_skills[@]}"; do
  skill="$PLUGIN_DIR/skills/engineering/$skill_name/SKILL.md"
  [[ -f "$skill" ]] || fail "missing engineering skill $skill_name"
  grep -Fq "\"./skills/engineering/$skill_name\"" "$PLUGIN_DIR/.cursor-plugin/plugin.json" ||
    fail "engineering skill not registered in plugin manifest: $skill_name"
done
pass "required engineering skills are registered in plugin manifest"

skill_count=0
while IFS= read -r skill; do
  skill_count=$((skill_count + 1))
  lines="$(wc -l <"$skill" | tr -d ' ')"
  [[ "$lines" -le 500 ]] || fail "SKILL.md exceeds 500 lines: $skill ($lines)"
  grep -q '^name:' "$skill" || fail "skill missing name frontmatter: $skill"
  grep -q '^description:' "$skill" || fail "skill missing description frontmatter: $skill"
  grep -q '^disable-model-invocation:[[:space:]]*true' "$skill" ||
    fail "engineering skill must be explicit/manual invocation: $skill"
done < <(find "$PLUGIN_DIR/skills/engineering" -name SKILL.md -type f | sort)
[[ "$skill_count" -gt 0 ]] || fail "no engineering skills discovered"
pass "engineering SKILL.md files are compact and explicit-only ($skill_count checked)"

while IFS= read -r ref; do
  lines="$(wc -l <"$ref" | tr -d ' ')"
  if [[ "$lines" -gt 100 ]]; then
    head -100 "$ref" | grep -q '^## Contents$' || fail "long reference lacks Contents in first 100 lines: $ref"
  fi
done < <(find "$PLUGIN_DIR/skills/engineering" -type f -name '*.md' ! -name SKILL.md | sort)
pass "long engineering references expose Contents within first 100 lines"

grep -q '"sessionStart"' "$PLUGIN_DIR/hooks/hooks.json" || fail "plugin sessionStart hook is not registered"
grep -Fq './hooks/run-hook.cmd session-start' "$PLUGIN_DIR/hooks/hooks.json" ||
  fail "plugin sessionStart hook command is not the expected cross-platform wrapper"
pass "plugin SessionStart hook registration"

session_json="$(bash "$PLUGIN_DIR/hooks/session-start" </dev/null)"
printf '%s' "$session_json" | grep -q '"additional_context"' || fail "sessionStart does not emit additional_context"
printf '%s' "$session_json" | grep -q 'cursor-dev-toolkit engineering context v2' || fail "SessionStart engineering anchor missing"
pass "SessionStart engineering context anchor"

source_root="$TOOLKIT_ROOT"
if [[ -f "$PLUGIN_DIR/.cursor-toolkit-install" ]]; then
  marker_source="$(sed -n '1p' "$PLUGIN_DIR/.cursor-toolkit-install" | tr -d '\r')"
  if [[ -n "$marker_source" ]]; then source_root="$marker_source"; fi
fi
if command -v cygpath >/dev/null 2>&1 && [[ "$source_root" =~ ^[A-Za-z]:\\ ]]; then
  source_root="$(cygpath -u "$source_root")"
fi

if [[ -d "$source_root/.git" && "$(cd "$source_root" && pwd)" != "$(cd "$PLUGIN_DIR" && pwd)" ]]; then
  mismatches=0
  while IFS= read -r file; do
    [[ -f "$PLUGIN_DIR/$file" ]] || { info "installed copy missing tracked file: $file"; mismatches=$((mismatches+1)); continue; }
    cmp -s "$source_root/$file" "$PLUGIN_DIR/$file" || { info "installed copy differs: $file"; mismatches=$((mismatches+1)); }
  done < <(git -C "$source_root" ls-files)
  [[ "$mismatches" -eq 0 ]] || fail "installed plugin is stale versus source checkout ($mismatches differing files); rerun install"
  pass "installed plugin matches source checkout"
else
  info "source/install byte comparison skipped (plugin is the checkout or source checkout unavailable)"
fi

if git -C "$PLUGIN_DIR" rev-parse HEAD >/dev/null 2>&1; then
  info "installed git revision: $(git -C "$PLUGIN_DIR" rev-parse HEAD)"
elif [[ -f "$PLUGIN_DIR/.cursor-toolkit-install" ]]; then
  info "installed source revision: $(sed -n '2p' "$PLUGIN_DIR/.cursor-toolkit-install" | tr -d '\r')"
fi

pass "engineering context verification complete"
