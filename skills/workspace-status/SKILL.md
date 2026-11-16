---
name: workspace-status
description: >
  Read-only survey of a Luckgrid clone workspace: grouped outline of worktrees
  (children inferred from -- in branch names), agents, behind/orphan findings,
  plus a fenced JSON block of the same data. Makes no changes. Loads
  workspace base rules. Requires herdr 0.9.5+ with HERDR_ENV=1.
license: Apache-2.0
compatibility: herdr >= 0.9.5; HERDR_ENV=1.
---

# workspace-status

Load the `workspace` skill's model, environment-policy preflight and hard rules first. If `workspace` is not installed, stop. This skill changes nothing.

## Preconditions

```bash
test "${HERDR_ENV:-}" = 1
herdr --version   # require 0.9.5 or newer
```

Never fetch or sync in this read-only survey. Compare local refs and report freshness as unknown unless independently known. A request for fresher data is a separate authorized environment action.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `survey` | [survey](references/survey.md) | Grouped outline plus versioned draft JSON projection |
