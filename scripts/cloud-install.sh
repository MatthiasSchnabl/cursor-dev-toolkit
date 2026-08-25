#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOLKIT_ROOT="${CURSOR_DEV_TOOLKIT_ROOT:-$SCRIPT_DIR/..}"
PROJECT_ROOT="${PROJECT_ROOT:-$(pwd)}"

export CURSOR_DEV_TOOLKIT_ROOT="$TOOLKIT_ROOT"
chmod +x "$TOOLKIT_ROOT/scripts/"*.sh "$TOOLKIT_ROOT/scripts/lib/"*.sh 2>/dev/null || true

"$TOOLKIT_ROOT/scripts/bootstrap.sh" --with-project-graph --project-root "$PROJECT_ROOT"
"$TOOLKIT_ROOT/scripts/verify.sh" --check-project-graph --project-root "$PROJECT_ROOT" || true
