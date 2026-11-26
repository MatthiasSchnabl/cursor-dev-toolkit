#!/usr/bin/env bash
# Installer contract tests; synthetic bytes only. Never changes actual release pins or HOME.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf -- "$tmp"' EXIT
mkdir -p "$tmp/toolkit/scripts/lib" "$tmp/home"
cp "$ROOT/scripts/docs-guard-install.sh" "$tmp/toolkit/scripts/"
cp "$ROOT/scripts/lib/common.sh" "$tmp/toolkit/scripts/lib/"
cp "$ROOT/versions.env" "$tmp/toolkit/"
cp "$ROOT/scripts/docs-guard-release.lock.tsv" "$tmp/toolkit/scripts/"
export HOME="$tmp/home"
script="$tmp/toolkit/scripts/docs-guard-install.sh"
lock="$tmp/toolkit/scripts/docs-guard-release.lock.tsv"
expect_failure() {
  if bash "$script" "$@" >"$tmp/out" 2>"$tmp/err"; then
    printf 'unexpected success: %s\n' "$*" >&2; exit 1
  fi
}
# Explicit pending state, even if a future checkout is approved.
printf '\nDOCS_GUARD_RELEASE_STATE="pending"\n' >>"$tmp/toolkit/versions.env"
expect_failure doctor
expect_failure install "$tmp/missing"
[[ ! -e "$HOME/.local" ]]
case "$(uname -s):$(uname -m)" in
  Linux:x86_64) target=x86_64-unknown-linux-gnu ;;
  Linux:aarch64|Linux:arm64) target=aarch64-unknown-linux-gnu ;;
  MINGW*:x86_64|MSYS*:x86_64) target=x86_64-pc-windows-msvc ;;
  *) printf 'unsupported target for installer test\n' >&2; exit 2 ;;
esac
printf '\nDOCS_GUARD_VERSION="9.8.7"\nDOCS_GUARD_RELEASE_STATE="approved"\n' >>"$tmp/toolkit/versions.env"
: >"$lock"
expect_failure verify
printf 'synthetic fixture; never executed\n' >"$tmp/asset"
printf 'wrong bytes\n' >"$tmp/wrong"
if command -v sha256sum >/dev/null; then result="$(sha256sum < "$tmp/asset")"
else result="$(shasum -a 256 < "$tmp/asset")"; fi
sha="${result%% *}"
printf '9.8.7\t%s\t0123456789abcdef0123456789abcdef01234567\t%s\n' "$target" "$sha" >"$lock"
expect_failure install "$tmp/wrong"
[[ ! -e "$HOME/.local" ]]
bash "$script" doctor | grep -q 'status=not_installed'
dest="$(bash "$script" install "$tmp/asset")"
[[ -f "$dest" ]]
cmp "$tmp/asset" "$dest"
[[ "$(bash "$script" verify)" == "$dest" ]]
[[ "$(bash "$script" install "$tmp/asset")" == "$dest" ]]
printf 'tampered\n' >"$dest"
expect_failure verify
expect_failure install "$tmp/asset"
grep -q 'tampered' "$dest"
printf 'DOCS_GUARD_INSTALLER_SELFTEST_PASS\n'
