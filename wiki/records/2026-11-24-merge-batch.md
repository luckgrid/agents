---
id: AS-REC-2026-11-24-MERGE-BATCH
kind: session
title: Merge batch — #23, #25, #27 and #26 landed on main
summary: "Owner-authorized squash landing in plan order (#23 → #25 → #27 → #26): published, landed-candidate and squash SHAs, conflict resolutions, checks run, and the open scope note on #27's session-capability record. Source only: nothing pinned or installed."
status: active
created: 2026-11-24
updated: 2026-11-24
related: [AS-REC-2026-11-24-GRILL, AS-REC-2026-11-24-AG1-1, AS-REC-2026-11-24-AUTONOMOUS-WORKFLOW-CONTRACTS, AS-REC-2026-11-24-CONDUCT-SESSION-COUNCIL-SPECS]
---

# Merge batch (2026-11-24)

The owner authorized a cloud session to merge the open PRs in the order of
the landing plan and to fix stack issues on the way. Each PR was qualified
at its exact candidate, squash-merged with a guard on that candidate's head
SHA, and read back. Each landed tree matched its candidate's tree. Branches
that needed `main` got an append-only merge: no rebase, no force-push.
GitHub's automatic head-branch deletion was undone by restoring each ref at
its merged head.

| Order | PR | Published head | Landed candidate | Squash on `main` |
| --- | --- | --- | --- | --- |
| 1 | [#23](https://github.com/luckgrid/agents/pull/23) integration delivery, capability direction, Conduct/Session and Council specs | `689730780a49c858801616c514bfebe0005d257e` | `7c2284889bdbfc090293c10051d8965e39d1bb47` | `7b44bf13edc7b920198f9441255f28d7a61395d8` |
| 2 | [#25](https://github.com/luckgrid/agents/pull/25) M5 `grill` | `7d8e319ed8e72e92a20e57601369502ef61d919f` | `f19aaaee67b5512e73ed3ec585471a0db19b48c7` | `41792744926c068ba3ef0b80ccb4eed971536ec1` |
| 3 | [#27](https://github.com/luckgrid/agents/pull/27) AG1.1 native validator parity | `4540a6ecefb211868fbddba71a2917a1ec350d4b` | `2dccb8c4a884f2b5fee3999723913e14ce890bb1` | `4c5609e11e0752018c3d41404e621aa66be26b14` |
| 4 | [#26](https://github.com/luckgrid/agents/pull/26) portable autonomous workflow research | `9cf025763985fa7f767611649ac27bc0d06e13fc` | `7cea5e15001b71f6936d50fe472fa7f7cb575dd6` | `7381f389076275501f28eccb38441a580af5ea23` |

`main` before the batch: `cf86b6ad3298c4468d1a11c07c3613670f991740`. After #26:
`7381f389076275501f28eccb38441a580af5ea23`. This record lands in its own
closeout PR after that.

## What changed on the branches

- **#23:** one whitespace fix (rumdl MD009 in
  `wiki/architecture/integration-delivery.md`).
- **#25:** merged landed #23. The files #23 owns took `main`'s version; the
  index added both grill records; STATE kept #17's Council provenance and
  the proposed specs and recorded grill as merged source. Also one MD076
  fix and `program.md`'s M5 status. Effective scope: the expected 11 files.
- **#27:** merged landed #25. The index and STATE were resolved as a union,
  with AG1.1 recorded as merged source. The shell check is retained;
  AG1.2 and Moon admission are still open.
- **#26:** merged landed #27. Index entry added once.

## Checks

At every candidate: `./scripts/check.sh --self-test` (50 cases),
`./scripts/check.sh` (11 skills at #23, 12 from #25 on) and
`rumdl check wiki/` (rumdl 0.3.2), all exit 0. At #27's candidate, with
grill present, these also passed: `cargo build -p agents-check`,
`./target/debug/agents-check --self-test`, `./scripts/ag1-compare.sh` and
`./scripts/ag1-native-proof.sh`, including its strace and no-python3-on-PATH
proofs. Exact output is in each PR's merge-batch comment.

## Open

- **Scope note on #27:** `2026-11-24-session-capability-prep.md` is a
  session-capability inventory, not AG1.1 work. It was kept as authored,
  and its `related` IDs resolve now that #23 has landed. It's for the owner
  to decide whether it stays where it is.
- Pins, guest installation, issue #12 closure and branch cleanup stay on
  their own authority paths.
