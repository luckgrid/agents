---
id: AS-REC-2026-11-24-AG1-1
kind: session
title: AG1.1 native Rust validator parity
summary: "Native agents-check (--engine native default) ports scripts/check.sh behavior; ag1-compare byte-matches corpus; native-proof shows no python/check.sh exec. Shell checker retained until cutover. Moon admission separate. No AG1.2 expansion."
status: active
created: 2026-11-24
updated: 2026-11-24
related: [AS-ADR-0003, AS-REC-2026-11-24-SKILL-SYSTEM-PLANNING, AS-ARCH-SKILL-CAPABILITY-EVOLUTION, AS-ARCH-AGENTS-WORKSPACE]
---

# AG1.1 native Rust validator parity (2026-11-24)

## Intent

Complete AG1.1 (AS-ADR-0003 D7 / planning P3): a native Rust validator that preserves `scripts/check.sh` invocation semantics, rules, diagnostics and exits on the full regression corpus, without AG1.2 expansion and without cutting over until independent review accepts replacement.

## What landed

| Item | Note |
| --- | --- |
| `crates/agents-check` | Native modules: util, frontmatter, markdown, skill, folder, selftest |
| CLI | `--self-test`, `--namespaces`, optional folder; `--engine native` (**default**) or `baseline` |
| Compare | `scripts/ag1-compare.sh` — `check.sh` vs `--engine native` (exit + stdout) |
| Non-delegation proof | `scripts/ag1-native-proof.sh` — source, strace, PATH without python3 |
| Shell baseline | `scripts/check.sh` **retained** until cutover acceptance |

## Checks (tip `bade3c846be40f20c17221237e2593efd71387f8`)

- `cargo build -p agents-check` PASS
- `./scripts/ag1-compare.sh` PASS (self-test, skills-default, skills-workspace)
- `./scripts/ag1-native-proof.sh` PASS
- `cargo test -p agents-check` PASS
- Independent review: **PASS WITH NOTES** (clap unknown-option wording; docs lag remediated here)

## Explicitly not done

- Cutover removing `scripts/check.sh`
- Moon / `.prototools` for luckgrid/agents (see admission handoff)
- AG1.2 schema/resource/dependency checks
- Pin or guest install

## Distinctions

Draft source on `feat/ag1-1-validator-parity` ≠ pin ≠ install ≠ runtime admission.
