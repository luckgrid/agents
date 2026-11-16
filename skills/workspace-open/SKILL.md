---
name: workspace-open
description: >
  Find or create the herdr workspace for a Git clone under ~/Workspaces/<ns>,
  resolve the base branch (develop if present else main), and report non-clone
  herdr workspaces without touching them. Use when a task needs the clone's
  workspace ID before worktree actions. Loads workspace base rules. Requires
  herdr 0.9.5+ with HERDR_ENV=1.
license: Apache-2.0
compatibility: herdr >= 0.9.5; HERDR_ENV=1.
---

# workspace-open

Load the `workspace` skill's model and hard rules first. If `workspace` is not installed, stop.

## Preconditions

```bash
test "${HERDR_ENV:-}" = 1
herdr --version   # require 0.9.5 or newer
```

Read IDs only from JSON. On socket refusal: stop, print the command, wait for the owner's paste.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `open` | [open](references/open.md) | Find or create the clone's herdr workspace and report base branch |
