---
id: AS-ADR-0003
kind: decision
title: Evolve Agents into a portable capability system
summary: "Owner accepts D1–D8 and D10; revises D9 to multiple session capability archetypes, including session-view rather than session-window. Target conduct, independent grill, collective Council, write, selectable wiki profiles, staged AG1, narrow math and verification, dependency-driven delivery. No runtime admission."
status: accepted
created: 2026-11-24
updated: 2026-11-24
related: [AS-ADR-0001, AS-ADR-0002, AS-PROGRAM, AS-STATE, AS-ARCH-SKILL-SOURCES, AS-ARCH-SKILL-CAPABILITY-EVOLUTION, AS-REC-2026-11-24-SKILL-SYSTEM-PLANNING]
---

# Evolve Agents into a portable capability system

**Owner decision, 2026-11-24:** D1–D8 and D10 accepted. D9 initially returned for revision, then amended by the owner to capability archetypes, with `session-view` rather than `session-window` and multiple session capabilities. All ten now have an explicit disposition. This is accepted direction, not completed implementation, reviewed migration, or installed runtime.

See the [research brief](../2026-11-24-skill-system-research-brief.md), [planner assessment and work packages](../2026-11-24-skill-system-planning.md), and [target architecture](../../architecture/skill-capability-evolution.md). Source baseline is Agents #24 squash `cf86b6ad3298c4468d1a11c07c3613670f991740`.

## Owner dispositions

| ID | Disposition | Direction |
| --- | --- | --- |
| D1 | Accepted | Adopt `conduct` as the target M3 namespace in a compatibility-aware responsibility refactor. |
| D2 | Accepted | Separate portable methods from backend implementation, including workspace; preserve working Herdr behavior through appropriate bindings. |
| D3 | Accepted | Retain independent `grill`; Council composes the questioning method rather than duplicating it. |
| D4 | Accepted | Expand Council into collective reasoning, deliberation, and bounded delegated decisions, preserving evidence, independence, dissent and owner authority. |
| D5 | Accepted | Introduce `write` as a foundational capability distinct from wiki topology and knowledge governance. |
| D6 | Accepted | Use selectable wiki profiles; reconcile the actual Luckgrid writing/wiki authorities before implementing the Luckgrid profile. |
| D7 | Accepted | Split AG1 into Rust validator parity (AG1.1), then validation expansion (AG1.2). |
| D8 | Accepted | Keep math and verification narrowly scoped; ground math in concrete uses and evaluate verification as a shared method before introducing a standalone namespace. |
| D9 | Revised by owner | Organize session/interface skills by archetype capability, not tool name. Prefer **`session-view`**, not `session-window`. Provide multiple session capabilities, not a single undifferentiated module. |
| D10 | Accepted | Dependency-driven parallel work, stable historical roadmap IDs, and publication boundaries based on independently acceptable deliveries, not worker count. |

## D9 — session capability family

The owner wants a capability-oriented public family for session interfaces, views and related uses. Herdr, Kitty, Herdr Core and other interface/session providers are possible realizations, not the namespace taxonomy. A view is more universal than a window; it may map to a window, tab, pane, split, document surface or another supported interface surface. Do not assume those native entities have identical semantics. Multiple distinct session capabilities should be specified. The proposed family in the architecture page includes interface discovery, view management, layout, navigation, observation, lifecycle, interaction and recovery. Apart from the owner's `session-view` naming preference, exact names, mode-versus-leaf structure and capability contracts are proposed for evidence-backed specification.

No concrete backend commands belong in general `conduct`. Keep session/interface mechanics separate from repository/worktree identity (`workspace`), task coordination (`conduct`), and model reasoning (`council`). Public backend mappings may document supported native behavior; Workstation owns its actual binding, versions, runtime, credentials, authority, policies and installation. Preserve native semantics, report unsupported features, and prove equivalent behavior before retiring existing integrations.

### Further owner refinement (2026-11-24)

Replace tmux and Zellij as preferred future comparison targets with Kitty and Herdr Core: lower-level/native interface and session infrastructure references for the session archetype family, alongside the current Herdr integration. Kitty supplies native terminal/view and layout examples; Herdr Core supplies PTY, session and client/API examples. The distinction is intentional; neither implies all capabilities of the other. Tool-specific mapping remains separate from portable methods. This does not retire existing runtime fallback paths, authorize either new installation, or alter historical evidence.

## Consequences

D1 amends the future naming target of [AS-ADR-0002](0002-orchestration-namespace.md); D2 and D9 refine its scope. Preserve historical wording, existing anchors and shipped source facts until migration. Update the actual registry, source, validator, consumers and installation evidence together at cutover; do not register an unimplemented target as installed.

Council may resolve routine, reversible, policy-covered or explicitly delegated choices within scope. Material out-of-scope choices and protected operations still require the authorized owner or environment. Agreement does not establish correctness or create authority. No merge, release, deployment, install, privilege, policy, pin, hook, dependency, file-exchange or archive-extraction admission is authorized here.

The [program](../../program.md#skill-capability-evolution-owner-2026-11-24) and planning record define the work packages. Their exact interfaces and implementation designs require scoped evidence, independent review and compatibility proofs. The documentation PR itself remains subject to review and merge.
