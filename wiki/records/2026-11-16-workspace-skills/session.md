---
id: AS-REC-2026-11-16-WORKSPACE-SESSION
kind: session
title: Workspace skills bootstrap session (build and survey, 2026-11-16)
summary: "Seventh round: workspace (M10) first, before M2 council, with expanded scope for herdr-backed workspace management. Bootstrap survey of herdr 0.9.5; owner answers on the model, five skills, herdr-direct backend, and first pin after these skills. Skills built on feat/workspace-skills from develop."
status: active
created: 2026-11-16
updated: 2026-11-16  # independent review remediated remove-ID wording, adopt path collapse, status survey gaps; rebased onto #13 directory record
related: [AS-REC-2026-11-16-WORKSPACE-SKILLS, AS-REC-2026-11-16-WORKSPACE-SKILLS-KICKOFF, AS-PROGRAM, AS-STATE, AS-REC-2026-11-14-MODULAR-REWRITE, AS-REC-2026-11-15-M0, AS-RES-SKILL-REFERENCES, AS-ARCH-WORKSPACE-COORDINATION]
---

# Workspace skills bootstrap session (build and survey, 2026-11-16)

## Intent

This is the bootstrap build session that followed the [kickoff](kickoff.md). The owner's intent for the seventh round, the repository's purpose and the herdr interface ideas for Workstation are in the [record](README.md#the-owners-intent-2026-11-16). They aren't repeated here. Under the records rule of 2026-11-16, this page captures intent. The answers below are the options the owner chose, and the survey replies are evidence.

## Seventh round (owner, 2026-11-16)

`workspace` (M10) comes first, before M2 `council`. Its scope grows from issue, pr and release modes to workspace management. The first `agent_skills_commit` pin follows these skills once they merge (not after M2).

## Decisions: questions and the owner's answers

These summaries record the options the owner selected. They preserve the later separator correction and pin timing, without presenting agent-authored option text as a verbatim owner quote.

| Topic | Selected direction |
| --- | --- |
| Plan/build order | Ask the model questions in Plan mode, then make the lifecycle survey the first build step |
| Initial binding | One clone per namespace; unrelated Herdr attachments remain allowed but out of scope |
| Base | Local develop, else main; do not create develop |
| Nesting | Initially proposed parent/name, corrected to parent--name after the Git ref-name conflict; at most one separator. Create from the clone ID with the parent branch as base; sidebar grouping is a convention |
| Maintenance | Report local-origin comparison/freshness and empty-folder candidates; apply approved fixes only |
| Status | Grouped human report and JSON with agent kind/state/pane; no invented orchestrator |
| Skills | Keep all five; status read-only, maintain begins with its survey |
| Backend | Direct Herdr initially, HERDR_ENV check and 0.9.5 floor; M3 extracts common mechanics later |
| First pin | Follow these skills once merged, with the pin handed to Workstation and its prerequisites still effective |
| Build | Dogfood a new feat/workspace-skills checkout from develop; stop/report a refused commit |
| Session | Owner starts a fresh Cursor session in that checkout; the survey session writes the hand-off |
| Socket | Record Cursor's observed reachability as a Workstation boundary gap; keep per-mutation owner approval for this bootstrap |

The earlier folder prediction is superseded by the survey's observed collapsed slug. Source behaviour and the review amendment below further distinguish display names from actual checkout paths.

## Survey evidence

Full verbatim replies with `~/` paths: `~/.workers/records/bootstrap-review-2026-11-16.md`.

Summary for the skills:

- herdr 0.9.5; `HERDR_ENV=1`; socket answered inside Cursor's sandbox (boundary gap).
- Sidebar flat; linked worktrees are top-level workspaces; `source_workspace_id` is always the clone.
- Create/open from a linked worktree ID fails with `linked_worktree_source`.
- Branch `scratch/herdr-survey--sub` works; folder/label become `scratch-herdr-survey-sub` (`--` collapsed).
- Dirty remove: `dirty_worktree_requires_force`; do not use `--force`.
- `worktree open` on an open worktree: `already_open: true`.
- `workspace report-metadata` tokens work (display-only); branch name remains the nesting record.
- No plugins installed; no orchestrator fields; no colour/parent in schema.

## The plan (agreed)

1. Survey done (review file + scratch lifecycle cleaned up).
2. Draft + build brief in `~/.workers/records/`.
3. `herdr worktree create --workspace w8 --branch feat/workspace-skills --base develop --no-focus`.
4. Owner starts a new Cursor session in that worktree.
5. New session writes five skills (one commit each) and wiki (one commit), runs checks, writes hand-off notes.
6. Owner runs `wb promote agents feat/workspace-skills` and `wb pr agents feat/workspace-skills`.

## Skills built

| Skill | Does |
| --- | --- |
| `workspace` | Model, names, Workbench may only narrow; mode table for planned issue/pr/release |
| `workspace-open` | Find or create herdr workspace for a clone; check base branch; report |
| `workspace-worktree` | Create/open/adopt via herdr from the clone workspace ID; remove with the linked worktree's own ID; `--` nesting; cleanup |
| `workspace-maintain` | Report-first sweep, then each approved fix |
| `workspace-status` | Nested survey + JSON contract |

Built on `feat/workspace-skills` from `develop` at `0d53e41` (M0 merged as #11). Lifecycle exercise in the build session reconfirmed clone-only create/open, `--` nesting, folder collapse, `already_open`, and clean remove. Committing from the herdr worktree succeeded with the guest identity.

## Independent review and remediation (2026-11-16)

Findings fixed on this branch:

1. `workspace-worktree` / `workspace` hard rules: create and open use the clone ID; **remove** uses the linked worktree's own workspace ID.
2. `workspace-worktree` adopt: folder derivation collapses `--` to `-` after `/` → `-` (for example `feat/x--y` → `feat-x-y`).
3. `workspace-status` survey: collect `non_clone_workspaces` from `workspace list`; when the clone has no herdr workspace, report with `workspace_id: null` and do not call `worktree list` without an ID.
4. Root README and M2 program wording updated for the seventh-round order (M10 before M2).

## Remediation of PR #14 (2026-11-16)

After promotion, a cloud session reviewed #14 and fixed the following on the branch:

- The public skills named Workbench's worktree root (`~/.workers/spaces`) and the guest sandbox. They now read the root from `herdr worktree list`, and `workbench-herdr` narrows them.
- The base model made the herdr label the identity. It now matches by checkout metadata, as `workspace-open` and the [coordination proposal](../../architecture/workspace-coordination.md) do. Created workspaces are still labelled `<ns>`.
- This page quoted the owner word for word, including the VM's host and user names, in a public repository. It now points to the record's intent, as the records rule asks.
- `workspace-worktree`'s credits table split on `|` characters.
- The private Workstation skills were credited as Apache-2.0.
- The status JSON had no field for agents in the clone's own workspace. It now has a top-level `agents` field.
- The README and STATE wording assumed the work was still on the branch.

## Further Workstation review (2026-11-16)

The [PR #14 review](pr14-review.md) records additional findings at `1200a5f` and their corrections. The skill instructions now apply environment policy before socket access, validate actual 0.9.5 result envelopes, retain Git-only/unknown observations, require activity and retention checks before retirement, and avoid inferring configured roots or identities from lossy slugs. The live PR remains unmerged at this review. The earlier private survey is reported evidence; no cloud session repeated it.
