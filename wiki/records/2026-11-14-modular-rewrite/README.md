---
id: AS-REC-2026-11-14-MODULAR-REWRITE
kind: session
title: Planning the modular skill rewrite (2026-11-14)
summary: "A cloud session took measure of the skills in this repository, site-system and Workstation, and the open Workstation PRs, and proposed how to carry out the custom-modules plan. The owner added that every dependency around the removed upstream skills goes too, the skills CLI and its package.json included. Six questions, each answered with the recommended option: the herdr copy dropped in M0; wb setup copies skill folders itself; public users copy a folder or use any installer; a new workspace module (M10) after M4; mode invocation, a shared check.sh and rules that only tighten downward. The kickoff brief for the guest session is beside this record."
status: active
created: 2026-11-14
updated: 2026-11-14
related: [AS-PROGRAM, AS-STATE, AS-REC-2026-11-12-SKILL-MODULES, AS-REC-2026-11-12-SITE-SYSTEM-SKILL, AS-REC-2026-11-14-MODULAR-REWRITE-KICKOFF, AS-ARCH-SKILL-SOURCES, AS-ARCH-GUEST-INSTALL, GW-ADR-0014]
---

# Planning the modular skill rewrite (2026-11-14)

The roadmap's fifth round ([program](../../program.md#fifth-round-owner-2026-11-14)) holds the rules recorded from this session; [STATE](../STATE.md) holds where each item stands. The brief that starts the session in the Workbench guest is [kickoff.md](kickoff.md).

## The owner's asks (2026-11-14)

Word for word:

> I would like you to check the agents repo wiki program, records and relevant docs - take measure of the current skills and propose a plan to implement the modular skill rewrite
>
> some tips:
>
> * most recent work on skills can be seen in site-system and workstation repo
> * other convenience skills were proposed recently in open workstation PRs that may provide context about using public skills with private skills (i.e. workspace skills in agents monorepo combined with workstation workbench skills to manage workspaces)

Sent while the session was reading:

> I also want to add that since the upstream skills are getting removed, let's remove any dependencies around them - this should remove the need for package.json and shells.sh cli

The session read "shells.sh cli" as the skills.sh CLI (Vercel's `skills`), which is what `package.json` in Workstation's `packages/workbench/guest/skills/` pins.

After the proposal:

> Please walk me through questions to help decide.
> Also, I think these details should be recorded in agents wiki
> Update your plan and share it as a markdown doc that can kick off a session inside wb guest

## What was read

Results this session ran, at these commits: this repository `f2e2bde` (`main`), luckgrid/site-system `160d188` (`main`), luckgrid/workstation `d177fe0` (`main`), and the heads of Workstation's open PRs #39, #43 and #44.

| Where | What it holds |
| --- | --- |
| This repository | Nine skills: `wb-guest-rules` (51 lines) and `wb-herdr` (53), Luckgrid's own; `herdr`, copied from herdr 0.9.5 (214 lines); six linked through three submodules and `upstream.toml`. About half of `scripts/check.sh` checks the links and submodules |
| site-system | M7, done: a 71-line router `SKILL.md`, seven references and `CREDITS.md` in `.agents/skills/site-system/`, with `.claude/skills/site-system` linking to it; the owner saw both Claude Code and Codex list it ([record](../2026-11-12-site-system-skill.md)) |
| Workstation | The `workbench` skill decided (owner, 2026-11-13), not built: its planned path is `packages/workbench/guest/skills/workbench/`, its modes the guest rules, herdr in the guest, hand-off, plan, orchestrate, code, write and issue-to-release. `roles.toml` names the old routines (`wb-code`, `wb-plan`, `wb-orchestrate`) and a `skill_prefix` per harness (`$` for Codex, `/` for Claude Code and Cursor) |
| U16 in `lg-guest-setup` | Installs nothing until `agent_skills_commit` and `skills_cli_version` are pinned. About 200 lines guard against the skills CLI: it replaces a same-named folder without asking, its `remove -g` deletes names it didn't install, it keeps its own lock file, and it needs both telemetry switches. `guest/skills/package.json` and its lockfile are not in `guest.sha256`'s file list, so they have never been sent to the guest |
| The catalog | The skills CLI's row is **proposed** (WC11, F1), never admitted |
| Workstation #43 (open) | The owner's 2026-11-14 checks. Proposes, for later: release notes from a template, "`manage-workspace` skill in luckgrid/agents"; for `wb-guest-rules`, one retry of a failed read-only look-up; for Codex, one approval pattern for `git -c core.hooksPath=/dev/null add\|commit\|switch` |
| Workstation #39 (open) | Harness integration direction: explicit skill and mode invocation, deterministic checks and receipts, interfaces derived from concrete work |
| Workstation #44 (open) | Changes `lg-guest-setup`, `guest.sha256`, `workbench-tools.toml` and `simulate-guest-setup`, the files the installer change also touches |

## The owner's choices (2026-11-14)

Asked as multiple-choice questions, recommended option first. The owner chose the recommended option each time. The questions, shortened, and the answers word for word:

1. **The copied `herdr` skill:** drop it in M0, or keep it until M3? "Drop in M0 (Recommended)".
2. **How U16 installs once the skills CLI is gone:** "wb setup copies (Recommended)": GW-ADR-0014 decision 2 amended so `wb setup` copies skill folders itself from two sources (this repository's `git archive` export at the pinned commit, and Workbench's own `guest/skills/`), with no installer CLI and no npm.
3. **What the public README tells people outside the guest:** "Copy or any installer (Recommended)": copy a `skills/<name>` folder into the harness's skills folder, or use any Agent Skills installer; none is tested here.
4. **The name of #43's module:** "workspace (Recommended)", with modes issue, pr and release.
5. **When `workspace` is built:** "After M4 wiki (Recommended)", numbered M10.
6. **Conventions for every tier** (multiple answers): "Invocation: <prefix><skill> <mode> (Recommended)", "Shared check.sh (Recommended)" and "Rules only tighten downward (Recommended)".

## The plan, as proposed and accepted

A proposal of this session's, accepted through the answers above. The rules taken from it are in the [fifth round](../../program.md#fifth-round-owner-2026-11-14).

1. **M0, in this repository:** remove `upstream/`, `.gitmodules`, `upstream.toml`, the six links, the copied `herdr` and its `UPSTREAM.md`, the `upstream/` exclusion in `.rumdl.toml`, and the README's sections on submodules and the skills CLI. `check.sh` checks modules only and takes a folder argument. `wb-guest-rules` and `wb-herdr` get a `CREDITS.md` and stay until `workbench` is installed.
2. **Its companion in Workstation, from the agents session** (the owner's rule of 2026-11-13: changes in other repositories that follow from the agents monorepo are tracked here): delete `guest/skills/package.json` and its lockfile; U16 copies skill folders from its two sources; drop `npm ci`, the telemetry switches, the CLI's lock file and `skills_cli_version`; new simulations; GW-ADR-0014 decisions 1 and 2 amended; the catalog's row for the CLI withdrawn. After #44 merges, since both change the same files.
3. **`workbench`**, in a Workstation session: takes `wb-guest-rules`, `wb-herdr`, #43's two rule proposals and the issue-to-release draft; `roles.toml` gets a `mode` key. Once it is installed, the two `wb-*` skills leave this repository.
4. **M2 `council`, then the first pin:** this repository's first `agent_skills_commit`, U16's first install, and Workbench's cell S1.
5. **AG1** alongside M3 `multiplex`, M4 `wiki`, M10 `workspace`, M5 `grill`, M6 `design-system` (site-system's design-system reference then becomes a pointer), M8 `frugal` and M9 `triage`.

## Checks

rumdl 0.3.2 (`rumdl check wiki/`) on the changed pages; relative links and `related:` IDs checked by hand. No skill or script changed.
