---
id: AS-REC-2026-11-16-WORKSPACE-SKILLS-KICKOFF
kind: brief
title: Kickoff brief for the workspace skills, built with herdr in Cursor's bootstrap mode (2026-11-16)
summary: "The brief that starts the Cursor session in the agents workspace, which is on develop with the herdr worktree skills/herdr-workflows-1 under it. Plan mode first: survey herdr 0.9.5 live with the owner approving each command, ask, and plan; then build the workspace namespace's first skills in the worktree. Records capture intent; work in another workspace is a hand-off or an approved edit on its own branch there."
status: active
created: 2026-11-16
updated: 2026-11-16
related: [AS-REC-2026-11-16-WORKSPACE-SKILLS, AS-PROGRAM, AS-STATE, AS-REC-2026-11-14-MODULAR-REWRITE, GW-ADR-0017, WS-ADR-0007]
---

# Kickoff: workspace skills, built with herdr (bootstrap session)

You are Cursor Agent in the Workbench guest. The guest has no network, so don't try to fetch anything this brief names.

**Where things stand:**

- **The agents workspace** (`~/Workspaces/agents`, luckgrid/agents) is on a local `develop`.
- **The worktree:** herdr has the worktree `skills/herdr-workflows-1` open under it, branched from `develop`. That worktree is where these skills are written. `develop`, `main` and the worktree's branch all point at `0d53e41`.
- **Where you are:** you may have been started in the main clone or in the worktree. Check with `pwd` and `git status -sb`.

**Work in Plan mode first:**

1. Read.
2. Survey herdr live.
3. Ask the owner the questions below.
4. Write the plan into the session record.

Build only after the owner approves the plan.

## Bootstrap mode: what the owner changed for this session

The guest rules hold except where the owner says otherwise. You'll find them as `workbench-guest-rules` if it is listed to you, or with `git -C ~/Workspaces/workstation show main:packages/workbench/guest/skills/workbench-guest-rules/SKILL.md`. For this session the owner says:

- **Run herdr commands yourself, socket commands included.** Cursor asks the owner to approve each one, by allowlist or allow-once. If the sandbox still answers `PermissionDenied`, say so once, then ask for the approval that runs that command outside the sandbox, or print it for the owner. Don't retry silently or look for another way to the socket.
- **Ignore the workarounds written for Codex's sandbox.** These are the printed-command routine in `workbench-herdr` and the `git -c core.hooksPath=/dev/null …` approval pattern. Cursor's approval prompt replaces them.
- **Configuration changes are allowed only after the owner approves each one.** That covers herdr's config, Cursor's settings and anything `wb setup` owns. Log each one in the bootstrap review file. The owner judges them after the bootstrap.
- **Other workspaces.** You may hand off to another workspace, or, with the owner's approval, edit skills there. Workstation's `workbench-*` and site-system's `site-system` build on or use this repository's public skills. Such an edit goes on a branch of its own in that workspace (a herdr worktree of it, created with the owner's approval), never on its `main` or `develop`. Otherwise write a hand-off note in `~/.workers/records/` for a session in that workspace.
- **Still never:**
  - push, or look for a credential;
  - read the credential paths the rules list;
  - install, update or link anything (`herdr update`, `herdr plugin link|install`, `npm`, `dnf`);
  - fetch from the network;
  - start a headless agent run;
  - start, prompt, send keys to or answer a dialog for another agent;
  - close a workspace, tab, pane or worktree you didn't create;
  - use `--force`, `--group` closes, `--trust-repository` or `git branch -D` without the owner's word.
- **Commit identity:** unchanged (FW1). `git -c user.name='Workbench guest' -c user.email='guest@workbench.invalid' commit …`, with explicit paths. Workstation's `wb pr` refuses any other author. A commit from the worktree may be refused, because its objects live in the main clone's `.git`. If that happens, say so and ask the owner. This is one of the things the survey should settle.

**The bootstrap review file** is `~/.workers/records/bootstrap-review-2026-11-16.md`. Log each approval asked for, each command run outside the sandbox, each config change (file, old value, new value) and each surprise, with the time.

**Suggested approvals:** allowlist the read-only commands (the `list` and `--help` forms, `git status -sb`, `git worktree list`). Allow once anything that creates, opens, moves or removes.

## Records: capture intent, in one shared schema

The owner set this rule on 2026-11-16:

- Record what the owner meant, in your own words, faithfully.
- Quote the owner word for word only when they ask for it.
- Evidence is quoted exactly: command output, hashes and herdr's JSON replies, with home paths written as `~`.
- When the intent is unclear, ask.

Records share one schema, so that agents can follow it. Use the front matter this wiki already uses, and keep the session record's sections as they are: the intent, what was checked, the proposals, and then what you add (questions and answers, the survey, the plan, the results). A Workbench tool to create and check records is proposed for a Workstation session. Don't build it here.

## Read first

1. In this repository:
   - `README.md` (it decides);
   - `wiki/README.md`;
   - `wiki/records/STATE.md`;
   - in `wiki/program.md`: the seventh round, "Tiers and composition", "Namespaces", M3 and M10;
   - the session record, `wiki/records/2026-11-16-workspace-skills/README.md`.
2. From `~/Workspaces/workstation`, its `main`, read only (`git -C ~/Workspaces/workstation show main:<path>`):
   - `packages/workbench/guest/skills/workbench-herdr/SKILL.md`;
   - `wiki/workbench/daily-use.md`;
   - `wiki/workbench/agent-roles.md`;
   - `wiki/records/decisions/0007-minimal-core-first.md` (§1 and §3);
   - `wiki/records/2026-11-15-fast-workflow-vm-direction.md`.
3. `herdr --version` (expect 0.9.5), `herdr --skill`, `herdr --help`, and the `--help` of `workspace`, `worktree`, `tab`, `pane`, `agent` and `plugin`, and of each subcommand as you need it. Never run bare `herdr`: it attaches the TUI.

## The goal

Write the `workspace` namespace's first skills, for managing and maintaining workspaces with herdr 0.9.5 as it is installed. Write them from commands you ran and replies you saw. Don't rely on help text alone. Lay the groundwork the owner's sidebar ideas need (see the record), without extending or forking herdr.

### The model the skills describe

Confirm each point with the owner:

- **A workspace** is one clone, `~/Workspaces/<namespace>`, and one herdr workspace labelled with the same name. It is the top-level item in herdr's sidebar.
- **Its base branch** is `develop`, or `main` where there is no `develop`. Creating `develop` with a tool comes later and is the owner's call. The agents workspace already has one.
- **A feature branch** is a herdr worktree of the workspace, made with `herdr worktree create` and never with plain `git worktree add`. A sub-worktree is one level below it, and nesting stops there.
- **Maintenance:**
  - report first: the base branch behind, merged branches and worktrees, dirty or orphaned worktrees, herdr workspaces with no clone, and clones with no workspace;
  - then act on each item the owner approves.

  The guest can't fetch. Bringing clones up to date is `wb sync` on the Mac.
- **Status:** for each workspace, its base branch, its worktrees (nested), and the agents and panes herdr reports in each. This is the data a future sidebar would show.
- **Across workspaces:** a hand-off note, or an approved edit on a branch in the other workspace, as in the bootstrap rules above.

### Proposed skills

Flat folders, each with a `CREDITS.md`, and a mode table only where a skill has references.

| Skill | Does |
| --- | --- |
| `workspace` | The base: the model, the names, where Workbench narrows it, and a mode table for the planned issue, pr and release uses (references written later) |
| `workspace-open` | Finds or creates the herdr workspace for a clone, checks the base branch, and reports |
| `workspace-worktree` | Creates, opens, adopts and removes worktrees and sub-worktrees through herdr, keeps the two-level rule, and cleans up |
| `workspace-maintain` | The report-first sweep, then each approved fix |
| `workspace-status` | One survey across all workspaces, nested, with the agents in each workspace and worktree |
| `workspace-handoff` (if the owner wants it) | A hand-off to a session in another workspace: what to do, on which branch, and which skills it builds on |

**Tiers:** what holds anywhere goes in these public skills: herdr commands, the model, the report format. What holds only in the Workbench guest stays in Workstation's `workbench-herdr`, which may only narrow. That covers the sandbox's refusal outside bootstrap, the owner-run `git worktree move`, `~/.workers/spaces/` as herdr's worktree root, and the `wb` commands.

**Public repository:** no hostnames, user or account names, emails or IPs. Write paths as `~/…`.

## Steps

1. **Survey (Plan mode).** Record each reply in the bootstrap review file.
   - Read-only first: `herdr workspace list`, `herdr worktree list` for the agents workspace, `herdr pane list`, `git worktree list`.
   - Then, approving each command, exercise the lifecycle once:
     - a sub-worktree `scratch/herdr-survey` under `skills/herdr-workflows-1`, if herdr allows one; otherwise a scratch worktree of the agents workspace;
     - list, open and focus it;
     - a test commit from inside a worktree;
     - removing the scratch worktree with herdr, then `git branch -d` on its branch.
   - Note what herdr does with nesting, labels, groups and the paths it picks.
   - Note what `herdr plugin --help` and `herdr agent --help` say a plugin or the CLI can read or change: sidebar items, labels, colours, per-agent metadata, parent and child relationships.
2. **Questions for the owner,** one at a time, recommended option first:
   - each point of the model;
   - the skill names, and whether `workspace-status` folds into `workspace-maintain` and `workspace-handoff` is wanted;
   - herdr used directly in `workspace-*` now, with M3 `multiplex` extracting a backend later (recommended), or `multiplex` first;
   - the first `agent_skills_commit` pin right after these skills, so the guest can use them (recommended), or after M2;
   - whether `workbench-herdr`'s narrowing is edited in a Workstation worktree from this session, or handed off (recommended: hand off);
   - anything the survey contradicted.
3. **Plan.** Add the agreed plan to the session record, then ask the owner to approve it.
4. **Where to build.** If you were started in the main clone, ask the owner: hand off to a new session started in the worktree (recommended, since the guest rules expect one checkout per session), or carry on from here and edit the worktree by its path.
5. **Build** on `skills/herdr-workflows-1`, with one commit per skill and one for the wiki.
   - Each skill is short and in Luckgrid's own words.
   - It names each herdr command with the options herdr 0.9.5 showed, says how to read IDs from the JSON reply and never guess one, and says what to do on refusal.
   - Before writing a mutating command into a skill, run it in the survey.
   - Also update:
     - the session record: questions and answers, survey evidence, plan, results;
     - `wiki/records/STATE.md`;
     - `wiki/resources/skill-references.md`: herdr 0.9.5's own skill and help text, Apache-2.0, ideas only and nothing copied; and Luckgrid's `workbench-herdr`.
   - Run `scripts/check.sh skills`, `scripts/check.sh --self-test`, and `rumdl check wiki/` if it's installed, and read their output.
6. **Hand-off notes** in `~/.workers/records/`, not committed:
   - `workspace-skills-report.md`: branch, head, commits, check outputs, what wasn't done, open questions;
   - `workstation-herdr-followups.md`, for a Workstation session:
     - the narrowing `workbench-herdr` needs so it defers to `workspace-*`;
     - the record-schema tool;
     - each of the owner's herdr interface ideas, with what the survey showed a plugin or the CLI can and can't do, and what would need a fork;
     - the config changes from the bootstrap review file.
7. **Finish.** Print `wb promote agents skills/herdr-workflows-1` and `wb pr agents skills/herdr-workflows-1` for the owner. The owner starts the independent review.

**Done when:**

- the owner approved the plan;
- the agreed skills pass `check.sh`, and each mutating command in them was seen working in the survey;
- the scratch worktree and branch are gone;
- the record and STATE say what was done;
- the bootstrap review file lists every approval and config change.

## Stop and ask

Stop, keep what you saw, and ask the owner when:

- a source and this brief disagree;
- herdr behaves differently from what a skill would say;
- a step needs the network, an install or a credential;
- a change would decide something nobody has recorded.

A proposal is not acceptance.
