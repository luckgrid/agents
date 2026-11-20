---
id: AS-REC-2026-11-20-BULK-TRANSFER
kind: session
title: Bulk script and file-transfer handoff method
summary: "Scoped follow-up: portable bulk-transfer guidance in workspace-handoff; exchange readiness is not claimed admitted. Does not amend the frozen closeout tip c5bab23…."
status: active
created: 2026-11-20
updated: 2026-11-20
related: [AS-STATE, AS-REC-2026-11-20-TAB-ROLE-AUDIT, AS-ARCH-WORKSPACE-COORDINATION]
---

# Bulk transfer handoff (2026-11-20)

## Intent

Record how orchestrators prepare inspectable host publication scripts for the owner via the environment’s file exchange, without inventing transfer commands or changing the frozen agents milestone candidate.

## What was checked

| Source | Result |
| --- | --- |
| Controller handoff `~/…/records/controller/handoffs/2026-11-20-bulk-script-transfer-workflow.md` | Assignment; packing/staging rules |
| Package `~/…/records/controller/checkpoint-transfer/` | `publish-checkpoint.tar`, `SHA256SUMS`, `README.md` present; hashes match SUMS |
| `wb outbox` | Lists existing sets; outbound directory usable |
| Workbench `daily-use.md` (Workstation main wiki) | `wb get` documented; **“Not yet for real work”** until GW-ADR-0010 exchange gate |
| GW-ADR-0010 | Still **proposed** in decisions index |

Distinguished: **package ready to stage** vs **exchange gate admitted** (not proved here).

## Decisions

- Add `workspace-handoff` mode `bulk-transfer` (portable method + Workbench binding with fallback).
- Retention explicitly keeps exchange packages/receipts.
- Do **not** amend freeze tip `c5bab2310f528858dccac2c81814b2b0099ab4f8`; land this as a queued child branch.
- Do not recommend transfer bypasses; coordinate gate confirmation with Workstation.

## Evidence

- Skill: `skills/workspace-handoff/references/bulk-transfer.md`
- Export: `~/…/records/agents-workspace/bulk-transfer-result.md`

## What's next

- Owner may stage with controller `stage-checkpoint-transfer.sh` after gate confirmation.
- Promote/land this docs branch after the frozen milestone PR, or in a later small docs visit — not a reason to reopen the freeze SHA.
