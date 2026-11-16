# survey

Collect a read-only picture of one namespace (default: the repository of the current checkout). Apply the base skill's environment-policy preflight before socket commands. Never fetch, sync, create an attachment or repair a checkout during this survey.

## Collect and reconcile

1. Inspect the clone with native Git: repository top-level/common directory, local base ref/tip and all registered worktrees (`git worktree list --porcelain`). Keep plain/unattached linked checkouts and detached HEAD entries in the report.
2. Obtain `herdr workspace list` through the permitted route. Require a complete successful `workspace_list` result. Find the clone by the exact metadata rules in `workspace-open` (load that reference; if unavailable, retain Git results and mark the Herdr association unverified). Multiple matches are ambiguous, not absent.
3. With exactly one clone ID, obtain `herdr worktree list --workspace <clone ID>`. Require `result.type: worktree_list`, matching `result.source.repo_root` and `source_workspace_id`. Read entries from `result.worktrees`: `path`, optional `branch`, `is_detached`, `is_linked_worktree`, optional `open_workspace_id`. Missing clone attachment means no Herdr worktree query, not an empty Git worktree set.
4. Reconcile by exact checkout path and repository identity. Linked attachments of this clone belong with its worktrees; unrelated attachments go into `non_clone_workspaces` (IDs/labels only). Missing metadata goes into `unclassified_workspaces`; never classify it as absent/non-clone or mutate it. Conflicting Git/Herdr data is a finding, not a guessed association.
5. If permitted, obtain `herdr agent list` and pane metadata when needed. Require `agent_list`; map `result.agents[].agent` to `kind`, `agent_status` to `state` and `pane_id` to `pane`; retain `terminal_id` for deduplication. Attach by verified workspace ID, using cwd only as corroboration. Put uncertain associations in `unassigned_agents`. A detector state is advisory; it is not proof no process is using the checkout.
6. Compare each observed checkout HEAD (including detached HEAD) against local `refs/remotes/origin/<base>` only when both resolve. Record that exact `comparison_ref` and `comparison_tip`, the checkout `head`, and ahead/behind counts. Missing origin/base, failed Git query or unknown HEAD means null counts and an explanation, never zero. Keep the selected local `base_ref`/`base_tip` separate: guest-local `develop` need not have a remote counterpart.
7. Report last-fetch information only if known and label its scope. `FETCH_HEAD` mtime is at most evidence of that file's update, not proof this particular remote/base was refreshed; default freshness is unknown.
8. Inspect empty-folder candidates only in an independently verified worktree root. Explicit checkout paths need not share a parent; a main-only/empty list cannot establish the root. If unknown, report root inspection unavailable. Candidates are findings only, not proof of orphanhood or permission to delete.

On a denied/failed/malformed provider query, preserve successful Git observations and mark provider state/reason. Do not probe around policy or repeatedly contact a refused socket. A missing executable is unavailable, a failed query is error, and only a successful complete query can establish absence. Record source and Unix-millisecond observation time per query; the combined view is not atomic.

## Grouping and display

A branch with exactly one `--` may be displayed beneath the matching parent branch; more separators, an empty suffix or a missing parent is a grouping finding. Keep it visible rather than dropping it. This is a naming convention, not verified Git ancestry or a removal dependency. Never read grouping from folder slugs.

Present an indented list or table with clone/base, each checkout/HEAD/attachment, grouping, observed agents and findings. Show unknown/missing/error values explicitly. When the clone attachment is absent, show Git-only worktrees with null workspace IDs. Do not render ASCII box trees.

## Draft JSON projection, version 1

Print a fenced `json` block of the same observations. This is the skill's derived report, **not Herdr's native schema or a shipped plugin API**. A future consumer must validate it and the native observations; prose generation alone is not machine-interface conformance.

```json
{
  "schema_version": 1,
  "namespace": "agents",
  "clone_path": "~/Workspaces/agents",
  "workspace_id": null,
  "base_branch": "develop",
  "base_ref": "refs/heads/develop",
  "base_tip": null,
  "comparison_ref": null,
  "comparison_tip": null,
  "last_fetch": null,
  "fetch_scope": "unknown",
  "worktree_root": null,
  "partial": true,
  "observations": [
    {
      "source": "git",
      "state": "present",
      "observed_at": 0
    },
    {
      "source": "herdr.workspace_list",
      "state": "error",
      "observed_at": 0,
      "reason": "policy_denied"
    }
  ],
  "agents": [],
  "worktrees": [
    {
      "branch": "feat/x",
      "head": null,
      "workspace_id": null,
      "path": "<observed checkout path>",
      "is_linked_worktree": true,
      "is_detached": false,
      "comparison_ref": null,
      "comparison_tip": null,
      "ahead_of_base": null,
      "behind_base": null,
      "children": [],
      "agents": []
    }
  ],
  "orphans": [],
  "non_clone_workspaces": [],
  "unclassified_workspaces": [],
  "unassigned_agents": [],
  "findings": ["Herdr unavailable to this session; Git-only report retained"],
  "errors": [
    {"source": "herdr.workspace_list", "code": "policy_denied"}
  ]
}
```

Example zero timestamps and null tips are placeholders; use actual observations when available, otherwise null with a reason. Paths are local observations; home-normalize public excerpts and omit private identifiers. Top-level `agents` covers the clone attachment; each worktree has `children` and `agents` arrays, including empty ones. Empty arrays never prove absence when their source was unavailable. `orphans` holds verified-root empty-folder candidates with supporting findings, not deletion approvals. Omit orchestrator keys until a native authority records them.
