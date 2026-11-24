---
id: AS-REC-2026-11-24-AG1-1
kind: session
title: AG1.1 Rust validator parity — crate layout and baseline lock
summary: "Cargo workspace crates/agents-check with CLI matching scripts/check.sh; baseline engine embeds extracted check.py for identical corpus results; compare harness green. Native Rust engine and Moon admission remain follow-ons. Old check.sh retained."
status: active
created: 2026-11-24
updated: 2026-11-24
related: [AS-ADR-0003, AS-REC-2026-11-24-SKILL-SYSTEM-PLANNING, AS-ARCH-SKILL-CAPABILITY-EVOLUTION, AS-ARCH-AGENTS-WORKSPACE]
---

# AG1.1 validator parity (2026-11-24)

## Intent

Begin AG1.1 (AS-ADR-0003 D7 / planning P3): preserve `scripts/check.sh` behavior while introducing a Cargo-backed `agents-check` binary and an offline compare harness. Do not expand into AG1.2 schema/resource checks. Do not cut over or remove the shell validator.

## What landed this tip

| Item | Note |
| --- | --- |
| `Cargo.toml` workspace | member `crates/agents-check` |
| `agents-check` CLI | `--self-test`, `--namespaces`, optional folder; `--engine baseline` |
| Baseline engine | `include_str!` of extracted `scripts/check.sh` Python body; run via `python3 -I` |
| `scripts/ag1-compare.sh` | Compares exit + stdout for self-test, default skills, and a namespaces case |
| Evidence | `ag1-cargo-build.txt`, `ag1-compare.txt` under the capability run folder |

## What was checked

- `cargo build -p agents-check` → success
- `./scripts/ag1-compare.sh` → `ok: ag1-compare matched check.sh and agents-check`
- Ordinary validation remains offline/read-only (no network in checker)

## Explicitly not done

- Native Rust reimplementation of the checker (follow-on before cutover review)
- Moon project admission for luckgrid/agents (tooling available via proto; version pin/admission still open)
- Removing or changing `scripts/check.sh`
- AG1.2 expansion

## Distinctions

Merged/draft source ≠ pin ≠ install ≠ runtime admission. This tip is draft source on `feat/ag1-1-validator-parity` only.
