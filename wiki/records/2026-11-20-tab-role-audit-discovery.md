---
id: AS-REC-2026-11-20-TAB-ROLE-AUDIT
kind: session
title: Control–Record–Audit role and profile discovery
summary: "Discovery-only: portable vocabulary for harness profiles vs tab roles vs audit assignments; Audit Controller and Record duties; handoff artifacts; council one-lens fit. Layout/ratios remain Workstation. No live session rearrange."
status: active
created: 2026-11-20
updated: 2026-11-20  # standards review minors: records index + evidence locators
related: [AS-STATE, AS-ARCH-CONTROL-RECORD-AUDIT, AS-ARCH-WORKSPACE-COORDINATION, AS-PROGRAM]
---

# Control–Record–Audit discovery (2026-11-20)

## Intent

Coordinate discovery and wiki design before broader implementation of workspace Control / Record / Audit tabs. Agents owns role/profile distinctions and handoff contracts. Workstation owns herdr/layout capabilities and configuration. Preserve existing sessions; continue useful review pilots in parallel; keep publication batched and context retention default.

## What was checked

| Source | Result |
| --- | --- |
| Controller handoff `~/…/records/controller/handoffs/2026-11-20-workspace-tabs-and-profile-discovery.md` | Assignment text; owner tab templates; five conceptual layers |
| Council tip `91fc63aca70d43543f43fc758ef488368e0373a8` on `feat/retain-worker-context`; profiles catalog `~/…/records/agents-workspace/REVIEW-PROFILES.md` | Execution profiles, emit adapter, dual-profile pilot evidence |
| `wiki/architecture/workspace-coordination.md`; `skills/workspace-handoff/**`; `skills/workspace-maintain/references/retention.md` | Existing portable handoff and durable-evidence rules |
| Guest note `~/…/records/agents-workspace/M3-MULTIPLEX-DESIGN.md` | Design-only; herdr sequence abstraction later |
| Guest note `~/…/records/workstation-herdr-capabilities.md` | Live-proved herdr ops; Cursor reviewer BLOCKED; not a tab-template matrix |
| Guest note `~/…/records/workstation-tab-profile-discovery.md` | **Not present yet** — coordinate when Workstation exports |

Distinguished: **owner direction** (templates, Control default, Audit Controller + specialty labels), **interpretation** (2–4 specialist panes as grid meaning), **proposal** (this record / architecture page), **not implemented** (layout templates, dedicated audit skill).

## Decisions (agents scope)

- Publish portable architecture [Control, Record and Audit](../architecture/control-record-audit.md) as **proposed**.
- Keep Workbench paths, 80/20 ratios and tab create/idempotency in Workstation; public skills define responsibilities/contracts only.
- Audit Controller = council + tab-role brief / existing coordinator pattern until a proved gap needs a new skill.
- Record = lightweight index/continuity role using records/run locators; not a second integrator.
- Council remains one-lens-per-turn; specialty labels map to assignment packages of turns, not collapsed multi-lens turns.
- Security / Design / Product are examples, not mandatory agents for every change.
- Defer broad skill implementation; proposed slices listed on the architecture page.
- Do not rearrange current long-running herdr sessions from this discovery.

## Evidence

- Architecture page added: `wiki/architecture/control-record-audit.md`.
- Export: `~/…/records/agents-workspace/tab-role-audit-discovery.md` (guest coordination export; not a wiki page).
- Walkthrough scenario on the architecture page (Control request → two Audit perspectives → Record index → later Control resume).
- No history prune; no layout apply; no `~/.config/herdr` edits.

## Proposals

- When Workstation exports capability/layout discovery, reconcile open questions (lazy vs eager Record/Audit tabs; confirmed specialist grid; herdr API gaps) into a joint follow-up note—do not invent herdr features here.
- Next agents source slice (separate brief): optional aggregate-receipt fields for omitted perspectives; Record index fields on handoff templates—only after contract agreement.

## What's next

- Workstation: `workstation-tab-profile-discovery.md` and wiki/layout design.
- Agents: hold this tip in the publication queue; continue review/retention pilots without waiting on tab templates.
- Joint QA/prototype milestone later: idempotent Control template on a **new** disposable workspace (not overwriting standing sessions), plus one Audit two-perspective run wired through existing council emit.
