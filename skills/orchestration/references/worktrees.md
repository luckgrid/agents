# Worktree coordination

First establish that the selected environment supports worktree operations and
that its rules authorize the requested change. Identify the repository, base
ref and tip, branch, destination, intended owner, and current checkout or
backend attachment. Refresh Git registration and backend state before a
mutation; refuse occupied branches, conflicting destinations, or ambiguous
matches.

Use the owning backend's documented creation and attachment sequence when
available. Otherwise report that capability as unavailable and follow the
environment's authorized worktree procedure. After creation, verify the actual
branch, path, and attachment from fresh results. For partial failure, preserve
and report surviving state before considering another attempt.

Before retirement, preserve durable briefs, findings, receipts, and required
checkout data. Verify the exact tip is retained or explicitly abandoned, inspect
tracked, untracked, and ignored data, and establish that no users or processes
remain. Unknown activity blocks unattended removal. Remove through the owning
environment's documented order and never force. Branch deletion needs separate
authorization. Refresh state before each step and report partial failure
without removing unrelated resources.
