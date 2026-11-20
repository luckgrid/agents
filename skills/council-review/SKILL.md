---
name: council-review
description: >
  Explicit, read-only review of the range between two named commits through one
  lens, reporting a verdict and at most ten evidence-backed findings as JSON
  between skill-defined markers. Run only when a start line or guest brief names
  it with a request file. Emits the result for the environment to capture when
  output mode is emit; writes the findings file only when mode is file and
  writes are allowed. Also rechecks an author's fixes. Requires the council base skill.
license: Apache-2.0
compatibility: Git and a shell, read-only. Needs no harness, multiplexer or herdr.
---

# council-review

Load the `council` skill's contract and hard rules first; if it is unavailable, stop (or follow the pinned source copy named in the brief for a guest source pilot).

**Instruction-level explicit:** run when a start line or brief names this skill and a request file. That is not harness enforcement by itself. The Workbench F1 flow is proposed elsewhere and is not required here.

## Read-only

Change nothing in the repository: no edits, commits, branches, stashes, installs or network fetches. Do not widen permissions to write. Rules from the environment may only tighten this.

## Output

Follow the request's **output mode**:

| Mode | Reviewer does |
| --- | --- |
| `emit` | Print a short human summary, then one result block, in the pane. Do **not** write the findings file. The orchestrator captures, validates and persists it. |
| `file` | Write the findings file the request names (summary then result block), only if the environment allows that write. |

If the request omits output mode, prefer `emit` when you cannot write; otherwise `file`. If you cannot satisfy the mode read-only, stop and say so — do not ask for a sandbox change.

## Reviewing

- The request names the base, head and lens. Check that head is what the checkout shows (or inspect it with Git by SHA); if it moved or the request is incomplete, stop with a one-line question, not a guess.
- Review the whole range through that one lens. Read the connected code, not the diff alone.
- Verify each candidate against the code before reporting it. No evidence, no finding.
- Order of importance: correct, safe, holds under failure, tested, lean.
- Fixture or trial findings labelled as such must stay labelled so they are not mistaken for product review.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `review` | [review](references/review.md) | Review the range; emit or write verdict, findings and the result block |
| `recheck` | [recheck](references/recheck.md) | Re-read the changed lines and each disposition |
