# questioning-contract

Settled contract for `grill` as an **independently reusable questioning primitive** ([AS-ADR-0003](../../../wiki/records/decisions/0003-portable-skill-capabilities.md) D3). Council may compose this method later; this skill does not implement Council expansion.

## Method

| Element | Rule |
| --- | --- |
| Answerable frontier | Ask only questions whose prerequisites are settled |
| Recommended alternatives | Each question offers a recommended answer ("yes" accepts) plus real alternatives when they exist |
| Evidence checks | Agent gathers facts from code/docs/records before asking; do not quiz the owner for look-ups |
| Conversational fallback | If structured choice tools are unavailable, use numbered prose questions |
| Delegated decision limits | Grill settles design preferences the owner (or an explicitly delegated routine chooser) may answer; it does not authorize merge, pin, install, admission or out-of-scope policy |

## Non-goals

- Not a session backend, orchestrator or publication skill
- Not a substitute for `council-review` independence or dissent recording
- Not automatic acceptance from silence, drafts or green checks

## Composition notes (future Council)

Council may call this frontier method for grilling inside deliberation. Keep grill invocable alone. Do not duplicate the frontier algorithm inside Council source in this delivery.
