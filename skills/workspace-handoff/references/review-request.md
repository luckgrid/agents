# review-request

Ask for review of an exact candidate against an exact base. This mode does not run `council`; it prepares a pinned request. When `council` is installed, name it for the review loop; if it is unavailable, stop after writing the request and wait for a human or another admitted reviewer path.

## Template

```markdown
# Review request: <short title>

## Intent
…

## Pins (full SHAs)
- Repository: <org/repo>
- Branch: <name>
- Base: <ref> @ <40-hex>
- Candidate: <40-hex>
- Range: <base>..<candidate> (confirm with git after checkout)

## What to review
- In scope: …
- Out of scope: …

## Checks already run
| Check | Covered revision | Result |
| --- | --- | --- |
| … | <must equal candidate unless noted> | … |

## Suggested lenses
Default when using `council`: spec, standards, correctness; add others by risk.
If `council` is unavailable: human review with the same lenses as a checklist.

## Stop rule
- Disposition every finding (fix / accept / defer with reason).
- Do not land, promote or widen scope from review alone.

## Recipient
- Role or harness: …
- Requested next action: review only / review then remediate handoff
```

## Rules for this mode

- Never ask for review of "current HEAD" without recording the full candidate SHA at request time.
- If the author commits after the request, issue a new request or an explicit SHA update; do not silently reuse the old pin.
- Keep the reviewer's checkout separate from the author's (one writer per checkout).
