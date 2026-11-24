# agents-check (AG1.1)

CLI-compatible skill-module validator. Default engine is the **native Rust**
port of `docs/baseline-check.py` (extracted from `scripts/check.sh`).
`--engine baseline` still runs the embedded Python body for comparison.

`scripts/check.sh` remains the production baseline until independent parity
review and cutover.

```bash
cargo build -p agents-check
./target/debug/agents-check --self-test
./scripts/ag1-compare.sh
./scripts/ag1-native-proof.sh
```
