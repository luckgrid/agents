# synthesize

Merge the findings files from several reviewers (or lenses / profiles) into one list. Skip this mode with a single reviewer.

1. **Validate each file first.** Take the last well-formed result block, check that its base, head and lens match that reviewer's request (and `profile_id` when present), and drop any file that doesn't. Note the drop.
2. **Keep attribution.** Record which profile/reviewer/session produced each input verdict and each finding before merging.
3. **Merge same place, same problem.** Findings on the same file and nearby lines with the same underlying defect become one; keep the highest severity and all reasons; note which profiles contributed.
4. **Drop speculation.** A finding with no evidence from the code is removed, or kept only as a question marked low confidence.
5. **Sort into three groups:** consensus (several reviewers), single-reviewer, contested (reviewers disagree on whether or how bad). **Preserve minority and contested findings** and their disposition later; majority agreement alone does not settle correctness.
6. **Cap and number.** Give each surviving finding an ID (`F1`, `F2`, …) in severity order. If the merged list is longer than the author can act on, the cap per reviewer still held; say what was left out.
7. **Overall gate.** Synthesis does **not** invent a `PASS` that hides an unresolved blocker or major from any **required** profile. If any required profile is missing, invalid or BLOCKED, the overall outcome is not acceptance — report unresolved work.

Write the synthesis to a file. It keeps the reviewers' verdicts. Reviewers see the synthesis only if a later optional cross-review round asks them to, after the independent pass.
