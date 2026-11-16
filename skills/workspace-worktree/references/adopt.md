# adopt

Adopt an existing **plain** Git worktree so herdr manages it, by moving it into the clone's worktree root (see the `workspace` model) and opening it. `git worktree move` writes the clone's `.git`; where a sandbox refuses that, print both commands for the owner instead of running them. In order:

```bash
git -C ~/Workspaces/<ns> worktree move <old path> <worktree root>/<folder>
herdr worktree open --workspace <clone ID> --path <worktree root>/<folder> --no-focus
```

Never add `--trust-repository`. Take the worktree's workspace ID from the open reply. Derive `<folder>` the same way herdr does: turn each `/` into `-`, then collapse every `--` run to a single `-` (so `feat/x--y` → `feat-x-y`). Do not invent paths.
