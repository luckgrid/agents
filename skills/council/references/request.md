# request

Build the files a reviewer needs. The requester (or orchestrator / review coordinator) writes; reviewers only read.

## Steps

1. **Pin the range.** Resolve `base` and `head` to full 40-hex SHAs with Git. Confirm base is an ancestor of head and the checkout is clean at head. Record both. Also record the **council source** SHA when skills are loaded from a checkout that is not globally installed.
2. **Choose profiles and lenses.** Spec, standards and correctness always, **one lens per turn** over the full range; add security, tests, lean, operability, UX/accessibility or dependency/API compatibility by risk. Several required lenses mean several turns (or reviewers), not one combined turn. A **review profile** may name harness, model and provider separately from the lens — record them; do not claim provider diversity without evidence.
3. **Write the briefing pack** to a file: the same factual text goes to every independent reviewer, word for word.
   - What changed, in a short summary of the files and intent (not the raw diff).
   - The spec: the issue, plan or decision the change answers, quoted or linked by locator.
   - The standards: the repository rules that apply (README, wiki decisions).
   - Not included: the author's session, reasoning, opinions, or **another reviewer's conclusions**.
4. **Write the request file** next to it, with these fields:

| Field | Content |
| --- | --- |
| Repository | Locator such as `luckgrid/agents`, never a machine path |
| Base, head | Full SHAs from step 1 |
| Lens | Exactly one, from the lens list |
| Profile id | Stable id from the environment's profile catalog (optional but recommended) |
| Harness / model / provider | As resolved for this turn; provider may be "unknown" or a family label — say so |
| Briefing | Location of the briefing file |
| Findings file | Destination path for the accepted findings artifact (run folder; outside the author checkout) |
| Output mode | `emit` (default in Workbench: reviewer prints the block; orchestrator persists) or `file` (reviewer writes the path when allowed) |
| Round / request id | Round number and a stable request id for binding captures |
| Council source | Full SHA of the skill tree the brief loads (source pilot) |

5. **Keep the markers out.** Neither the briefing, the request file nor the fixed start prompt may contain the result markers defined by `council-review`'s `review` reference. Check with a text search before handing over.
6. **Hand over by file.** The start prompt is fixed: it names `council-review` (or a guest brief that loads that skill's text from the pinned source) and the request file's location. The requester does not paste findings, hints or opinions into it.
7. **Accepting a capture.** Before treating output as the findings file: check schema, verdict rules, base/head/lens match, target freshness and binding ids. Invalid or stale output is rejected; it never becomes a `PASS` acceptance. Persist only after validation. Label source-pilot receipts as such.

If the head moves after the request is written, write a new request; don't edit the old one in place. Don't overwrite an earlier accepted receipt.
