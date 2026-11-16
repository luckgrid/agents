---
id: AS-REC-2026-11-16-WORKSPACE-SESSION
kind: session
title: Workspace skills bootstrap session (build and survey, 2026-11-16)
summary: "Seventh round: workspace (M10) first, before M2 council, with expanded scope for herdr-backed workspace management. Bootstrap survey of herdr 0.9.5; owner answers on the model, five skills, herdr-direct backend, and first pin after these skills. Skills built on feat/workspace-skills from develop."
status: active
created: 2026-11-16
updated: 2026-11-16  # independent review remediated remove-ID wording, adopt path collapse, status survey gaps; rebased onto #13 directory record
related: [AS-REC-2026-11-16-WORKSPACE-SKILLS, AS-REC-2026-11-16-WORKSPACE-SKILLS-KICKOFF, AS-PROGRAM, AS-STATE, AS-REC-2026-11-14-MODULAR-REWRITE, AS-REC-2026-11-15-M0, AS-RES-SKILL-REFERENCES, AS-ARCH-WORKSPACE-COORDINATION]
---

# Workspace skills bootstrap session (build and survey, 2026-11-16)

## Intent

This is the bootstrap build session that followed the [kickoff](kickoff.md). The owner's intent for the seventh round, the repository's purpose and the herdr interface ideas for Workstation are in the [record](README.md#the-owners-intent-2026-11-16). They aren't repeated here. Under the records rule of 2026-11-16, this page captures intent. The answers below are the options the owner chose, and the survey replies are evidence.

## Seventh round (owner, 2026-11-16)

`workspace` (M10) comes first, before M2 `council`. Its scope grows from issue, pr and release modes to workspace management. The first `agent_skills_commit` pin follows these skills once they merge (not after M2).

## Decisions: questions and the owner's answers

Each answer is the option the owner chose, as offered.

Write steps (Plan mode): "Stay in Plan mode: ask the model questions now, and write the plan with the lifecycle exercise as its first build step"

Workspace: "(Recommended) Yes. Workspaces that aren't clones (like `~` or `Workspaces`) are allowed but outside the model: the skills report them and never touch them"

Base branch: "(Recommended) Yes. The base branch is develop, else main, and new worktrees branch from it (`--base develop` or `--base main`). The skills never create develop"

Nesting (first): "(Recommended) By branch name: a sub-worktree's branch is `<parent-branch>/<name>` (for example feat/x/y), branched from the parent's branch. Depth is read from the name, so nothing extra is written. I'll confirm in the build's first step that herdr accepts it and what source_workspace_id says"

Nesting separator (after Git `feat/x` + `feat/x/y` correction): "(Recommended) `<parent-branch>--<name>`, for example feat/x--y. Its folder is feat-x--y, a double dash never appears in a top-level branch, and depth is the number of `--`, at most one"

Maintenance: "(Recommended) Yes as written. 'Behind' is against the local origin/<base> ref, with the date of the last fetch stated. Empty folders under the worktree root (like herdr-probe) count as orphaned and are reported. The public skill says 'refresh remotes first' and names no command; only workbench-herdr names `wb sync`"

Status: "(Recommended) A nested text tree for you, plus a fenced JSON block of the same data (workspace, base, worktrees with children, agents with kind, state and pane). The JSON is the contract a future sidebar plugin would read. No orchestrator field until herdr or Workbench records one"

Skills set: "(Recommended) Keep all five. Status is read-only and safe to run anytime. Maintain runs status's survey as its first step and adds the findings and fixes"

Backend: "(Recommended) herdr directly in workspace-* now, with a HERDR_ENV=1 check and a herdr 0.9.5 version floor. Later, M3 multiplex pulls the backend parts out, and workspace-* then names multiplex, keeping a fallback line"

Pin: "(Recommended) The first pin follows the workspace skills once they merge. Record it in the seventh round and STATE, and hand the pin to a Workstation session"

Build branch: "(Recommended) Dogfood it: create feat/workspace-skills as a herdr worktree from develop, and work and commit there. If committing from the worktree is refused, stop and report, as the guest rules say"

Who builds: "(Recommended) You start a fresh Cursor session in the new worktree's pane, with a hand-off brief I write to ~/.workers/records/. It edits inside its own checkout (no rule bent) and tests committing from a worktree. This session does the survey, the record and the plan"

Socket: "(Recommended) Public skills: run the command; if refused, stop, print it for the owner and take IDs from the pasted reply. Record the finding as evidence, put it first in workstation-herdr-followups.md as a boundary gap for Workstation (Cursor's sandbox doesn't refuse the socket), and keep asking you to approve every mutating herdr command this session"

After `linked_worktree_source` during survey: owner chose **1** — create sub-worktrees from the clone workspace with `--base <parent-branch>` and branch `<parent>--<name>`. Nesting is a skill convention; herdr stays flat.

## Survey evidence

Full verbatim replies with `~/` paths: `~/.workers/records/bootstrap-review-2026-11-16.md`.

Summary for the skills:

- herdr 0.9.5; `HERDR_ENV=1`; socket answered inside Cursor's sandbox (boundary gap).
- Sidebar flat; linked worktrees are top-level workspaces; `source_workspace_id` is always the clone.
- Create/open from a linked worktree ID fails with `linked_worktree_source`.
- Branch `scratch/herdr-survey--sub` works; folder/label become `scratch-herdr-survey-sub` (`--` collapsed).
- Dirty remove: `dirty_worktree_requires_force`; do not use `--force`.
- `worktree open` on an open worktree: `already_open: true`.
- `workspace report-metadata` tokens work (display-only); branch name remains the nesting record.
- No plugins installed; no orchestrator fields; no colour/parent in schema.

## The plan (agreed)

1. Survey done (review file + scratch lifecycle cleaned up).
2. Draft + build brief in `~/.workers/records/`.
3. `herdr worktree create --workspace w8 --branch feat/workspace-skills --base develop --no-focus`.
4. Owner starts a new Cursor session in that worktree.
5. New session writes five skills (one commit each) and wiki (one commit), runs checks, writes hand-off notes.
6. Owner runs `wb promote agents feat/workspace-skills` and `wb pr agents feat/workspace-skills`.

## Skills built

| Skill | Does |
| --- | --- |
| `workspace` | Model, names, Workbench may only narrow; mode table for planned issue/pr/release |
| `workspace-open` | Find or create herdr workspace for a clone; check base branch; report |
| `workspace-worktree` | Create/open/adopt via herdr from the clone workspace ID; remove with the linked worktree's own ID; `--` nesting; cleanup |
| `workspace-maintain` | Report-first sweep, then each approved fix |
| `workspace-status` | Nested survey + JSON contract |

Built on `feat/workspace-skills` from `develop` at `0d53e41` (M0 merged as #11). Lifecycle exercise in the build session reconfirmed clone-only create/open, `--` nesting, folder collapse, `already_open`, and clean remove. Committing from the herdr worktree succeeded with the guest identity.

## Independent review and remediation (2026-11-16)

Findings fixed on this branch:

1. `workspace-worktree` / `workspace` hard rules: create and open use the clone ID; **remove** uses the linked worktree's own workspace ID.
2. `workspace-worktree` adopt: folder derivation collapses `--` to `-` after `/` → `-` (for example `feat/x--y` → `feat-x-y`).
3. `workspace-status` survey: collect `non_clone_workspaces` from `workspace list`; when the clone has no herdr workspace, report with `workspace_id: null` and do not call `worktree list` without an ID.
4. Root README and M2 program wording updated for the seventh-round order (M10 before M2).

## Remediation of PR #14 (2026-11-16)

After promotion, a cloud session reviewed #14 and fixed the following on the branch:

- The public skills named Workbench's worktree root (`~/.workers/spaces`) and the guest sandbox. They now read the root from `herdr worktree list`, and `workbench-herdr` narrows them.
- The base model made the herdr label the identity. It now matches by checkout metadata, as `workspace-open` and the [coordination proposal](../../architecture/workspace-coordination.md) do. Created workspaces are still labelled `<ns>`.
- This page quoted the owner word for word, including the VM's host and user names, in a public repository. It now points to the record's intent, as the records rule asks.
- `workspace-worktree`'s credits table split on `|` characters.
- The private Workstation skills were credited as Apache-2.0.
- The status JSON had no field for agents in the clone's own workspace. It now has a top-level `agents` field.
- The README and STATE wording assumed the work was still on the branch.
