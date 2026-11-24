#!/usr/bin/env bash
# Compare scripts/check.sh and crates/agents-check --engine native on the same
# invocations. Ordinary validation stays offline/read-only. Exit 0 only when
# both exit statuses and stdout match for each case.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

bin="${AGENTS_CHECK_BIN:-}"
if [[ -z "$bin" ]]; then
  if [[ -x "$root/target/debug/agents-check" ]]; then
    bin="$root/target/debug/agents-check"
  else
    echo "FAIL: build agents-check first (cargo build -p agents-check) or set AGENTS_CHECK_BIN" >&2
    exit 1
  fi
fi

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

compare() {
  local name="$1"
  shift
  local a_out="$tmp/${name}.sh.out" b_out="$tmp/${name}.rs.out"
  local a_ec=0 b_ec=0
  set +e
  "$root/scripts/check.sh" "$@" >"$a_out" 2>&1
  a_ec=$?
  AGENTS_CHECK_ROOT="$root" "$bin" --engine native "$@" >"$b_out" 2>&1
  b_ec=$?
  set -e
  if [[ "$a_ec" -ne "$b_ec" ]]; then
    echo "FAIL $name: exit $a_ec vs $b_ec"
    return 1
  fi
  if ! cmp -s "$a_out" "$b_out"; then
    echo "FAIL $name: stdout differs"
    diff -u "$a_out" "$b_out" | head -80 || true
    return 1
  fi
  echo "pass $name (exit $a_ec)"
}

compare self-test --self-test
compare skills-default
compare skills-workspace --namespaces workspace skills

echo "ok: ag1-compare matched check.sh and agents-check --engine native"
