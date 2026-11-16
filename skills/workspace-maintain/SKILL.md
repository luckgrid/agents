---
name: workspace-maintain
description: >
  Report-first maintenance for a Luckgrid clone workspace: run the
  workspace-status survey, list findings, then apply each approved fix using
  workspace-open and workspace-worktree rules. Use when cleaning orphans,
  opening a missing clone workspace, or aligning worktrees after review. Loads
  workspace base rules. Requires herdr 0.9.5+ with HERDR_ENV=1.
license: Apache-2.0
compatibility: herdr >= 0.9.5; HERDR_ENV=1.
---

# workspace-maintain

Load the `workspace` skill's model and hard rules first. If `workspace` is not installed, stop. Prefer `workspace-status` for the survey; if it is unavailable, perform the same survey steps from that skill's `survey` reference and stop if those rules are required and missing.

## Preconditions

```bash
test "${HERDR_ENV:-}" = 1
herdr --version   # require 0.9.5 or newer
```

Refresh remotes first when behind checks matter. This public skill names no sync command; only `workbench-herdr` may name `wb sync`.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `sweep` | [sweep](references/sweep.md) | Survey, report findings, then each approved fix |
