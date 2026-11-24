# choices

When the harness provides a multiple-choice question tool (or an equivalent
structured prompt the environment already admits), use it for frontier
decisions that have a small set of discrete options.

## How to present

1. Put the **recommended option first**.
2. Keep option labels short and mutually exclusive.
3. Mirror the same recommendation in prose so a free-text "yes" still works if
   the tool is unavailable.
4. After the owner picks, update the design tree and continue the round.

## When not to use

- Open-ended questions that need free-form intent.
- Questions that are really requests for facts you should look up yourself.
- Environments with no admitted choice tool — stay with numbered prose
  questions from [grill](grill.md); do not invent a custom UI or install one.

## Authority

A selected option is the owner's decision only for that question. It does not
authorize publication, installation or broadening of scope.
