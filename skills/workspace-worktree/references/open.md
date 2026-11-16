# open

Reopen a worktree that already exists on disk or as a Git worktree entry.

```bash
herdr worktree open --workspace <clone ID> --branch <branch> --no-focus
# or
herdr worktree open --workspace <clone ID> --path ~/.../path --no-focus
```

If the reply has `already_open: true`, reuse the returned workspace ID; do not create a second workspace. Still use the **clone** workspace ID as `--workspace`.
