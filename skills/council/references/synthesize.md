# synthesize

Merge the findings files from several reviewers (or lenses) into one list. Skip this mode with a single reviewer.

1. **Validate each file first.** Take the last well-formed result block, check that its base, head and lens match the request, and drop any file that doesn't. Note the drop.
2. **Merge same place, same problem.** Findings on the same file and nearby lines with the same underlying defect become one; keep the highest severity and all reasons.
3. **Drop speculation.** A finding with no evidence from the code is removed, or kept only as a question marked low confidence.
4. **Sort into three groups:** consensus (several reviewers), single-reviewer, contested (reviewers disagree on whether or how bad).
5. **Cap and number.** Give each surviving finding an ID (`F1`, `F2`, …) in severity order. If the merged list is longer than the author can act on, the cap per reviewer still held; say what was left out.

Write the synthesis to a file. It keeps the reviewers' verdicts and adds none of its own beyond `PASS` when no blocker or major remains. Reviewers see it only if a later round asks them to.
