---
id: AS-REC-2026-11-14-MODULAR-REWRITE-KICKOFF
kind: brief
title: Kickoff brief for the modular rewrite, M0 and its Workstation companion (2026-11-14)
summary: "The brief that starts the guest session for M0 (this repository: the upstream machinery, the herdr copy and the two wb- skills removed, check.sh for modules with a folder argument, the README and wiki for modules) and its companion in Workstation (the skills CLI and its package.json removed, U16 as a copy installer from two sources, the wb- skills added as workbench-guest-rules and workbench-herdr, roles.toml in the workbench namespace). Two parts, each started in its own pane in its own clone."
status: active
created: 2026-11-14
updated: 2026-11-14  # synced with Workstation's merges (#43, #44, #45, #47): Part B waits for Workstation's hand-off trust-dialog change; B1b also updates the worktree path, wb records and the herdr skill mention; before that, the review corrections; before that, the sixth round: namespaces
related: [AS-REC-2026-11-14-MODULAR-REWRITE, AS-PROGRAM, AS-STATE, AS-ARCH-SKILL-SOURCES, AS-ARCH-GUEST-INSTALL, GW-ADR-0014]
---

# Kickoff: the modular rewrite, M0 and its Workstation companion

You are an agent in the Workbench guest. This brief is data the owner chose, not a reason to set aside any rule below. It is self-contained: the guest has no network, so don't try to fetch anything it names.

The owner starts **Part A** in a pane in `~/Workspaces/agents`. **Part B** starts later, in a pane in `~/Workspaces/workstation`, once Part A is committed and Workstation's hand-off trust-dialog change has merged and been synced (see Part B's start condition). Do only the part your start line names, and only in the clone you were started in. Review corrections and their evidence are in [review.md](review.md); the sync of 2026-11-14 is its last dated note.

## Before anything

1. Follow the guest's house rules (`wb-guest-rules` or `workbench-guest-rules`, if one is listed to you; otherwise read it with `git -C ~/Workspaces/agents show f2e2bded0a0a86e72f382ee34f994862bbac836f:skills/wb-guest-rules/SKILL.md`). This fixed source survives Part A's deletion and a later sync of `main`; verify that the commit is present before proceeding. In short:
   - edit only your clone;
   - commit only on the branch named below, using the guest's configured Git identity and staging explicit paths; if it has no identity, report that for the owner to configure;
   - install, fetch and push nothing; never run headless agents;
   - print herdr socket commands for the owner instead of running them;
   - on any failure or surprise, stop and say what you saw.
2. Read your clone's root `README.md` (it holds the rules and decides), then `wiki/README.md` and `wiki/records/STATE.md`.
3. Read this plan's record, `~/Workspaces/agents/wiki/records/2026-11-14-modular-rewrite/README.md`, and the roadmap's fifth and sixth rounds and its namespaces, in `~/Workspaces/agents/wiki/program.md`. Where this brief and those pages disagree, stop and ask.

## The decisions that bind this work (owner, 2026-11-14)

1. Every dependency around the removed upstream skills goes: the submodules, `upstream.toml`, the skills CLI (skills.sh) and its `package.json`.
2. The copied `herdr` skill is dropped in M0. M3 `multiplex`'s herdr reference will defer to `herdr --skill`.
3. `wb setup` (U16) copies skill folders itself from two sources: this repository's `git archive` export at the pinned commit, and Workbench's own `packages/workbench/guest/skills/`. There is no installer CLI and no npm.
4. The public README tells people to copy a `skills/<name>` folder into their harness's skills folder, or use any Agent Skills installer, none of them tested here.
5. A new module `workspace` (M10: modes issue, pr, release) is built after M4. It is not built in this session.
6. The conventions:
   - **invocation:** `<prefix><skill> [<mode>] [args]`. A use invoked explicitly is its own namespaced skill (`$workbench-code <brief>`); a smaller use is a mode of its base skill, where the first column of the mode table is the literal mode word in backticks, and that word is the reference's file name without `.md`;
   - **a shared `check.sh`:** it takes a folder argument and the namespaces its skills may use;
   - **rules only tighten downward:** public method, guest restrictions, project restrictions. A project may narrow a guest or public workflow, never relax the guest boundary. Skills do not override harness instructions, permissions or sandbox enforcement. Modules refer to each other by skill and mode name, each such reference with a one-line fallback for when that skill isn't installed; a fallback must stop if a required rule or prerequisite is unavailable.
7. **Namespaces** (sixth round): every skill, in every repository, is named `<namespace>` or `<namespace>-<name>`, in full words, never a shorthand like `wb-`. A namespace's base skill carries the name alone; uses that are invoked explicitly or load on their own description are `<namespace>-<name>` skills in flat folders beside it. Each namespace is owned by one repository, which alone holds its skills: `council`, `multiplex`, `wiki`, `workspace`, `grill`, `design-system`, `frugal` and `triage` here; `workbench` in Workstation; `site-system` in site-system. Commands are not skills: `wb`, `wb-flow`, `wb-layout` and the other `wb-` commands keep their names.

## Part A: M0 in luckgrid/agents

**Branch:** `skills/m0-module-foundation`, from `main`. **Clone:** `~/Workspaces/agents`.

### A1. Remove the upstream machinery

`git rm` these:

- `upstream/` and `.gitmodules`;
- the six links in `skills/`: `verification-before-completion`, `receiving-code-review`, `systematic-debugging`, `differential-review`, `grill-me`, `grilling`;
- `upstream.toml`;
- `skills/herdr/`, with its `UPSTREAM.md` and `LICENSE`.

Remove the `upstream/` exclusion from `.rumdl.toml`. Commit this on its own.

Before deleting the two skills, verify that `git cat-file -e f2e2bded0a0a86e72f382ee34f994862bbac836f^{commit}` succeeds and that both skills can be read at that commit. Part B uses that immutable source, not a moving branch. Then, in a second commit, `git rm -r skills/wb-guest-rules skills/wb-herdr`: they move to Workstation as `workbench-guest-rules` and `workbench-herdr` in part B (decision 7). This repository then holds no skills until M2. Do not claim the Workstation move is complete until Part B lands.

### A2. `scripts/check.sh` for modules

Rewrite it. It needs only `python3` (3.9 or later, run with `-I`) and no `git`. Its argument is a skills folder, `skills/` by default, resolved against the repository root; use an absolute path for another repository. For each immediate child folder it requires `SKILL.md` and checks the whole folder, rather than discovering only folders that already contain that file:

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
- **Namespaces:** with `--namespaces a,b,…`, every `name` is one of them or starts with one followed by `-`. For the default folder the list is this repository's namespaces (decision 7), kept in one constant that names the roadmap's registry as its source.
- **No skills** is not a failure: an empty folder gives `ok: 0 skills checked`. Git does not retain empty folders, so the absent default `skills/` after M0 also passes. A missing explicitly supplied folder fails, so a mistyped external path cannot pass as an empty collection.

Keep the output style: `FAIL …` lines, then `ok: N skills checked`. Run it on `skills/` (it must pass, with no skills) and on site-system's `.agents/skills` with `--namespaces site-system`. Record both outputs word for word. site-system's router doesn't yet use the mode-word table, so its run is expected to fail on that check alone. The fix belongs to a site-system session; don't touch site-system.

### A3. A module fixture for the check

With no skills left, `check.sh` would prove nothing. Add a small fixture under `scripts/fixtures/` (not under `skills/`, so nothing installs it): one valid skill with a base mode table and one reference, and one invalid copy per failure the check knows, each named for what it breaks. Teach `check.sh` a `--self-test` that runs on the fixtures and expects the valid one to pass and each invalid one to fail with its own `FAIL` line. Record its output word for word.

### A4. The README

- **Skills:** none on `main` until M2; a line pointing to the roadmap for the modules and its namespaces registry, and that Workbench's own skills (`workbench-*`) are in Workstation.
- **Replace "Linked upstream skills", "How the guest installs it" and "Adding a skill"** with:
  - **Writing a module:** the module shape (router `SKILL.md`, `references/`, `CREDITS.md`), own words only, ideas credited, the namespaces rule (decision 7), the invocation convention, the three tiers and the tighten-downward rule;
  - **Using these skills:** decision 4's wording, and that the Workbench guest installs them through `wb setup`, whose page decides.
- **Licence:** say that there are no third-party files.
- **Keep** the no-secrets rule and the Workstation note.

### A5. The wiki

- **`architecture/skill-sources.md`:** keep its ID, `AS-ARCH-SKILL-SOURCES`. Retitle it "Skill modules" and rewrite it:
  - the module shape;
  - the three tiers (public modules here, the `workbench` namespace in Workstation, project skills in their repositories) with where each is installed;
  - the namespaces rule, pointing to the roadmap's registry rather than copying it;
  - the composition rules;
  - what `check.sh` proves and what it doesn't.
- **`architecture/guest-install.md`:**
  - the copy installer as decided, with Workstation's daily-use page deciding the details;
  - drop the skills CLI's test results and the F1 submodule changes, leaving a line that the earlier records keep them.
- **`resources/skill-references.md`:** add `herdr` (herdrdev/herdr, Apache-2.0, 0.9.5) to "Removed in M0", feeding `multiplex`; note under `workbench` that `wb-guest-rules` and `wb-herdr` moved to Workstation as `workbench-guest-rules` and `workbench-herdr`.
- **`program.md`:** M0's done-when met, and P2 (WB-06) closed. Don't restate status; that lives in STATE.
- **`records/STATE.md`:**
  - the skills on this branch (none); keep the current `main` baseline until merge, and track the two skills as removed here with their Workstation move pending Part B;
  - `check.sh`'s new output;
  - M0 done on this branch, pending review and merge;
  - WB-06 closed;
  - the decisions row.
- **A new record** `records/<today>-m0.md`: what was removed and why, `check.sh`'s outputs word for word, the site-system run, and anything that surprised you. Add it to `records/README.md`.
- **Check** with `rumdl check wiki/` and `rumdl fmt --check wiki/` if `rumdl` is installed. If it isn't, say so; don't install it. Also check each relative link and `related:` ID you touched.

### A6. Finish

- One commit per step A1 to A5, each message saying what and why.
- Run `scripts/check.sh` and `scripts/check.sh --self-test` last and read their output.
- Write a short report to `~/.workers/records/m0-report.md`: the branch, its head, each commit, both `check.sh` outputs, what you didn't do and why, and questions for the owner.
- Then print, for the owner to run on the Mac: `wb promote agents skills/m0-module-foundation` and `wb pr agents skills/m0-module-foundation`.
- An independent review by a different model follows. The owner starts it; don't start or prompt another agent.

**Done when:** `scripts/check.sh` and its self-test pass in a clone with no submodules and no `upstream/`; no skill and no `wb-` skill name is left in this repository outside records and dated notes; every removed skill is in the skill references; the README has no submodule or skills CLI instructions; the record and STATE say what was done.

## Part B: U16 as a copy installer in luckgrid/workstation

**Start only when** the owner's start line says both of these are true:

- Part A is committed in `~/Workspaces/agents` (B1b runs its validator);
- Workstation's branch `workbench/handoff-trust-dialog` is merged and `wb sync workstation` has run.

Workstation #43 and #44 merged on 2026-11-14, and #45 and #47 (docs only) after them. The hand-off branch, started from a separate kickoff, changes files this part also changes: `guest/wb-flow`'s tests in `simulate-wb-flow`, `wiki/workbench/agent-roles.md`, `daily-use.md`, `guest.sha256` and `wb`'s pin. If `git log origin/main` shows neither #44 (`wb done, wb install`) nor the hand-off change (`hand-off waits for a trust dialog`), stop.

**Branch:** `workbench/u16-copy-installer`, from `main`. **Clone:** `~/Workspaces/workstation`. Read `packages/workbench/README.md` for how guest files, `guest.sha256` and `wb`'s pin change (`./update-manifest`).

### B1. Remove the CLI's package

`git rm packages/workbench/guest/skills/package.json packages/workbench/guest/skills/package-lock.json`. They were never in `guest.sha256`'s file list, so the guest never received them. `guest/skills/` becomes the home of the `workbench` namespace: the base `workbench` and the role skills are written later in a Workstation session; this part adds the two below.

### B1b. The two guest skills, renamed

Read them from agents commit `f2e2bded0a0a86e72f382ee34f994862bbac836f` (`git -C ~/Workspaces/agents show f2e2bded0a0a86e72f382ee34f994862bbac836f:skills/wb-guest-rules/SKILL.md`, and the same for `wb-herdr`), and write them to `packages/workbench/guest/skills/workbench-guest-rules/SKILL.md` and `packages/workbench/guest/skills/workbench-herdr/SKILL.md`. If that commit is absent, stop for the owner to sync it; do not substitute `main`. Change only:

- `name:` to the new folder name, and each mention of the other skill (`wb-herdr` becomes `workbench-herdr`, `wb-guest-rules` becomes `workbench-guest-rules`), descriptions included;
- the notes folder `~/.workers/notes/`, which is now `~/.workers/records/`, and `wb notes`, now `wb records` (its old name still works);
- the worktree folder `~/.workers/worktrees/<repo>/`, which is now `~/.workers/spaces/<repo>/` (the cheatsheet's folder table);
- `wb-herdr`'s mention of "the herdr … skills": the copied `herdr` skill is dropped in M0, so it names herdr's own skill, printed by `herdr --skill` for the installed version.

Confirm each new path or name in `wiki/workbench/daily-use.md` or `wiki/workbench/cheatsheet.md` first; where neither page confirms it, leave it and report it.

The `wb` and `wb-flow` commands they name are commands, not skills, and keep their names. Add a `CREDITS.md` to each: Luckgrid's own, moved from luckgrid/agents at the full source commit above, with the agents records they came from (`2026-11-10-first-skills`). Run the committed Part A validator read-only: `~/Workspaces/agents/scripts/check.sh ~/Workspaces/workstation/packages/workbench/guest/skills --namespaces workbench`. Confirm the agents checkout contains that validator; otherwise report the prerequisite as unmet.

Update `guest/wb-agent-hook`'s session-start context to name `workbench-guest-rules`, and its simulations and current documentation to match. A skill description alone does not guarantee loading: retain the explicit house-rules instruction at session start. M1 must prove that independently invoked `workbench-*` skills load the base rules, and cell S1 must check discovery and invocation in each target harness. A required rule that cannot be loaded stops the workflow.

### B1c. `roles.toml` in the `workbench` namespace

Rename its skill names: `wb-orchestrate` to `workbench-orchestrate`, `wb-plan` to `workbench-plan`, `wb-code` to `workbench-code`, and its comment, which says the routines become modes of one skill, to say they become `workbench-*` skills beside the base `workbench` (owner, 2026-11-14). Update `simulate-wb-flow`'s expected start lines to match (`$workbench-code …`, `/workbench-code …`), and the skill names in `wiki/workbench/agent-roles.md` (`wb-write`, `wb-handoff`, `wb-review` and `wb-remediate` included: the first two become `workbench-write` and `workbench-handoff`, the last two stay planned in `council`). The skills themselves are written in the Workstation session.

### B2. `update-manifest`

Its file list is fixed. Add every regular file under `guest/skills/`, sorted, so a skill written there is sent and hashed with the other guest files. Reject symlinks and special files instead of silently skipping them. The two skills and their credits added in B1b must appear in the resulting manifest. Run it as `packages/workbench/update-manifest` from the repository root, or `./update-manifest` from that package's directory.

### B3. U16 in `guest/lg-guest-setup`

Rewrite the step.

**Sources:**

- **A:** the `git archive` export of `~/Workspaces/agents` at `agent_skills_commit`, as today, extracted with Python's tar `data` filter into `~/.local/share/lg-workbench/agent-skills/<commit>`.
- **B:** the payload's `skills/` folder.

With no pin, source A is skipped. A missing `skills/` in an otherwise valid pinned M0 export means an empty collection, since Git does not retain empty directories. A malformed pin or failed export is an error, never an empty source. With neither source holding a skill, still reconcile any previously owned skills as drops before saying `MEASURE U16 … nothing installed`.

**Each skill folder** `<source>/skills/<name>/` (A) or `skills/<name>/` (B):

- `SKILL.md` exists, with front matter whose `name` equals the folder name and matches `^[a-z0-9]+(-[a-z0-9]+)*$`, at most 64 characters;
- regular files and folders only, with no symlinks;
- a name in both sources stops the step before any change.

**Installing:**

- **Conflicts:** `skills.owned` alone establishes ownership, without the CLI lock file. Validate its names and digests; malformed state stops the step. Preflight both complete sources and all destinations, including dropped skills, before changing any installed skill. A foreign or edited folder or link stops the step. Reject symlinked destination parents so installation cannot be redirected.
- **Copying:** stage each skill beside `~/.agents/skills/<name>`. An ordinary rename cannot replace a nonempty directory: move an owned destination to a backup, rename the stage into place, and retain enough state to restore it on failure or recover after interruption. Write `~/.claude/skills/<name>` as the relative link `../../.agents/skills/<name>`. Serialize U16 runs and do not advance the success receipt until folders, links, drops and ownership agree; do not claim atomic replacement of the whole set.
- **Recording:** each folder's digest goes into `skills.owned`.
- **Dropped skills:** a name `skills.owned` lists that neither source has any more is removed from those two places, only if it still matches.
- **The state file:** it records the installer format version, the agents commit (or no pin), and a digest of source B. Before saying `PASS U16 shared skills already installed …`, revalidate installed folder digests, expected links and ownership. A matching source receipt alone is insufficient; an edited skill must fail even on an unchanged rerun.
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

Also cover a pinned M0 export with no `skills/`, a malformed child without `SKILL.md`, malformed ownership state, a symlinked destination parent, a foreign conflict in the last planned skill (earlier destinations stay unchanged), an edit after a successful install with the same receipt, an empty desired set removing owned skills, and interruption between backup, replacement and receipt followed by recovery. Exercise the renamed session-start context and manifest completeness too.

Every guard in B3 has a mutation that the simulation catches.

### B5. Docs and records

- **GW-ADR-0014** (proposed): dated notes with the owner's words of 2026-11-14 (in this plan's record):
  - decision 1 has two sources;
  - decision 2 becomes "no installer CLI: `wb setup` copies the skill folders itself", with the conditions that still hold (local only, outside every agent's sandbox, from an export of a pinned commit).
  - decision 1's note of 2026-11-13 (one `workbench` skill with modes) gets a dated note: the `workbench` namespace, a base skill plus `workbench-*` skills, `workbench-guest-rules` and `workbench-herdr` among them (owner, 2026-11-14, sixth round).
- **The agent package catalog:** the skills CLI's row becomes **withdrawn (owner, 2026-11-14)**.
- **Pages to update:**
  - `wiki/workbench/daily-use.md`: U16's steps;
  - `wiki/workbench/agent-workflows.md`: the installer bullet and F1's row;
  - `wiki/workbench/devvm-reference.md`: the skills row.
- **A Workstation record** of this change, with each check's output word for word, and a STATE line.
- **This repository's [guest install](../../architecture/guest-install.md):** don't edit it from here. Note in the report that it must match.

### B6. Finish

- From the repository root, run `packages/workbench/update-manifest`, then `scripts/check.sh` and all five Workbench simulations, and read their output. Follow the root README for Moon tasks and the package README for any sandbox limits; record owner-run checks as pending until their output is received.
- One commit per step.
- Write a report to `~/.workers/records/u16-copy-installer-report.md`, like Part A's.
- Print `wb promote workstation workbench/u16-copy-installer` and `wb pr workstation workbench/u16-copy-installer` for the owner.

**Done when:** `scripts/check.sh` and the five simulations pass; no file names the skills CLI, and no skill is named with `wb-`, except in records and dated notes; `workbench-guest-rules` and `workbench-herdr` are in the manifest; U16 installs from both sources in the simulation and refuses each hostile case.

## Stop and ask

Stop, keep what you saw word for word, and ask the owner (with a multiple-choice question where your harness has one, recommended option first) when:

- a source and this brief disagree;
- a check fails for a reason you can't trace to your change;
- a step needs a credential, the network or an install;
- a change would make a decision nobody has recorded.

A proposal is not acceptance.
