---
id: AS-REC-2026-11-16-WORKSPACE-REVIEW
kind: session
title: Workstation review of the workspace kickoff (2026-11-16)
summary: "Review of agents #13 at e236496, against Workstation at 99da1da: separate the public workspace model from Herdr topology, add common coordination/workflow API ideas, and correct contradictory kickoff and record rules."
status: active
created: 2026-11-16
updated: 2026-11-16
related: [AS-REC-2026-11-16-WORKSPACE-SKILLS, AS-ARCH-WORKSPACE-COORDINATION, AS-PROGRAM, AS-STATE]
---

# Workstation review of the workspace kickoff (2026-11-16)

## Intent

The owner asked for a review of [agents #13](https://github.com/luckgrid/agents/pull/13) from a Workstation lens and help adding common workspace, worktree, workbranch coordination and workflow skill ideas toward a universal API. This author reviewed the existing PR and authored the corrections below; this is not an independent review of those new corrections.

## What was checked

- agents #13 head: `e2364963b894f68bfdef835f00df5d444fcc85c0`, branch `docs/workspace-skills-kickoff`; base `0d53e418c2f272fe9f04676eda620655e58422cb`.
- Workstation source snapshot: `99da1da21ece13965d0f0cb765347695782ac6db`. Read its work model, execution/continuity, authority map, architecture contracts, CLI contract, WS-ADR-0007, fast-workflow direction, namespace decision GW-ADR-0018 and installed-payload `workbench-herdr` source.
- agents root README, module composition, architecture boundary, roadmap, wiki rules and every file changed by the original PR.
- Official Git worktree and branch documentation, accessed 2026-11-16. The [standing proposal](../../architecture/workspace-coordination.md#sources-and-limits) links the source pages and exact Workstation snapshot.
- No Herdr runtime or guest access, no installs and no sandbox changes. Guest state and screenshots described by the earlier record remain reported evidence, not independently repeated observations.

## Decisions

This session makes no new owner architecture acceptance, install pin, namespace allocation or publication decision. The owner authorized review and additions to the PR. New operation names, field shapes and extra skills remain proposed. Workstation's first-pin prerequisites and FW gates remain effective until their owners amend and admit them.

## Evidence

### Findings and dispositions

| ID | Priority | Finding at reviewed head | Correction |
| --- | --- | --- | --- |
| WSC-01 | P1 | The kickoff model and roadmap define a workspace as one clone plus one Herdr sidebar item, while Workstation's execution model permits several repositories/worktrees. This would export a guest binding as the universal model and conflate labels with identity. | Add the standing coordination proposal; narrow the clone rule to the initial Workbench binding and require native checkout association. |
| WSC-02 | P1 | Feature branches are equated with worktrees and sub-worktrees without separating branch base, shared Git state and sidebar grouping. There is no common coordination rule for competing checkout writers or shared integration refs. | Define workbranch separately; record base/target/tips and dependencies; add writer/integrator responsibilities, stale-state refusal, partial-failure recovery and cleanup retention. |
| WSC-03 | P2 | The root README requires owner decisions word for word and says no skills until M2, contradicting the PR's intent-summary rule and M10-first sequence. The kickoff also gives a different section order from the wiki's schema. | Align the entry-point rules and kickoff, retain historical record wording through dated additions, and limit formal report schema to durable hand-offs. |
| WSC-04 | P2 | The survey makes a test commit then requires branch deletion with `-d` and says all scratch resources must be gone. An unmerged fixture commit can make safe deletion refuse, leaving the done condition unattainable without an additional disposition. | Split the no-commit lifecycle from the commit fixture; retain/report an unmerged test tip unless its disposal is expressly approved. Never merge fixture commits into develop for cleanup. |
| WSC-05 | P2 | The proposed base skill promises mode-table rows for references written later, while the root module contract requires one actual linked file per row. | Keep future modes in roadmap prose until their reference files ship. |
| WSC-06 | P2 | The kickoff recommends a pin immediately after M10 while the roadmap and STATE still require M2 and cross-repository proof gates. | Treat early pinning as an explicit Workstation amendment proposal; keep existing prerequisites until changed and proved. |
| WSC-07 | P2 | Snapshot refs and cleanup instructions lack handling for an already-created name, changed head, failed attachment or uncertain active-session state. Replaying them could overwrite or retire unrelated work. | Refresh actual refs, refuse collisions, inspect partial mutations before retrying and block unattended cleanup when activity is unverified. |

### Validation

Executed against a local source snapshot with these documentation edits:

```text
bash scripts/check.sh
ok: 0 skills checked

bash scripts/check.sh --self-test
ok: self-test, 36 cases as expected

Documentation link/front-matter check
30 Markdown files; 25 unique page IDs; 269 relative links checked
0 problems
```

The documentation check parsed YAML front matter, required fields, duplicate IDs, local Markdown destinations/heading anchors and local `AS-` related IDs. It did not query external link availability or render Mermaid. `rumdl` is unavailable in this environment, so Markdown lint/format checks were not run; the original PR's lint evidence predates these edits.

These are documentation and validator checks only. The acceptance examples on the architecture page remain future guest behaviour tests.

## Proposals

The [common coordination API](../../architecture/workspace-coordination.md) proposes:

- a vocabulary separating workspace, repository, workbranch, worktree, Work Unit, workflow/run and session;
- operation contracts for inspection, opening, branch/checkout preparation, advisory assignment, status, hand-off/resume, integration and retirement;
- versioned result/error concepts and explicit partial/unavailable/stale outcomes, aligned with Workstation's proposed CLI contract;
- later `workspace-workbranch`, `workspace-coordinate` and possibly `workspace-workflow` skills or modes after the first concrete Herdr lifecycle;
- ownership boundaries preserving native Git/Herdr state, Workbench guest controls and Workstation's observer scope.

No generic adapter framework, daemon or central status ledger is prescribed.

## What's next

1. Review this proposal and the corrections on the PR; owner acceptance remains separate.
2. Sync the reviewed brief into the guest using the owner workflow, preserving concurrent local work.
3. Survey installed Herdr 0.9.5 in the explicitly approved bootstrap session; implement only the selected skills and behaviour proved there.
4. Hand off Workbench narrowing, current installed-rule alignment and any first-pin change to a Workstation session, with this proposal and the exact agents candidate linked.
5. Keep reusable API details small until dogfooding provides a concrete consumer; obtain an independent review of implemented skills before installation.
