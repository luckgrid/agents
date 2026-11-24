---
name: orchestration
description: >
  Coordinate agents, tasks, workflows, handoffs, and execution systems with
  portable practices and explicit environment capabilities. Use when assigning
  work, observing execution, coordinating worktrees, or handing off results.
license: Apache-2.0
compatibility: Herdr 0.9.5+ with HERDR_ENV=1 for its mapping; tmux for bounded pane workflows. Capabilities vary by environment.
---

# Orchestration

This module describes reusable coordination practices. It is not an executable
orchestration API and does not make a particular session manager the definition
of coordination. Identify the current environment, load the matching reference,
and state missing capabilities plainly. Do not infer that a backend supports an
operation merely because another backend does.

## Rules

- Load the host, repository, and session rules that govern the operation first.
  In a Workbench guest, load `workbench-guest-rules`; Workbench remains the
  authority for guest operations. If required policy is unavailable, stop.
- Assign each writer a separate checkout. Coordinate shared targets through
  one designated integrator; session status is not a lock or authorization.
- Use native dispatch, status, and wait features when the selected environment
  supports them. Preserve native labels as observations and report unknown
  state as unknown.
- Manage worktrees through the authorized environment's lifecycle. Revalidate
  refs, paths, attachments, and activity before mutation. Never force removal.
- Put briefs, results, findings, and handoffs in durable files. Do not type
  one agent's handoff into a foreign pane. Recipients revalidate the live state.
- Never install or update an execution system as part of a coordination task.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `coordinate` | [coordinate](references/coordinate.md) | Plan bounded parallel work and integrate results |
| `dispatch` | [dispatch](references/dispatch.md) | Assign work and observe native execution status |
| `worktrees` | [worktrees](references/worktrees.md) | Coordinate authorized worktree lifecycle |
| `herdr` | [herdr](references/herdr.md) | Map workflows to Herdr capabilities |
| `tmux` | [tmux](references/tmux.md) | Use the bounded tmux alternative |
