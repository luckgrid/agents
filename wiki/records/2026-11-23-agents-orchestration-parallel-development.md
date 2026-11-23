---
id: AS-REC-2026-11-23-AGENTS-ORCHESTRATION
kind: session
title: Agents orchestration decision and parallel Workbench delivery
summary: "The owner broadened M3 from multiplex to orchestration and directed a standing Workbench repository operator. One operator coordinated validator, M3, and M4 candidates, now draft PRs #19–#21; a separate independent review is pending."
status: active
created: 2026-11-23
updated: 2026-11-23
related: [AS-STATE, AS-PROGRAM, AS-ARCH-WORKSPACE-COORDINATION, AS-ARCH-AGENTS-WORKSPACE, AS-REC-2026-11-18-WORKSPACE-HANDOFF, AS-REC-2026-11-20-TAB-ROLE-AUDIT]
---

# Agents orchestration and parallel development (2026-11-23)

## Intent

Continue developing the public Agents skills package from the Workbench guest using autonomous, parallel agents, with the human owner steering scope and accepting results instead of manually operating each worker. The repository operator should coordinate the work, preserve evidence and return exact-head publication packets through Workbench's owner-controlled handoff.

The owner then changed **M3 from `multiplex` to `orchestration`**. The name should express coordination of agents, tasks, workflows and execution systems, not just multiplexing terminal sessions. Herdr can be an initial integration, but neither Herdr nor a terminal tool defines the namespace. This is intended to remain useful beyond coding and terminal workflows. Broader product, marketing and business skills were mentioned as future directions, **not** authorized M3 deliverables.

The owner also requested a kickoff using `/workbench` and explicit workspace-operator responsibilities, including the correct standing profile. The operation was to use Workbench's existing **`operator` profile for `agents`**, rather than inventing an `orchestrator` profile. This role is a repository-level execution coordinator under the separate Workspaces Controller; the human owner retains acceptance and publication.

## What was checked

The ChatGPT session fetched repository/wiki guidance and GitHub pull-request metadata and changed-file lists. It did **not** inspect the guest's local run folders or rerun guest checks.

| Source | Observation in this session |
| --- | --- |
| `agents` `main` | `ffd9c5dbb18a7f7ff8cb46fa39cfc9f02b9b1355`, the reported common candidate base |
| [Issue #12](https://github.com/luckgrid/agents/issues/12) | Three validator gaps: multiline Markdown link targets, YAML inline comments, and mode tables without outer pipes |
| [PR #19](https://github.com/luckgrid/agents/pull/19) | Draft, head `6930c6570dc18c70d84bb5414269896b4b70cea9`; `scripts/check.sh` only |
| [PR #20](https://github.com/luckgrid/agents/pull/20) | Draft, head `b826d07c32890e5de4c2bb2cd9257a13f02d7e67`; orchestration module, namespace/decision/architecture/roadmap updates, and `scripts/check.sh` |
| [PR #21](https://github.com/luckgrid/agents/pull/21) | Draft, head `0f907e09b14f0f1fea8b875b8da0712b3312056b`; wiki skill and its eight mode references |
| [PR #18](https://github.com/luckgrid/agents/pull/18) | Separate open draft; not part of the parallel delivery or this review |
| Workstation `profiles.toml` on `main` | Per-repository standing `operator` profile: `--repo agents --mode cursor` selects Cursor Auto and base `workbench` skill |
| Workstation `workbench` and guest rules | The Workspaces Controller dispatches requests; the repository operator coordinates worktrees and child agents; guest feature writes belong in authorized worktrees, not the coordination clone |

The [wiki writing standard](../README.md) requires evidence types to remain distinct and the [STATE](STATE.md) register to own the current position. The existing [program](../program.md) still describes historical `multiplex` planning on `main`; the proposed current rewrite is in PR #20, not accepted as of this record.

## Decisions

### Owner: orchestration replaces multiplex

The owner's question was how to make M3 less technical and less bound to any single terminal tool. The answer for the current plan is the **`orchestration`** namespace, with Herdr/tmux as possible runtime-specific mappings rather than a naming constraint.

- Propose `skills/orchestration/` instead of the planned `skills/multiplex/`.
- Update current registry, roadmap, architecture and decision references inside the M3 feature worktree; preserve historical wording with its context.
- Do not create a second Controller, runtime engine, or speculative adapter framework merely to satisfy the broader name.
- PR #20 includes proposed `AS-ADR-0002` (`wiki/records/decisions/0002-orchestration-namespace.md`). It is not a merged decision file on `main` at the time of this record.

### Owner: a standing repository operator, not a second Controller

The existing Workbench `operator` profile is the authorized standing entry for the Agents workspace; in this run the preferred harness was Cursor Auto, in part because the owner's reported Claude usage was exhausted. `/workbench` loads the Workbench entry skill; installed guest rules remain authoritative. The planned `workbench-orchestrate` role skill is not a prerequisite and must not be treated as installed.

The standing operator is responsible for planning bounded repository tasks, assigning builders and independent reviewers, worktree ownership, status and evidence, checks and recovery, and a final handoff to the Workspaces Controller and owner. It does not publish or merge from the guest. One standing operator coordinates the repository; workers do not become competing repository operators.

### Session execution plan

The first concurrent lanes were issue #12's validator correction and M3 orchestration; M4 wiki was queued for the next available builder slot. PR #18 remained separate. The controller's existing request and event flow was preferred over restarting sessions, modifying immutable dispatched requests, or copying agent messages between panes.

## Evidence

**Owner-supplied Controller and operator reports, not independently reproduced in this wiki-writing session:**

- One standing Cursor Auto operator coordinated two builders in parallel, then started M4 when a slot became available.
- Claude hit a usage limit; Codex builders and fresh read-only Codex reviewers were used instead.
- All three candidate heads were reported to share the base `ffd9c5dbb18a7f7ff8cb46fa39cfc9f02b9b1355`.
- Validator #12: `6930c6570dc18c70d84bb5414269896b4b70cea9`; reported 44 self-tests, full validator and independent review passing.
- M3 orchestration: `b826d07c32890e5de4c2bb2cd9257a13f02d7e67`; reported validator, self-tests, changed-file Markdown lint and independent review passing.
- M4 wiki: `0f907e09b14f0f1fea8b875b8da0712b3312056b`; reported validator, self-tests, module Markdown lint and independent review passing.
- The owner subsequently published these exact heads as draft PRs #19, #20 and #21, as independently confirmed by GitHub metadata in this session.
- No merge, installation, pin, cleanup or acceptance was reported. The superseded M3 `multiplex` branch and packets were preserved as historical context.

The Controller's reported terminal result was named `20261123-agents-exact-head-publication.md`, stored in its guest records. That guest-local receipt was **not fetched or verified** here. Reports of completed reviews are prior review claims, not independent acceptance.

**Independent review kickoff prepared in this ChatGPT session:** a separate read-only GitHub review of the exact three PR heads, with parser-edge-case coverage for #19, architecture and namespace boundaries for #20, wiki authority/provenance for #21, and a **cross-PR merge assessment**. The shared file `scripts/check.sh` changes in both #19 and #20; their combined behavior must be checked even if the isolated changes pass. No findings or independent verdict are recorded yet.

## Proposals

- Prefer a staged acceptance order: #19 validator, #20 orchestration, #21 wiki, subject to independent findings and fresh exact-head compatibility checks after each squash merge.
- Keep public `orchestration` guidance distinct from Workstation's installed operator/Controller machinery and from any future workflow engine.
- Reuse the standing operator and Controller handoff for next development lanes rather than adding redundant roles or another status system.
- Once accepted changes land, do a **serial** update of STATE and shared indexes against the then-current `main` so overlapping feature branches do not each publish conflicting status.

These are recommendations and sequencing proposals, not authorization to merge or install.

## What's next

1. Complete the separate independent ChatGPT review of [#19](https://github.com/luckgrid/agents/pull/19), [#20](https://github.com/luckgrid/agents/pull/20) and [#21](https://github.com/luckgrid/agents/pull/21). The review is read-only and must verify exact heads, actual source and cross-PR integration.
2. If defects are found, hand bounded remediation briefs to the existing Agents operator; recheck changed exact heads independently.
3. The owner decides acceptance, publication and merge order. Leave [#18](https://github.com/luckgrid/agents/pull/18) on its separate track.
4. After accepted merges, reconcile [STATE](STATE.md), the roadmap and indexes in a serial docs pass; do not predeclare proposed source as installed functionality.
5. Treat pinning and installing Agents skills into Workbench as a separate owner-controlled integration/verification gate. The guest's current success does not establish live skill installation.
