---
id: AS-ARCH-AG12-CHECK-CONTRACT
kind: architecture
title: AG1.2 check contract
summary: "Proposed P12 contract for AG1.2 expansion after AG1.1 parity: schema, resource containment and related-ID or reference-graph checks. Status proposed. Depends on open P11 packaging and receipt contracts for content/revision parity. Not validator code, Moon admission, cutover, pin or install."
status: proposed
created: 2026-11-27
updated: 2026-11-27
related: [AS-ADR-0003, AS-REC-2026-11-24-AG1-1, AS-REC-2026-11-24-SKILL-SYSTEM-PLANNING, AS-ARCH-SKILL-CAPABILITY-EVOLUTION, AS-ARCH-SKILL-SOURCES, AS-PROGRAM, AS-STATE, AS-ARCH-P11-PACKAGING-CONTRACT]
---

# AG1.2 check contract

**Status: proposed.** This page adopts planning package **P12** (AS-ADR-0003 D7 second stage) as a check contract only. It does not implement validator code, admit Moon, cut over away from `scripts/check.sh`, pin skills, install runtime, merge or publish.

AG1.1 (planning **P3**) already landed native parity with the shell checker and retains the shell baseline until a separate cutover acceptance. AG1.2 adds only the **adopted** static checks below. No script executor, installer, network crawler or hook manager belongs in the validator ([capability architecture](skill-capability-evolution.md)).

## Non-goals (deferred elsewhere)

| Topic | Owner |
| --- | --- |
| Moon / `.prototools` guest admission | Separate admission track |
| Removing or demoting `scripts/check.sh` | Cutover acceptance after parity evidence |
| First pin, U16 install, Cell S1 | Owner gates |
| P11 packaging / receipt / shared-resource content digests | Open: [P11 packaging and receipt contract](p11-packaging-contract.md), proposed; see slice 2 deferred choices |
| Module implementation that assumes an installed Agents pin | After pin |

## Shared rules for every slice

| Rule | Behavior |
| --- | --- |
| Engine | Read-only static checks on a tree the caller names (default: repository `skills/` as today; optional wiki paths only when the invocation opts in) |
| Diagnostics | Stable machine-readable codes plus one human line; no network; no writes |
| AG1.1 corpus | Existing positive skills and self-test fixtures keep today's pass/fail unless a slice documents a new **WARN** that does not fail the default skills run |
| Fixtures | Each slice ships at least one positive and one negative fixture under the validator's fixture tree |
| Opt-in expansion | New failing rules default to an explicit AG1.2 flag or namespace until the owner accepts them as default |

## Slice 1 — Schema checks

**Priority:** first. **P11 dependency:** none for the skill and wiki field sets already known in-repo.

### Inputs

- Skill module `SKILL.md` YAML frontmatter (Agent Skills shape; router fields already enforced by AG1.1 parity).
- Optional: wiki pages with record frontmatter when a wiki path is passed: `id`, `kind`, `title`, `summary`, `status`, `created`, `updated`, `related` ([wiki rules](../README.md)).

### Known rules (adopted here)

| Code family | Pass | Fail |
| --- | --- | --- |
| Skill required keys | Present `name` / `description` (and AG1.1 namespace / naming rules) | Missing, empty or wrong type |
| Skill name form | Matches folder and namespace policy from AG1.1 | Drift from folder or forbidden namespace |
| Wiki required keys (opt-in) | All eight keys present with scalar or list types as today | Missing key or wrong YAML type |
| Wiki `id` / `related` shape | `id` is a single `AS-…` token; `related` is a list of ID tokens | Path-like or slug values where IDs are required |

### Minimum fixtures

- Positive: one minimal valid skill frontmatter; one minimal valid wiki record frontmatter.
- Negative: missing `description`; wiki page missing `related`; wiki `related` containing a path instead of an ID.

### Pass / fail

Default skills invocation: skill schema failures **fail** the run (aligned with AG1.1). Wiki schema failures apply only when wiki paths are included; otherwise skipped. Unknown future keys: **WARN** until separately adopted.

### Deferred choices

- Whether wiki schema becomes part of the default repository check or stays opt-in.
- Extra Agent Skills metadata keys beyond today's parity corpus.

## Slice 2 — Resource containment and missing-resource fixtures

**Priority:** second. **P11 dependency:** content/revision parity and multi-consumer packaging proofs wait on the open P11 packaging and receipt contract. Path existence and escape refusal do **not** wait on P11.

### Inputs

- Skill folder as the install root (after pin: the copied skill folder; before pin: repository `skills/<module>/`).
- Paths named from the router mode table (`references/<mode>.md`), `CREDITS.md` when present, and any relative links inside those Markdown files that stay under the skill folder.

### Known rules (adopted here without P11)

| Code family | Pass | Fail |
| --- | --- | --- |
| Mode reference exists | Mode table target resolves to a file under `references/` | Missing file |
| No escape | Resolved path stays inside the skill folder after normalization | `..`, symlink escape or absolute path outside the folder |
| CREDITS present when required by module policy | File exists if the module shape requires it | Missing where required |

### Minimum fixtures

- Positive: skill with one mode and matching `references/<mode>.md`.
- Negative: mode pointing at a missing reference; mode path attempting `../` escape.

### Pass / fail

Missing in-folder reference or escape: **fail**. Network URLs in prose are ignored by this slice (not fetched).

### Deferred choices (blocked on P11)

- Content digest / revision parity for a shared resource copied into two skill folders.
- Declared resource manifests beyond mode-table and relative links.
- Proof that U16-installed folders keep the same relative layout (runtime; needs pin).

Until P11 is accepted, implementers may land fixtures and path-existence checks only; they must not claim packaging parity.

## Slice 3 — Related-ID and reference-graph checks

**Priority:** third. **P11 dependency:** none for dangling in-repo IDs and broken relative links. Optional “undeclared hard skill dependency” warnings wait on a dependency vocabulary from P11 or a later ADR.

### Inputs

- Wiki `related:` ID lists (when wiki paths are in scope).
- Relative Markdown links inside skill `SKILL.md` / `references/` that target files under the same skill folder.
- Searchable ID set: pages under `wiki/` that declare an `id:` in frontmatter (exact set documented by the implementation's fixture README).

### Known rules (adopted here)

| Code family | Pass | Fail |
| --- | --- | --- |
| Related ID resolves | Each `related` token matches an `id` in the searchable set | Dangling token |
| Relative skill link | Target file exists under the skill folder | Broken relative link |
| Self `id` uniqueness | One page per `id` in the searchable set | Duplicate `id` |

### Minimum fixtures

- Positive: two wiki pages that cross-`related` each other; skill with a valid relative link to its reference.
- Negative: `related` token with no page; relative link to a missing file; two pages sharing one `id`.

### Pass / fail

Dangling related IDs and broken relative skill links: **fail** when that corpus is included. Default skills-only run: relative skill links **fail**; wiki related-graph runs when wiki paths are included.

### Deferred choices

- Treating cross-skill “must load” edges as hard dependencies (needs adopted dependency contract).
- Closing GitHub issue #12 or other host workflows (not AG1.2).

## Adoption order

1. Document this contract (this page) — **proposed**.
2. Implement slice 1 behind an explicit AG1.2 path; keep AG1.1 compare green.
3. Implement slice 2 path-existence and escape refusal; leave digest parity until P11.
4. Implement slice 3 for skill relative links, then opt-in wiki related-graph.
5. Owner decides default-on vs flag; pin/install remain separate.

## Relation to other pages

- Decision: [AS-ADR-0003](../records/decisions/0003-portable-skill-capabilities.md) D7.
- Parity record: [AG1.1](../records/2026-11-24-ag1-1-validator-parity.md).
- Direction: [skill capability evolution](skill-capability-evolution.md).
- Module shape: [skill modules](skill-sources.md).
- P11 contract these digest checks wait on (proposed): [P11 packaging and receipt contract](p11-packaging-contract.md).
