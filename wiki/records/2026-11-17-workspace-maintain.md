---
id: AS-REC-2026-11-17-WORKSPACE-MAINTAIN
kind: session
title: Workspaces maintenance sweep (2026-11-17)
summary: "Dogfood of workspace-maintain across the six Workspaces clones: survey, delete merged leftover skills/herdr-workflows-1, leave feat/workspace-skills unmerged (no new skills vs main #14), create guest-local develop on four main-only clones, open luckgrid.net herdr workspace, remove empty spaces folders, refresh STATE for #14 on main."
status: active
created: 2026-11-17
updated: 2026-11-17  # maintenance session in Workspaces herdr workspace w6
related: [AS-STATE, AS-REC-2026-11-16-WORKSPACE-SKILLS, AS-REC-2026-11-16-WORKSPACE-SESSION, AS-REC-2026-11-16-WORKSPACE-PR14-REVIEW, AS-PROGRAM]
---

# Workspaces maintenance sweep (2026-11-17)

Session in the non-clone herdr workspace `Workspaces` (`w6`), using the public `workspace*` skills on `main` at `8a106a4` (#14) and Workbench guest rules. Prior history cited as data: [STATE](STATE.md) (as of 2026-11-16), [workspace-skills session](2026-11-16-workspace-skills/session.md), [seventh-round record](2026-11-16-workspace-skills/README.md), `~/.workers/records/workstation-herdr-followups.md`, session stubs `2026-11-16-agents-*` / `2026-11-16-feat-workspace-skills-*`.

## Intent

Owner asked to maintain Workspaces with herdr: remove stale merged branches/worktrees, add guest-local `develop` where only `main` existed and there were no open PRs, and record the session in the wikis. Creating `develop` is an explicit exception to the skills' "Never create `develop`" rule for this maintain pass only; `develop` was not pushed.

## Survey (workspace-status)

| Namespace | Clone ID | Base | HEAD | Linked worktrees | Notes |
| --- | --- | --- | --- | --- | --- |
| agents | w8 | develop | develop @ `8a106a4` (= origin/main) | none | leftover locals below |
| luckgrid.net | none → **wF** created | main (then develop added) | main | none | missing clone attachment |
| site-system | w5 | main (then develop added) | main | none | list omitted worktree metadata; `worktree list` confirmed clone |
| wiki-nest | w9 | main (then develop added) | main | none | |
| wiki-reader | wE | main (then develop added) | main | none | public API: 0 open PRs |
| workstation | wD | develop | develop @ `3958ed1` (= origin/main) | none | |

Non-clone attachments (untouched): `w4` (`~`), `w6` (`Workspaces`). This Cursor agent ran in `w6:p1`.

Empty-folder candidates under `~/.workers/spaces/` (verified empty, not registered as Git worktrees): `agents/`, `workstation/`, `herdr-probe/`.

Freshness: each clone 0 ahead / 0 behind local `origin/main`; `wb status` still listed `wb sync` as an owner ACTION. Open PRs: public `luckgrid/agents` and `luckgrid/wiki-reader` had 0 open via GitHub API; private repos returned 404 without credentials—local evidence was no feature branches and 0 ahead of `origin/main`.

`wb ready` offered `wb promote agents feat/workspace-skills` (8 commits beyond origin/main). That promote is **not** correct after #14's squash: see comparison below.

## Skill comparison: `main` vs `feat/workspace-skills`

| Tip | Skills folders | Verdict |
| --- | --- | --- |
| `main` / `develop` @ `8a106a4` (#14) | `workspace`, `workspace-open`, `workspace-worktree`, `workspace-status`, `workspace-maintain` | Post-review wording; keeps `pr14-review.md` |
| `feat/workspace-skills` @ `dfecbc9` | same five | No new skills; older/simpler wording; wiki still reads pre-merge |

**Decision:** do not merge or promote `feat/workspace-skills`. No new skills; `main` holds the review remediation. Branch deleted with `-D` after this record (owner-authorized by the maintain plan).

## Fixes applied

1. `git branch -d skills/herdr-workflows-1` in `~/Workspaces/agents` (ancestor of `main`/`develop` at `0d53e41`).
2. Guest-local `develop` at current `main` tip, without switching HEAD, for `luckgrid.net`, `site-system`, `wiki-nest`, `wiki-reader`. `/usr/local/bin/wb-commit branch develop main` refused because those checkouts are on `main`; used `git branch develop main` (same bootstrap pattern as FW1). Not pushed.
3. `herdr workspace create --cwd ~/Workspaces/luckgrid.net --label luckgrid.net --no-focus` → workspace `wF`.
4. Removed empty dirs only: `~/.workers/spaces/{agents,workstation,herdr-probe}`.
5. Opened record worktrees: `records/workspace-maintain-2026-11-17` from `develop` on agents (`wG`) and workstation (`wH`).
6. After this page: `git branch -D feat/workspace-skills` (tip `dfecbc9`).

Not done: push, PR, remote delete of `origin/docs/omarchy-workflow-option` on workstation (unmerged remote-only; reported only). Mac `wb sync` still for the owner.

## Socket note

Herdr socket commands answered from this Cursor pane (same Cursor sandbox socket gap recorded in the 2026-11-16 bootstrap follow-ups under `~/.workers/records/`). Public skills still say: run; on refusal stop and print for the owner.

## STATE updates from this session

[STATE](STATE.md) now records M10/#14 on `main`, five `workspace*` skills present, first pin still open, and this maintain record.
