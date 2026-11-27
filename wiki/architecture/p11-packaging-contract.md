---
id: AS-ARCH-P11-PACKAGING-CONTRACT
kind: architecture
title: P11 packaging and receipt contract
summary: "Proposed minimum P11 contract for one canonical resource with self-contained copies in two skills: identity, layout, digest and revision parity, receipt fields, missing-resource and installed-folder fixtures. Distinct from U16 folder ownership and bulk-transfer package digests. Consumer-pair selection and acceptance remain with the owner. Not implementation, pin, install or cutover."
status: proposed
created: 2026-11-27
updated: 2026-11-27
related: [AS-ADR-0003, AS-REC-2026-11-24-SKILL-SYSTEM-PLANNING, AS-ARCH-SKILL-CAPABILITY-EVOLUTION, AS-ARCH-SKILL-SOURCES, AS-ARCH-GUEST-INSTALL, AS-PROGRAM, AS-STATE, AS-ARCH-AG12-CHECK-CONTRACT]
---

# P11 packaging and receipt contract

**Status: proposed.** Planning package **P11** (resource packaging proof) is not adopted until the owner accepts a contract and a two-consumer use case. This page states the **minimum** contract so AG1.2 digest-parity checks (planning **P12**) have something to adopt later. It is not validator code, Moon admission, checker cutover, pin, install, merge or publication.

Written against Agents `main` at `adc1972cc6708fa8204506783c365904695d7e7d`. Since then `main` gained the STATE reconciliation (#29) and the [AG1.2 check contract](ag12-check-contract.md) (#30), whose digest-parity checks wait on this page; neither changes the contract below.

## Non-goals and domain boundaries

| Domain | What it is | Not P11 |
| --- | --- | --- |
| **P11 skill-resource packaging** | One canonical authoring resource; deterministic self-contained copies inside two skill folders; content/revision parity | — |
| **U16 folder ownership digests** | Workbench records a digest per **installed skill folder** for ownership ([guest install](guest-install.md)) | Whole-folder install receipt, not shared-resource parity |
| **Bulk-transfer package digests** | Exchange artifact identity, inventory and checksums (`workspace-handoff` bulk-transfer) | Host file exchange, not skill-folder resources |
| **Council / handoff process receipts** | Review and session evidence retention | Process outcomes, not packaging |

Do not reuse U16 or bulk-transfer receipt shapes as P11 resource receipts without an explicit owner amendment.

## Minimum contract

### 1. Identity

| Field | Rule |
| --- | --- |
| `resource_id` | Stable token (ASCII, no paths), unique in the contract registry for this repository |
| `title` | Short human label |
| `canonical_digest` | Hex digest of the canonical resource bytes under the declared algorithm and manifest order (this is the parity value) |
| `revision_label` | Optional human or owner pin label (for example a short version string); **never** compared to digests |
| `consumers` | Exactly two skill folder names that must carry a self-contained copy |

The owner selects the `resource_id` and the two consumers when accepting a use case. This page may only **propose** pairs (below). Parity always uses `canonical_digest`, not `revision_label`.

### 2. Layout (self-contained copies)

| Rule | Behavior |
| --- | --- |
| Copy location | Under each consumer skill folder, at a path declared in that skill’s P11 receipt (default suggestion: `resources/<resource_id>/…`) |
| Completeness | Each copy is usable after U16 folder copy with **no** repository-relative path outside that skill folder |
| Transforms | Byte-identical to canonical unless the contract names an allowed deterministic transform |
| Escape | No `..`, no symlink out of the skill folder, no network fetch at check or install time |
| Privacy | No private preferences, credentials or host-local paths in the resource bytes |

### 3. Digest and revision parity

| Rule | Behavior |
| --- | --- |
| Algorithm | One named algorithm for the whole contract (proposal default: SHA-256 over canonical file bytes in a declared ordered manifest) |
| Per-copy digest | Each consumer copy’s digest **equals** `canonical_digest` when the transform is identity |
| Mismatch | Fail when either copy’s digest differs from `canonical_digest` or from the other copy (after allowed transform) |
| Manifest | Optional ordered file list inside the resource tree; if present, digests cover listed files only |
| Labels | `revision_label`, if present, is documentation only and must not participate in pass/fail |

Until this contract is **accepted**, validators must not claim digest-parity PASS for shared resources.

### 4. Receipt fields (P11 resource receipt)

Minimum machine-readable receipt (Markdown front matter or adjacent JSON — implementation choice deferred):

| Field | Required |
| --- | --- |
| `resource_id` | yes |
| `canonical_digest` | yes |
| `digest_algorithm` | yes |
| `revision_label` | no |
| `consumer_skill` | yes (this folder’s skill name) |
| `copy_path` | yes (path relative to the skill folder) |
| `copy_digest` | yes |
| `checked_at` / binding commit | when used as durable evidence |

Distinct from: U16 `skills.owned` folder digests; bulk-transfer package identifiers; council aggregate receipts.

### 5. Fixtures and failure behavior

| Case | Expected |
| --- | --- |
| Both copies present, digests match | Pass |
| Missing copy in one consumer | Fail: missing-resource |
| Path escape / symlink out | Fail: escape |
| Digest mismatch | Fail: revision-mismatch |
| Receipt missing required field | Fail: malformed-receipt |
| Installed-folder layout (post U16) | Same relative `copy_path` and digest as source export |

**Installed-folder fixtures** need an owner pin, U16 install and Cell S1 before they can be proven in the guest. Source-tree fixtures (two skill folders in `skills/`) can be prepared earlier without pin.

## Proposed consumer pairs (owner selects at most one to start)

At most two concrete pairs from **existing** tip skills. Neither is accepted by this page.

| ID | Consumers | Candidate canonical resource | Why it fits P11 |
| --- | --- | --- | --- |
| A | `grill` + `council-review` | Shared questioning / answerable-frontier method excerpt (AS-ADR-0003 D3: Council composes grill rather than duplicating it) | Two real consumers already on tip; tests composition without a new skill |
| B | `wiki` + `workspace-handoff` | Shared durable-evidence / record frontmatter field list (id, kind, title, summary, status, created, updated, related + evidence locator shape) | Two tip skills already teach overlapping provenance fields; tests wiki vs handoff copy isolation |

Owner actions: pick pair A, pair B, neither, or a different pair; then accept or amend this contract. Do not treat either pair as scheduled implementation.

## Relation to AG1.2 (P12)

- **Path existence and escape refusal** inside a single skill folder may proceed from AG1.1 / module-shape rules without P11 acceptance.
- **Content/revision digest parity across two skill copies** requires this contract (or a successor) to be owner-accepted and a consumer pair chosen.
- **Dependency-declaration vocabulary** (hard cross-skill loads) is out of scope here unless the owner folds it into a later amendment.

## Adoption order

1. Owner reviews this proposed contract and optionally selects a consumer pair.
2. Docs or fixtures land only after that acceptance (separate request).
3. AG1.2 digest checks adopt the accepted receipt fields and fail codes.
4. Installed-folder proof follows pin / U16 / Cell S1.

## References

- AG1.2 (P12) checks that adopt this contract: [AG1.2 check contract](ag12-check-contract.md).
- Decision direction: [AS-ADR-0003](../records/decisions/0003-portable-skill-capabilities.md) (D7 staged AG1; D10 dependency-driven delivery).
- Planning P11/P12: [skill-system planning](../records/2026-11-24-skill-system-planning.md).
- Packaging direction: [skill capability evolution](skill-capability-evolution.md).
- Module shape / in-folder checks: [skill modules](skill-sources.md).
- Install digests (distinct): [guest install](guest-install.md).
