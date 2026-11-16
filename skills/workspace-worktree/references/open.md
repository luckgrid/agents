# open

Reopen an existing registered linked checkout. Apply the base skill's environment-policy preflight first.

1. Resolve the clone ID and current Git worktree registration. Match the exact checkout/branch to that repository; detached HEAD is valid only when explicitly requested. A missing checkout, conflicting branch/path or ambiguous attachment stops.
2. Use one selector:

   ```bash
   herdr worktree open --workspace <clone ID> --branch <branch> --no-focus
   # or
   herdr worktree open --workspace <clone ID> --path <verified checkout path> --no-focus
   ```

3. Require `result.type: worktree_opened`; take `result.workspace.workspace_id`, `result.worktree.path` and `result.already_open`. Validate `result.workspace.worktree` against the requested clone and checkout. If already open, reuse that attachment; do not start a second editing session in an occupied checkout.
4. On failure or interrupted output, inspect surviving state before an authorized retry. Never add `--trust-repository`.
