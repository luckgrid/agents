---
id: AS-REC-2026-11-14-MODULAR-REWRITE-REVIEW
kind: review
title: Modular rewrite plan review (2026-11-14)
summary: "Review of agents PR 9 at 5cd9fd3: corrected the deleted migration source, cross-repository validator path, empty export handling, session hook rename, installer ownership and recovery requirements, and first-pin prerequisites. Documentation only; no installer or skill implemented."
status: active
created: 2026-11-14
updated: 2026-11-14  # dated note: sync with Workstation's merges
related: [AS-PROGRAM, AS-STATE, AS-REC-2026-11-14-MODULAR-REWRITE, AS-REC-2026-11-14-MODULAR-REWRITE-KICKOFF]
---

# Modular rewrite plan review (2026-11-14)

The owner requested review, edits where needed, then squash merge of [agents PR 9](https://github.com/luckgrid/agents/pull/9). Reviewed head: `5cd9fd3d351065c5ed20e99e0fba93c9357fb9b5`; base: `f2e2bded0a0a86e72f382ee34f994862bbac836f`. The scope is the plan and executable kickoff brief, not implementation of M0 or its installer. The migration source is the base commit, whose two skill blobs match the reviewed head; unlike the PR head, that source remains in main history after squash.

Read all seven changed files, the root README, wiki rules, current validator, and Workstation's current root and package READMEs, U16, manifest generator, roles and session hook. Workstation is private for now. Its [PR 43](https://github.com/luckgrid/workstation/pull/43) and [PR 44](https://github.com/luckgrid/workstation/pull/44) were both confirmed merged during review. That satisfies the upstream merge dependency, but does not prove the guest has synced them.

## Findings and dispositions

| ID | Severity | Finding | Correction |
| --- | --- | --- | --- |
| MP-01 | Major | Part A deletes both guest skills; Part B and the house-rules fallback read them from moving `main`, which will lose them after merge and sync | Both reads use the base commit above, retained in main history after squash; verify the object before removal. Track removal here separately from delivery in Workstation |
| MP-02 | Major | The validator resolves relative paths against agents, but B1b passes a Workstation-relative path. Empty Git directories also disappear after clone/export | Use the absolute Workstation path and committed Part A validator; accept absent default `skills/` and a valid empty M0 export, but reject missing explicit validator paths and malformed child folders |
| MP-03 | Major | The rename omits `wb-agent-hook`, whose session context still tells every harness to follow `wb-guest-rules` | Rename that instruction and its checks; require explicit loading and harness proof rather than assuming descriptions or base skills load automatically |
| MP-04 | Major | The copy plan lacks a replacement for CLI-derived ownership, permits a source-receipt fast path without checking installed bytes, and implies a rename can replace a nonempty folder | Specify ownership from validated `skills.owned`, complete preflight, serialized runs, staged replacement with backup and interruption recovery, and receipt publication only after reconciliation |
| MP-05 | Major | No desired skills could skip removal of previously owned skills; destination-parent links and late conflicts were unspecified | Reconcile drops even for an empty desired set; reject redirected destinations; add simulations for these cases, unchanged-receipt edits and interrupted recovery |
| MP-06 | Minor | B2 says the manifest stays unchanged despite B1b adding files; B6 gives `./update-manifest` without its package directory | Include both skills and credits, reject nonregular payload entries, and give the repository-relative command |
| MP-07 | Major | First pin says only M2 must merge, although it depends on Workstation's installer and required Workbench skills | Make all three prerequisites explicit in roadmap and STATE; distinguish payload-only installs from the first agents pin |
| MP-08 | Minor | Tier wording reverses the narrowing order, and current M1 prose mixes the superseded one-folder design with namespaced skills | State public method, guest restrictions, project restrictions; no skill overrides harness enforcement. Make M1's current shape explicit while retaining dated owner history |
| MP-09 | Minor | The kickoff hard-codes a replacement Git identity, conflicting with current setup's configured guest identity and the public README's no-email rule | Use the configured identity and report an absent identity to the owner |

All findings above are corrected in the plan. Original owner quotations and the fifth/sixth-round history are preserved. No Workstation file, installed tool, skill or script was changed in this review. Future implementation still needs its own checks, independent review and owner-run runtime evidence.

## Validation

Runtime tests are outside this documentation-only change; the expanded simulation requirements are instructions for the implementation PR. Checks run with rumdl 0.3.2:

```text
$ rumdl check wiki/
Success: No issues found in 24 files (13ms)
$ rumdl fmt --check wiki/
Success: No issues found in 24 files (26ms)
$ python3 check_wiki.py
Wiki links, heading anchors and AS related IDs: 0 problems in 24 files
```

The link check is a temporary local review helper, not a new repository dependency. Both migrated skill blobs were fetched at the base and head and matched: guest rules `ff80011a9762ef8b970ee606e9c738277936b009`; herdr rules `68e72bc20d46a27bed6de75ec0272c9d0aee99fd`.

## Dated note, 2026-11-14 evening: sync with Workstation's merges

A cloud session, asked by the owner to "update the next steps and kickoff based on recent merges - need to sync everything up properly", read luckgrid/workstation at `24fc34c` (`main`) and this repository at `79a7c8b` (`main`).

- **Merged in Workstation since the review:** #44 (`wb done`, `wb install`, public HTTPS clones, Cursor's approvals unrestricted with its sandbox on, strace), #43 (the owner's checks of 2026-11-14), and #45 and #47 (documentation only, nothing under `packages/`). `wb` on `main` is `9899ebd9d6624a75f2466b44b9a5f16b2deb12e5115e5b8c4c21c09ef663a2b4`.
- **New ordering:** the owner chose to build the hand-off's trust-dialog handling next in Workstation (wait for the dialog, then offer the request again; never pre-trust), on the branch `workbench/handoff-trust-dialog`. It changes files Part B also changes (`simulate-wb-flow`, `agent-roles.md`, `daily-use.md`, `guest.sha256` and `wb`'s pin), so Part B now starts after it merges and is synced, as well as after Part A is committed. Part A doesn't touch Workstation and can run alongside it.
- **B1b's list of changes:** besides the notes folder, the two skills also name `wb notes` (now `wb records`), `~/.workers/worktrees/<repo>/` (now `~/.workers/spaces/<repo>/`) and the copied `herdr` skill, which M0 drops (now herdr's own `herdr --skill`). The kickoff adds them, each to be confirmed in Workstation's daily-use page or cheatsheet first.
- **Stale status lines** in the roadmap and STATE that still waited for #44 now name the hand-off change.
