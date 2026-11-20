# remove

Retire exactly one linked checkout only when authorized. Who authorizes removal, abandonment and branch deletion is the environment's rule; where it grants nothing, it is the owner. Removing the checkout and deleting its branch are separate actions.

In Workbench, a disposable `test/<topic>` branch and its worktree are not removed through these steps: the operator discards them with `wb-flow tidy` (`workbench-herdr`), which removes the registered worktree and deletes the branch together.

1. Refresh Git registration and the Herdr attachment. Resolve the linked checkout's own workspace ID, exact path, branch and current tip; refuse a clone ID, ambiguous association or changed approved tip.
2. Verify no actor/session/pane/process is using the checkout. An idle agent or empty agent list does not prove this; inspect available pane/process evidence and obtain confirmation from whoever the environment's rules name (otherwise the owner) where that evidence cannot establish quiescence. Unavailable evidence blocks automatic removal. Never close another session or answer its dialogs.
3. Check tracked and untracked changes (`git -C <path> status -sb`) and ignored local files separately. Preserve required data. Confirm work is retained at an exact recovery/integrated ref and tip, or explicitly abandoned by whoever may abandon it under the environment's rules (otherwise the owner). A clean checkout, promotion alone or a branch label is not retention evidence; squash integration may not satisfy ancestry checks.
4. **Durable context outside the checkout.** Before removal, confirm kickoff/handoff notes, review findings, dispositions, check receipts and any recovery material a later session needs already live in the environment's durable locations (records / run notes), not only under this worktree. Removing a worktree must not strand the only useful briefing or result inside it. Closing the checkout is resource cleanup; it is not context pruning and does not delete retained history elsewhere.
5. Preview the exact checkout/path/tip and consequences. Revalidate immediately before the authorized action; preflight alone is not a lock:

   ```bash
   herdr worktree remove --workspace <linked worktree's own workspace ID>
   ```

6. Require `result.type: worktree_removed` with matching `workspace_id` and `path`, then confirm Git/Herdr state. On `dirty_worktree_requires_force` or any refusal, stop and report; never pass `--force`.
7. Herdr retains the branch. Delete it only when branch deletion was separately authorized, its tip still matches the preserved/abandoned candidate and no checkout uses it:

   ```bash
   git -C ~/Workspaces/<ns> branch -d <branch>
   ```

   Follow the environment's Git-write rules; on refusal, report it. If `-d` refuses, retain the branch and report (including squash-merge cases). Never use `-D` unless the owner decides it or an environment rule allows it (in Workbench, only the operator's `test/<topic>` discard through `wb-flow tidy`).

Nesting is a display convention; no descendant deletion is implied. Never remove unrelated worktrees unless the owner, or the task the environment's rules authorize, names them.
