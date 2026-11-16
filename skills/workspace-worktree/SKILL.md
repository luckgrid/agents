---
name: workspace-worktree
description: >
  Create, open, adopt, or remove herdr-linked Git worktrees from the clone
  workspace ID only, using -- nesting for one level of sub-worktrees. Use when
  starting a feature branch worktree, reopening one, adopting a plain Git
  worktree, or cleaning up after promote. Loads workspace base rules. Requires
  herdr 0.9.5+ with HERDR_ENV=1.
license: Apache-2.0
compatibility: herdr >= 0.9.5; HERDR_ENV=1.
---

# workspace-worktree

Load the `workspace` skill's model and hard rules first. If `workspace` is not installed, stop. Resolve the clone workspace ID with `workspace-open` (or the same steps) before mutating.

## Preconditions

```bash
test "${HERDR_ENV:-}" = 1
herdr --version   # require 0.9.5 or newer
```

Every `worktree create` and `worktree open` uses `--workspace <clone workspace ID>`. Never pass a linked worktree's workspace ID (`linked_worktree_source`). Never `--force`, never `--trust-repository`, never plain `git worktree add`.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `create` | [create](references/create.md) | New linked worktree from the clone workspace |
| `open` | [open](references/open.md) | Reopen an existing worktree by branch or path |
| `adopt` | [adopt](references/adopt.md) | Bring a plain Git worktree under herdr (owner runs move) |
| `remove` | [remove](references/remove.md) | Clean remove of a linked worktree, then delete the branch |
