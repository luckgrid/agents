# kickoff

Compact session start. Write it before substantive edits so a later handoff or resume has a stable intent.

## Template

```markdown
# Kickoff: <short title>

## Intent
<one paragraph: what the owner asked>

## Scope
- In: …
- Out: …

## Acceptance
- …

## Location
- Repository: <org/repo>
- Branch: <feature-branch>
- Base: <develop|main> @ <full SHA at start, if known>
- Integration target: <develop|main|…>
- Checkout: resolve locally (do not hard-code machine paths in portable copies)

## Rules to load
- `workspace` (required)
- `workspace-handoff` (this note)
- In Workbench: `workbench-guest-rules`, and `workbench-herdr` when using herdr
- Project skill if present: `<namespace>`

## Starting observations
- Git: branch, HEAD full SHA, clean/dirty (tracked only)
- Herdr: available / denied / not queried; IDs only if ephemeral and local
- Installed vs admitted vs source: say which you are using

## First next action
<one concrete step>
```

## Rules for this mode

- Prefer full SHAs when recording tips. Short SHAs are for prose only when the full value is also present.
- Do not invent a base tip: if not yet observed, write `unknown` and obtain it before mutation that depends on it.
- Keep the kickoff short; move progress into a `handoff` note as work proceeds.
