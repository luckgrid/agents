---
id: AS-STATE
kind: state
title: State
summary: "Where this repository (luckgrid/agents, named agent-skills until 2026-11-13) stands. No skills on main since M0 merged (#11); Workbench's U16 copy installer merged in Workstation (#51), with no agents commit pinned yet. The seventh round (owner, 2026-11-16): workspace (M10) first, for herdr workspaces and worktrees, from a kickoff brief run in Cursor's bootstrap mode; records capture intent in one schema. Also the agents monorepo, open items and the decisions in force."
status: active
created: 2026-11-12
updated: 2026-11-16  # M0 and its Workstation companion merged; seventh round: workspace first, records capture intent; before that, M0 done on its branch, pending review and merge; WB-06 closed; before that, M0's companion now waits for Workstation's hand-off change (#44 merged); before that, sixth round: namespaces; before that, fifth round: the plan for M0 and its Workstation companion, M10, the conventions; before that, M1 workbench moves to Workstation, WC13's role skills as its modes; M7 done, the rename done; before that, agents monorepo: the owner's decisions after the independent review; before that, agents monorepo direction and knowledge consolidation; before that, M7 in progress (site-system#9); before that, the custom-modules plan (owner, 2026-11-12)
related: [AS-REC-2026-11-16-WORKSPACE-SKILLS, AS-REC-2026-11-14-MODULAR-REWRITE, AS-ADR-0001, AS-ARCH-AGENTS-WORKSPACE, AS-RES-AGENTS-LINEAGE, AS-REC-2026-11-12-SKILL-MODULES, AS-RES-SKILL-REFERENCES, AS-PROGRAM, AS-ARCH-SKILL-SOURCES, AS-ARCH-GUEST-INSTALL, GW-ADR-0014]
---

# State

**As of 2026-11-16.** Runtime results remain as their records give them; the documentation research baseline is recorded in the [agents monorepo session](2026-11-12-agents-monorepo.md).

## Where it stands

| Area | Position |
| --- | --- |
| Skills on `main` | **None** since M0 merged (#11, `0d53e41`, 2026-11-15). Before it, nine: `wb-herdr` and `wb-guest-rules` written here; `herdr` copied from herdr 0.9.5; six linked from obra/superpowers, trailofbits/skills and mattpocock/skills ([skill sources](../architecture/skill-sources.md)) |
| `scripts/check.sh` before M0 | `ok: 9 skills checked (6 linked from 3 upstream repositories)` with the submodules at their pinned commits, before agent-skills#2 merged on 2026-11-11 ([record](2026-11-10-linked-upstream-skills.md#the-independent-reviews-wb-06-and-the-merge)); agent-skills#3 and #4 changed only the README and the wiki |
| M0, merged as #11 (`0d53e41`) | **No skills.** M0 removed the six links, the three submodules, `upstream.toml` and the `herdr` copy, and `wb-guest-rules` and `wb-herdr`, which move to Workstation as `workbench-guest-rules` and `workbench-herdr` in M0's companion (merged as Workstation #51, `1827922`, 2026-11-15). Their source is `f2e2bde` ([record](2026-11-15-m0.md)) |
| `scripts/check.sh` on `main` (since #11) | For modules, with a folder argument and `--namespaces`. `scripts/check.sh --self-test`: `ok: self-test, 36 cases as expected`. `scripts/check.sh` in the session's sandbox: `FAIL .gitmodules: the upstream machinery was removed in M0`, from the sandbox's placeholder at that path; the owner's run outside the sandbox: `ok: 0 skills checked`, exit 0, and `ok: self-test, 36 cases as expected` ([record](2026-11-15-m0.md#checkshs-outputs)) |
| In the guest | **No skills from this repository.** Workbench's U16 copies skills itself since Workstation #51, installing `workbench-guest-rules` and `workbench-herdr` from Workbench's payload; no `agent_skills_commit` is pinned yet ([guest install](../architecture/guest-install.md)). The owner's screenshots of 2026-11-16 show the agents clone on a local `develop` at `0d53e41`, with a herdr worktree `skills/herdr-workflows-1` under it, and Cursor waiting there for the [kickoff](2026-11-16-workspace-skills/kickoff.md) |
| The wiki | Added on 2026-11-12 ([record](2026-11-12-wiki.md)) |
| The plan | Fully custom skill modules, planned on 2026-11-12 ([record](2026-11-12-skill-modules.md), [roadmap](../program.md)); M7 done; the fifth round (2026-11-14, [record](2026-11-14-modular-rewrite/README.md)) sets how M0 and its Workstation companion are carried out, from a [kickoff brief](2026-11-14-modular-rewrite/kickoff.md) for the guest; the seventh round (2026-11-16, [record](2026-11-16-workspace-skills/README.md)) builds `workspace` first |

## Agents monorepo (owner, 2026-11-12)

[AS-ADR-0001](decisions/0001-agents-monorepo.md) records the owner's decisions: an agents monorepo, renamed `luckgrid/agents` on 2026-11-13; skills as a package, staying at `skills/`; Workstation's wiki as the latest knowledge source; public config defaults here with a private overlay in Groundwork's agent-config repository, the stricter value winning for sandbox settings; Rust after M2, starting with the validator; Claude Code and Codex for the first shared-config proof. The [workspace design](../architecture/agents-workspace.md) is proposed. Nothing is built or installed yet. Workflow Factory stays independent WIP, WfOS resumption is optional, and wiki-nest context creates no migration task.

## Open

| Item | Gate | Page |
| --- | --- | --- |
| M7: `site-system`, a project skill in the site-system repository, with `design-system`'s consume rules until M6 | **Done:** site-system#9 squash-merged as `fee220c` (2026-11-13), after its independent review and recheck ([record](2026-11-12-site-system-skill.md)) | [M7](../program.md#m7-site-system-a-project-skill-in-site-system) |
| Rename to `luckgrid/agents` | **Done** by the owner, 2026-11-13; Workstation's PR updates `wb sync`, U16's clone path and its current pages | [step 2](../architecture/agents-workspace.md#steps-and-proof-gates) |
| WC13's role skills | **Decided** (owner, 2026-11-13): modes of the `workbench` skill, which moves to Workstation and ships with Workbench ([fourth round](../program.md#fourth-round-owner-2026-11-13)) | [M1](../program.md#m1-workbench) |
| M0: remove the linked skills, the herdr copy, the two `wb-` skills and every dependency around the upstream skills; `check.sh` (with a folder argument and namespaces) and the README for modules | **Done:** merged as #11 (`0d53e41`, 2026-11-15) ([record](2026-11-15-m0.md)) | [M0](../program.md#m0-the-module-foundation) |
| M0's companion in Workstation: the skills CLI and its `package.json` removed; `workbench-guest-rules` and `workbench-herdr` added to Workbench's guest skills, `roles.toml`'s skill names in the `workbench` namespace; U16 copies skill folders from two sources; GW-ADR-0014 decisions 1 and 2 amended | **Done:** merged as Workstation #51 (`1827922`, 2026-11-15) | [M0](../program.md#m0-the-module-foundation) |
| M1: `workbench` (the base skill and the `workbench-*` skills: WC13's role routines, and `workbench-guest-rules` and `workbench-herdr` from M0's companion) | **Moved to Workstation:** written there, shipped with Workbench and installed guest-wide by U16 | [M1](../program.md#m1-workbench) |
| M2: `council` (takes in `wb-review`, `wb-remediate`; reviewers with lenses, in rounds) | After M10's workspace skills (seventh round); Workbench's step F1 needs it and Workbench's `workbench` skill | [M2](../program.md#m2-council) |
| AG1: Rust validator replaces `check.sh`; Moon admitted for this repository | After M2; alongside M3 to M9 | [agents items](../program.md#agents-monorepo-owner-2026-11-12) |
| AG2: shared user-level instructions for Claude Code and Codex | After AG1 | [steps](../architecture/agents-workspace.md#steps-and-proof-gates) |
| AG3: settings and sandbox baseline with the agent-config overlay (stricter wins); GW-ADR-0011 amendment and gate A2 in Workstation | After AG2 | [steps](../architecture/agents-workspace.md#steps-and-proof-gates) |
| M3: `multiplex` (replaces `herdr`; herdr the default backend, H1 and H3) | After M2 | [M3](../program.md#m3-multiplex) |
| First pin: this repository's first `agent_skills_commit`, U16's first install, cell S1 | After M2, M1's required Workbench skills and M0's Workstation installer companion merge | [order](../program.md#order) |
| M4: `wiki` | After M3 | [M4](../program.md#m4-wiki) |
| M10: `workspace`: first the workspace and worktree skills with herdr (seventh round); later issue, pr and release (Workstation #43's `manage-workspace`) | **Next**, from the [kickoff brief](2026-11-16-workspace-skills/kickoff.md) in Cursor's bootstrap mode, on the worktree `skills/herdr-workflows-1` | [M10](../program.md#m10-workspace) |
| M5: `grill` | After M4 | [M5](../program.md#m5-grill) |
| M6: `design-system`; M7 then points to it | After M5 | [M6](../program.md#m6-design-system) |
| M8: `frugal`; rtk and qmd as separate Workbench catalog items | After M6 | [M8](../program.md#m8-frugal) |
| M9: `triage` (debugging, isolation, queues, incidents; takes `systematic-debugging`) | Last | [M9](../program.md#m9-triage) |

WB-06 is closed: M0 removed the skills it is about ([record](2026-11-15-m0.md)).

## Decisions in force

The owner's decisions that bind this repository, newest first. Groundwork's GW-ADR-0014 is still **proposed**; its decisions 1 and 2 are the owner's choices and are followed here, but GW-ADR-0014 itself is accepted only in Workstation.

| Date | Decision | Where recorded |
| --- | --- | --- |
| 2026-11-16 | Seventh round: this repository records common patterns for agents and agent workflows, from the owner's preferred tools, apps and configs, used through Workstation. `workspace` (M10) comes first, for herdr workspaces (one per namespace folder, on `develop` or else `main`) and worktrees (nested at most two levels). Agents may hand off to another workspace, or edit skills there on a branch of its own with approval. Records capture intent, not quotes unless asked, in one schema; a Workbench tool for it is proposed. Cursor bootstrap sessions run with the owner approving each command, and config changes are logged for later review | [record](2026-11-16-workspace-skills/README.md#the-owners-intent-2026-11-16) |
| 2026-11-15 | M0 in the guest: `check.sh`'s root check stays strict where the sandbox shows a placeholder at a removed path; the owner runs it outside the sandbox, and that run is the evidence. The owner removes a file the sandbox mounts read-only (`.gitmodules`) | [record](2026-11-15-m0.md#the-owners-choices-2026-11-15) |
| 2026-11-14 | Sixth round: every skill named `<namespace>` or `<namespace>-<name>` in full words (no `wb-`), in every repository; a base skill plus namespaced skills for uses invoked explicitly or loaded on their own; flat folders; `wb-guest-rules` and `wb-herdr` move to Workstation as `workbench-guest-rules` and `workbench-herdr`; the planned names kept, each a namespace owned by one repository | [record](2026-11-14-modular-rewrite/README.md#the-sixth-round-namespaces-2026-11-14) |
| 2026-11-14 | Fifth round: every dependency around the upstream skills removed, the skills CLI and its `package.json` included; `wb setup` copies skill folders from two sources (GW-ADR-0014 to be amended in Workstation); the herdr copy dropped in M0; public users copy a folder or use any installer; M10 `workspace` after M4; invocation by mode word, a shared `check.sh`, rules that only tighten downward | [record](2026-11-14-modular-rewrite/README.md#the-owners-choices-2026-11-14) |
| 2026-11-13 | `workbench` (M1) moves to Workstation as one skill folder with modes, WC13's role routines among them; shipped with Workbench and installed guest-wide by U16 | [fourth round](2026-11-12-skill-modules.md#fourth-round-workbench-moves-to-workstation-2026-11-13) |
| 2026-11-12 | After the independent review: public config defaults here, private overlay in agent-config; the stricter value wins for sandbox, deny and permission settings; Rust after M2, validator first; Claude Code and Codex for the first shared-config proof; rename to `luckgrid/agents` | [AS-ADR-0001](decisions/0001-agents-monorepo.md), [session](2026-11-12-agents-monorepo.md#the-review-and-the-owners-choices-2026-11-12) |
| 2026-11-12 | Expand into an agents monorepo; skills become a package; consolidate relevant knowledge using Workstation as latest source; Workflow Factory stays independent and WfOS optional | [AS-ADR-0001](decisions/0001-agents-monorepo.md), [session](2026-11-12-agents-monorepo.md) |
| 2026-11-12 | Third round: `wiki` publishes the wiki template's rules in its own words, with `wiki-structure.md` the authority; the `site-system` project skill is built first; a `triage` module, with debugging part of it | [record](2026-11-12-skill-modules.md#third-round-the-open-questions-answered-2026-11-12) |
| 2026-11-12 | Second round: the review module is `council` (several models, each with a lens, in rounds); `multiplex` replaces `herdr`, with herdr the default backend; new modules `design-system` here and `site-system` as a project skill in the site-system repository | [record](2026-11-12-skill-modules.md#second-round-the-owners-feedback-2026-11-12) |
| 2026-11-12 | Fully custom skill modules instead of maintaining an upstream, with a reference list of skills; own words only, ideas credited; the linked skills removed in the first refactor PR; the review module named `peer-review` (renamed `council` the same day); `frugal` as instructions first, rtk and qmd later through Workbench's catalog | [record](2026-11-12-skill-modules.md#the-owners-choices-2026-11-12) |
| 2026-11-12 | Pages here may link to Luckgrid's private Workstation, with a note that it is still in development and private for now (CR-08) | [record](2026-11-12-workstation-links.md) |
| 2026-11-11 | Replace the copied `herdr` skill with a thin Luckgrid skill; third-party herdr facts only once `--help` confirms them, in Luckgrid's words (H1, H3) | [record](2026-11-10-linked-upstream-skills.md#the-owners-decision-on-the-herdr-skill-2026-11-11) |
| 2026-11-11 | Merge agent-skills#2 with WB-06 open, gated on F1; with it, public skills are linked from their authors at reviewed commits, not copied (the PR's design; superseded by the 2026-11-12 decision once M0 merges) | [record](2026-11-10-linked-upstream-skills.md#the-independent-reviews-wb-06-and-the-merge) |
| 2026-11-10 | The repository's licence is Apache-2.0 | [record](2026-11-10-first-skills.md#what-the-session-wrote) |
| 2026-11-07 | One public skills repository, `luckgrid/agent-skills`; the skills CLI pinned and run on a local checkout only (GW-ADR-0014 decisions 1 and 2, proposed) | [record](2026-11-07-origin.md#the-choices-owner-2026-11-07) |
