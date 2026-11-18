---
id: AS-REC-2026-11-18-WORKSPACE-HANDOFF
kind: session
title: Workspace handoff skill and M10/M1 continuity refinement
summary: "Built workspace-handoff (kickoff, handoff, review-request, resume) on feat/workspace-handoff; companion Workbench guidance and workstation project skill tracked in Workstation. Scoped M10/M1 refinement; does not reorder council, multiplex or the wider refactor."
status: active
created: 2026-11-18
updated: 2026-11-18  # review round: environment-policy wording in workspace and workspace-handoff; IDs removed; checks re-run
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
| Workstation #59 / issue #58 | Read from local records only, not online from the guest (#59 later merged as `8b40d3d`) |

Distinguished: **source** (repository), **installed** (guest payload / helper), **admitted** (owner gates). Feature worktrees created from `main` because `wb-commit branch` refuses when the checkout is on `main`.

## Decisions

- Separate skill `workspace-handoff` (independent invocation by recipients).
- Companion Workstation branch based on `main`/`develop` path avoiding FW2.1's dirty tree; rebase after #59 lands.
- Role skills `workbench-plan` / `orchestrate` / `code` / `write` remain missing; document vs first-pin gate.
- Commits via owner-run `/usr/local/bin/wb-commit` (Cursor has no admitted helper rule).

## Evidence

- herdr workspaces (owner-pasted): each clone's workspace matched by checkout metadata; the workstation clone's label was stale (`fw2-1-followups`) while its checkout was the main clone. Live IDs are left out of this public record.
- Worktrees created through herdr: `~/.workers/spaces/agents/feat-workspace-handoff` on `feat/workspace-handoff` and `~/.workers/spaces/workstation/feat-workbench-m1-handoff` on `feat/workbench-m1-handoff`, each opened as its own worktree workspace.
- `scripts/check.sh --namespaces workspace` → `ok: 6 skills checked`; `--self-test` → `ok: self-test, 36 cases as expected`. These ran on the uncommitted working tree before `c99400f`, so they cover no SHA; the review round below re-ran them at the remediated head.

## Proposals

- First pin still needs landed M1 required Workbench skills (base + handoff at minimum; role skills recorded missing), then owner sets `agent_skills_commit` and runs cell S1 including explicit base-rule loading.
- Do not widen harness permissions for Cursor commits; keep owner-run helper.

## What's next

- Finish Workstation companion on `feat/workbench-m1-handoff`.
- Dogfood: handoff at SHA A, owner commit to SHA B, resume detects mismatch.
- Owner: promote, check exact SHA, draft PR; pin procedure after both land.

## Review round (2026-11-18)

A Claude review on #16 at `c99400f` found five issues, and a ChatGPT cloud recheck found a sixth (R16-6). Claude remediated them in a cloud session, committing as the owner's GitHub noreply address. That session had no guest access.

| ID | Finding | Disposition |
| --- | --- | --- |
| R16-1 | `references/resume.md` step 5 stated an owner-run socket policy itself, unlike its siblings | Fixed: it applies the base skill's environment-policy preflight (in Workbench, `workbench-herdr`). Its cleanup line defers to the environment's rules too |
| R16-2 | Live herdr IDs in this public record | Fixed: replaced by descriptions |
| R16-3 | The record's checks covered no SHA | Fixed: re-run at the remediated head; the head and results are in #16's review-round section |
| R16-4 | Lines that go stale at merge (#59 open; STATE's "not on main until merged") | Fixed: worded to stay true |
| R16-5 | The hard rule named `workbench-handoff` directly | Fixed: "the environment's publication skill (in Workbench, `workbench-handoff`)" |
| R16-6 | The base `workspace` skill also hard-coded "ordinary owner-run socket policy" and owner execution after a refusal, so fixing `resume.md` alone would leave a contradiction | Fixed: the environment's rules decide who runs socket commands (in Workbench, the operator; workers hand commands to it), and the stricter rule wins on disagreement. `git branch -D` is allowed where an environment rule allows it. `workspace-worktree`'s adopt row no longer says the owner always runs the move |

These follow Workstation's agent-operated bootstrap policy (Workstation #61, `wiki/architecture/guest-bootstrap-workflow.md`). Workbench's own skills implement it in Workstation #60.

**Deferred:** the changed-SHA dogfood (a handoff at one SHA, then a resume that detects the next). It goes to the first guest operator session after merge, because the cloud session can't write guest notes.
