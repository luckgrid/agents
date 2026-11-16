# Credits

Written in Luckgrid's own words. Ideas only; no upstream text copied.

| Source | Licence | Commit / version | Ideas taken |
| --- | --- | --- | --- |
| Luckgrid `workspace` (this repository) | Apache-2.0 | this branch | Clone-only create/open; `--` nesting; folder collapse |
| Luckgrid `workbench-herdr` (Workbench guest skill) | Luckgrid's (private, luckgrid/workstation) | as installed | Adopt via owner `git worktree move`; remove without `--force`; branch `-d` |
| herdrdev/herdr CLI help | Apache-2.0 | 0.9.5 | `worktree create`, `open` and `remove` flags; `linked_worktree_source`; `already_open` |
| Bootstrap survey 2026-11-16 (Luckgrid) | Luckgrid's | session evidence | Verified create from clone, sub-branch `--`, clean remove |

Review additions learned from [Herdr's 0.9.5 source](https://github.com/herdrdev/herdr/tree/2ecf19a92fc3a7fff75b6ce3655ef04f4625ec01) (Apache-2.0), especially CLI parsing, response/workspace/worktree/agent schemas and path slugging; and this repository's coordination proposal (Apache-2.0, main `7986642`). Ideas only: exact associations, explicit unknown observations, shared-state coordination and safe retirement. No upstream text copied.
