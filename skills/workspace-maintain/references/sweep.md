# sweep

## 1. Survey first

Run `workspace-status` `survey` (or the same steps). Present the nested tree and JSON. Do not change anything yet.

## 2. Findings

Turn the survey into a numbered list of proposed fixes. Examples:

- Clone has no herdr workspace → create via `workspace-open`
- Feature branch exists as a plain Git worktree → adopt via `workspace-worktree` `adopt` (owner runs move)
- Linked worktree wanted but missing → `create` or `open` from the clone ID
- Worktree abandoned and clean → `remove` (no `--force`)
- Empty orphan folder in the clone's worktree root → remove the folder only when the owner approves and it is not an active checkout
- Behind `origin/<base>` → report only unless the owner asks for a specific Git action the environment's rules allow

Never propose operating on non-clone herdr workspaces. Never propose `--force`, `--trust-repository`, or `git branch -D` without the owner's explicit word.

## 3. Approve then act

For each finding, wait for approval (or a clear skip). Apply only approved items, one at a time, through `workspace-open` / `workspace-worktree` rules. After each fix, re-check that item. On any surprise or refusal, stop and report; do not retry with a workaround.

## 4. Close

Reprint a short status summary (tree or JSON) after the approved set, or state what remains open for the owner.
