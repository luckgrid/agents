---
name: grill
description: >
  Settle a design by grilling: ask every ready frontier question with a
  recommended answer the owner can accept with "yes", gather facts yourself,
  and stop when the frontier is empty and the owner confirms. Use with-docs
  when a Luckgrid wiki must orient, challenge terms, check claims, and record
  settled decisions; use choices when the harness offers multiple-choice.
license: Apache-2.0
---

# Grill

This module runs a **questioning process**, not a build, install or runtime.
The agent finds facts; the **owner** makes decisions. Do not invent owner
choices from silence, drafts or passing checks. Prefer loading `wiki` when
`with-docs` will write or challenge wiki pages.

## Rules

- Maintain an explicit **design tree** (topics and open questions) and a
  **frontier** (questions whose prerequisites are settled). Ask only the
  frontier in each round.
- Number every question. Give a **recommended answer** worded so that saying
  "yes" accepts that recommendation. Offer alternatives when useful; never
  force a choice the owner did not make.
- Gather evidence yourself (code, docs, records). Do not ask the owner for
  facts you can look up. Ask only for decisions, preferences and confirmations.
- Stop when the frontier is empty **and** the owner confirms the design is
  settled. A quiet owner is not confirmation.
- Never expand this skill into a new session manager, CLI, pin or third-party
  tool install. Ideas only from the credited sources.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `grill` | [grill](references/grill.md) | Rounds over a design tree until the frontier is empty |
| `with-docs` | [with-docs](references/with-docs.md) | Grill against a wiki; write settled decisions through `wiki` |
| `choices` | [choices](references/choices.md) | Prefer harness multiple-choice when available |
| `questioning-contract` | [questioning-contract](references/questioning-contract.md) | Settled independent-primitive contract (AS-ADR-0003 D3) |
