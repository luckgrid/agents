---
id: AS-ARCH-GUEST-INSTALL
kind: architecture
title: Guest install
summary: "How the Workbench guest is to install these skills: Workbench's step U16 copies skill folders itself from two sources, an export of a pinned commit of this repository and Workbench's own guest skills, with no installer CLI (owner, 2026-11-14); Workstation's daily-use page decides the details."
status: active
created: 2026-11-12
updated: 2026-11-15  # M0: rewritten for the copy installer; the skills CLI's test results and F1's submodule changes left to their records; before that, the skills CLI to be replaced (fifth round); before that, created
related: [GW-ADR-0014, GW-ARCH-WORKBENCH-AGENT-WORKFLOWS, GW-GUIDE-WORKBENCH-DAILY-USE, AS-ARCH-SKILL-SOURCES, AS-REC-2026-11-15-M0, AS-REC-2026-11-14-MODULAR-REWRITE, AS-REC-2026-11-10-LINKED-UPSTREAM-SKILLS, AS-STATE]
derived_from: GW-ADR-0014
---

# Guest install

**Status:** planned; see [STATE](../records/STATE.md). Nothing installs these skills in the guest yet. The step is Workbench's U16 in `wb setup`, described in full in its [daily use](https://github.com/luckgrid/workstation/blob/main/wiki/workbench/daily-use.md#guest-tools-wb-setup) page, in luckgrid/workstation (private for now). This page records what it needs from this repository. Where the two disagree, Workbench's page decides for the step, and this page is corrected.

## The decided shape

The owner's choice of 2026-11-14 ("wb setup copies (Recommended)", [fifth round](../program.md#fifth-round-owner-2026-11-14)), amending GW-ADR-0014 decision 2 (proposed) in Workstation:

- **No installer CLI and no npm.** `wb setup` copies skill folders itself.
- **Two sources:** this repository's `git archive` export at the commit Workbench pins, and Workbench's own `packages/workbench/guest/skills/` (the `workbench` namespace). A name in both stops the install.
- **Conditions that still hold** from GW-ADR-0014 decision 2: local only, nothing fetched at install time; run only from `wb setup`, outside every agent's sandbox; from an export of a pinned commit, so an agent's uncommitted edit in the guest's clone is never installed.
- **Ownership:** U16 changes only what it installed itself, recorded with each folder's digest, and never removes or replaces a folder or link it didn't put there.

How it copies, records, drops and recovers is Workbench's to decide, on its daily-use page. M0's installer companion merged as Workstation #51; first pin and runtime cell S1 remain separate Workstation actions. The seventh-round owner answer selects pinning after M10 merges rather than M2; carry that prerequisite amendment into Workstation before installation.

## What it needs from this repository

- **`scripts/check.sh` passes at every commit that may be pinned.** An export of a commit with no `skills/` (M0 onwards, until M10) is a valid empty source; a child folder without `SKILL.md` is not.
- **Plain folders:** no symlinks, no submodules and no third-party files in a skill, which `check.sh` enforces, so a `git archive` export holds every module whole.
- **Names in this repository's namespaces only,** so no name collides with Workbench's skills.

## Earlier designs

The skills CLI's test results (1.7.6, 2026-11-10), Groundwork's read of its code, and the changes F1 would have made to U16 for linked skills and submodules are kept in the records that hold them: [linked upstream skills](../records/2026-11-10-linked-upstream-skills.md) and Workstation's records. M0 removed what they were about.

## What installing doesn't prove

A skill that installs is not a workflow that works. Cell S1 of Workbench's step F1 is the runtime check: each harness lists the skills, a skill turned off is hidden, and a namespaced skill loads its base rules.
