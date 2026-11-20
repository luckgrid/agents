# recheck

Use after the author reports fixes. The request names the original base, the new head and the same lens; if it names a different lens or an unrelated range, treat it as a new `review`.

1. Read the author's dispositions for each earlier finding.
2. Re-review the lines changed since the earlier head, and each fixed or rejected finding against the code. A rejection with evidence stands; a rejection without it doesn't.
3. Look for what the fixes broke, not only whether they landed.
4. Report in the `review` format: verdict, at most ten findings, one result block, with the new head. Carry an unresolved earlier finding forward under its old `id` with a note.

Stop condition: the agreed number of rounds is reached, or no blocker or major remains (`PASS`).
