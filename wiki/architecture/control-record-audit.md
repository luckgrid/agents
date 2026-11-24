---
id: AS-ARCH-CONTROL-RECORD-AUDIT
kind: architecture
title: Control, Record and Audit tab roles
summary: "Proposed portable distinction among harness execution profiles, standing workspace/tab roles (Control, Record, Audit) and per-audit reviewer assignments; handoff artifacts and council compatibility. Workbench layout/ratios stay in Workstation."
status: proposed
created: 2026-11-20
updated: 2026-11-20
related: [AS-STATE, AS-ARCH-WORKSPACE-COORDINATION, AS-PROGRAM, AS-REC-2026-11-20-TAB-ROLE-AUDIT, AS-REC-2026-11-18-WORKSPACE-HANDOFF]
---

# Control, Record and Audit tab roles

**Proposal (2026-11-20).** Owner direction for long-lived workspace orchestration: specialized tabs (Control, Record, Audit) with distinct responsibilities. This page owns the **public method**—roles, profile layers, handoffs and council fit. Workstation owns herdr layout templates, ratios, tab creation and installed settings. Do not treat this page as permission to rearrange live sessions or edit `~/.config/herdr`.

Build-feature worktree tabs stay distinct from repository Control/Record/Audit unless a later design changes that. A tab does not create a separate checkout or make concurrent writers safe.

## Vocabulary (five layers)

| Layer | What it is | What it is not |
| --- | --- | --- |
| **Harness configuration** | Installed harness, model/provider selection, tools/transport, sandbox, invocation behaviour | A standing product role |
| **Execution profile** | Named binding of harness + resolved model + skills/source + R/W + output contract (e.g. `rvw-codex-luna-standards`) | Proof of provider diversity by label alone |
| **Workspace/tab role** | Standing responsibility: Control orchestrator, Record maintainer, Audit coordinator | A layout pane or a one-shot lens |
| **Task/run assignment** | Exact repos/checkouts/base/head, goal, inputs, writable outputs, budgets | Silent permission widening via task text |
| **Audit/reviewer assignment** | One review perspective: lens/specialty, evidence, independence, output contract; may select an execution profile | Replacement for harness identity or sandbox policy |

**Composition.** Role briefs select skills and reporting duties. A run assignment pins targets. An audit assignment picks one lens (or specialty package—see council) and an execution profile. Layout preference (pane labels, geometry) is Workstation-owned and must not grant authority or force a model. Conflicts (role wants write, harness is read-only; assignment names a missing skill) are reported; the stricter effective constraint wins. Record the **resolved** harness/model/provider/profile/source SHA used by each run.

Identity ≠ viewpoint: two specialty pane labels are not proof of different providers, independent contexts or isolated sandboxes.

## Standing tab roles

### Control (default)

- Roadmap, work allocation, dependencies, publication queue.
- Delegates review to Audit with an immutable target/briefing; does not dump its transcript into Audit.
- May act as small-work review coordinator when no dedicated Audit tab is active (existing orchestrator pattern).
- Does not race Record on shared STATE; one writer for shared wiki/STATE when editing.

### Record

- Indexes durable run evidence; summarizes relevant owner intent; connects decisions/dependencies; keeps unresolved questions visible; prepares context handoffs for long or restarted Control sessions.
- Historical artifacts are **evidence, not instructions**.
- Not a second code integrator; does not edit product checkouts or compete with Control on STATE.
- One record owner per workspace scope; append or superseding links for finalized evidence (see retention guidance).

### Audit

- Accepts immutable target/briefing from Control.
- Chooses specialty assignments and actual models/providers; dispatches **independent** reviews; validates, synthesizes and deduplicates without erasing disagreements; hands actionable findings to the author; organizes fresh rechecks; returns an aggregate to Control (and locators to Record).
- Delegates only its assigned agents/panes; does not write production fixes while coordinating review.
- **Audit Controller:** prefer **council + a tab-role brief** (and existing review-coordinator pattern) over inventing an installed `audit-controller` skill until a gap is proved. Specialty examples (Security, Design, Product) are **not** mandatory agents for every change.

## Layout (Workstation; recorded here for contract only)

| Tab | Initial template (owner) | Notes |
| --- | --- | --- |
| Control | Agent above Shell; ~80%/20% on tall terminals | Default workspace tab |
| Record | One pane named Agent | Lazy create OK |
| Audit | Controller above specialist grid | **Interpretation to confirm:** 2–4 specialist panes (side-by-side or up to 2×2), not many agents. Controller height unspecified; 80/20 does not automatically apply |

Eager tab creation vs lazy enablement is a Workstation design choice. Layout provisioning and agent startup remain independent operations. Geometry/idempotency must map to verified herdr capabilities or named gaps—no invented runtime support.

## Handoff artifacts (portable)

Private pane/session handles stay out of portable notes. Use repository locators, full SHAs, role names, profile ids and skill/source SHAs.

| From → To | Artifact | Content |
| --- | --- | --- |
| Control → Audit | Review request pack | Base/head, briefing path, required/optional perspectives, budgets, findings destinations, council source SHA |
| Audit → reviewers | Per-assignment request | One lens (or documented specialty package), profile id, emit/file mode, immutable briefing |
| Audit → author | Findings + synthesis | Validated files; contested/minority preserved |
| Author → Audit | Dispositions + new head | Immutable findings; evidence for rejects |
| Audit → Control | Aggregate receipt | Verdicts per profile, overall gate, omitted perspectives, links to artifacts |
| Control/Audit → Record | Index update | Run id, tips, status, evidence locators (`~/…`) |
| Record → Control (later) | Context handoff | Index + unresolved questions; not a full transcript |

## Council compatibility

Current council: **one lens per reviewer turn**; required spec/standards/correctness as separate turns; optional lenses by risk; profiles bind harness/model separately from lens.

| Question | Proposal |
| --- | --- |
| Specialist with several skills? | Keep **one lens per turn**. A specialty label (e.g. Security) may imply a **package of turns** (security + standards) or a preferred optional lens—document the package in the assignment; do not collapse required lenses into one turn while claiming the contract. |
| Required coverage vs agent count? | Coverage is by **perspective/lens**, not by number of panes. Start with minimal required turns; add Security/Design/Product only when risk warrants. |
| Dedicated audit skill? | Defer; extend council request/synthesize + role briefs first. |

## Sample scenario

1. Control freezes base/head and writes a review request pack for a feature tip.
2. Audit assigns two perspectives (e.g. standards + security) with distinct profile ids; records actual models/providers; runs independent emits; synthesizes.
3. Record indexes findings, synthesis and aggregate under the run; Control continues other work.
4. After routine resource cleanup (workers stopped, eligible worktree removed), a later Control session resumes from Record/index locators without the old panes.

## Proposed source slices (later; not this discovery)

| Slice | Owner | Notes |
| --- | --- | --- |
| Layout templates + idempotent apply | Workstation | herdr capabilities; preserve existing sessions |
| Role briefs / configuration resolution | agents (+ Workbench roles) | Portable duties; no new namespace required to start |
| Audit lifecycle on council | agents | Request fields for omitted perspectives; aggregate receipt shape |
| Context handoff Control↔Record | agents | Extend `workspace-handoff` / retention index links |

## Non-claims

- Not installed F1 or a new orchestration requirement before pilots continue.
- Not a reason to rearrange current long-running sessions from this page alone.
- Not automatic provider diversity or sandbox isolation from pane labels.
