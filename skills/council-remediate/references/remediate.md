# remediate

One round, from a findings file to a new head.

1. **Read and restate** each finding in one line. Take them one at a time.
2. **Verify against the code** before changing anything. If the finding is wrong, don't fix it: note the evidence that shows it.
3. **Decide.** Fix blockers and majors. Fix minors where the change is small. Nits are optional. Anything needing a decision that isn't yours becomes an owner question.
4. **Fix** only what the finding needs. No unrelated cleanup.
5. **Check.** Run the project's checks and read the output (`verify`).
6. **One commit for the round**, its message listing the finding IDs it addresses. Never amend an earlier commit.
7. **Disposition list**, written to a new file of your own (not the findings file): each finding as fixed (with the commit), not fixed (with the evidence), or an owner question.
8. **Print the new head**: the full 40-hex SHA and the branch, so the next request can pin it. Don't push.

Stop and report instead of guessing when: a finding names a file that doesn't exist at the reviewed head, two findings conflict, or the checks can't run.
