#!/usr/bin/env bash
# Cheap preflight only — no network installs on session start.
set -euo pipefail

export PATH="$HOME/.local/bin:$HOME/.bun/bin:$PATH"

missing=()
for cmd in bun graphify gbrain; do
  command -v "$cmd" >/dev/null 2>&1 || missing+=("$cmd")
done

if [[ ${#missing[@]} -gt 0 ]]; then
  msg="cursor-dev-toolkit: missing on PATH: ${missing[*]}. Run /tooling-setup or scripts/bootstrap.sh"
  # sessionStart: emit additional_context for Cursor
  escaped="${msg//\\/\\\\}"
  escaped="${escaped//\"/\\\"}"
  escaped="${escaped//$'\n'/\\n}"
  printf '{\n  "additional_context": "%s"\n}\n' "$escaped"
fi

exit 0
