# with-docs

Grill a design while a Luckgrid wiki is in scope. Load the `wiki` skill. If it
is unavailable, stop before writing wiki pages and continue only with
non-wiki notes the environment allows.

## Flow

1. **Orient** with `wiki` `orient`: scope, STATE, glossary or decisions index,
   and the pages that already cover this design.
2. Run [grill](grill.md) rounds. Before recommending an answer:
   - challenge terms against the wiki's glossary and accepted decisions;
   - sharpen fuzzy language into checkable claims;
   - test settled wording with a concrete scenario when ambiguity remains;
   - verify claims against code or cited records when the claim is factual;
   - mark speculation as speculation.
3. When the owner settles a decision that belongs in the wiki, write it through
   `wiki` `decide` (and `record` / `state` as that skill requires): the owner's
   words or faithful summary, then the durable rule. Do not invent acceptance.
4. Prefer updating existing decision or architecture pages over parallel
   notes. Keep historical observations intact; append later outcomes rather
   than rewriting them to look current.
5. Finish with [grill](grill.md)'s Done criteria, plus a pointer to any new or
   updated wiki pages.

## Boundaries

- This mode does not replace `wiki` lint, crystallize or query.
- Public wikis stay free of secrets, credentials and private machine paths.
- Settling a design does not authorize merge, pin, install or admission.
