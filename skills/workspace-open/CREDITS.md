# Credits

Written in Luckgrid's own words. Ideas only; no upstream text copied.

| Source | Licence | Commit / version | Ideas taken |
| --- | --- | --- | --- |
| Luckgrid `workspace` (this repository) | Apache-2.0 | this branch | Shared model and hard rules |
| Luckgrid `workbench-herdr` (Workbench guest skill) | Luckgrid's (private, luckgrid/workstation) | as installed | One workspace per repository; create with `--cwd` and `--label` |
| herdrdev/herdr CLI help | Apache-2.0 | 0.9.5 | `workspace list` / `create` flags |

Review additions learned from [Herdr's 0.9.5 source](https://github.com/herdrdev/herdr/tree/2ecf19a92fc3a7fff75b6ce3655ef04f4625ec01) (Apache-2.0), especially CLI parsing, response/workspace/worktree/agent schemas and path slugging; and this repository's coordination proposal (Apache-2.0, main `7986642`). Ideas only: exact associations, explicit unknown observations, shared-state coordination and safe retirement. No upstream text copied.
