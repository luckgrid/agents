# survey

Collect a read-only picture of one namespace `<ns>` (default: the repo of the current checkout).

## Collect

1. Run `herdr workspace list` once. From that reply:
   - find the clone workspace for `~/Workspaces/<ns>` (`workspace-open` find steps, **without creating** unless the owner asked to open);
   - collect every workspace that is **not** a clone under `~/Workspaces/` into `non_clone_workspaces` (id and label only). Report them; never operate on them.
2. If there is **no** clone workspace ID yet: still report the base branch from the clone on disk, list Git worktrees with `git worktree list`, list empty dirs under `~/.workers/spaces/<ns>/`, fill `workspace_id` as `null`, leave `worktrees` empty or Git-only notes, and stop the herdr worktree/agent steps for this namespace. Do not invent an ID and do not call `herdr worktree list` without a real clone ID.
3. With a clone ID: `herdr worktree list --workspace <clone ID>`.
4. `herdr agent list` (and pane list if needed to place agents). Attach agents to worktrees by `workspace_id` / pane cwd.
5. For the clone, note the base branch and whether local `origin/<base>` exists. Record the date/time of the last fetch when known (for example from `FETCH_HEAD` mtime); if unknown, say so.
6. Compare each worktree branch tip to local `origin/<base>` when that ref exists: report ahead/behind counts. "Behind" means behind that local origin ref, not a live network fetch.
7. List empty directories under `~/.workers/spaces/<ns>/` as orphaned path findings (for example leftover probe folders). Do not delete them here.

## Nesting

Build parent/child edges from **branch names**: a branch with one `--` is a child of the prefix before `--`, when that parent branch appears in the worktree list. Depth is 0 or 1. Do not use folder names (they collapse `--`).

## Text tree

Print a nested outline for the owner, for example:

```text
agents (w8) base=develop
├── develop @ ~/Workspaces/agents
├── feat/workspace-skills (wK) @ ~/.workers/spaces/agents/feat-workspace-skills
│   agent: cursor working pane=wK:p1
└── skills/herdr-workflows-1 (wG) @ ...
orphans: (none)
behind: (none)  last fetch: <iso or unknown>
```

Include agent **kind**, **state**, and **pane** when present. No orchestrator field. When `workspace_id` is null, say that the clone has no herdr workspace yet and list `non_clone_workspaces` from step 1.

## JSON contract

After the tree, print one fenced `json` block with the same data. Shape:

```json
{
  "namespace": "agents",
  "clone_path": "~/Workspaces/agents",
  "workspace_id": "w8",
  "base_branch": "develop",
  "last_fetch": null,
  "worktrees": [
    {
      "branch": "feat/x",
      "workspace_id": "wK",
      "path": "~/.workers/spaces/agents/feat-x",
      "is_linked_worktree": true,
      "ahead_of_base": 0,
      "behind_base": 0,
      "children": [
        {
          "branch": "feat/x--y",
          "workspace_id": "wN",
          "path": "~/.workers/spaces/agents/feat-x-y",
          "is_linked_worktree": true,
          "ahead_of_base": 0,
          "behind_base": 0,
          "children": [],
          "agents": []
        }
      ],
      "agents": [
        {"kind": "cursor", "state": "working", "pane": "wK:p1"}
      ]
    }
  ],
  "orphans": [],
  "non_clone_workspaces": []
}
```

Paths use `~/…`. Omit any orchestrator key. Children arrays are always present (empty when none). This block is the contract a future sidebar plugin would read.
