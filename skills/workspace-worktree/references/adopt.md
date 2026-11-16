# adopt

Adopt a plain registered linked Git worktree. Apply the base skill's environment-policy preflight; in Workbench, the installed rules may require the owner to run the move even when the socket is reachable.

1. Confirm the old checkout's Git registration, repository, branch/tip, clean state including untracked/ignored data, and that no session/process is using it. Unknown activity blocks a move.
2. Resolve the configured worktree root from the owning environment or explicit owner confirmation. Do not infer it from existing checkout parents: Herdr supports explicit paths, and a clone with no linked worktrees provides none.
3. Obtain explicit approval for the exact destination and check it does not exist or collide with any registered/attached checkout. Herdr 0.9.5's default slug lowercases ASCII letters and replaces runs of non-alphanumeric characters with `-`, trimming edge dashes. It is lossy (case, punctuation and separators can collide); it is not a reversible path API. If the root/destination cannot be verified, stop.
4. Run the approved move and open in order, quoting actual paths:

   ```bash
   git -C ~/Workspaces/<ns> worktree move <old path> <verified destination>
   herdr worktree open --workspace <clone ID> --path <verified destination> --no-focus
   ```

   If the move succeeds but opening fails, report the moved checkout and retained tip. Do not move it again or remove it to retry. Resolve registration and attachments afresh.
5. Validate the `worktree_opened` result as in `open`. Never add `--trust-repository`.
