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

Load the `workspace` skill's model, environment-policy preflight and hard rules first. If `workspace` is not installed, stop. Load `workspace-status` and its `survey` reference; if either is unavailable, stop. Before each fix, load the named `workspace-open` or `workspace-worktree` reference; stop if required rules are missing.

## Preconditions

```bash
test "${HERDR_ENV:-}" = 1
herdr --version   # require 0.9.5 or newer
```

Survey local refs without fetching or syncing. Report stale/unknown freshness; refresh, if requested, belongs to the environment's separately authorized workflow.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `sweep` | [sweep](references/sweep.md) | Survey, report findings, then each approved fix |
