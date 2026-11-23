---
id: AS-REC-2026-11-20-BULK-TRANSFER
kind: session
title: Bulk script and file-transfer handoff method
summary: "Scoped follow-up: portable bulk-transfer guidance in workspace-handoff; exchange readiness is not claimed admitted. Does not amend the frozen closeout tip c5bab23…."
status: active
created: 2026-11-20
updated: 2026-11-23
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
| GW-ADR-0019 | **Proposed** exchange layout/names only — not claimed as current Workbench implementation |

Distinguished: **package ready to stage** vs **exchange gate admitted** (not proved here). **Current Workbench path** remains set-based `~/Resources/exchange/out/<SET>/`; GW-ADR-0019 is proposed documentation only.

## Decisions

- Add `workspace-handoff` mode `bulk-transfer` (portable method + Workbench binding with fallback).
- Retention explicitly keeps durable manifests, receipts, checks, decisions and package identity; redundant exchange staging may retire only under an authorized environment exchange-cleanup procedure (no pruning commands invented here).
- Require member-level archive inspection and isolated safe extraction under the receiving environment’s procedure before guest-authored script content is considered for host execution; Agents does not implement an extractor.
- Package identity (id, inventory, digests, provenance) binds the artifact to preparation evidence; a changed package needs fresh review.
- When file exchange is unavailable, stop that path and point to admitted host publication handoff (`workbench-handoff` `dev`, including `wb draft`) without selecting `wb ship` or implying merge.
- Do **not** amend freeze tip `c5bab2310f528858dccac2c81814b2b0099ab4f8`; land this as a queued child branch.
- Do not recommend transfer bypasses; coordinate gate confirmation with Workstation.
- **Index integration (BT-05):** `wiki/records/README.md` also gains entries from concurrent PR #22. When integrating sequentially, **preserve both** this bulk-transfer index line and #22’s appended entries. Do not force-push or rebase merely to avoid that overlap.

## Evidence

- Skill: `skills/workspace-handoff/references/bulk-transfer.md`
- Export: `~/…/records/agents-workspace/bulk-transfer-result.md`
- Independent review remediation (BT-01…BT-05): PR #18 on `feat/bulk-transfer-handoff`

## What's next

- Owner may stage with controller `stage-checkpoint-transfer.sh` after gate confirmation.
- Promote/land this docs branch after the frozen milestone PR, or in a later small docs visit — not a reason to reopen the freeze SHA.
- Independent exact-head recheck after remediation tip.
