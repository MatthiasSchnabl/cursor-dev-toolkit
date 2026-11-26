#!/usr/bin/env bash
# Explicit offline install of an already approved, locally supplied release binary.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
source "$ROOT/scripts/lib/common.sh"
load_versions "$ROOT"

usage() { printf '%s\n' 'Usage: docs-guard-install.sh doctor | install <local-release-binary> | verify'; }
case "${1:-}" in
  doctor|verify) [[ $# == 1 ]] || die "unexpected arguments" ;;
  install) [[ $# == 2 ]] || die "supply exactly one local release binary" ;;
  --help|-h) usage; exit 0 ;;
  *) usage >&2; exit 2 ;;
esac

[[ "${DOCS_GUARD_RELEASE_STATE:-}" == approved ]] || die "Documentation Guard release is not approved; installation/verification disabled"
version="${DOCS_GUARD_VERSION:?missing version pin}"
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "invalid version pin"
case "$(uname -s):$(uname -m)" in
  Linux:x86_64) target=x86_64-unknown-linux-gnu; name=docs-guard ;;
  Linux:aarch64|Linux:arm64) target=aarch64-unknown-linux-gnu; name=docs-guard ;;
  MINGW*:x86_64|MSYS*:x86_64) target=x86_64-pc-windows-msvc; name=docs-guard.exe ;;
  *) die "unsupported OS/architecture; no approved binary" ;;
esac

lock="$ROOT/scripts/docs-guard-release.lock.tsv"
[[ -f "$lock" ]] || die "missing release lock"
# A single approved row must bind version/target, source revision and artifact SHA-256.
row="$(awk -F '\t' -v v="$version" -v t="$target" '$1==v && $2==t {if(NF!=4) exit 2; print; n++} END {if(n!=1) exit 1}' "$lock")" || die "missing or ambiguous release pin for $version/$target"
IFS=$'\t' read -r pinned_version pinned_target source_sha sha extra <<<"$row"
[[ "$pinned_version" == "$version" && "$pinned_target" == "$target" && "$source_sha" =~ ^[0-9a-f]{40}$ && "$sha" =~ ^[0-9a-f]{64}$ && -z "$extra" ]] || die "invalid release lock entry"

digest() {
  local result
  if command -v sha256sum >/dev/null 2>&1; then result="$(sha256sum < "$1")"
  elif command -v shasum >/dev/null 2>&1; then result="$(shasum -a 256 < "$1")"
  else die "SHA-256 tool missing"; fi
  printf '%s\n' "${result%% *}"
}
dest="$HOME/.local/share/cursor-dev-toolkit/docs-guard/$version/$target/$name"
# Reject symlinked destination ancestors inside the managed home-relative path.
# This is not a security boundary against a concurrent malicious local writer.
check_dest() {
  local parent="$HOME"
  local part
  for part in .local share cursor-dev-toolkit docs-guard "$version" "$target"; do
    parent="$parent/$part"
    [[ ! -L "$parent" ]] || die "symlinked installation directory: $parent"
    [[ ! -e "$parent" || -d "$parent" ]] || die "not a directory: $parent"
  done
  [[ ! -L "$dest" ]] || die "symlinked installed binary"
  [[ ! -e "$dest" || -f "$dest" ]] || die "installed binary target is not a regular file"
}
verify_binary() {
  [[ -f "$1" && ! -L "$1" ]] || die "missing regular binary: $1"
  [[ "$(digest "$1")" == "$sha" ]] || die "release artifact checksum mismatch: $1"
}
check_dest
case "$1" in
  doctor)
    printf 'version=%s target=%s source=%s path=%s\n' "$version" "$target" "$source_sha" "$dest"
    if [[ -f "$dest" ]]; then verify_binary "$dest"; [[ -x "$dest" ]] || die "installed binary is not executable"; printf 'status=verified\n'; else printf 'status=not_installed\n'; fi ;;
  verify)
    verify_binary "$dest"
    [[ -x "$dest" ]] || die "installed binary is not executable"
    printf '%s\n' "$dest" ;;
  install)
    asset="$2"
    verify_binary "$asset"
    if [[ -e "$dest" ]]; then
      verify_binary "$dest"
      [[ -x "$dest" ]] || die "installed binary is not executable"
      printf '%s\n' "$dest"
      exit 0
    fi
    mkdir -p "$(dirname "$dest")"
    temp="$(mktemp "$(dirname "$dest")/.docs-guard.XXXXXXXX")"
    trap 'rm -f -- "$temp"' EXIT
    cp -- "$asset" "$temp"
    verify_binary "$temp"
    chmod 755 "$temp"
    [[ -x "$temp" ]] || die "installation filesystem does not permit execution"
    # No destructive replacement: an already existing version is immutable.
    # The lock pin also gets rechecked on every verify.
    if ! ln -T -- "$temp" "$dest"; then
      die "could not publish immutable binary (existing file or filesystem limitation)"
    fi
    verify_binary "$dest"
    [[ -x "$dest" ]] || die "installed binary is not executable"
    printf '%s\n' "$dest"
    ;;
esac
