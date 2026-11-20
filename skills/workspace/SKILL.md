---
name: workspace
description: >
  Shared coordination model and initial Luckgrid clone-to-Herdr binding:
  base branch, nested feature worktrees, and issue/pr/release drafting.
  Use when opening or surveying a repo workspace, creating feature worktrees,
  or drafting issue, PR, or release text. Names workspace-open, workspace-worktree,
  workspace-status, workspace-maintain, and workspace-handoff for management
  workflows. Requires herdr 0.9.5+ with HERDR_ENV=1 for herdr-backed steps.
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

If either check fails, say so and stop herdr-backed steps. Never install or upgrade herdr. The recorded behaviour is for 0.9.5. For a newer version, check installed help and response shapes before using these instructions; the version floor alone promises no compatibility. Never run bare `herdr` (it attaches the TUI). Use `herdr --skill` and `herdr <group> --help` for syntax.

Before any socket or Git write, load the environment/project rules that govern this session. In Workbench, load `workbench-guest-rules` and `workbench-herdr`; if required rules are unavailable, stop. Those rules decide who may run socket commands (in Workbench, an operator session runs them and workers hand them to it); reachability alone is not permission. If installed policy and these instructions disagree, follow the more restrictive one and report the discrepancy; do not infer that a documented future step is enabled.

Read IDs from complete successful JSON replies, checking `result.type` and the requested checkout association. Reject an `error` envelope, missing metadata, unknown shapes or multiple matches. Never guess an ID or select by label. After a refusal, stop and report the exact command for whoever the environment's rules authorize to run it; use a fresh validated reply before continuing.

## Model

A logical workspace may span repositories, worktrees and services; a Work Unit is the intent, not its branch or pane. These skills implement the initial Luckgrid one-clone binding below, not a universal restriction. A **workbranch** is a topic ref plus its purpose, explicit base ref/tip and intended integration target. Git owns refs and checkout state; Herdr owns attachments. Keep machine paths local and session IDs ephemeral.

| Term | Meaning |
| --- | --- |
| Initial clone binding | A Git clone at `~/Workspaces/<ns>` **and** the herdr workspace whose checkout is that clone (exact `worktree.repo_root` and `worktree.checkout_path`, with `is_linked_worktree: false`). Match by that metadata, never by label alone; a workspace this skill creates is labelled `<ns>` |
| Non-clone workspace | Allowed in herdr (for example `~` or `Workspaces`) but **outside** this model: report it, never operate on it |
| Worktree root | The configured destination for this clone, supplied by the owning environment or explicitly confirmed by the owner. An existing checkout's parent is not proof of the configured root: explicit paths may differ and the list may be empty. If unknown, report unknown and stop root-dependent adoption/cleanup |
| Base branch | `develop` if that local branch exists, otherwise `main`. New worktrees use `--base` that ref. **Never create `develop`** |
| Feature worktree | A herdr-linked worktree for a feature branch, opened as its own herdr workspace |
| Sub-worktree | Branch `<parent-branch>--<name>` (at most one `--`), created from the **clone** workspace with `--base <parent-branch>`. Depth is the count of `--` in the **branch name** (0 or 1). Names group the display only; they do not prove Git ancestry. Herdr's default path slug lowercases ASCII letters, replaces non-alphanumeric runs with `-` and trims edge dashes; it is lossy. Read actual paths, never infer identity or depth from a slug |

herdr's sidebar stays flat. Nesting is a display convention, not Git isolation or permission to remove descendants. Retain explicit base/dependency tips when creating a child; existing names alone cannot prove its base. `worktree list` always reports `source.source_workspace_id` as the clone. Create and open worktrees **only** with that clone workspace ID (`linked_worktree_source` otherwise).

## Management skills

| Skill | Job |
| --- | --- |
| `workspace-open` | Find or create the herdr workspace for a clone; check the base branch; report |
| `workspace-worktree` | Create, open, adopt, or remove worktrees via the clone workspace ID |
| `workspace-status` | Nested text survey plus fenced JSON (read-only) |
| `workspace-maintain` | Run status first, then each approved fix |
| `workspace-handoff` | Kickoff, progress/blocker handoff, review request, and resume notes |

## Hard rules

- Give each editing session its own checkout and serialize shared target-ref writes through one designated integrator. Branch labels and agent states are advisory, not locks or authorization.
- Before mutation, refresh Git registration, exact refs/tips, checkout path and Herdr association. Refuse occupied branches, conflicting destinations and ambiguous matches. Preflight reads alone are not atomic.
- On interrupted or partial creation/move/open, inspect and report surviving state before any authorized retry; never reset, delete, stash or recreate unrelated work to force success.
- Removal requires no active users/processes, preservation or explicit abandonment of the exact tip, and checks for tracked, untracked and ignored data. Unavailable activity data is not idle; branch deletion needs separate authorization. Preserve durable briefs, findings, receipts and recovery evidence outside the checkout before removal; ordinary cleanup is not permission to prune retained history (see `workspace-maintain` `retention`).

- Never `herdr worktree create` or `open` with a linked worktree's workspace ID; `remove` uses that linked workspace's own ID.
- Never `--force` on `worktree remove`, never `--trust-repository`, never `git branch -D` without the owner's word or an environment rule that allows it (in Workbench, an operator's `test/<topic>` discard).
- Never plain `git worktree add` for feature work; use herdr so its events fire.
- Never file, push, tag, or publish from the `issue`, `pr`, or `release` modes; in the guest, `workbench` says who does those steps. If `workbench` is not installed, stop before any guest-only action.
- Status is read-only: use local refs and state their limits; never fetch or sync as part of a survey. If fresher data is needed, request the authorized environment workflow separately, then survey again.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `issue` | [issue](references/issue.md) | Draft issue text from findings |
| `pr` | [pr](references/pr.md) | Draft a PR description |
| `release` | [release](references/release.md) | Draft release notes and pre-tag checks |
