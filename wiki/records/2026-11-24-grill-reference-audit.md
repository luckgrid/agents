---
id: AS-REC-2026-11-24-GRILL-REF-AUDIT
kind: session
title: M5 grill scoped reference audit
summary: "Pinned upstream ideas for grill at skill-references commits; questioning contract settled per AS-ADR-0003 D3; no third-party install; Council composition deferred."
status: active
created: 2026-11-24
updated: 2026-11-24
related: [AS-REC-2026-11-24-GRILL, AS-ADR-0003, AS-RES-SKILL-REFERENCES, AS-PROGRAM]
---

# M5 grill reference audit (2026-11-24)

## Intent

Complete the scoped reference audit before treating grill as delivery-ready: confirm credited sources, settle the questioning contract, and keep Council expansion out of this delivery.

## Sources checked (ideas only)

| Source | Commit / version | Use in grill |
| --- | --- | --- |
| mattpocock/skills `grilling` | `4ae2317d` | Frontier rounds; recommended yes-answer |
| mattpocock `grill-me` / `grill-with-docs` | `4ae2317d` | Entry composition; docs/domain pairing |
| mattpocock `domain-modeling` | `4ae2317d` | Glossary challenge; scenario wording; record decisions |
| obra/superpowers `brainstorming` | `22f0b463` | Settle design before build |
| Luckgrid `wiki` skill | `main` @ `cf86b6a…` | with-docs orient/decide |

No upstream text copied. No third-party tooling installed.

## Contract

See `skills/grill/references/questioning-contract.md`: answerable frontier, recommended alternatives, evidence checks, conversational fallback, delegated decision limits. Matches AS-ADR-0003 D3 (independent grill; Council composes later).

## Out of scope this delivery

- Council deliberation/decide expansion
- Session capability family
- Pin/install of grill
