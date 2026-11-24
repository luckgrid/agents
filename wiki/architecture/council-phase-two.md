---
id: AS-ARCH-COUNCIL-PHASE-TWO
kind: architecture
title: Council phase-two specification
summary: "Proposed P7 expansion after M5 grill acceptance: collective deliberation, grill composition, evidence and dissent, model/context independence, and bounded delegated decisions. Not implemented in the grill delivery."
status: proposed
created: 2026-11-24
updated: 2026-11-24
related: [AS-ADR-0003, AS-PROGRAM, AS-ARCH-SKILL-CAPABILITY-EVOLUTION, AS-REC-2026-11-24-SKILL-SYSTEM-PLANNING]
---

# Council phase-two specification

**Status:** proposed specification for planning package **P7**. Implementation is gated on **M5 `grill` acceptance** (P6). Do not expand Council inside the grill delivery. This page is not runtime policy, a registry change, pin or install.

Baseline skills on `main` (#24): `council`, `council-review`, `council-remediate`. Direction: [AS-ADR-0003](../records/decisions/0003-portable-skill-capabilities.md) D3–D4 and the [capability architecture](skill-capability-evolution.md).

## Goals

1. Keep `grill` as the independent questioning primitive; Council **composes** it rather than forking a second grill.
2. Add collective deliberation, synthesis and bounded delegated decisions without treating agreement as authority.
3. Preserve evidence, dissent, uncertainty and escalation; record context independence separately from model/provider diversity.
4. Retain `council-review` and `council-remediate` as independently invocable roles.

## Non-goals

- Expanding the grill tip or merging Council changes into M5.
- Authorizing merge, pin, install, privilege, policy or publication by Council agreement.
- Requiring full Council for every routine reversible choice.
- Treating simulated multi-persona replies as independent reviews.

## Capability map

| Mode / role | Responsibility | Activation |
| --- | --- | --- |
| `council-review` | Independent critique against stated criteria | Existing leaf (retain) |
| `council-remediate` | Apply accepted findings within scope | Existing leaf (retain) |
| `council` (deliberate) | Multi-perspective debate and synthesis | Composition / base modes |
| Council × `grill` | Structured questioning of an answerable frontier | Compose grill; do not duplicate |
| `council-decide` (candidate) | Routine or explicitly delegated choice with receipt | Only if independent activation proves useful |

## Deliberation contract

For a deliberation:

1. **Question** — single decision or recommendation request, with scope and authority class.
2. **Evidence** — cited artifacts, exact tips, check results; mark missing evidence.
3. **Perspectives** — distinct roles or contexts; prefer real separate runs over simulated personas.
4. **Alternatives** — at least two concrete options when a choice is required.
5. **Synthesis** — summary that preserves dissent and uncertainty; no silent collapse.
6. **Outcome class** — one of: apply accepted rules; recommend; decide within delegation; escalate.

## Authority and escalation

| Class | Council may | Council must not |
| --- | --- | --- |
| Apply accepted rules | Restate and apply published criteria | Invent new policy |
| Routine reversible / in-budget | Decide with receipt when explicitly in scope | Expand scope silently |
| Explicitly delegated | Decide within written limits | Exceed limits or redefine them |
| Material / protected | Recommend and escalate | Merge, pin, install, publish, privilege, admit |

Protected operations remain owner/environment gates regardless of unanimous Council output.

## Independence requirements

- **Context independence:** separate worktrees, prompts or sessions where the claim depends on independence; record when simulation was used instead.
- **Model/provider diversity:** record known identities; unknown is allowed but must be labeled; diversity is not proven by role labels alone.
- **Dissent:** minority positions and unresolved uncertainty stay in the receipt.
- **Malformed receipts:** refuse to treat incomplete receipts as decisions; remediate or escalate.

## Decision receipt (minimum fields)

`question`, `scope`, `authority_class`, `evidence_refs`, `alternatives`, `decision_or_recommendation`, `rationale`, `dissent_or_uncertainty`, `conditions`, `reconsider_if`, `context_independence`, `model_provider_notes`.

## Acceptance proof (when implementing P7)

- Fixtures for routine decide, recommend-only, escalate, dissent retained, malformed receipt refusal.
- At least one grill-composed Council questioning path using the accepted grill contract.
- Simulated-versus-separate-context demonstration with explicit labels.
- No change to grill's independent invocability.
- Independent exact-head review of the Council tip alone (not bundled with grill unless owner chooses a later integrate).

## Distinctions

Proposed docs ≠ implemented skills ≠ pinned ≠ installed ≠ admitted. Grill acceptance unlocks P7 implementation work; this specification may land earlier as documentation only.
