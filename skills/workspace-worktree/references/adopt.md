# adopt

Adopt an existing **plain** Git worktree so herdr manages it. The agent does not run `git worktree move` (it writes the clone's `.git` and `~/.workers/spaces`, which the guest sandbox refuses). Print both commands for the owner, in order:

```bash
git -C ~/Workspaces/<ns> worktree move <old path> ~/.workers/spaces/<ns>/<branch with / made ->
herdr worktree open --workspace <clone ID> --path ~/.workers/spaces/<ns>/<branch with / made -> --no-focus
```

Never add `--trust-repository`. Take the worktree's workspace ID from the open reply. Do not invent paths; derive the folder name the same way herdr does (`/` → `-`).
