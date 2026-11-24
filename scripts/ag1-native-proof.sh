#!/usr/bin/env bash
# Prove --engine native does not exec check.sh or python3 for validation.
# Uses strace when available; also greps the native call path in source.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

bin="${AGENTS_CHECK_BIN:-$root/target/debug/agents-check}"
if [[ ! -x "$bin" ]]; then
  echo "FAIL: missing $bin (cargo build -p agents-check)" >&2
  exit 1
fi

# Source-level: native arm must call run_native, not run_baseline.
if ! grep -q 'Engine::Native' "$root/crates/agents-check/src/main.rs"; then
  echo "FAIL: Engine::Native missing from main.rs" >&2
  exit 1
fi
if ! grep -q 'run_native' "$root/crates/agents-check/src/main.rs"; then
  echo "FAIL: run_native not used in main.rs" >&2
  exit 1
fi

# Native library modules must not spawn processes.
hits="$(
  grep -nE 'Command::new|std::process::Command|python3' \
    "$root/crates/agents-check/src/lib.rs" \
    "$root/crates/agents-check/src/util.rs" \
    "$root/crates/agents-check/src/frontmatter.rs" \
    "$root/crates/agents-check/src/markdown.rs" \
    "$root/crates/agents-check/src/skill.rs" \
    "$root/crates/agents-check/src/folder.rs" \
    "$root/crates/agents-check/src/selftest.rs" \
    2>/dev/null || true
)"
if [[ -n "$hits" ]]; then
  echo "FAIL: native modules appear to spawn or shell out:" >&2
  echo "$hits" >&2
  exit 1
fi
echo "pass source: native modules have no process spawn / python3"

# Runtime: strace execve while running native self-test.
trace="$root/target/ag1-native-proof.strace"
set +e
strace -f -o "$trace" -e trace=execve \
  env AGENTS_CHECK_ROOT="$root" "$bin" --engine native --self-test >/dev/null 2>&1
ec=$?
set -e
if [[ "$ec" -ne 0 ]]; then
  echo "FAIL: native --self-test exited $ec under strace" >&2
  exit 1
fi
if grep -E 'execve\("([^"]*/)?python3"' "$trace" >/dev/null; then
  echo "FAIL: strace saw python3 exec under --engine native" >&2
  grep -E 'execve\("([^"]*/)?python3"' "$trace" | head -5 >&2 || true
  exit 1
fi
if grep -E 'execve\("[^"]*check\.sh"' "$trace" >/dev/null; then
  echo "FAIL: strace saw check.sh exec under --engine native" >&2
  exit 1
fi
echo "pass strace: no python3/check.sh execve during --engine native --self-test"

# PATH containing only the binary (no python3): native self-test must still pass.
proof_path="$(mktemp -d)"
trap 'rm -rf "$proof_path"' EXIT
cp "$bin" "$proof_path/agents-check"
set +e
env -i HOME="$HOME" PATH="$proof_path" AGENTS_CHECK_ROOT="$root" \
  "$proof_path/agents-check" --engine native --self-test >/dev/null 2>&1
ec=$?
set -e
if [[ "$ec" -ne 0 ]]; then
  echo "FAIL: native --self-test failed with PATH containing only agents-check (exit $ec)" >&2
  exit 1
fi
echo "pass PATH: native self-test with no python3 on PATH"

echo "ok: ag1-native-proof"
