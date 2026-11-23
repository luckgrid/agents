# Coordinate work

Start from the requested outcome, then make the work inspectable and bounded.
Write a brief with the goal, named checkout and branch, expected starting
revision, allowed files, exclusions, checks, result-file path, timeout, and
report format. Split work only where tasks can proceed without concurrent edits
to shared files or shared target refs.

Give each editing task its own checkout and writer. Name one integrator for any
shared target. A branch label, agent state, open pane, or successful dispatch
does not reserve a resource or authorize a write. Before a shared integration,
the integrator rechecks the target tip and each candidate revision, then records
the integration result.

Use native backend support for agent dispatch and status when available. A
backend without these features may still support a narrower workflow; say what
is unavailable and do not emulate reliable identity or completion from pane
text alone. Keep long-running processes observable with a named owner, purpose,
target, output location, and stop condition.

Audit every result against its brief: inspect the changed paths, candidate
revision, check output, and open questions. A completion label is an observation,
not verification. Prefer the installed `council` / `council-review` skills for
independent review when they are available; if they are not installed, use the
environment's admitted review route (for example Workbench's reviewer role and
a read-only brief) and say which fallback you used. Do not invent a parallel
review framework. Close sessions only after preserving needed evidence and
when authorized to do so.

For partial failures, record what succeeded, what remains, and the next safe
action. Do not blindly repeat a timed-out operation. The next coordinator
receives a durable handoff naming outstanding tasks, observed handles, result
paths, candidate revisions, constraints, and next actions; they revalidate live
state before acting.
