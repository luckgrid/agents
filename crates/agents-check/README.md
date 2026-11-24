# agents-check (AG1.1)

CLI-compatible skill-module validator. This tip ships the **baseline** engine
(extracted `scripts/check.sh` Python body) behind a Rust binary so Cargo layout,
compare harness and future native port share one entrypoint.

`scripts/check.sh` remains the production baseline until independent parity
review and cutover. Native Rust checking is not complete here.

```bash
cargo build -p agents-check
./target/debug/agents-check --self-test
./scripts/ag1-compare.sh
```
