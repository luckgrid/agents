# open

Open means: ensure a herdr workspace exists for one clone and report it. Do not create feature worktrees here (`workspace-worktree` does that).

## Steps

1. Confirm `<ns>` and that `~/Workspaces/<ns>` is a Git checkout (has `.git`). If it is not a clone, stop: this skill only opens clone workspaces.
2. Run `herdr workspace list`. Prefer a workspace whose `worktree.repo_root` or `worktree.checkout_path` is `~/Workspaces/<ns>` and `worktree.is_linked_worktree` is false. Take its `workspace_id`.
3. If none matches, create once:

   ```bash
   herdr workspace create --cwd ~/Workspaces/<ns> --label <ns> --no-focus
   ```

   Take the new `workspace_id` from the reply. Do not reuse a linked worktree's ID as the clone workspace.
4. Resolve the **base branch** in that clone: local `develop` if it exists, else `main`. Report which. **Never create `develop`.**
5. In the same list reply, note every herdr workspace that is not a clone under `~/Workspaces/` (for example `~` or `Workspaces`). Report them as outside the model. Never create, rename, close, or run worktree commands against them.

## Report

Print at least:

- namespace `<ns>`
- clone path `~/Workspaces/<ns>`
- clone workspace ID
- base branch name
- non-clone workspaces seen (ids and labels only)
