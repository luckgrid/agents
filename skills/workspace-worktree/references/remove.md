# remove

Retire exactly one linked checkout only when authorized. Removing the checkout and deleting its branch are separate actions.

1. Refresh Git registration and the Herdr attachment. Resolve the linked checkout's own workspace ID, exact path, branch and current tip; refuse a clone ID, ambiguous association or changed approved tip.
2. Verify no actor/session/pane/process is using the checkout. An idle agent or empty agent list does not prove this; inspect available pane/process evidence and obtain owner confirmation where that evidence cannot establish quiescence. Unavailable evidence blocks automatic removal. Never close another session or answer its dialogs.
3. Check tracked and untracked changes (`git -C <path> status -sb`) and ignored local files separately. Preserve required data. Confirm work is retained at an exact recovery/integrated ref and tip, or explicitly abandoned by the owner. A clean checkout, promotion alone or a branch label is not retention evidence; squash integration may not satisfy ancestry checks.
4. Preview the exact checkout/path/tip and consequences. Revalidate immediately before the authorized action; preflight alone is not a lock:

   ```bash
   herdr worktree remove --workspace <linked worktree's own workspace ID>
   ```

5. Require `result.type: worktree_removed` with matching `workspace_id` and `path`, then confirm Git/Herdr state. On `dirty_worktree_requires_force` or any refusal, stop and report; never pass `--force`.
6. Herdr retains the branch. Delete it only when branch deletion was separately authorized, its tip still matches the preserved/abandoned candidate and no checkout uses it:

   ```bash
   git -C ~/Workspaces/<ns> branch -d <branch>
   ```

   Follow the environment's Git-write rules; on refusal, report for owner action. If `-d` refuses, retain the branch and report (including squash-merge cases). Never use `-D` without the owner's explicit decision.

Nesting is a display convention; no descendant deletion is implied. Never remove unrelated worktrees without the owner naming them.
