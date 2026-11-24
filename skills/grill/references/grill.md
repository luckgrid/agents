# grill

Run questioning rounds until the design is settled.

## Before the first round

1. State the design goal and what is in or out of scope.
2. Sketch a **design tree**: topics, dependencies and open questions. Reuse an
   existing brief or kickoff when one exists; do not invent a parallel ledger.
3. Mark prerequisites. The **frontier** is every open question whose
   prerequisites are already settled (or that has none).

## Each round

1. List the current frontier. If it is empty, go to **Done**.
2. Ask **all** frontier questions in this round (do not drip one-by-one unless
   the owner asks to go slower). Number them.
3. For each question, give:
   - the decision needed,
   - a **recommended answer** that "yes" accepts,
   - brief rationale and any fact you already verified,
   - optional alternatives when they are real choices.
4. Where the harness supports structured choices, follow [choices](choices.md).
5. Record the owner's answers. Update the tree: settle nodes, open newly
   unblocked questions, drop or rewrite questions that no longer apply.
6. Start the next round with the new frontier.

## Done

Stop only when:

1. the frontier is empty, and
2. the owner **confirms** the design is settled for the stated scope.

Hand off a short summary: settled decisions, remaining out-of-scope items, and
where durable notes live (records/run notes). Building, merging, installing or
publishing remain separate owner-authorized steps.

## Hard limits

- Do not treat a recommendation as accepted without the owner's yes (or an
  explicit alternate choice).
- Do not hide unresolved prerequisites by asking downstream questions early.
- Do not substitute this mode for code review (`council`) or coordination
  (`orchestration`).
