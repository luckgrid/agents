---
name: workspace-status
description: >
  Read-only survey of a Luckgrid clone workspace: nested text tree of worktrees
  (children inferred from -- in branch names), agents, behind/orphan findings,
  plus a fenced JSON block of the same data. Safe to run anytime. Loads
  workspace base rules. Requires herdr 0.9.5+ with HERDR_ENV=1.
license: Apache-2.0
compatibility: herdr >= 0.9.5; HERDR_ENV=1.
---

# workspace-status

Load the `workspace` skill's model and hard rules first. If `workspace` is not installed, stop. This skill changes nothing.

## Preconditions

```bash
test "${HERDR_ENV:-}" = 1
herdr --version   # require 0.9.5 or newer
```

Refresh remotes first when the owner wants an up-to-date behind check. This public skill names no sync command; in the guest, `workbench-herdr` may name `wb sync`. If that guest skill is unavailable, ask the owner to refresh remotes and continue with local refs.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `survey` | [survey](references/survey.md) | Nested text tree plus fenced JSON contract |
