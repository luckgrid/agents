# handoff

Progress or blocker note for another session, role or repository. Fill every required field from the skill router. Omit empty optional detail rather than inventing it.

## Template

```markdown
# Handoff: <short title>

## Intent, scope, acceptance
- Intent: …
- Scope (in / out): …
- Acceptance: …

## Location
- Repository: <org/repo>
- Branch: <name>
- Base ref: <develop|main|…>
- Base tip (full SHA): <40-hex or unknown>
- Integration target: <ref>
- Candidate tip (full SHA): <40-hex>
- Related repositories (if any): <org/repo @ full SHA or pending>

## Completed
- …

## Unresolved
- …

## Checks
| Check | Covered revision (full SHA) | Result |
| --- | --- | --- |
| … | … | pass / fail / not run |

## Review
- Findings and dispositions: … / none yet

## Recipient
- Workspace or role: …
- Requested next action: …

## Recovery
- Interrupted or partial operations: none / describe surviving state
- Do not: <destructive retries to avoid>

## Durable evidence
- Records / run notes (~/…): briefs, findings, receipts, board links
- Provenance: repository, branch, base/head SHAs, run id, review/check status

## Limits
- Partial or unavailable observations: …
```

## Rules for this mode

- The **candidate tip** is the full SHA the recipient must re-read. If the recipient sees a different `HEAD` or branch tip, treat checks and review coverage as stale until re-run at the new tip.
- Portable copies stay free of `/home/…` paths and Herdr IDs. A Workbench-local copy under `~/…` may add a "local attachment" section the recipient discards when moving notes.
- Blockers name the exact refused command or missing authorization; do not work around guest boundaries.
- For publication or host steps, point at the environment's publication skill when available (in Workbench, `workbench-handoff`, its `dev` mode); otherwise list the public next action and stop.
- Point at durable evidence so a recipient can resume after workers stop and the worktree is removed; do not leave the only copy inside a disposable checkout.
