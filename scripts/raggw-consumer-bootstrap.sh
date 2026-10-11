#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../versions.env
source "$ROOT/versions.env"

REPOSITORY="https://github.com/MatthiasSchnabl/RAGGW.git"
TAG="$RAGGW_CONSUMER_BOOTSTRAP_TAG"
SHA="$RAGGW_CONSUMER_BOOTSTRAP_SHA"
TREE_SHA="$RAGGW_CONSUMER_BOOTSTRAP_TREE_SHA"
LOCK="$ROOT/scripts/raggw-consumer-bootstrap.lock"
CACHE_ROOT="${RAGGW_BOOTSTRAP_CACHE_DIR:-${HOME}/.cursor/raggw-consumer-bootstrap}"
DEST="$CACHE_ROOT/$SHA"
SKILL="$DEST/raggw-consumer-bootstrap"

fail() { printf 'raggw-consumer-bootstrap: ERROR: %s\n' "$*" >&2; exit 1; }

[[ "$SHA" =~ ^[0-9a-f]{40}$ && "$TREE_SHA" =~ ^[0-9a-f]{40}$ ]] || fail "invalid toolkit pin"
[[ "$TAG" =~ ^[a-zA-Z0-9._-]+$ ]] || fail "invalid toolkit tag"

verify_skill() {
  local directory="$1" hash path actual count=0
  [[ -f "$directory/SKILL.md" && -f "$LOCK" ]] || return 1
  while read -r hash path; do
    [[ -n "$hash" ]] || continue
    [[ "$hash" =~ ^[0-9a-f]{40}$ && "$path" =~ ^(SKILL\.md|references/[a-z0-9-]+\.md)$ ]] || return 1
    [[ -f "$directory/$path" && ! -L "$directory/$path" ]] || return 1
    actual="$(git hash-object -- "$directory/$path")" || return 1
    [[ "$actual" == "$hash" ]] || return 1
    count=$((count + 1))
  done < "$LOCK"
  [[ "$count" -eq 9 ]] || return 1
  [[ "$(find "$directory" -type f | wc -l | tr -d ' ')" == "$count" ]] || return 1
}

case "${1:-install}" in
  --pin)
    printf 'tag=%s\ncommit=%s\nskill_tree=%s\n' "$TAG" "$SHA" "$TREE_SHA"
    exit 0
    ;;
  --check)
    verify_skill "$SKILL" || fail "private pinned skill not installed or modified; run the installer"
    printf 'RAGGW_CONSUMER_SKILL=%s\n' "$SKILL/SKILL.md"
    exit 0
    ;;
  install) ;;
  *) fail "usage: $0 [install|--pin|--check]" ;;
esac

command -v git >/dev/null 2>&1 || fail "git is required"
if [[ -e "$DEST" ]]; then
  verify_skill "$SKILL" || fail "existing pinned cache is incomplete or modified: $DEST; inspect it before removing"
  printf 'RAGGW_CONSUMER_SKILL=%s\n' "$SKILL/SKILL.md"
  exit 0
fi

umask 077
mkdir -p "$CACHE_ROOT"
TEMP="$(mktemp -d "$CACHE_ROOT/.fetch.XXXXXXXX")" || fail "cannot allocate secure temporary directory"
trap 'rm -rf -- "$TEMP"' EXIT

# The RAGGW repository is private. Git must already have access (GCM/gh auth setup-git).
# Avoid interactive prompts in agents. Never put a token in the clone URL or log credentials.
GIT_TERMINAL_PROMPT=0 git clone --quiet --depth 1 --branch "$TAG" "$REPOSITORY" "$TEMP/repo" \
  || fail "cannot fetch private pinned RAGGW tag; configure GitHub Git authentication first"

ACTUAL_SHA="$(git -C "$TEMP/repo" rev-parse HEAD)"
[[ "$ACTUAL_SHA" == "$SHA" ]] || fail "tag commit differs from the fixed toolkit pin"
TAG_SHA="$(git -C "$TEMP/repo" rev-parse "refs/tags/$TAG^{commit}")"
[[ "$TAG_SHA" == "$SHA" ]] || fail "tag does not resolve to the fixed commit"
ACTUAL_TREE="$(git -C "$TEMP/repo" rev-parse "HEAD:skills/raggw-consumer-bootstrap")"
[[ "$ACTUAL_TREE" == "$TREE_SHA" ]] || fail "private skill tree differs from the fixed toolkit pin"

mkdir -p "$TEMP/payload"
cp -R "$TEMP/repo/skills/raggw-consumer-bootstrap" "$TEMP/payload/raggw-consumer-bootstrap"
verify_skill "$TEMP/payload/raggw-consumer-bootstrap" || fail "private skill file digests differ from the lock"

# The installed cache contains only the pinned skill, not the entire private repository.
[[ ! -e "$DEST" ]] || fail "cache changed concurrently; retry after checking it"
mv "$TEMP/payload" "$DEST"
verify_skill "$SKILL" || fail "installed skill failed verification"
printf 'RAGGW_CONSUMER_SKILL=%s\n' "$SKILL/SKILL.md"
