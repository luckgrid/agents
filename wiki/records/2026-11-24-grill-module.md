---
id: AS-REC-2026-11-24-GRILL
kind: session
title: M5 grill skill module (source candidate)
summary: "Implement grill with modes grill, with-docs and choices per wiki/program.md M5; CREDITS from skill-references; source only—not pinned or installed. Base main cf86b6a… after #24."
status: active
created: 2026-11-24
updated: 2026-11-24
related: [AS-STATE, AS-PROGRAM, AS-RES-SKILL-REFERENCES, AS-REC-2026-11-24-INTEGRATION-DELIVERY]
---

# M5 grill module (2026-11-24)

## Intent

Deliver the M5 `grill` skill as specified in [program.md](../program.md#m5-grill): replace the planned `grill-me` / `grilling` / `grill-with-docs` shape with one public module whose modes run design-tree questioning, wiki-backed grilling, and harness choices—without a new runtime or third-party tooling.

## What was checked

| Source | Result |
| --- | --- |
| [M5 section](../program.md#m5-grill) | Modes grill / with-docs / choices; learns-from list |
| [Skill references](../resources/skill-references.md#grill-m5) | mattpocock grilling family + obra brainstorming at pinned commits |
| `wiki` skill on `main` @ `cf86b6a…` | Orient / decide / record used by with-docs |
| `scripts/check.sh` on this tip | See evidence under the delivery run folder |

## Decisions

- Settled [questioning-contract](../../skills/grill/references/questioning-contract.md) per AS-ADR-0003 D3; reference audit recorded.
- One skill folder `skills/grill/` with flat references `grill.md`, `with-docs.md`, `choices.md`.
- Own words only; credits in `CREDITS.md`.
- No CLI, pin, install or exchange admission in this delivery.
- STATE and program note M5 as **draft source on the feature branch** until merge; pin/install remain separate.

## Evidence

- Skill: `skills/grill/`
- Branch: `feat/grill-module`

## What's next

1. Source checks, independent review, remediation if needed.
2. One draft PR for this delivery (integrated publication strategy).
3. Owner acceptance, then pin/install remain Workstation-owned follow-ups.
