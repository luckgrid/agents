---
id: AS-ADR-0002
kind: decision
title: Name M3 orchestration
summary: "The owner amends the planned M3 namespace from `multiplex` to `orchestration`: portable coordination of agents, tasks, workflows, handoffs and execution systems, with Herdr as one supported implementation environment rather than the namespace definition. Historical records retain the original plan."
status: accepted
created: 2026-11-23
updated: 2026-11-23
related: [AS-PROGRAM, AS-ARCH-WORKSPACE-COORDINATION, AS-ARCH-AGENTS-WORKSPACE, AS-ADR-0001]
---

# Name M3 orchestration

**Status:** accepted by the owner for this branch (2026-11-23; authoritative
branch amendment).

## The owner's decision

The owner replaces the planned M3 namespace **`multiplex`** with
**`orchestration`**.

- `orchestration` means reusable coordination of agents, tasks, workflows,
  handoffs, and execution systems.
- Herdr is a supported implementation environment, not the definition of the
  namespace.
- Update the authoritative roadmap, architecture mentions that prescribe the
  M3 module name, the namespace registry, and `scripts/check.sh` together.
- Preserve historical evidence: existing dated records that say `multiplex`
  continue to describe the original plan and are not rewritten.

## Consequences

M3 ships as `skills/orchestration/`. Its guidance defines portable coordination
and maps supported environments with explicit capability limits. Herdr's
mapping checks `HERDR_ENV=1` and its version floor, then defers to installed
`herdr --skill` and relevant help. The roadmap and registry use `orchestration`;
the validator uses the same namespace. No adapter, daemon, TUIOS requirement,
or Herdr-core dependency follows from this naming decision.

The original 2026-11-12 namespace choice and subsequent `multiplex` proposal
remain historical evidence. This amendment changes current M3 naming and scope
without rewriting that record.
