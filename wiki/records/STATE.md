---
id: AS-STATE
kind: state
title: State
summary: "Where this repository (luckgrid/agents) stands. M0 merged (#11); U16 copy installer merged in Workstation (#51); no agents commit pinned yet. Seventh round: workspace (M10) merged as #14; five workspace* skills on main; first agent_skills_commit still open (with M0 companion and M1 Workbench skills). Common coordination API proposed (#13)."
status: active
created: 2026-11-12
updated: 2026-11-17  # Workspaces maintain: #14 on main; leftover branches tidied; guest-local develop on four clones; before that, M10 on open #14 after review remediation
related: [AS-REC-2026-11-17-WORKSPACE-MAINTAIN, AS-REC-2026-11-16-WORKSPACE-PR14-REVIEW, AS-ARCH-WORKSPACE-COORDINATION, AS-REC-2026-11-16-WORKSPACE-REVIEW, AS-REC-2026-11-16-WORKSPACE-SKILLS, AS-REC-2026-11-16-WORKSPACE-SESSION, AS-REC-2026-11-14-MODULAR-REWRITE, AS-ADR-0001, AS-ARCH-AGENTS-WORKSPACE, AS-RES-AGENTS-LINEAGE, AS-REC-2026-11-12-SKILL-MODULES, AS-RES-SKILL-REFERENCES, AS-PROGRAM, AS-ARCH-SKILL-SOURCES, AS-ARCH-GUEST-INSTALL, GW-ADR-0014]
---

# State

**As of 2026-11-17.** Runtime results remain as their records give them; the documentation research baseline is recorded in the [agents monorepo session](2026-11-12-agents-monorepo.md). Workspaces hygiene for this guest is in the [maintain session](2026-11-17-workspace-maintain.md).

## Where it stands

| Area | Position |
| --- | --- |
| Skills on `main` | **Five** after M10/#14 (`8a106a4`, 2026-11-16): `workspace`, `workspace-open`, `workspace-worktree`, `workspace-status`, `workspace-maintain` ([session](2026-11-16-workspace-skills/session.md), [maintain](2026-11-17-workspace-maintain.md)). Before M0 there were nine; M0 left none until #14 |
| `feat/workspace-skills` | **Removed** in the 2026-11-17 maintain pass: same five skills as `main`, older wording, no merge ([maintain](2026-11-17-workspace-maintain.md)) |
| `scripts/check.sh` on `main` (since #11) | For modules, with a folder argument and `--namespaces`. `scripts/check.sh --self-test`: `ok: self-test, 36 cases as expected`. `scripts/check.sh skills --namespaces workspace` → `ok: 5 skills checked` |
| M0, merged as #11 (`0d53e41`) | **No skills.** M0 removed the six links, the three submodules, `upstream.toml` and the `herdr` copy, and `wb-guest-rules` and `wb-herdr`, which move to Workstation as `workbench-guest-rules` and `workbench-herdr` in M0's companion (merged as Workstation #51, `1827922`, 2026-11-15). Their source is `f2e2bde` ([record](2026-11-15-m0.md)) |
| In the guest | **No pin yet.** Workbench's U16 copies skills itself since Workstation #51, installing `workbench-guest-rules` and `workbench-herdr` from Workbench's payload ([guest install](../architecture/guest-install.md)). The five `workspace*` skills are on `main` but not installed until `agent_skills_commit` |
| Common coordination API | **Proposed** by the Workstation-lens review of #13: workspace/repository/worktree/workbranch distinctions, native authority, operation contracts, writer/integrator coordination and hand-off/recovery; no runtime API ([proposal](../architecture/workspace-coordination.md), [review](2026-11-16-workspace-skills/workstation-review.md)) |
| The wiki | Added on 2026-11-12 ([record](2026-11-12-wiki.md)); seventh-round records under [2026-11-16-workspace-skills/](2026-11-16-workspace-skills/README.md); maintain [2026-11-17](2026-11-17-workspace-maintain.md) |
| The plan | Fully custom skill modules ([roadmap](../program.md)); seventh round's M10 merged; first pin next |

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
| M10: `workspace`: herdr management skills first (seventh round); later issue, pr and release | **Done on main:** squash-merged as #14 (`8a106a4`, 2026-11-16); leftover `feat/workspace-skills` removed 2026-11-17 ([maintain](2026-11-17-workspace-maintain.md)) | [M10](../program.md#m10-workspace) |
| First pin: this repository's first `agent_skills_commit`, U16's first install, cell S1 | **Open:** M10 is on main; still needs M1's required Workbench skills and the owner's pin hand-off (seventh-round answer) | [order](../program.md#order) |
| M2: `council` (takes in `wb-review`, `wb-remediate`; reviewers with lenses, in rounds) | After M10's workspace skills (seventh round); Workbench's step F1 needs it and Workbench's `workbench` skill | [M2](../program.md#m2-council) |
| AG1: Rust validator replaces `check.sh`; Moon admitted for this repository | After M2; alongside M3 to M9 | [agents items](../program.md#agents-monorepo-owner-2026-11-12) |
| AG2: shared user-level instructions for Claude Code and Codex | After AG1 | [steps](../architecture/agents-workspace.md#steps-and-proof-gates) |
| AG3: settings and sandbox baseline with the agent-config overlay (stricter wins); GW-ADR-0011 amendment and gate A2 in Workstation | After AG2 | [steps](../architecture/agents-workspace.md#steps-and-proof-gates) |
| M3: `multiplex` (replaces `herdr`; herdr the default backend, H1 and H3); then `workspace-*` names it | After M2 | [M3](../program.md#m3-multiplex) |
| M4: `wiki` | After M3 | [M4](../program.md#m4-wiki) |
| M5: `grill` | After M4 | [M5](../program.md#m5-grill) |
| M6: `design-system`; M7 then points to it | After M5 | [M6](../program.md#m6-design-system) |
| M8: `frugal`; rtk and qmd as separate Workbench catalog items | After M6 | [M8](../program.md#m8-frugal) |
| M9: `triage` (debugging, isolation, queues, incidents; takes `systematic-debugging`) | Last | [M9](../program.md#m9-triage) |

WB-06 is closed: M0 removed the skills it is about ([record](2026-11-15-m0.md)).

## Decisions in force

The owner's decisions that bind this repository, newest first. Groundwork's GW-ADR-0014 is still **proposed**; its decisions 1 and 2 are the owner's choices and are followed here, but GW-ADR-0014 itself is accepted only in Workstation.

| Date | Decision | Where recorded |
| --- | --- | --- |
| 2026-11-16 | Seventh round: this repository records common patterns for agents and agent workflows, from the owner's preferred tools, apps and configs, used through Workstation. `workspace` (M10) comes first, for herdr workspaces (one per namespace folder, on `develop` or else `main`) and worktrees (nested at most two levels as a UI convention). Agents may hand off to another workspace, or edit skills there on a branch of its own with approval. Records capture intent, not quotes unless asked, in one schema; a Workbench tool for it is proposed. Cursor bootstrap sessions run with the owner approving each command, and config changes are logged for later review. Bootstrap session answers: five `workspace*` skills; herdr-direct until M3; first pin after these skills merge; nesting `<parent>--<name>` from the clone workspace ID | [record](2026-11-16-workspace-skills/README.md#the-owners-intent-2026-11-16), [session](2026-11-16-workspace-skills/session.md) |
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
