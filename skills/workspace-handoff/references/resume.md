# resume

Revalidate before continuing, or record state after work lands. Always re-read refs and checkout state; never trust the previous note's SHAs without confirmation.

## Revalidation procedure

1. Resolve the repository and checkout locally (clone or linked worktree). Confirm branch name.
2. Read `HEAD` as a **full SHA**. Compare to the note's candidate tip.
3. Read the base ref tip if the note names one. Compare to the note's base tip when present.
4. Read `git status` (tracked dirtiness). Report untracked only when relevant; do not invent cleanliness from silence.
5. If Herdr observations are needed: follow ordinary owner-run socket policy unless an explicit session exception applies. Mark denied, unavailable or not queried explicitly.
6. **If candidate SHA ≠ note's candidate:** invalidate checks and review coverage that claimed the old SHA. Report the mismatch. Re-run required checks at the new tip before any integrate/publish step. Do not continue a plan that assumed the old tip.
7. **If base SHA moved:** report; reassess the range and whether rebase/merge authorization exists (usually owner/integrator only).
8. Only then perform the requested next action, still within one-writer / one-integrator rules.

## Post-landing resume template

```markdown
# Resume after landing: <short title>

## Landed
- Repository: <org/repo>
- Integration target: <ref>
- Landed via: <squash merge / merge / …> (as observed or owner-reported)
- Pre-land candidate (full SHA): …
- Target tip after landing (full SHA): … / unknown until fetch

## Local follow-up (guest)
- `develop` / feature branch state: …
- Realign needed: yes / no / unknown (owner runs `wb sync` on the host first when fetching is required)
- Worktree cleanup: only when clean, tip preserved or abandoned, and owner authorizes remove

## Invalidated by landing
- Prior candidate checks do not prove the landed revision unless re-run at the target tip.

## Next action
- …
```

## Rules for this mode

- Squash merges break naive ancestry checks: retain the pre-land candidate SHA and owner-approved preservation evidence before deleting branches.
- Closing a Herdr sidebar item does not approve worktree removal.
- Cross-repository resumes track each repo's tip separately; there is no atomic multi-repo transaction.
