---
id: AS-REC-2026-11-16-WORKSPACE-SKILLS
kind: session
title: Workspace skills first, built with herdr in a bootstrap session (2026-11-16)
summary: "The owner's seventh round. This repository's purpose: the place for common patterns for agents and agent workflows, from the owner's preferred tools, apps and configs, used through Workstation. workspace (M10) comes first, scoped to managing and maintaining herdr workspaces and worktrees. An agent may hand off to, or edit skills in, another workspace. Records capture intent and share one schema, which should become a Workbench tool. Cursor runs the work in bootstrap mode, from the kickoff brief beside this record. The owner's herdr interface ideas go to a Workstation session."
status: active
created: 2026-11-16
updated: 2026-11-16
related: [AS-PROGRAM, AS-STATE, AS-REC-2026-11-16-WORKSPACE-SKILLS-KICKOFF, AS-REC-2026-11-14-MODULAR-REWRITE, AS-ADR-0001, GW-ADR-0017, GW-ADR-0018, WS-ADR-0007]
---

# Workspace skills first, built with herdr in a bootstrap session (2026-11-16)

The roadmap's [seventh round](../../program.md#seventh-round-owner-2026-11-16) holds the rules recorded here. [STATE](../STATE.md) holds where each item stands. The guest session starts from the [kickoff brief](kickoff.md), and adds its questions, answers, survey evidence and plan to this record.

## The owner's intent (2026-11-16)

These are summaries, not quotes. Under the new records rule below, records capture intent unless the owner asks for exact words.

- **What this repository is for.** It is the place to record and keep common patterns for agents and agent workflows, drawn from the owner's preferred tools, apps and configs. The patterns are described here and used through Workstation.
- **What comes next.** Continue with skills for managing and maintaining workspaces. Write them with herdr commands, as Workstation's `main` provides them today, and toward the goals the wikis set for each workspace or repository.
- **How the work runs.** The work runs from the guest as it stands, in Cursor, in a bootstrap mode:
  - the agent is told to set aside the workarounds that exist for Codex's sandbox;
  - the owner approves each command, by allowlist or allow-once;
  - risky configuration changes are checked and fixed after the bootstrap and dogfooding.
- **Workspaces in herdr.** A herdr workspace is the top-level item in herdr's sidebar. The owner wants one herdr workspace per namespace folder in the guest's `~/Workspaces`.
- **Where the agents workspace starts.** It is set up already:
  - the clone is on a local `develop`;
  - a herdr worktree, `skills/herdr-workflows-1`, is open under it for developing these skills;
  - a Cursor session waits in the workspace for the kickoff.
- **Working across workspaces.** An agent in one workspace may sometimes hand off to another workspace, or edit skills there. Workstation and site-system have their own skills, which build on or use the public skills in this repository.
- **Records.**
  - Capture intent rather than word for word, unless word for word is asked for. When the intent is unclear, ask.
  - Records should share one schema, so that agents understand it and follow it in shared responses. That schema should become a Workbench tool.
- **herdr's interface, later.** The owner wants to extend herdr, by plugins or a fork, until Workbench's workflows settle and a Workbench TUI can be built:
  - **Sidebar:**
    - a workspace starts on `develop` (`main` if there is none, and later creates `develop`);
    - feature branches are herdr worktrees, with sub-worktrees nested and collapsible, at most two levels;
    - a collapsed workspace shows a status dot with the count of agents working in it;
    - worktree groups get their own background shade.
  - **Agents list:** shows which agents work under the same orchestrator, and which workspace or worktree each belongs to. Later it shows higher tiers, such as a Workspaces-level agent over per-workspace orchestrators.
  - **Panel:**
    - a top toolbar, with agent, workspace and worktree details on the left and actions on the right. The first action is a settings icon to clear or restart the session, copy a hand-off summary and show herdr's session state;
    - a bottom drawer with a prompt to the workspace's top orchestrator, with model, mode, skill and tool controls when an agent is present.
  - **Skills:** skills and workflows refined from daily use, eventually including skills for managing skills.

## What the session that wrote this checked (2026-11-16)

- **This repository at `0d53e41`:** M0 merged as #11 on 2026-11-15. STATE still said "pending review and merge", and this PR corrects it. No `agent_skills_commit` is pinned, so the guest has no skills from here.
- **luckgrid/workstation at `99da1da`:**
  - #51 merged: U16 copies skills itself. `workbench-guest-rules` and `workbench-herdr` live in `packages/workbench/guest/skills/`.
  - #52, #49 and #53 merged as documentation only. They cover the fast-workflow VM (one guest account, a guest-local `develop`, the owner publishing `main`), GW-ADR-0018 on names, and WS-ADR-0007's plan, starting with FW1.
  - Harness sandboxes stay. Outside a bootstrap session, herdr's socket stays refused to sandboxed shells until FW4. `workbench-herdr` still tells agents to print socket commands for the owner.
  - herdr is pinned at 0.9.5.
- **The owner's screenshots:**
  - the agents workspace on `develop`, with the `skills/herdr-workflows-1` worktree nested under it in herdr's sidebar;
  - `develop`, `main` and `skills/herdr-workflows-1`, all at `0d53e41`;
  - Workstation on `feat/fw1-guest-rules-and-attribution`;
  - a Cursor session in the agents clone, at its workspace trust dialog.

## Proposals from this session

These are not yet decided. The guest session puts them to the owner:

- **The skills** named in the [kickoff brief](kickoff.md#proposed-skills): `workspace` as the base skill, plus `workspace-open`, `workspace-worktree`, `workspace-maintain` and `workspace-status`, and possibly `workspace-handoff` for work across workspaces.
- **The record schema.** One schema for every Luckgrid wiki's records:
  - the front matter this wiki already uses (`id`, `kind`, `title`, `summary`, `status`, `created`, `updated`, `related`);
  - fixed section names for each kind of record: the intent, what was checked, the decisions, the evidence and what's next.

  A Workbench command would create a record and check it against the schema. That command is for a Workstation session to design. This repository's M4 `wiki` would teach the schema.
