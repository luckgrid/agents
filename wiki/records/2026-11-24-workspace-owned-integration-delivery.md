---
id: AS-REC-2026-11-24-INTEGRATION-DELIVERY
kind: session
title: Workspace-owned integration delivery: one reviewable outcome from parallel worktrees
summary: "Follow-up to the Nov 23 Agents parallel delivery: owner rejects routine per-worktree draft PRs followed by a duplicate aggregate PR; chooses an operator-owned integration worktree, targeted builder remediation and event-driven Controller role, with separate integrated/independent publication modes."
status: active
created: 2026-11-24
updated: 2026-11-24
related: [AS-STATE, AS-ARCH-INTEGRATION-DELIVERY, AS-ARCH-WORKSPACE-COORDINATION, AS-ARCH-CONTROL-RECORD-AUDIT, AS-PROGRAM]
---

# Workspace-owned integration delivery (2026-11-24)

## Intent

Following the [November 23 Agents parallel development session in draft PR #22](https://github.com/luckgrid/agents/pull/22), the owner requested a repeatable workflow that avoids unnecessary review surfaces and release cycles. Several builders had worked successfully in separate guest worktrees, but their work was first published as draft PRs #18–#22 and then incorporated into an additional integration candidate. The owner approved that approach as a one-time workaround, **not** the default operating pattern.

The requested change is to preserve parallel task autonomy while making the **intended accepted delivery**, rather than each builder or worktree, the unit of PR publication. The owner explicitly described the standing workspace operator as the orchestrator who coordinates worktree owners and integration, with the Controller involved on cross-workspace matters and meaningful steering events.

## What was checked

| Source | Evidence type and observation |
| --- | --- |
| [PR #18](https://github.com/luckgrid/agents/pull/18) | Draft bulk-transfer guidance; later remediation tip reported `4cc2f9ff400488bb72b83c5b76472b5382fc78bf` |
| [PR #19](https://github.com/luckgrid/agents/pull/19) | Draft validator fix; later remediation tip reported `07d129847be68df142093ab39cbe5fd9cfb747d3` |
| [PR #20](https://github.com/luckgrid/agents/pull/20) | Draft orchestration skill/namespace; later remediation tip reported `bbef7ba647405905ac06ae184784f605df8113ef` |
| [PR #21](https://github.com/luckgrid/agents/pull/21) | Draft wiki skill; later remediation tip reported `1a3df71cd95b6fcebcccd860797a344d08be02bf` |
| [PR #22](https://github.com/luckgrid/agents/pull/22) | Draft November 23 session record; original tip `e9c0a138a20bc8e483f54d1fcf4692e365e3047b` |
| Operator integration report supplied by owner | `integrate/agents-19-22` / `promote/agents-19-22` at `30698403b8dda6bed440e3bdca93c8e828b6675e`, **not on main**, awaiting owner acceptance; one assembled candidate rather than five independent landings |
| Workstation architecture/docs | Existing one standing operator per repository, separate Controller, `workspace-*`/Workbench boundaries, exact-SHA host publication; proposed inbox and agile task lifecycle in Workstation PRs #100 and #101 |

**Evidence qualification:** the guest integration receipts were not fetched or rerun in this documentation session. The operator reported: a combined validator with `orchestration` namespace; 50 passing validator self-tests, 11 skills checked, rumdl on 96 files, 510 relative links with none broken, and 34 unique stable IDs. Those are **operator-reported** passes on the integrated SHA, distinct from the earlier separate-PR independent reviews. The owner supplied the reviewer recommendation to open one integrated draft for independent exact-head checking; that was **not** an acceptance of final merge or installation.

The same report explicitly said file exchange/archive extraction remained **unadmitted**. No combined candidate is claimed landed or installed here.

## Decisions and owner direction

### 1. Worktree ownership, not one agent per worktree

The **workspace operator/orchestrator** decomposes the approved goal and conducts or delegates implementation in independent guest worktrees. An assigned builder, a subagent, a temporary specialist or the operator can be the worktree owner. Separate builders are optional; the operator may handle small tasks directly. Preserve one active writer per checkout, however agents are assigned.

Worktrees normally stay local in the guest. When authorized, a specific branch can be promoted for examination on Mac `dev`; ordinary `wb sync` is not evidence that unpublished feature branches have crossed that boundary. Do not require each worktree to produce a GitHub PR.

### 2. Integration worktree and designated integrator

When a grouped delivery is ready, the operator creates and maintains a dedicated **integration worktree** and combines the verified source tips there. The operator may perform integration or delegate it to one named integrator, retaining responsibility for coordination. Shared target writes are serialized; the integration branch's history and combined checks belong to that new exact tip.

A candidate that merges reviewed heads is **not** automatically covered by those earlier reviews. Combined changes, conflicts, generated records, and module interactions require an integrated check and review.

### 3. One integrated PR and targeted remediation loop

For an **integrated delivery**, publish one integration draft PR; the independent reviewer reviews that unified result while source-worktree evidence remains traceable. Findings go back to the workspace operator, not as uncontrolled direct edits from the reviewer.

The operator chooses between:

- Sending component-specific findings with exact file, issue and acceptance condition back to the current worktree owners.
- Starting new bounded workers/subtasks where the original builder is unavailable or scope changes.
- Fixing small integration-specific problems directly in the integration checkout, or delegating that checkout to a dedicated integrator.

After changes, incorporate updated tips, rerun relevant source and combined checks, obtain a fresh review of the new integrated SHA, and **update the existing integration PR**. Do not open new component PRs merely to shuttle remediation, or silently graft integration-only changes onto unrelated builder histories.

### 4. Controller is selective, owner retains host authority

The repository operator runs normal planning, worker assignments, checks, integration and remediation autonomously. The **Workspaces Controller** handles cross-workspace dependencies, strategic reprioritization, significant blockers and completion/event reporting, not each worktree instruction. Owner retains final acceptance, Mac promotion, PR publication/merges, release/install, credentials and privileged gates.

### 5. Not every delivery is integrated

Publication is selected by **acceptance/release boundary** rather than agent count. If changes are independently acceptable or releasable, use **separate PRs** and optionally qualify their combined compatibility locally **without opening another aggregate PR**. If they genuinely constitute one delivery, use one integrated PR and never open component PRs by default. Do not collapse unrelated changes just to reduce PR count.

## Evidence and rationale

The motivating run exposed unnecessary repetition: a successful standing Cursor operator coordinated multiple worktrees, independent reviews and corrections; the owner then opened component drafts before receiving an additional integrated-head qualification. The integrated tip preserved both overlapping records-index contributions and reconciled `multiplex` versus `orchestration` in the validator. That is useful proof of integration practice, but its publication duplication should not be institutionalized.

The later Workstation proposals [#100](https://github.com/luckgrid/workstation/pull/100) (dev inbox/PR sequencing) and [#101](https://github.com/luckgrid/workstation/pull/101) (agile program and queue priorities) should be composed with the **delivery-unit selection before implementation**, not treated as installed code or as a mandate to Git-stack/rebase independent branches.

## Proposals

- Add a small, durable **delivery unit** correlation between existing task briefs, source tips, integrated tip, reviewers, checks, candidate PR and owner acceptance. This is a proposed work contract, **not** a new central state store or globally mandated schema.
- Decide `integrated` versus `independent` at kickoff; permit an explicit replan with provenance, without automatic duplicate PRs.
- Extend Workstation's existing `wb` and `wb-flow` handoff/reconciliation tools incrementally; no second orchestrator engine, daemon or new host privilege.
- Preserve source SHA-to-integration SHA mappings and immutable review/check coverage. Do not rewrite historical acceptance claims when a new tip appears.
- Test crash/restart, stale heads, duplicate PR prevention, one-writer ownership, review-route triage, and both publication modes with disposable fixtures.

These are implementation proposals derived from the owner's chosen operating pattern. They do not authorize new commands, file-exchange admission, installation or auto-merge.

## What's next

1. Review and accept/revise the portable [integration delivery architecture](../architecture/integration-delivery.md) and its Workstation counterpart.
2. Keep the ongoing Agents integrated candidate at its recorded tip until exact-head independent review and owner publication/merge decisions. This documentation does not revise that candidate or close component PRs.
3. Implement the narrow Workbench delivery lifecycle in a separate feature PR, using current profiles, Controller events, guest worktree rules, and host `wb draft` / `wb pr --update` mechanisms.
4. Prove the integrated and independent cases before selecting a default in automation; preserve durable records when worker panes or worktrees are cleaned.

## Closeout update — 2026-11-24

**Later outcome, appended to preserve the original pre-merge session snapshot:** the owner accepted the independent integrated review and squash-merged [Agents PR #24](https://github.com/luckgrid/agents/pull/24). GitHub confirms the reviewed integration head `30698403b8dda6bed440e3bdca93c8e828b6675e` was merged as `cf86b6ad3298c4468d1a11c07c3613670f991740` on `main` (2026-11-24). The independent review's verdict was **PASS WITH NOTES**: no blocking findings, with a post-merge M3/M4 STATE reconciliation still required. The earlier guest qualification results remain operator-reported; the reviewer did not independently execute the full guest check suite.

GitHub now confirms that the five original drafts [#18](https://github.com/luckgrid/agents/pull/18), [#19](https://github.com/luckgrid/agents/pull/19), [#20](https://github.com/luckgrid/agents/pull/20), [#21](https://github.com/luckgrid/agents/pull/21) and [#22](https://github.com/luckgrid/agents/pull/22) were **closed without separately merging**. Each original final head was independently checked as an ancestor of the integrated candidate, and each PR received a supersession comment referencing #24. Their discussion and review provenance remain available. GitHub branch searches for the five source branches and the integration branch returned no matches after the owner completed cleanup. The owner reports that Mac `dev` and guest cleanup also finished; those machine-local states were **not** directly inspected in this session.

Draft [#23](https://github.com/luckgrid/agents/pull/23) is the separate **integration-delivery architecture follow-up**, not part of the #24 squash. Keep it open for review against current `main` and reconcile any evolved records and current-state declarations before landing. M3/M4 source merged is **not** evidence that the corresponding Workbench pin, skills installation or runtime admission has occurred. File exchange and archive extraction remain unadmitted.

### STATE reconciliation (same documentation candidate)

The PASS-WITH-NOTES follow-up—stale M2/M3/M4 and validator positions in [STATE](STATE.md)—is addressed on this PR #23 tip: `main` @ `cf86b6a…` is recorded as **merged source** for council, orchestration, wiki, bulk-transfer and the issue-#12 validator line; pin/install remain open. Historical decision rows keep their original wording; a 2026-11-24 row records the squash without rewriting earlier observations. Issue-#12 closeout evidence is in [2026-11-24-issue-12-validator-closeout.md](2026-11-24-issue-12-validator-closeout.md) (GitHub close remains host-authorized).

### Follow-up workflow friction observed during cleanup

Closing five superseded PRs still left one `wb done REPO BRANCH` invocation and confirmation per branch. The owner requests **bounded bulk command handling**: inspect a named set once, approve the exact plan with a single `yes`, and automatically process eligible items until the first warning, stale precondition or failure, with durable per-item receipts and safe resume. Workstation owns the concrete `wb` design; the portable principle is grouped requests with explicit scope and per-item native guards, **not** a new agent permission or permission to execute arbitrary guest-authored scripts. See the companion [Workstation documentation PR #102](https://github.com/luckgrid/workstation/pull/102), whose detailed batch-operation proposal is part of this session follow-up.
