---
id: AS-REC-2026-11-16-WORKSPACE-PR14-REVIEW
kind: session
title: Workstation review and remediation of workspace skills PR 14
summary: "Review of agents #14 at 1200a5f: apply guest policy before socket use, validate Herdr's actual result fields, preserve unknown/partial status, fix root and slug assumptions, protect active/unretained work, and correct premature merge claims."
status: active
created: 2026-11-16
updated: 2026-11-16
related: [AS-REC-2026-11-16-WORKSPACE-SESSION, AS-REC-2026-11-16-WORKSPACE-SKILLS, AS-ARCH-WORKSPACE-COORDINATION, AS-STATE, AS-PROGRAM]
---

# Workstation review and remediation of workspace skills PR 14

## Intent

The owner requested review of agents #14 and direct updates as in the previous Workstation-lens review. This session reviewed the existing skills and authored the changes below; it does not independently approve its own remediation.

## What was checked

- PR #14 head `1200a5f260ea8df70856368dacca9164a57cbb5d`, branch `feat/workspace-skills`; base `798664252b58a54b72fc748d1df466c8bd651097`.
- Every original changed file, the previous review comment, root/wiki/module rules, coordination proposal and install boundary.
- Workstation main `99da1da21ece13965d0f0cb765347695782ac6db`: current payload guest rules and Herdr skill, together with the Workstation architecture already referenced by the coordination proposal. Main remains unchanged since the #13 review.
- Herdr `2ecf19a92fc3a7fff75b6ce3655ef04f4625ec01`: CLI parsing and response, workspace, worktree and agent schemas, plus `branch_to_path_slug` and `default_checkout_path`.
- GitHub's live PR response: open, not merged. No guest/Herdr runtime survey, install, policy change, network refresh or agent dispatch was run by this session.

## Decisions

Preserved the recorded owner selections: five skills, direct Herdr backend initially, one `--` separator for display grouping, and first pin after M10 merges. Workstation's installation prerequisites and runtime gates still apply; this PR cannot install or admit itself. Extra API/coordination modules remain proposals.

## Evidence

### Findings and dispositions

| ID | Priority | Finding at reviewed head | Correction |
| --- | --- | --- | --- |
| WS14-01 | P1 | Base skill contacts the socket and only hands off on refusal. Current guest policy requires owner execution before the attempt; Cursor's recorded socket reachability means refusal is not a reliable policy gate. | Explicit environment/project preflight, required Workbench rules, ordinary owner-run policy and scoped bootstrap exception before access. |
| WS14-02 | P1 | Removal checks only `status -sb` and owner desire, then automatically deletes the branch. Clean status omits ignored data and does not prove no live users, preserved work or a current approved tip. | Activity/pane/process and ignored-data checks, exact retention/abandonment, fresh target checks, separate branch-deletion authorization and partial-failure reporting. |
| WS14-03 | P1 | Status omits Git-only worktrees when the clone lacks a Herdr attachment and has no representation of denied/malformed queries; zero counts and empty arrays can appear healthy. It also says read-only while instructing remote refresh first. | Always retain Git registration, reconcile attachments, explicit provider states/time/partial/errors, null unknown counts and no fetch/sync inside status. |
| WS14-04 | P1 | Root is inferred from parents of reported checkout paths, although `--path` overrides exist and main-only lists cannot identify the configured linked-worktree destination. Adoption/empty-folder maintenance may target an unrelated directory. | Environment/owner-verified root and destination; unknown roots block dependent mutations. |
| WS14-05 | P2 | Creation says to check `worktree.repo_root`, absent in native `WorktreeInfo`; exact reply shapes and agent field mappings are unspecified. | Validate response tags, use `result.workspace.worktree.repo_root`, `result.worktree.path`, returned workspace IDs and native agent/status/pane fields. |
| WS14-06 | P2 | Adoption slug logic only replaces slash and collapses dashes, while the source also lowercases ASCII letters, replaces all non-alphanumeric runs and trims edge dashes. It is lossy and permits collisions. | Correct the description; actual verified paths/registration remain authoritative, collisions refuse, no reverse inference. |
| WS14-07 | P2 | Shared model still defines workspace as clone plus Herdr item and omits writer/integrator or interrupted-mutation rules carried by the #13 proposal. | Separate logical vocabulary from initial binding, preserve branch/base/target intent, one checkout writer/integrator and inspect partial outcomes before retry. |
| WS14-08 | P2 | Clone opening accepts either repo root or checkout path, says prefer a match, and treats missing metadata as non-clone. This permits ambiguity and duplicate attachment creation. | Require exact consistent root/path/non-linked metadata and one match; unknown/ambiguous is not absent. |
| WS14-09 | P2 | STATE, its update note and the episode record claim #14 merged/on main although the live PR is open. The roadmap also claims the kickoff required verbatim owner answers when it did not. | Correct current status; summarise the selected options faithfully, retain historical survey claims as labelled evidence and add dated review notes. |
| WS14-10 | P2 | Issue/PR modes can paste raw evidence despite public-safe rules, and base/fallback instructions lack explicit missing-base and missing-required-reference outcomes. | Label public redactions, preserve evidence privately as appropriate, stop unsupported branch-dependent actions and load/require the named references. |

### Source evidence

Herdr sources are primary implementation evidence, not a live guest test:

- [CLI worktree parser](https://github.com/herdrdev/herdr/blob/2ecf19a92fc3a7fff75b6ce3655ef04f4625ec01/src/cli/worktree.rs): explicit paths and selectors; no invented flags added here.
- [Response schema](https://github.com/herdrdev/herdr/blob/2ecf19a92fc3a7fff75b6ce3655ef04f4625ec01/src/api/schema/response.rs), [workspace schema](https://github.com/herdrdev/herdr/blob/2ecf19a92fc3a7fff75b6ce3655ef04f4625ec01/src/api/schema/workspaces.rs), [worktree schema](https://github.com/herdrdev/herdr/blob/2ecf19a92fc3a7fff75b6ce3655ef04f4625ec01/src/api/schema/worktrees.rs) and [agent schema](https://github.com/herdrdev/herdr/blob/2ecf19a92fc3a7fff75b6ce3655ef04f4625ec01/src/api/schema/agents.rs): result tags and exact field locations.
- [Slug/path source](https://github.com/herdrdev/herdr/blob/2ecf19a92fc3a7fff75b6ce3655ef04f4625ec01/src/worktree.rs): default destinations and lossy branch normalization.
- [Workbench guest rules](https://github.com/luckgrid/workstation/blob/99da1da21ece13965d0f0cb765347695782ac6db/packages/workbench/guest/skills/workbench-guest-rules/SKILL.md) and [Herdr skill](https://github.com/luckgrid/workstation/blob/99da1da21ece13965d0f0cb765347695782ac6db/packages/workbench/guest/skills/workbench-herdr/SKILL.md) (private for now): pre-existing owner socket route and scope. A discrepancy in its older field/base wording is a Workstation follow-up, not permission for these public skills to expand the guest boundary.

### Validation

Executed on the local source snapshot with these edits:

```text
bash scripts/check.sh
ok: 5 skills checked

bash scripts/check.sh skills --namespaces workspace
ok: 5 skills checked

bash scripts/check.sh --self-test
ok: self-test, 36 cases as expected

Documentation/YAML/link/table/JSON checks
52 Markdown files; 27 unique page IDs; 294 relative links; 1 skill JSON example parsed
0 problems
```

The documentation check covered root README, wiki and delivered skill Markdown; parsed required wiki front matter, duplicate IDs, local links/heading anchors, local related IDs, table widths and the skill JSON example. It did not render Markdown or query external destinations. `rumdl` is unavailable here, so the earlier review's lint/format result does not cover these new edits. No live guest behaviour is claimed.

### Instruction walkthroughs

Manual author walkthroughs of the revised instructions, not live execution or independent agent tests:

| Input situation | Result required by the revised skill |
| --- | --- |
| Socket reachable but ordinary guest owner-run policy | Hand off before contacting it; reachability grants no permission |
| Git worktree exists, clone Herdr attachment missing | Retain Git-only row with null workspace ID, no bogus worktree-list call |
| Agent query denied and checkout otherwise clean | Partial/error status; removal waits for activity evidence/owner confirmation |
| Only main checkout, or linked checkout created with explicit path | Root stays unknown unless independently resolved; no inferred adoption/cleanup directory |
| Branches whose punctuation/case normalize to the same slug | Report destination conflict, never overwrite/reverse-infer identity |
| Move completed, Herdr open failed | Preserve moved checkout and tip, report partial result, inspect before retry |
| Child branch named parent--child | Display grouping only; recorded base/tip establishes dependency |
| Missing local origin/develop ref | Null comparison/counts, not zero and not a network refresh |
| Clean checkout with ignored files or unretained tip | Block retirement until preservation or explicit abandonment |
| Approved tip changes before cleanup | Invalidate the plan; preserve current work |

## Proposals

The status JSON is now a versioned draft projection with native mappings and explicit uncertainty. It is not a shipped plugin API or runtime schema validator. Keep new universal API operation names proposed; the first working backend remains Herdr.

## What's next

- Review these changes, then have an independent session evaluate the corrected skills before pinning.
- In the guest, prove unknown/denied-provider handling, path collision, active/ignored-data cleanup refusal and interrupted create/adopt recovery under the installed policy.
- Workstation follow-up: align `workbench-herdr` with the selected local-develop base, native field paths, the observed Cursor socket gap and these public skills; carry the owner's M10-first pin timing into its own prerequisites and cell S1.
- Owner handles merge and installation. This session changes neither.
