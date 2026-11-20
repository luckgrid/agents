---
name: council
description: >
  Review-and-remediate loop between files and roles: pin a base and head commit,
  brief reviewers once, collect findings in a fixed contract (verdict, at most ten
  evidence-backed findings, JSON between markers), synthesize, record and verify.
  Use when preparing a review request, merging findings from several reviewers,
  keeping a review record, or proving a check before saying done. Names
  council-review and council-remediate for the reviewer and author turns.
  Supports environments that capture a reviewer's emitted result block (emit mode).
  Supports named review profiles and multi-reviewer synthesis without requiring a large council for every change.
license: Apache-2.0
compatibility: Git and a shell; no harness, multiplexer or herdr needed. Moving text between agents is the environment's job.
---

# Council

Public, file-based review loop. One author edits; one or more reviewers read a fixed range, each through one lens (optionally under a named **review profile**). Nothing here installs a tool, starts an agent or pushes. Environments (in Workbench, `workbench-*`) may only **tighten** these rules.

The Workbench review flow (F1, `wb-flow review`) is **proposed elsewhere**; this skill doesn't need it and doesn't claim it is installed. A parser for the contract below is likewise not assumed. Guest **source pilots** may load these skill folders from a pinned checkout as task data until U16 installs them.

An optional **review coordinator** (often the same orchestrator on small work) selects profiles, dispatches independent reviews, validates and synthesizes, then requests remediation and rechecks. While coordinating review it does not write production fixes. Do not invent an installed audit role; use these skills when available.

## Roles and files

| Role | Does | Skill |
| --- | --- | --- |
| Requester / orchestrator / coordinator | Pins the range, builds the briefing, picks profiles and lenses; may persist captured results | this skill, `request` |
| Reviewer | Read-only; one lens (one profile turn); emits or writes the result block | `council-review` |
| Author | Verifies each finding, fixes, commits, never pushes | `council-remediate` |
| Synthesizer | Merges several reviewers' files; keeps contested/minority findings | this skill, `synthesize` |

Text an agent wrote reaches another agent **through a file**, never typed into its prompt. The prompt that starts a turn is fixed: it names the skill (or a brief that loads skill text from a pinned source), the request file's location and nothing else. Only the owner answers a harness's dialogs.

## Findings contract

1. The request fixes a **base** and a **head** commit (full SHAs) and **one lens** per reviewer turn. A branch name alone is not a fixed point.
2. The result has a **verdict**: `PASS` (no blocker or major) or `CHANGES`.
3. **At most ten** findings, each with a severity, a file, a line and a reason backed by evidence read in the code. Finding `id`s are unique (`F1`, `F2`, …).
4. Severities: `blocker`, `major`, `minor`, `nit`.
5. The machine-readable result is one JSON object between two markers that `council-review` defines (in its `review` reference). The markers never appear in a fixed prompt or a request file, so echoed input can't be mistaken for output.
6. **Output mode** on the request: `emit` (reviewer prints the block; the environment captures and persists the findings file) or `file` (reviewer writes that path). Workbench's read-only reviewer uses `emit`. Direct `file` remains valid where the environment allows reviewer writes.
7. Missing, truncated, mismatched or invalid captures are **not** a `PASS`. The environment may record an operational `BLOCKED` or reject; that is outside the JSON `verdict` field.
8. A **profile** may bind harness, resolved model and provider (when known) separately from the lens. Record actual choices; two model names are not automatically two providers. Optional `profile_id` on the request and result binds the turn.

Lenses: `spec`, `standards`, `correctness`, `security`, `tests`, `lean`, `operability` (environments may add UX/accessibility or dependency/API as extra perspectives). Every review gets **spec, standards and correctness** as separate turns (full range each time); add the rest by risk. Do not collapse the three required lenses into one turn while claiming this contract. Do not require a large multi-agent team for every small change. Two **remediation** rounds is a round budget, not a limit of two lens turns. Overall acceptance needs required checks and all required profiles' outcomes — one PASS does not erase another's unresolved major.

## Hard rules

- Reviewers are read-only and see a crafted briefing, never the author's session.
- Every reviewer reviews the whole range through its lens; no one gets a slice.
- A moved head invalidates the review: a new request, not a continued one. Never overwrite an earlier accepted receipt.
- Findings artifacts are immutable for the author; dispositions go in a new file.
- An author's rejection needs evidence; a recheck must re-verify it — no automatic acceptance of a rejection.
- Public-safe files: `~/…` paths, no hostnames, accounts, emails or live session IDs in portable notes.
- Name the skill you rely on, with a one-line fallback if it is missing (below). If a required rule source is unavailable, stop.
- Never install, update or configure tools to run a council.
- `council-review` and `council-remediate` are **instruction-level** explicit (named by start line or brief). That is not harness enforcement until admitted metadata is proved at install time.

| Needs | Fallback if missing |
| --- | --- |
| `council-review` | The reviewer follows the contract above from this page alone, read-only, and emits the result block |
| `council-remediate` | The author verifies each finding, fixes what holds up, and prints the new head |
| An environment's review flow | Hand the request file to the reviewer by hand; the orchestrator captures or reads the findings file back |

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `request` | [request](references/request.md) | Pin the range and build the briefing pack |
| `synthesize` | [synthesize](references/synthesize.md) | Merge several findings files |
| `record` | [record](references/record.md) | Keep the review and its dispositions |
| `verify` | [verify](references/verify.md) | Run the check and read it before claiming done |

`review` and `recheck` are `council-review`; `remediate` is `council-remediate`.
