# sweep

## 1. Survey first

Load `workspace-status` and its `survey` reference; stop if required rules are missing. Present the read-only outline and draft JSON projection. Never fetch or sync as part of this sweep.

## 2. Findings

Number proposed fixes with exact checkout paths, refs/tips, provider IDs, evidence and consequences:

- Missing clone attachment after a complete successful query: `workspace-open`.
- Plain linked checkout: `workspace-worktree` `adopt`, after root/destination and no active users are verified.
- Requested existing checkout without attachment: `open`; a new topic: `create`, after collision checks.
- Clean linked checkout no longer needed: `remove`, only with activity, tip-retention and ignored-data checks, and only after durable briefs/findings/receipts live outside that checkout (`retention` reference).
- Empty folder in a verified root: candidate only; confirm no registration, active use, symlink or retained data before an approved empty-directory-only removal. Never recursive removal.
- Behind a local origin ref: report comparison ref/tip and freshness. Refresh/integration is a separate authorized environment workflow.

**Never** propose deleting environment records, historical run folders, admission results or other retained context as a sweep fix. Resource cleanup (panes, eligible worktrees, fixtures) is not context pruning; pruning is an explicit environment action outside this sweep (see `retention`).

Unavailable provider state, conflicting metadata or unknown root/activity blocks dependent fixes. Never propose operating on unrelated or unclassified attachments.

## 3. Approve then act

Apply only the approved scope, one item at a time, through the relevant skill/reference; stop if it is missing. Do not ask again when the current session already explicitly approved that exact action and consequences. Before acting, revalidate paths, tips, associations and active users. A changed candidate invalidates the old approval/plan.

After each action, query actual state; report surviving resources on partial failure. On refusal, stop without a workaround. Never force-remove, answer a trust dialog or terminate another session. Branch deletion is separately approved and preserves the exact candidate or explicit abandonment decision.

## 4. Close

Report changed targets, retained work, evidence and unresolved findings; no healthy/idle conclusion from missing observations.
