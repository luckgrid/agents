# Architecture

Standing descriptions and clearly marked proposals for how this repository works. They change when the repository changes; the history behind them is in [records](../records/README.md).

- [Skill modules](skill-sources.md): the module shape, the three tiers and where each is installed, namespaces, composition, and what `scripts/check.sh` proves.
- [Guest install](guest-install.md): how the Workbench guest is to install these skills (its step U16, copying folders from two sources), and what it needs from this repository.
- [Agents workspace and configuration boundaries](agents-workspace.md): the proposed layout (skills at the root, `configs/`, `crates/`), who interprets which configuration, public defaults with a private overlay, and proof steps AG1 to AG3, derived from AS-ADR-0001.
- [Common workspace coordination and workflow API](workspace-coordination.md): proposed shared vocabulary, workbranches, operation contracts, coordination, hand-offs and staged skill ideas; Herdr is the first backend, and Workbench narrows it.
- [Control, Record and Audit tab roles](control-record-audit.md): proposed portable distinction among harness profiles, standing tab roles and per-audit assignments; handoffs and council fit (layout owned by Workstation).
- [Workspace-owned integration delivery](integration-delivery.md): proposed portable delivery-unit contract, optional builders, designated integration worktree, one integrated PR or independent PRs, exact-head review and operator-directed remediation.
- [Portable skill capability architecture](skill-capability-evolution.md): owner-selected direction; proposed conduct/workspace separation, multiple session capabilities, Council/grill, write/wiki profiles, resources and migration proofs.
- [Conduct and session migration](conduct-session-migration.md): proposed P4/P5a mapping from orchestration/workspace consumers to portable conduct and the session-view family; Herdr compatibility; Kitty/Herdr Core as future references only.
- [Council phase two](council-phase-two.md): proposed P7 deliberation and bounded decisions after M5 grill acceptance; not part of the grill delivery.
