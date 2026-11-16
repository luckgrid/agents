# open

Ensure a Herdr attachment exists for the requested clone; no feature checkout is created here. Load the base skill's environment-policy preflight before socket commands.

## Steps

1. Resolve the requested clone path (initial binding: `~/Workspaces/<ns>`). Use Git's top-level, private/common Git directories and worktree registration to confirm it is the main checkout, not just a directory with a `.git` file. Quote actual path arguments.
2. Obtain a successful `herdr workspace list` response with `result.type: workspace_list`. Match both `worktree.repo_root` and `worktree.checkout_path` to the resolved clone, with `is_linked_worktree: false`. Select exactly one `result.workspaces[].workspace_id`. Multiple matches or missing/contradictory metadata are ambiguous: stop, never create another attachment to resolve uncertainty.
3. Only a complete successful list with no matching attachment establishes absence. If creation is authorized, run once:

   ```bash
   herdr workspace create --cwd ~/Workspaces/<ns> --label <ns> --no-focus
   ```

   Require `result.type: workspace_created`; take `result.workspace.workspace_id` and validate `result.workspace.worktree` against the clone. On interrupted or malformed creation, inspect the list before retrying; retain/report a created attachment.
4. Resolve a local base ref: `refs/heads/develop` if present, else `refs/heads/main`. If neither exists, report no supported base and stop branch-dependent work. Never create or switch either branch. Record the selected ref and observed tip.
5. Report unrelated or out-of-binding attachments separately. A linked checkout of this clone is not an unrelated non-clone workspace. Missing metadata means unclassified, not proven non-clone; do not mutate either category.

## Report

Print namespace, resolved clone path (home-normalized for public notes), clone workspace ID or absence/ambiguity, base ref/tip, observation source/time and unrelated/unclassified attachments (IDs and labels only).
