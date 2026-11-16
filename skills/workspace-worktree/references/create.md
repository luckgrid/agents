# create

Create a herdr-linked worktree and open it as its own workspace.

## Inputs

- Clone workspace ID (from `workspace-open`)
- Branch name
- Base ref: repo base (`develop` or `main`) for a top-level feature branch; for a sub-worktree, the **parent branch** name
- Optional: explicit `--path` / `--label` only when the owner asks

## Sub-worktrees

Branch name is `<parent-branch>--<name>` with at most one `--`. Example: parent `feat/x`, child `feat/x--y`. Create from the **clone** workspace with `--base <parent-branch>`. Depth is read from the branch name, not the folder (herdr collapses `--` to `-` in paths and labels).

## Command

```bash
herdr worktree create --workspace <clone ID> --branch <branch> --base <base> --no-focus
```

Use `--focus` only when the owner asks. Take the new worktree workspace ID and path from the JSON. Confirm `worktree.repo_root` is the clone.
