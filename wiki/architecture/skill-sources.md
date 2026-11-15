---
id: AS-ARCH-SKILL-SOURCES
kind: architecture
title: Skill modules
summary: "What a skill module is (a router SKILL.md, flat references, CREDITS.md), the three tiers and where each is installed, the namespaces rule, how modules compose, and what scripts/check.sh proves and what it doesn't."
status: active
created: 2026-11-12
updated: 2026-11-15  # M0: rewritten for modules, retitled from "Skill sources"; before that, created
related: [AS-PROGRAM, AS-STATE, AS-ARCH-GUEST-INSTALL, AS-REC-2026-11-15-M0, AS-REC-2026-11-14-MODULAR-REWRITE, AS-REC-2026-11-10-LINKED-UPSTREAM-SKILLS, GW-ADR-0014]
derived_from: GW-ADR-0014
---

# Skill modules

Every skill here is a module written in Luckgrid's own words (owner, 2026-11-12). M0 removed the linked upstream skills, their submodules and `upstream.toml`, the copied `herdr` skill, and the two `wb-` skills, which move to Workstation ([record](../records/2026-11-15-m0.md)); the earlier kinds of skill (written here, copied, linked) are described in the records that added them, from [first skills](../records/2026-11-10-first-skills.md) to [linked upstream skills](../records/2026-11-10-linked-upstream-skills.md). The rules for writing a module are in the root README; this page says how the pieces fit and why. That the repository is one, public, and installed at a pinned commit comes from Groundwork's GW-ADR-0014 decision 1 (proposed).

## The module shape

A module follows the [Agent Skills format](https://agentskills.io/specification) and its progressive disclosure: the name and description load at startup, `SKILL.md` loads when the skill is activated, and files under `references/` load only when a task needs them.

```text
skills/<module>/
├── SKILL.md        the router: when to use it, its hard rules, a mode table naming the reference for each mode
├── references/     one Markdown file per mode or workflow, flat, loaded on demand
└── CREDITS.md      what the module learned from where: repository, commit, licence, ideas taken
```

- **`SKILL.md` is a router:** short, always correct, and enough on its own for the module's hard rules.
- **The mode table:** its first column is the literal mode word in backticks, and that word is the reference's file name without `.md`, so `<prefix><skill> <mode>` names a file.
- **`CREDITS.md` names ideas, never copied text.** Every source is read for ideas only, whatever its licence ([skill references](../resources/skill-references.md)); no third-party file is committed here.
- **Tools:** a module that needs one says so in `compatibility`, checks that it is installed, and never installs it.

## Three tiers

| Tier | Where it lives | Installed | Holds |
| --- | --- | --- | --- |
| Public modules | `skills/` here | in the Workbench guest by `wb setup` (U16), from an export of a pinned commit ([guest install](guest-install.md)); anyone else copies a folder | Methods that hold in any workspace |
| Workbench's skills | the `workbench` namespace in luckgrid/workstation (private for now), under `packages/workbench/guest/skills/` | by U16, from Workbench's own guest files | The guest's rules, `wb` and `wb-flow`, the roles |
| Project skills | `.agents/skills/<name>/` in their repository, with a `.claude/skills/<name>` link | read by the harness in that checkout; not installed | That repository's rails (`site-system`) |

Rules only tighten downward: public method, then guest restrictions, then project restrictions. A project may narrow a guest or public workflow, never relax the guest boundary, and no skill overrides harness instructions, permissions or sandbox enforcement. This is a rule for what modules say, not a claim that harnesses enforce a precedence order.

## Namespaces

Every skill, in every repository, is named `<namespace>` or `<namespace>-<name>`, in full words, never a shorthand like `wb-` (owner, 2026-11-14, sixth round). A namespace's base skill carries the name alone; a use that is invoked explicitly, or that must load on its own description, is a `<namespace>-<name>` skill in a flat folder beside it. Each namespace is owned by one repository, which alone holds its skills, so names stay unique across the tiers. The registry is the roadmap's [namespaces](../program.md#namespaces-owner-2026-11-14) table; this page doesn't copy it. `scripts/check.sh` keeps this repository's list in one constant that names the registry as its source.

## Composition

- **Name, don't path.** Modules refer to each other by skill and mode name, each such reference with a one-line fallback for when that skill isn't installed. If the missing skill supplies a required rule or prerequisite, the fallback stops the workflow.
- **The guest's authority is `workbench`.** A public module may name it, never restate what it holds.
- **The generic half is public.** Where a workflow has a part that holds anywhere and a part that holds only in the guest, the first is a public mode and the second a `workbench` skill or mode that points to it.
- **Loading is not assumed.** A namespaced skill that needs its base skill's rules loads them explicitly; installing a base skill doesn't make it load. Each harness's behaviour is proved at Workbench's cell S1.

## What `scripts/check.sh` proves

`scripts/check.sh [--namespaces a,b,…] [<skills folder>]` needs only `python3` (3.9 or later, run with `-I`) and changes nothing. The folder is `skills/` by default; a relative folder is resolved against the repository root, so another repository's is given as an absolute path. For each immediate child folder:

1. **Files:** `SKILL.md` and `CREDITS.md` are present; no symlink anywhere in the folder; no `UPSTREAM.md`; text is UTF-8.
2. **Front matter:** only the fields the specification lists (`name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools`), each once; `name` equals the folder name, uses lowercase letters, digits and single hyphens, and is at most 64 characters; `description` is present and at most 1024 characters; `compatibility` at most 500.
3. **Namespaces:** with `--namespaces`, every `name` is one of them or starts with one followed by `-`. For the default folder, this repository's namespaces are used.
4. **References:** `references/` holds only `.md` files and no subfolders; every relative link in `SKILL.md` and the references (outside code) resolves to a file inside the skill's folder; every reference is linked from `SKILL.md`.
5. **The mode table:** where there are references, `SKILL.md` has a table whose rows' first cells are the mode words in backticks, one row per reference.
6. **Length:** a warning, not a failure, above 500 lines of `SKILL.md`.
7. **The repository root**, for the default folder only: no `upstream/`, `upstream.toml` or `.gitmodules`.

An empty folder gives `ok: 0 skills checked`, and so does an absent default `skills/`, since Git keeps no empty folders. A missing folder given on the command line fails, so a mistyped path can't pass as an empty collection.

`scripts/check.sh --self-test` runs the check on `scripts/fixtures/`: one valid module and one copy per failure above, each named for what it breaks, plus cases Git can't hold (symlinks, non-UTF-8, a long `SKILL.md`, empty and missing folders, the root leftovers), built in a temporary folder. Each invalid case must give exactly its own `FAIL` line. The fixtures are outside `skills/`, so nothing installs them.

**What it doesn't prove.** It checks files, not behaviour: not that a module's instructions are right, complete or safe, that its description makes it load when it should, or that a fallback works. Its front-matter reader handles the YAML that `SKILL.md` uses, not all of YAML. Those are the review's and cell S1's jobs. AG1's Rust validator replaces it after M2 ([roadmap](../program.md#agents-monorepo-owner-2026-11-12)).
