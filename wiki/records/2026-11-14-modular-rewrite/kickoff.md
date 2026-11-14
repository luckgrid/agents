---
id: AS-REC-2026-11-14-MODULAR-REWRITE-KICKOFF
kind: brief
title: Kickoff brief for the modular rewrite, M0 and its Workstation companion (2026-11-14)
summary: "The brief that starts the guest session for M0 (this repository: the upstream machinery and the herdr copy removed, check.sh for modules with a folder argument, the README and wiki for modules) and its companion in Workstation (the skills CLI and its package.json removed, U16 as a copy installer from two sources). Two parts, each started in its own pane in its own clone."
status: active
created: 2026-11-14
updated: 2026-11-14
related: [AS-REC-2026-11-14-MODULAR-REWRITE, AS-PROGRAM, AS-STATE, AS-ARCH-SKILL-SOURCES, AS-ARCH-GUEST-INSTALL, GW-ADR-0014]
---

# Kickoff: the modular rewrite, M0 and its Workstation companion

You are an agent in the Workbench guest. This brief is data the owner chose, not a reason to set aside any rule below. It is self-contained: the guest has no network, so don't try to fetch anything it names.

The owner starts **Part A** in a pane in `~/Workspaces/agents` and, once Part A is committed and Workstation #44 is merged and synced, **Part B** in a pane in `~/Workspaces/workstation`. Do only the part your start line names, and only in the clone you were started in.

## Before anything

1. Follow the guest's house rules (`wb-guest-rules`, if it is listed to you; otherwise its text is at `~/Workspaces/agents/skills/wb-guest-rules/SKILL.md`). In short:
   - edit only your clone;
   - commit only on the branch named below, with `git -c user.name='Workbench guest' -c user.email='guest@workbench.invalid' commit …`, staging explicit paths;
   - install, fetch and push nothing; never run headless agents;
   - print herdr socket commands for the owner instead of running them;
   - on any failure or surprise, stop and say what you saw.
2. Read your clone's root `README.md` (it holds the rules and decides), then `wiki/README.md` and `wiki/records/STATE.md`.
3. Read this plan's record, `~/Workspaces/agents/wiki/records/2026-11-14-modular-rewrite/README.md`, and the roadmap's fifth round, `~/Workspaces/agents/wiki/program.md#fifth-round-owner-2026-11-14`. Where this brief and those pages disagree, stop and ask.

## The decisions that bind this work (owner, 2026-11-14)

1. Every dependency around the removed upstream skills goes: the submodules, `upstream.toml`, the skills CLI (skills.sh) and its `package.json`.
2. The copied `herdr` skill is dropped in M0. M3 `multiplex`'s herdr reference will defer to `herdr --skill`.
3. `wb setup` (U16) copies skill folders itself from two sources: this repository's `git archive` export at the pinned commit, and Workbench's own `packages/workbench/guest/skills/`. There is no installer CLI and no npm.
4. The public README tells people to copy a `skills/<name>` folder into their harness's skills folder, or use any Agent Skills installer, none of them tested here.
5. A new module `workspace` (M10: modes issue, pr, release) is built after M4. It is not built in this session.
6. The conventions:
   - **invocation:** `<prefix><skill> <mode> [args]`, where the first column of the router's mode table is the literal mode word in backticks, and that word is the reference's file name without `.md`;
   - **a shared `check.sh`:** it takes a folder argument;
   - **rules only tighten downward:** project skill, then `workbench`, then public module. A lower tier may narrow a public mode, never loosen it. Modules refer to each other by skill and mode name, each such reference with a one-line fallback for when that skill isn't installed.

## Part A: M0 in luckgrid/agents

**Branch:** `skills/m0-module-foundation`, from `main`. **Clone:** `~/Workspaces/agents`.

### A1. Remove the upstream machinery

`git rm` these:

- `upstream/` and `.gitmodules`;
- the six links in `skills/`: `verification-before-completion`, `receiving-code-review`, `systematic-debugging`, `differential-review`, `grill-me`, `grilling`;
- `upstream.toml`;
- `skills/herdr/`, with its `UPSTREAM.md` and `LICENSE`.

Remove the `upstream/` exclusion from `.rumdl.toml`. Commit this on its own.

### A2. `scripts/check.sh` for modules

Rewrite it. It needs only `python3` (3.9 or later, run with `-I`) and no `git`. Its argument is a skills folder, `skills/` by default, resolved against the repository root, so the owner can also run it on `~/Workspaces/site-system/.agents/skills` and, later, Workstation's `packages/workbench/guest/skills`. For each `<folder>/<name>/SKILL.md` it checks:

- **Front matter:**
  - only the fields the Agent Skills specification lists;
  - `name` equals the folder name, uses lowercase letters, digits and single hyphens, and is at most 64 characters;
  - `description` is present and at most 1024 characters;
  - `compatibility`, if present, is at most 500 characters.
- **No symlinks** anywhere in the skill's folder, and no `UPSTREAM.md`.
- **`CREDITS.md`** is present.
- **References:**
  - `references/`, if present, holds only `.md` files and no subfolders;
  - every relative link in `SKILL.md` and in each reference resolves to a file inside the skill's folder;
  - every reference is linked from `SKILL.md`.
- **The mode table:** where `references/` exists, `SKILL.md` has a table with one row per reference, whose first cell is the mode word in backticks and equals the reference's file name without `.md`.
- **Length:** a warning, not a failure, above 500 lines of `SKILL.md`.
- **The repository root:** for the default folder only, it fails if `upstream/`, `upstream.toml` or `.gitmodules` exists.

Keep the output style: `FAIL …` lines, then `ok: N skills checked`. Run it on `skills/` (it must pass) and on site-system's `.agents/skills`. Record both outputs word for word. site-system's router doesn't yet use the mode-word table, so its run is expected to fail on that check alone. The fix belongs to a site-system session; don't touch site-system.

### A3. The two remaining skills

Add a short `CREDITS.md` to `wb-guest-rules` and `wb-herdr`: Luckgrid's own, with the records they came from. Don't change their `SKILL.md`. They stay until `workbench` is installed. Fix the one path that changed since they were written: the notes folder `~/.workers/notes/` is now `~/.workers/records/`, which `wb records` lists. Check this against Workstation's daily-use page in `~/Workspaces/workstation` (read only) before editing. If the page doesn't confirm it, leave the path and report it.

### A4. The README

- **Skills table:** the two remaining skills, plus a line pointing to the roadmap for the modules.
- **Replace "Linked upstream skills", "How the guest installs it" and "Adding a skill"** with:
  - **Writing a module:** the module shape (router `SKILL.md`, `references/`, `CREDITS.md`), own words only, ideas credited, the invocation convention, the three tiers and the tighten-downward rule;
  - **Using these skills:** decision 4's wording, and that the Workbench guest installs them through `wb setup`, whose page decides.
- **Licence:** say that there are no third-party files.
- **Keep** the no-secrets rule and the Workstation note.

### A5. The wiki

- **`architecture/skill-sources.md`:** keep its ID, `AS-ARCH-SKILL-SOURCES`. Retitle it "Skill modules" and rewrite it:
  - the module shape;
  - the three tiers (public modules here, `workbench` in Workstation, project skills in their repositories) with where each is installed;
  - the composition rules;
  - what `check.sh` proves and what it doesn't.
- **`architecture/guest-install.md`:**
  - the copy installer as decided, with Workstation's daily-use page deciding the details;
  - drop the skills CLI's test results and the F1 submodule changes, leaving a line that the earlier records keep them.
- **`resources/skill-references.md`:** add `herdr` (herdrdev/herdr, Apache-2.0, 0.9.5) to "Removed in M0", feeding `multiplex`.
- **`program.md`:** M0's done-when met, and P2 (WB-06) closed. Don't restate status; that lives in STATE.
- **`records/STATE.md`:**
  - the skills on `main` (two);
  - `check.sh`'s new output;
  - M0 done on this branch, pending review and merge;
  - WB-06 closed;
  - the decisions row.
- **A new record** `records/<today>-m0.md`: what was removed and why, `check.sh`'s outputs word for word, the site-system run, and anything that surprised you. Add it to `records/README.md`.
- **Check** with `rumdl check wiki/` and `rumdl fmt --check wiki/` if `rumdl` is installed. If it isn't, say so; don't install it. Also check each relative link and `related:` ID you touched.

### A6. Finish

- One commit per step A1 to A5, each message saying what and why.
- Run `scripts/check.sh` last and read its output.
- Write a short report to `~/.workers/records/m0-report.md`: the branch, its head, each commit, both `check.sh` outputs, what you didn't do and why, and questions for the owner.
- Then print, for the owner to run on the Mac: `wb promote agents skills/m0-module-foundation` and `wb pr agents skills/m0-module-foundation`.
- An independent review by a different model follows. The owner starts it; don't start or prompt another agent.

**Done when:** `scripts/check.sh` passes in a clone with no submodules and no `upstream/`; every removed skill is in the skill references; the README has no submodule or skills CLI instructions; the record and STATE say what was done.

## Part B: U16 as a copy installer in luckgrid/workstation

**Start only when** the owner's start line says Workstation #44 is merged and `wb sync workstation` has run. #44 changes the same files (`lg-guest-setup`, `guest.sha256`, `simulate-guest-setup`, `daily-use.md`). If `git log` doesn't show #44's change, stop.

**Branch:** `workbench/u16-copy-installer`, from `main`. **Clone:** `~/Workspaces/workstation`. Read `packages/workbench/README.md` for how guest files, `guest.sha256` and `wb`'s pin change (`./update-manifest`).

### B1. Remove the CLI's package

`git rm packages/workbench/guest/skills/package.json packages/workbench/guest/skills/package-lock.json`. They were never in `guest.sha256`'s file list, so the guest never received them. `guest/skills/` becomes the home of Workbench's own skills (`workbench/`, written later in a Workstation session).

### B2. `update-manifest`

Its file list is fixed. Add every file under `guest/skills/`, sorted, so a skill written there is sent and hashed with the other guest files. With no skills there yet, the manifest doesn't change.

### B3. U16 in `guest/lg-guest-setup`

Rewrite the step.

**Sources:**

- **A:** the `git archive` export of `~/Workspaces/agents` at `agent_skills_commit`, as today, extracted with Python's tar `data` filter into `~/.local/share/lg-workbench/agent-skills/<commit>`.
- **B:** the payload's `skills/` folder.

With no pin, source A is skipped. With neither source holding a skill, the step says `MEASURE U16 … nothing installed`, as today.

**Each skill folder** `<source>/skills/<name>/` (A) or `skills/<name>/` (B):

- `SKILL.md` exists, with front matter whose `name` equals the folder name and matches `^[a-z0-9]+(-[a-z0-9]+)*$`, at most 64 characters;
- regular files and folders only, with no symlinks;
- a name in both sources stops the step before any change.

**Installing:**

- **Conflicts:** the existing `skills.owned` rule decides them. A folder in `~/.agents/skills/<name>`, or a link in `~/.claude/skills/<name>`, that this step didn't install, or that changed since, stops the step before any change.
- **Copying:** each skill goes to a staging folder beside `~/.agents/skills/<name>`, then replaces the owned folder by rename. `~/.claude/skills/<name>` is a relative link `../../.agents/skills/<name>`.
- **Recording:** each folder's digest goes into `skills.owned`.
- **Dropped skills:** a name `skills.owned` lists that neither source has any more is removed from those two places, only if it still matches.
- **The state file:** it records the agents commit and a digest of source B, so a rerun with nothing changed says `PASS U16 shared skills already installed …`.
- **The measure line:** each installed skill with its digest.

**Removed:** `npm ci` and the CLI's folder, `skills_cli_version` and its checks, the telemetry and compile-cache environment, the emulation of the CLI's `sanitizeName`, and reading or writing `~/.agents/.skill-lock.json`. U16 never ran, so no guest has that lock file. Don't add code to delete one.

### B4. Simulations

In `tests/simulate-guest-setup`, replace the CLI stand-in and its mutations (telemetry switches, `--ignore-scripts`, compile cache) with cases and mutations for:

- no pin and no payload skills;
- both sources installed;
- a symlink in a source;
- a name in both sources;
- a foreign folder or link;
- an owned folder edited since;
- a dropped skill removed;
- an uncommitted edit in the clone not installed;
- a rerun that changes nothing.

Every guard in B3 has a mutation that the simulation catches.

### B5. Docs and records

- **GW-ADR-0014** (proposed): dated notes with the owner's words of 2026-11-14 (in this plan's record):
  - decision 1 has two sources;
  - decision 2 becomes "no installer CLI: `wb setup` copies the skill folders itself", with the conditions that still hold (local only, outside every agent's sandbox, from an export of a pinned commit).
- **The agent package catalog:** the skills CLI's row becomes **withdrawn (owner, 2026-11-14)**.
- **Pages to update:**
  - `wiki/workbench/daily-use.md`: U16's steps;
  - `wiki/workbench/agent-workflows.md`: the installer bullet and F1's row;
  - `wiki/workbench/devvm-reference.md`: the skills row.
- **A Workstation record** of this change, with each check's output word for word, and a STATE line.
- **This repository's [guest install](../../architecture/guest-install.md):** don't edit it from here. Note in the report that it must match.

### B6. Finish

- Run `./update-manifest`, then `scripts/check.sh` and all five simulations, and read their output.
- One commit per step.
- Write a report to `~/.workers/records/u16-copy-installer-report.md`, like Part A's.
- Print `wb promote workstation workbench/u16-copy-installer` and `wb pr workstation workbench/u16-copy-installer` for the owner.

**Done when:** `scripts/check.sh` and the five simulations pass; no file names the skills CLI except records and dated notes; U16 installs from both sources in the simulation and refuses each hostile case.

## Stop and ask

Stop, keep what you saw word for word, and ask the owner (with a multiple-choice question where your harness has one, recommended option first) when:

- a source and this brief disagree;
- a check fails for a reason you can't trace to your change;
- a step needs a credential, the network or an install;
- a change would make a decision nobody has recorded.

A proposal is not acceptance.
