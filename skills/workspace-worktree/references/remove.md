# remove

Remove a linked worktree only when it is clean and the owner wants it gone (work promoted or abandoned).

1. Check clean: `git -C <worktree path> status -sb`.
2. Remove with the **worktree's own** workspace ID (not the clone's):

   ```bash
   herdr worktree remove --workspace <worktree workspace ID>
   ```

3. If herdr returns `dirty_worktree_requires_force`, stop and report. Do not pass `--force`.
4. herdr keeps the branch. Delete it with:

   ```bash
   git -C ~/Workspaces/<ns> branch -d <branch>
   ```

   Run that yourself only from a session that can write the clone's `.git`. From a linked worktree session, if the write is refused, print the command for the owner. If `-d` refuses, report it. Use `-D` only on the owner's word.

Do not remove workspaces you did not create unless the owner names them. Never touch unrelated worktrees (for example another session's feature branch) without the owner's word.
