# Herdr mapping

Use this mapping only when `HERDR_ENV=1`. Check `herdr --version` and require at
least 0.9.5. If either precondition fails, stop Herdr operations. The version
floor does not guarantee that later releases preserve a command or response
shape: defer to installed `herdr --skill` and relevant `herdr <group> --help`
for current syntax, prerequisites, and capabilities. Do not run bare `herdr`
for discovery or assume socket reachability grants permission.

Map dispatch and observation to the installed `pane` and `agent` workflows when
present. Read identifiers only from successful current replies and validate
their associations. Treat `working`, `blocked`, `idle`, `done`, and `unknown`
as observations, not proof that a brief is satisfied. Inspect after timeouts or
uncertain submissions before retrying, and retain the durable file handoff.

Map worktree creation, opening, and removal only to the installed `worktree`
workflow when present. Validate the source and resulting checkout association
using fresh responses. Follow the lifecycle in [worktrees](worktrees.md); never
force. A refused or malformed reply is an error, not evidence that a resource
is absent. Respect the environment's authorization rules even when Herdr
documents an available operation.
