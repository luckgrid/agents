# agents wiki

Start with the repository's root `README.md`: it is the entry point and holds the rules. This wiki holds the details: how the repository works and why, where it stands, what comes next, and the records behind it.

It follows Luckgrid's shared wiki template, the wiki structure kept in luckgrid/wiki-nest (`wiki-structure.md`, private), with IDs prefixed `AS-`. The decisions that bind how the Workbench guest installs these skills are not made here: they are Groundwork's [GW-ADR-0014](https://github.com/luckgrid/workstation/blob/main/wiki/groundwork/records/decisions/0014-shared-skills-and-agent-workflows.md) (proposed) and Workbench's [agent workflows](https://github.com/luckgrid/workstation/blob/main/wiki/workbench/agent-workflows.md), both in luckgrid/workstation, which is still in development and private for now (see the root README's note).

## Reading order

1. [records/STATE.md](records/STATE.md): where the repository stands, what is open, and the decisions in force.
2. [architecture/skill-sources.md](architecture/skill-sources.md): skill modules: the module shape, the three tiers, namespaces, composition, and what `scripts/check.sh` proves.
3. [architecture/guest-install.md](architecture/guest-install.md): how the Workbench guest is to install these skills: `wb setup` copies skill folders from two sources, and what it needs from this repository.
4. [Agents monorepo decision](records/decisions/0001-agents-monorepo.md), [workspace proposal](architecture/agents-workspace.md) and [knowledge lineage](resources/agents-knowledge-lineage.md): the owner's decisions, the proposed layout, defaults and overlay, and the sources.
5. [program.md](program.md): the module roadmap and the bounded monorepo foundation.
6. [records/README.md](records/README.md): the dated history, one record per episode.
7. [resources](resources/README.md): the sources behind it, and the [skill references](resources/skill-references.md) each planned module learns from.

## Which page decides what

| Question | Page |
| --- | --- |
| Why this workspace expands, what the owner decided, and what remains independent | [Agents monorepo decision](records/decisions/0001-agents-monorepo.md) |
| Shared configuration-source direction and historical knowledge dispositions | Workstation's current wiki, then the [agents-specific design](architecture/agents-workspace.md) and [lineage](resources/agents-knowledge-lineage.md) |
| The rules for working in this repository (no secrets, how to write a module, licences) | the root `README.md` |
| Whether and how the Workbench guest installs skills | Groundwork's GW-ADR-0014, then Workbench's daily use for U16 (Workstation) |
| The review-and-remediate workflow these skills serve (step F1 onwards) | Workbench's agent workflows (Workstation) |
| What a module holds, the tiers and namespaces, and what the check proves | [architecture/skill-sources.md](architecture/skill-sources.md) |
| What the guest's install step needs from this repository | [architecture/guest-install.md](architecture/guest-install.md) |
| Which skills each planned module learns from, and under which licence | [resources/skill-references.md](resources/skill-references.md) |
| What is open and what is next | [records/STATE.md](records/STATE.md), then [program.md](program.md) |
| Control / Record / Audit roles vs harness profiles (proposed) | [architecture/control-record-audit.md](architecture/control-record-audit.md) |

## Writing standard

- Non-README pages carry `id`, `kind`, `title`, `summary`, `status`, `created`, `updated` and `related` (IDs, never slugs or paths). IDs here use the `AS-` prefix: `AS-ARCH-…`, `AS-ADR-NNNN` (in `records/decisions/`), `AS-REC-…`, `AS-PROGRAM`, `AS-STATE`, `AS-RESOURCES`, `AS-RES-…`. An ID never changes when a page moves.
- **Status lives in one place:** [STATE](records/STATE.md). Other pages link to it instead of restating it.
- **Records keep their history.** A dated record is named for the date its first content happened and keeps its wording; corrections go in a later record or a dated note. Evidence (check output, hashes, tool replies) is quoted word for word.
- **Records capture intent** (owner, 2026-11-16). Say what the owner meant, faithfully, in your own words. Quote the owner word for word only when they ask for it. When the intent is unclear, ask. Records written before 2026-11-16 keep their quotes.
- **One record schema.** A record has the front matter listed above and, for a session, these sections in order: the intent; what was checked; the decisions, with the questions asked; the evidence; proposals; what's next. Leave out a section with nothing in it. Durable agent hand-off reports use those fields and sections where applicable; ordinary chat replies need no front matter. A Workbench tool to create and check records is proposed for a Workstation session ([seventh round](program.md#seventh-round-owner-2026-11-16)).
- **Label evidence.** Tell apart the owner's decisions, results the owner reported, results a session ran, upstream claims and proposals. A proposal is not acceptance.
- **This repository is public.** The root README's no-secrets rule holds for the wiki too: no hostnames, user or account names, email addresses, IPs, keys or tokens, and paths written as `~/…`. Links to Luckgrid's private repositories are allowed, as the owner decided on 2026-11-12 ([record](records/2026-11-12-workstation-links.md)), and say that they are private.

Checks: `scripts/check.sh` checks the skills, not the wiki. The wiki's Markdown is checked with [rumdl](https://github.com/rvben/rumdl) 0.3.2, the version Luckgrid's other wikis pin, as `rumdl check wiki/`, using the root `.rumdl.toml` (the same settings as site-system's and Workstation's).
