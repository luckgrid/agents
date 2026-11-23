# Dispatch and observation

Dispatch only when requested or authorized by the governing environment.
Prepare a bounded brief that names the goal, checkout, branch, expected
revision, scope, checks, durable result path, timeout, and reporting contract.
Give every writer a separate checkout and use the selected backend's native
dispatch feature when it has one.

Observe native status and wait results with their source and time. Use portable
labels only as a summary: **active** means work is observed in progress;
**blocked** means an obstacle is reported; **hand-off** means a result is ready
for a recipient to inspect; **done** means the brief has been verified. Missing
or ambiguous observation is **unknown**. Backend lifecycle labels do not prove
that a brief is satisfied.

After a timeout or uncertain submission, inspect the current state before any
retry. Do not answer unfamiliar prompts automatically. Audit the result file,
changed paths, candidate revision, checks, and limitations before claiming
completion. Keep handoffs in files; do not type another agent's text into a
foreign pane. Close a pane only when authorized and after retaining required
evidence.
