# verify

Before anyone says a change is done, passes, or is fixed: run the check and read its output.

1. Name the command that proves the claim, and the commit it runs on.
2. Run it in the checkout at that commit. Read the whole result, not the last line.
3. Report the command, the commit and what it showed, pass or fail. A check that can't run (a refused sandbox, a missing tool) is reported as blocked with the exact command; it is not reported as passed.
4. A check run before a later commit says nothing about that commit; run it again.
5. Never install a tool to make a check run; report the gap.

Used by the author after fixes and by the requester before closing a round.
