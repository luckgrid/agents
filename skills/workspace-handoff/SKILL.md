---
name: workspace-handoff
description: >
  Hand off and resume bounded work across sessions, roles or repositories:
  kickoff notes, progress or blocker handoffs, review requests pinned to exact
  commits, and post-landing resume. Loads workspace base rules. Use when leaving
  work for another actor, picking up a note, or invalidating stale checks after
  a tip moves. No central ledger.
license: Apache-2.0
compatibility: herdr >= 0.9.5 when Herdr observations are included; HERDR_ENV=1 for those steps.
---

# workspace-handoff

Load the `workspace` skill's model, environment-policy preflight and hard rules first. If `workspace` is not installed, stop. In Workbench, also load `workbench-guest-rules` and `workbench-herdr` when those steps apply; if required guest rules are unavailable, stop before guest-only actions.

This skill writes **resumable notes**, not a runtime ledger. It never promotes, opens PRs, lands, installs or starts agents. The recipient **revalidates** current refs, tips and checkout state before acting. A changed candidate SHA, base tip or check coverage invalidates the note's checks and plan: report and stop, do not continue on stale evidence.

## Portability

Portable notes use repository locators (for example `luckgrid/agents`), branch names, full commit SHAs and role or workspace names. Do **not** put machine-specific paths (`/home/…`) or Herdr workspace/pane IDs in a note meant to travel. Local paths and live IDs belong only in an ephemeral session attachment the recipient resolves again.

## Required fields (every handoff or resume note)

| Field | Content |
| --- | --- |
| Intent, scope, acceptance | What was asked; what is in and out of scope; how done is judged |
| Location | Repository, branch, base ref, integration target, **full candidate SHA** |
| Progress | Completed work; unresolved work |
| Checks | Commands or gates run, and the **exact revision** each covered |
| Review | Findings and dispositions (or "none yet") |
| Recipient | Receiving workspace or role, and the requested next action |
| Recovery | State of any interrupted or partial operation; what not to retry blindly |

## Hard rules

- One writer per checkout; one integrator per shared target. A handoff does not authorize editing another session's checkout.
- Refresh Git registration, refs/tips and (when used) Herdr association before mutation after resume.
- Partial or unavailable Git or Herdr observations stay explicit; never invent a clean or idle state.
- When the note asks for publication, keep promote, exact-candidate check, PR create/update and landing as distinct host-owner steps. Cleanup is a separate step too, run by whoever the environment's rules authorize (in Workbench, the operator), not necessarily the owner. Name the environment's publication skill for the host packet (in Workbench, `workbench-handoff`) when installed; if it is unavailable, stop before environment-only publication steps and keep the public fields above.

## Modes

| Mode | Reference | Use |
| --- | --- | --- |
| `kickoff` | [kickoff](references/kickoff.md) | Compact session start note |
| `handoff` | [handoff](references/handoff.md) | Progress or blocker handoff |
| `review-request` | [review-request](references/review-request.md) | Review pinned to base and candidate commits |
| `resume` | [resume](references/resume.md) | Revalidate and continue, or record post-landing state |
