---
name: workspace
description: >
  Shared model for Luckgrid repository workspaces: clone plus herdr workspace,
  base branch, nested feature worktrees, and planned issue/pr/release writing.
  Use when opening or surveying a repo workspace, creating feature worktrees,
  or drafting issue, PR, or release text. Names workspace-open, workspace-worktree,
  workspace-status, and workspace-maintain for management workflows. Requires
  herdr 0.9.5+ with HERDR_ENV=1 for herdr-backed steps.
license: Apache-2.0
compatibility: herdr >= 0.9.5; HERDR_ENV=1 for herdr-backed steps.
---

# Workspace

Public half of working on a Luckgrid repository through herdr. Workbench may only **narrow** these rules (for example by naming `wb sync`); it must not relax them. Load this skill's rules explicitly from every `workspace-*` skill; if this base skill is unavailable, stop.

## Preconditions

```bash
test "${HERDR_ENV:-}" = 1
herdr --version   # require 0.9.5 or newer
```

If either check fails, say so and stop herdr-backed steps. Never install or upgrade herdr. The installed binary's `herdr --skill` and `herdr <group> help` are the authority for syntax; third-party facts only after help confirms them.

Read every workspace and worktree ID from JSON replies. Never guess an ID. If a herdr socket command is refused, stop, print the exact command for the owner, and continue only from the pasted reply.

## Model

| Term | Meaning |
| --- | --- |
| Workspace | A Git clone at `~/Workspaces/<ns>` **and** the herdr workspace whose checkout is that clone (`worktree.repo_root` or `checkout_path`, not a linked worktree). Match by that metadata, never by label alone; a workspace this skill creates is labelled `<ns>` |
| Non-clone workspace | Allowed in herdr (for example `~` or `Workspaces`) but **outside** this model: report it, never operate on it |
| Worktree root | The folder herdr puts a clone's linked worktrees in: the parent of the paths `herdr worktree list` reports. Read it, never assume it; Workbench names its own |
| Base branch | `develop` if that local branch exists, otherwise `main`. New worktrees use `--base` that ref. **Never create `develop`** |
| Feature worktree | A herdr-linked worktree for a feature branch, opened as its own herdr workspace |
| Sub-worktree | Branch `<parent-branch>--<name>` (at most one `--`), created from the **clone** workspace with `--base <parent-branch>`. Depth is the count of `--` in the **branch name** (0 or 1). Folders and labels turn `/` into `-` and collapse `--` to `-`; do not read depth from the folder |

herdr's sidebar stays flat. Nesting is a skill convention. `worktree list` always reports `source.source_workspace_id` as the clone. Create and open worktrees **only** with that clone workspace ID (`linked_worktree_source` otherwise).

## Management skills

| Skill | Job |
| --- | --- |
| `workspace-open` | Find or create the herdr workspace for a clone; check the base branch; report |
| `workspace-worktree` | Create, open, adopt, or remove worktrees via the clone workspace ID |
| `workspace-status` | Nested text survey plus fenced JSON (read-only) |
| `workspace-maintain` | Run status first, then each approved fix |

## Hard rules

- Never `herdr worktree create` or `open` with a linked worktree's workspace ID; `remove` uses that linked workspace's own ID.
- Never `--force` on `worktree remove`, never `--trust-repository`, never `git branch -D` without the owner's word.
- Never plain `git worktree add` for feature work; use herdr so its events fire.
- Never file, push, tag, or publish from the `issue`, `pr`, or `release` modes; in the guest, `workbench` says who does those steps. If `workbench` is not installed, stop before any guest-only action.
- Public skills say "refresh remotes first" and name no sync command or guest path; Workbench's `workbench-herdr` narrows them for the guest.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `issue` | [issue](references/issue.md) | Draft issue text from findings |
| `pr` | [pr](references/pr.md) | Draft a PR description |
| `release` | [release](references/release.md) | Draft release notes and pre-tag checks |
