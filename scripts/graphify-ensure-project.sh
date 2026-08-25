#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="${PROJECT_ROOT:-$(pwd)}"
graph_file="$PROJECT_ROOT/graphify-out/graph.json"

if [[ -f "$graph_file" ]]; then
  exit 0
fi

command -v graphify >/dev/null 2>&1 || exit 0

cd "$PROJECT_ROOT"
graphify extract . --code-only --no-cluster
