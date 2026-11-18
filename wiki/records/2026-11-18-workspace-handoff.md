---
id: AS-REC-2026-11-18-WORKSPACE-HANDOFF
kind: session
title: Workspace handoff skill and M10/M1 continuity refinement
summary: "Built workspace-handoff (kickoff, handoff, review-request, resume) on feat/workspace-handoff; companion Workbench guidance and workstation project skill tracked in Workstation. Scoped M10/M1 refinement; does not reorder council, multiplex or the wider refactor."
status: active
created: 2026-11-18
updated: 2026-11-18
related: [AS-STATE, AS-PROGRAM, AS-ARCH-WORKSPACE-COORDINATION, AS-REC-2026-11-16-WORKSPACE-SKILLS]
---

# Workspace handoff (2026-11-18)

## Intent

Complete a bounded connection between public workspace skills, Workbench's guest workflow and clear handoffs to the owner on `dev`: handoff/resume guidance, M1 entry/lifecycle documentation companions in Workstation, templates including a dev action packet, and dogfood of changed-candidate-SHA detection. Owner retains installation, admission, promotion, publication and merge.

## What was checked

| Source | Revision read |
| --- | --- |
| agents `main` | `0ed7919b3bf0b4f818d1ea604f2e04326f9bde76` |
| agents local `develop` before FF | `8a106a4ecf94970bd0f6c59bc283286af196d0b5` (one maintain commit behind `main`) |
| workstation `main` | `75c002c724dc68787fb8aaa08a02b31abd7c4247` |
| workstation `develop` | `3958ed18d3ab58c6e12be2b337a1c58998e63bcb` |
| Installed `/usr/local/bin/wb-commit` | sha256 `7facfee5…` matches FW2 on workstation `main` (FW2.1 source not installed) |
| Installed skills | `workbench-guest-rules`, `workbench-herdr` only; no `agent_skills_commit` |
| herdr | 0.9.5; `HERDR_ENV=1` |
| FW2 admission | Claude Code only (owner gates); no Cursor `wb-commit` rule |
| PR #59 / issue #58 | Open in local records; not verified online from the guest |

Distinguished: **source** (repository), **installed** (guest payload / helper), **admitted** (owner gates). Feature worktrees created from `main` because `wb-commit branch` refuses when the checkout is on `main`.

## Decisions

- Separate skill `workspace-handoff` (independent invocation by recipients).
- Companion Workstation branch based on `main`/`develop` path avoiding FW2.1's dirty tree; rebase after #59 lands.
- Role skills `workbench-plan` / `orchestrate` / `code` / `write` remain missing; document vs first-pin gate.
- Commits via owner-run `/usr/local/bin/wb-commit` (Cursor has no admitted helper rule).

## Evidence

- herdr workspaces (owner-pasted): agents clone `w8`; workstation clone `wD` (label stale `fw2-1-followups`, checkout is main clone).
- Worktrees created: `wK` → `~/.workers/spaces/agents/feat-workspace-handoff` on `feat/workspace-handoff`; `wM` → `~/.workers/spaces/workstation/feat-workbench-m1-handoff` on `feat/workbench-m1-handoff`.
- `scripts/check.sh --namespaces workspace` → `ok: 6 skills checked`; `--self-test` → `ok: self-test, 36 cases as expected` (on the feature tip before commit).

## Proposals

- First pin still needs landed M1 required Workbench skills (base + handoff at minimum; role skills recorded missing), then owner sets `agent_skills_commit` and runs cell S1 including explicit base-rule loading.
- Do not widen harness permissions for Cursor commits; keep owner-run helper.

## What's next

- Finish Workstation companion on `feat/workbench-m1-handoff`.
- Dogfood: handoff at SHA A, owner commit to SHA B, resume detects mismatch.
- Owner: promote, check exact SHA, draft PR; pin procedure after both land.
