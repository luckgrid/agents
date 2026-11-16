# agents

**Direction (owner, 2026-11-12):** this repository becomes an agents monorepo for reusable configurations, skills and tools. It was named `luckgrid/agent-skills` until 2026-11-13; GitHub redirects the old name. Skills stay a package at `skills/`. The [decision](wiki/records/decisions/0001-agents-monorepo.md), [proposed workspace](wiki/architecture/agents-workspace.md) and [knowledge lineage](wiki/resources/agents-knowledge-lineage.md) record it. Workstation's wiki is the latest source for shared configuration principles and component boundaries; older WfOS/Workstreams material supplies lineage.

Agent Skills modules for the Workbench guest's harnesses (Claude Code, Codex and Cursor) and for anyone else who wants them. Each module is a folder `skills/<name>/` in the Agent Skills format, written in Luckgrid's own words.

## Skills

M10's five `workspace*` skills: `workspace`, `workspace-open`, `workspace-worktree`, `workspace-status` and `workspace-maintain` (seventh round). The first install pin follows them, with Workbench's prerequisites. The modules, their order and the namespaces registry are in the [roadmap](wiki/program.md); Workbench's own skills (`workbench-*`) are in Workstation.

## Wiki

This README is the entry point and holds the rules. The [wiki](wiki/README.md) holds the details: how the repository works and why, where it stands ([STATE](wiki/records/STATE.md)), the [roadmap](wiki/program.md), and dated records of each change. Records follow Luckgrid's shared wiki template (see `wiki/README.md`), and preserve evidence exactly while summarising the owner's intent faithfully, quoting the owner only when asked. Record material changes, decisions, evidence and review rounds; incidental internal steps need no separate record.

## No secrets

This repository is public. It holds no hostnames, user names, account names, email addresses, IPs, keys or tokens, and paths are written as `~/…`. Keep it that way, in the wiki too.

## Writing a module

A module is a folder `skills/<name>/`:

- `SKILL.md`: the router. YAML front matter with `name` (equal to the folder name) and `description` (when to use it, at most 1024 characters), then when to use the module, its hard rules, and a mode table that says which reference to read for each mode. Short, always correct, and enough on its own for the hard rules.
- `references/`: one Markdown file per mode or workflow, flat, loaded only when a task needs it.
- `CREDITS.md`: what the module learned from where (repository, commit, licence, ideas taken).

The rules:

- **Own words only.** No text is copied from another skill; ideas are credited in `CREDITS.md`. A module that needs a tool says so in `compatibility`, checks that the tool is installed, and never installs it.
- **Namespaces.** Every skill is named `<namespace>` or `<namespace>-<name>`, in full words, never a shorthand like `wb-`. A namespace's base skill carries the name alone; a use that is invoked explicitly, or that must load on its own description, is a `<namespace>-<name>` skill in a flat folder beside it. Each namespace is owned by one repository, which alone holds its skills; a new one is added to the roadmap's [registry](wiki/program.md#namespaces-owner-2026-11-14) in the same PR as its first skill.
- **Invocation:** `<prefix><skill> [<mode>] [args]`, with the harness's prefix (`$` for Codex, `/` for Claude Code and Cursor). In the mode table, the first column is the literal mode word in backticks, and that word is the reference's file name without `.md`.
- **Three tiers, and rules only tighten downward:** public modules here, Workbench's skills in Workstation, project skills in their own repositories. Public methods are narrowed by the guest's restrictions, then a project's. A project may narrow a guest or public workflow, never relax the guest boundary, and no skill overrides harness instructions, permissions or sandbox enforcement.
- **Name, don't path.** Modules refer to each other by skill and mode name, each such reference with a one-line fallback for when that skill isn't installed. A fallback stops if a required rule or prerequisite is unavailable.

Run `scripts/check.sh` (it needs only `python3`, 3.9 or later). It takes a skills folder (`skills/` by default; another repository's as an absolute path) and `--namespaces a,b,…`; `scripts/check.sh --self-test` checks the check against its fixtures in `scripts/fixtures/`. [Skill modules](wiki/architecture/skill-sources.md) says what it proves and what it doesn't.

## Using these skills

Copy a `skills/<name>` folder into your harness's skills folder, or use any Agent Skills installer; none of them is tested here.

> **About Workstation.** [luckgrid/workstation](https://github.com/luckgrid/workstation) is Luckgrid's monorepo for Workstation, Groundwork and Workbench: the tools that set up a Mac's accounts and the isolated Lima guest where agents work, and that install this repository's skills there. It is still in development and private for now, so its links open only for collaborators until it becomes public when it's ready.

The Workbench guest installs them through `wb setup` (step U16), which copies skill folders from an export of a pinned commit of this repository; its page, Workbench's [daily use](https://github.com/luckgrid/workstation/blob/main/wiki/workbench/daily-use.md#guest-tools-wb-setup) in Workstation, decides how.

## Licence

Apache-2.0, see `LICENSE`. Every file in this repository is Luckgrid's own; there are no third-party files.
