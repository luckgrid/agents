# create

Create a linked checkout and open its Herdr attachment. Load the base skill's environment-policy preflight; this workflow never starts an agent.

## Inputs and preflight

- Resolve exactly one clone workspace ID through `workspace-open`. If the skill or required rules are missing, stop.
- Record branch name, work intent, explicit base ref and observed base tip, and intended integration target. Use the clone's base for a top-level topic and the parent topic for a child.
- For a child, use `<parent-branch>--<name>`, at most one `--`. Confirm the parent ref exists and the suffix is nonempty. Reserve `--` for display grouping; it is not proof of Git ancestry.
- Inspect registered Git worktrees, existing refs, Herdr attachments and destination collisions before mutation. An existing branch goes to `open` after its purpose/base is confirmed; never silently reuse/reset it. An occupied or colliding path refuses.
- Use `--path` / `--label` only when the owner requested that override. Never derive a path by reversing a slug; different branch names can produce one slug.

## Command and result

```bash
herdr worktree create --workspace <clone ID> --branch <branch> --base <base> --no-focus
```

Use `--focus` only when requested. Require `result.type: worktree_created`. Take the ID from `result.workspace.workspace_id` and checkout path from `result.worktree.path`; check `result.workspace.worktree.repo_root`, `checkout_path` and linked flag against Git registration and the requested clone/branch. `result.worktree` itself has no `repo_root` field in 0.9.5.

Record the actual new tip and base/dependency snapshot; if the base moved since preflight, report stale evidence and do not integrate from the earlier claim. A preflight check does not lock the ref.

If creation fails or the reply is interrupted, inspect surviving refs, checkout and attachment before any authorized retry. Report partial success; never delete/recreate the checkout just to repeat the operation.
