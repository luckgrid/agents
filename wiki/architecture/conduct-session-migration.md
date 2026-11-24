---
id: AS-ARCH-CONDUCT-SESSION-MIGRATION
kind: architecture
title: Conduct and session capability migration specification
summary: "Proposed migration from orchestration/workspace consumers to portable conduct and the session-view family: boundaries, identity model, Kitty/Herdr Core references, Herdr compatibility, missing-capability behavior, validation and rollback. No new backends installed."
status: proposed
created: 2026-11-24
updated: 2026-11-24
related: [AS-ADR-0003, AS-ADR-0002, AS-ARCH-SKILL-CAPABILITY-EVOLUTION, AS-REC-2026-11-24-SKILL-SYSTEM-PLANNING, AS-ARCH-WORKSPACE-COORDINATION]
---

# Conduct and session capability migration specification

**Status:** proposed (planning packages **P4** and **P5a**). Accepted direction is [AS-ADR-0003](../records/decisions/0003-portable-skill-capabilities.md). This does **not** install Kitty, Herdr Core or any new backend; does **not** remove working Herdr paths; does **not** introduce a universal runtime abstraction.

Consumer inventory baseline: capability-run `evidence/associated-file-audit.md` on Agents `#24` tip `cf86b6ad3298c4468d1a11c07c3613670f991740` (orchestration ~13 refs; workspace family ~48+ leaf refs).

## 1. Portable conduct responsibilities

| Responsibility | Stays in `conduct` (target) | Leaves `conduct` |
| --- | --- | --- |
| Decompose goals, plan, delegate, supervise, integrate, recover work | Yes | — |
| Task/workflow/handoff coordination methods | Yes | — |
| Concrete session backend commands (Herdr/Kitty/…) | No | Session family + environment adapters |
| Repository/worktree identity and Git maintenance | No | `workspace` |
| Host promote/check/PR/pin/install | No | Workstation / owner |
| Model deliberation and delegated decisions | No | `council` (+ `grill`) |

Compatibility mapping from shipped names:

| Current (shipped / ADR-0002) | Target | Migration rule |
| --- | --- | --- |
| `orchestration` namespace | `conduct` | Rename/refactor with dual naming or aliases until consumers update; preserve anchors in historical records |
| `orchestration` Herdr examples | Session adapters / Workstation binding | Strip concrete commands from general method; keep environment-specific mapping docs |
| `workspace-*` calling orchestration or herdr-direct | `conduct` for coordination intent; session/workspace for mechanics | Callers keep herdr-direct fallback until session binding proven |
| AS-ADR-0002 text | Amended by ADR-0003 D1 for future naming | Preserve historical ADR-0002 wording; new cutover updates registry together |

## 2. Session capability boundaries

Public family is capability-oriented, not tool-named. Candidate contracts (exact leaf vs mode packaging still open):

| Capability | Owns | Does not own |
| --- | --- | --- |
| `session` | Shared concepts, discovery, guardrails | Git identity, inference context |
| `session-interface` | Surfaces, available operations, interaction bounds | Work planning |
| `session-view` | Logical views (pane/tab/window/document/…) | Process ownership inference |
| `session-layout` | Arrangements when supported | Inventing unsupported splits |
| `session-navigate` | Focus/switch/traverse by verified ids | Guessing stale ids |
| `session-status` | Read-only freshness/activity/capability | Mutating sessions |
| `session-interact` | Bounded I/O with readiness checks | Typing into foreign panes without authority |
| `session-lifecycle` | Create/attach/detach/end where supported | Host publication |
| `session-recover` | Stale association reconcile | Destructive cleanup without workspace rules |

## 3. Identity model

Treat these as **separate** identities even when a backend links them:

| Identity | Meaning |
| --- | --- |
| Session | Durable or attachable interaction context in a backend |
| View | One logical surface (not necessarily a window) |
| Process | OS/PTY process, if any |
| Agent | Harness/agent instance and inference context |
| Workspace | Repository / herdr workspace / worktree checkout identity |

Never infer agent ownership from a visible view title alone. Never collapse workspace dirty-state policy into session layout restore.

## 4. Preferred future implementation references

| Reference | Informs | Does not authorize |
| --- | --- | --- |
| [Kitty sessions](https://sw.kovidgoyal.net/kitty/sessions/) / [remote control](https://sw.kovidgoyal.net/kitty/remote-control/) | Native terminal views, layout, navigation, interface control | Installing Kitty in the guest or replacing Herdr |
| [Herdr Core](https://herdr.dev/core/) | PTY, session lifecycle, client/API control | Installing Herdr Core or equating it with Kitty |
| Current Herdr TUI | **Operational** Workbench binding today | Claiming full coverage of the portable family |

Compare versioned capabilities as native, safely composed, unavailable or unverified. Do not force equivalent semantics across incompatible systems.

## 5. Current Herdr compatibility

- Keep existing `workspace-*` and Workbench herdr paths working through migration.
- Workstation owns socket, version floor, agent ownership, permissions and host/guest controls.
- Portable docs may describe Herdr mappings; they must not become the only functioning path's silent removal trigger.
- tmux (and any historical Zellij research) remain intact until a **separate** reviewed runtime migration.

## 6. Missing-capability behavior

For each operation define: intent, target identity, required capability, authority, preconditions, freshness, output, failure, recovery.

When a backend lacks a capability:

1. Prefer an **allowed alternative** (e.g. status-only, manual owner step).
2. Otherwise return an **explicit limitation** (machine-readable where receipts exist).
3. Never guess mutations, never invent ids, never no-op success.

## 7. Consumer migration, validation and rollback

### Migration stages

1. **Specify** — this document + architecture contracts; registry still lists shipped names.
2. **Dual path** — new portable methods alongside herdr-direct / orchestration; consumers opt in.
3. **Prove** — read-only status first; then isolated open/create/focus; then lifecycle; then guarded maintenance.
4. **Cut over** — update registry, validators, credits and live pins together under owner authority.
5. **Retire** — remove fallback only after compatibility evidence and pin pair known.

### Validation

- Inventory every consumer (skills, wiki, Workstation adapters, roles, hooks).
- Regression: absent backend, stale id, dirty worktree refusal, unsupported layout.
- No universal runtime abstraction layer in Agents; adapters stay environment-owned.

### Rollback

Roll back to a known compatible **source + consumer + pin** triple. Never remove the only working Herdr path in the same change that introduces an unproven binding.

## 8. Workstation adapter requirements (Controller coordination)

Concrete adapter work is **out of Agents source** and must be requested through the Controller for Workstation:

| Need | Notes |
| --- | --- |
| Capability discovery API surface | Map portable operations → Herdr (and later optional Kitty/Core) with version gates |
| Explicit unsupported results | Match §6; no silent success |
| Preserve current operator MCP bounds | Do not widen pane authority via Agents docs |
| Pin/install sequencing | Agents draft ≠ Workstation pin; first pin remains separate |
| No new backend install from this spec | Kitty / Herdr Core remain comparison references until owner admits them |

Controller note: `~/…/state/runs/20261124-agents-capability/notes/workstation-session-adapter-request.md`.

## Distinctions

Proposed migration ≠ implemented `conduct`/`session-*` skills ≠ merged ≠ pinned ≠ installed ≠ admitted.
