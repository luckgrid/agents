---
id: AS-REC-2026-11-24-CONDUCT-SESSION-COUNCIL-SPECS
kind: session
title: Conduct/session migration and Council phase-two specifications
summary: "Proposed architecture pages for P4/P5a conduct+session migration and P7 Council phase two. Grill remains independent; no backend install; Workstation adapters via Controller."
status: active
created: 2026-11-24
updated: 2026-11-24
related: [AS-ADR-0003, AS-ARCH-CONDUCT-SESSION-MIGRATION, AS-ARCH-COUNCIL-PHASE-TWO, AS-ARCH-SKILL-CAPABILITY-EVOLUTION, AS-REC-2026-11-24-SKILL-SYSTEM-PLANNING]
---

# Conduct/session migration and Council phase-two specifications

## Intent

On the docs tip, record proposed specifications that follow AS-ADR-0003 without implementing cutover, installing backends or expanding the M5 grill delivery.

## What landed

| Page | Package | Note |
| --- | --- | --- |
| [conduct-session-migration.md](../architecture/conduct-session-migration.md) | P4 / P5a | Conduct responsibilities, session boundaries, identity model, Kitty/Herdr Core references, Herdr compatibility, missing-capability, migration/rollback |
| [council-phase-two.md](../architecture/council-phase-two.md) | P7 | Deliberation, grill composition, evidence/dissent, independence, bounded decisions — **implementation after grill acceptance** |

## Explicitly not done

- Renaming shipped `orchestration` in the registry
- New `conduct` / `session-*` skill folders
- Council skill expansion on any feature branch
- Kitty or Herdr Core installation
- Workstation adapter implementation (Controller note only)

## Distinctions

Proposed documentation on draft #23 lineage ≠ qualified AG1.1 tip ≠ grill tip ≠ pin ≠ install.
