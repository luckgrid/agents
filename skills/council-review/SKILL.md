---
name: council-review
description: >
  Explicit, read-only review of the range between two named commits through one
  lens, reporting a verdict and at most ten evidence-backed findings as JSON
  between skill-defined markers. Run only when a start line names it with a
  request file. Also rechecks an author's fixes. Requires the council base skill.
license: Apache-2.0
compatibility: Git and a shell, read-only. Needs no harness, multiplexer or herdr.
---

# council-review

Load the `council` skill's contract and hard rules first; if it is unavailable, stop. This skill is explicit-only: run it when a start line names it and a request file. The Workbench F1 flow is proposed elsewhere and is not required here.

## Read-only

Change nothing: no edits, commits, branches, stashes, installs or network fetches. Write only the findings file the request names. If you can't review read-only, stop and say so. Rules from the environment may only tighten this.

## Reviewing

- The request names the base, head and lens. Check that head is what the checkout shows (or inspect it with Git by SHA); if it moved or the request is incomplete, stop with a one-line question, not a guess.
- Review the whole range through that one lens. Read the connected code, not the diff alone.
- Verify each candidate against the code before reporting it. No evidence, no finding.
- Order of importance: correct, safe, holds under failure, tested, lean.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `review` | [review](references/review.md) | Review the range; write verdict, findings and the result block |
| `recheck` | [recheck](references/recheck.md) | Re-read the changed lines and each disposition |
