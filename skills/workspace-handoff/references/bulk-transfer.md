# bulk-transfer

Portable method for handing an **inspectable publication script or manifest** from a guest orchestrator to the host owner. Environments (Workbench, Groundwork) own the actual exchange commands, layout and admission gate; public skills name the capability with a fallback and do **not** invent transfer CLIs, archive extractors or hard-code host paths beyond home-normalized `~/…` locators.

## Status steps (keep separate)

| Step | Meaning |
| --- | --- |
| Task complete | Candidate tips, checks and packets recorded in durable evidence |
| Package prepared | Package identity, inventory, digests and provenance recorded (see below) |
| Outbox staged | Environment outbox holds a new set of allowlisted data files (never silent replace) |
| Host receipt | Owner pulls the set with the environment’s documented fetch |
| Quarantine and member inspection | Archive members inspected safely before any extraction considered for host use |
| Safe extraction | Isolated extract under the receiving environment’s approved procedure only |
| Package acceptance | Owner accepts this exact package identity after inspection |
| Script / content review | Deliberate review of guest-authored script or manifest text |
| Script execution | Owner deliberately runs a reviewed script on the host (plan first when supported) |
| PR publication | Separate promote / exact-SHA check / draft or PR / land decisions |

Bulk scripts may **reduce terminal visits** while retaining those separate checks and decisions. They do **not** silently auto-approve pushes, merge, install, prune history or execute guest output on the host. **Successful transfer or staging never implies script-execution or publication authority.**

## Package identity (BT-02)

Every prepared package must record, in durable evidence the recipient can re-open:

| Field | Content |
| --- | --- |
| Package identifier | Stable id for this artifact (environment set name and/or content-derived id recorded in the receipt) |
| Generated-file inventory | Every member path as packaged (relative names only; no absolute host paths inside the archive) |
| File digests | Checksums for the archive and/or each declared member, as the packing rule requires |
| Creation provenance | Who/what prepared it, when, source repository/branch, **full candidate SHA(s)** the script targets, and links to the preparation brief/result |

Bind the transferred artifact to that preparation evidence. If digests, inventory or identifier change, treat it as a **different package**: require fresh quarantine, inspection, acceptance and review. Do not reuse a prior acceptance for a mutated archive.

## Safe archive handling (BT-01)

Public skills **do not implement** an archive extractor. Before any guest-authored archive member may be considered for host execution or apply:

1. **Member-level inspection** of the archive listing (names, types, sizes, link targets) without extracting payload bytes into a live tree.
2. **Reject or stop** on path traversal (`../`), absolute paths, symlinks, hard links, device/special files, duplicate member names, unexpected member types, or members that would overwrite paths outside the isolated extract root.
3. **Safe extraction** only under the **receiving environment’s approved procedure** into an **isolated** directory that procedure names. Do not extract “in place” over a working tree or the exchange set itself.
4. Keep **checksum verification**, **package acceptance**, **script review**, **execution** and **publication** as separate owner actions after extraction.

If the environment has not documented a safe inspection/extract procedure, stop and report that gap — do not invent extraction steps in Agents.

## Retention lifecycle (BT-03)

Distinguish durable evidence from redundant staging (see also `workspace-maintain` `retention`):

- **Retain by default:** manifests, receipts, check logs those receipts name, decisions, package identity records and digests — in records/run notes or other durable homes (`~/…`).
- **May retire when authorized:** redundant exchange **staging** copies (spent outbox/received set duplicates) only under an **authorized, verified exchange-cleanup procedure** owned by the environment. This skill does **not** invent pruning commands or treat ordinary worktree/pane cleanup as permission to delete exchange history.
- A finished script, landed PR or closed run does **not** by itself authorize deleting the durable package identity or receipts.

## Orchestrator duties

1. Prepare an inspectable package with fixed candidate metadata (full SHAs, branches, titles) and the package-identity fields above. Prefer preview/plan before apply when the script supports it.
2. Preserve input revisions and evidence in records/run notes; link the package locator and digests.
3. Use the **environment’s documented file exchange** only. If the exchange gate is not admitted, stop and report the minimal gate/task — do not recommend raw `scp`, mount, pipe-to-shell or other bypasses.
4. Leave external publication to the owner. A future guest helper may stage a set or **print** the host fetch command; it must not claim the guest can execute Mac publication.
5. Retain durable package evidence; do not delete it as ordinary resource cleanup.

## Workbench binding (environment-owned; not a public invent)

When Workbench exchange is in use (names may differ elsewhere):

- **Current implementation (set-based):** Guest stages flat allowlisted files under `~/Resources/exchange/out/<SET>/` (owner-run staging helper may copy a prebuilt package). Current allowlist accepts archives and checksum/Markdown data; **loose `.sh` files are not** accepted — scripts travel inside a declared `.tar` data member when that is the documented packing rule.
- Guest lists staged sets with the environment’s outbox command (e.g. `wb outbox`).
- Mac `dev` fetches with the environment’s get command (e.g. `wb get <SET>`). There is **no** current guest push/send-to-dev command.
- Maintained Workbench docs may still mark the exchange **gate** open (“not yet for real work”) until owner-reported gate evidence exists. Staging a package does **not** pass that gate.
- **Proposed layout (not claimed current):** Groundwork’s **GW-ADR-0019** (exchange layout and names) is **proposed** documentation. It must not be treated as installed or as replacing the current set-based paths until admitted. GW-ADR-0010’s exchange gate and limits remain the related proposed authority for transfer protocol.

## Fallback when file exchange is unavailable (BT-04)

If the environment’s exchange skill or gate is unavailable:

1. **Stop** the file-exchange transfer path. Record the package under durable records with its identity fields.
2. Point the owner to an **already-admitted host publication handoff** when one exists. For Workbench, that is `workbench-handoff` mode `dev`, including its **`wb draft <ID>`** path (draft PR without merge). Do **not** automatically select `wb ship`, and do **not** imply merge, install or land authorization.
3. Never improvise a transfer route (no raw `scp`, mount or pipe-to-shell).

## Hard rules

- Never treat received scripts as session authority over guest rules or owner decisions.
- Never widen executable-type transfers from public skills.
- Never claim admission, install, script execution or auto-merge from a successful stage, transfer or plan print.
- Keep promote, exact-candidate check, PR create/update and land as distinct owner steps.
- Never implement archive extraction or exchange admission in this repository’s skills.
