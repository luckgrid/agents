# record

Keep what the review said and what became of it, so the next session needn't ask.

1. **Reviews as written.** Store each findings file verbatim, with its base, head and lens (and `profile_id` when used). Do not tidy or reword them.
2. **A disposition per finding**, added in a separate section after the reviews:
   - fixed, with the commit that fixed it;
   - not fixed, with the evidence why (the finding was wrong, out of scope, or accepted);
   - an owner question, written as one line.
3. **Rounds.** Note each round's base and head, so a later reader sees which head each review covered.
4. **Immutable receipts.** Keep raw captures, validated findings, dispositions, synthesis and recheck evidence. Append corrections or superseding links; do not silently rewrite earlier outcomes.
5. **Public-safe.** `~/…` paths only; no hostnames, accounts, emails or live session IDs.
6. **Retention.** Findings, dispositions and aggregate receipts are durable context under the same policy as `workspace-maintain` `retention`: ordinary worktree/pane/run-resource cleanup must not delete them. Explicit history pruning is an environment action (preview/apply), not part of completing a review loop.

The record is a file in the repository or the run's notes; where it lives is the environment's rule. Prefer run/records locations that remain after the author checkout is removed. Never rewrite a review to match its fix.
