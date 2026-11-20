# record

Keep what the review said and what became of it, so the next session needn't ask.

1. **Reviews as written.** Store each findings file verbatim, with its base, head and lens. Do not tidy or reword them.
2. **A disposition per finding**, added in a separate section after the reviews:
   - fixed, with the commit that fixed it;
   - not fixed, with the evidence why (the finding was wrong, out of scope, or accepted);
   - an owner question, written as one line.
3. **Rounds.** Note each round's base and head, so a later reader sees which head each review covered.
4. **Public-safe.** `~/…` paths only; no hostnames, accounts, emails or session IDs.

The record is a file in the repository or the run's notes; where it lives is the environment's rule. Never rewrite a review to match its fix.
