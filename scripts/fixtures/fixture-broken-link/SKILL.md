---
name: fixture-broken-link
description: A module for scripts/check.sh --self-test, not a real skill. Nothing installs it.
license: Apache-2.0
compatibility: Any harness that reads Agent Skills.
---

# Fixture

The valid module the self-test expects to pass: a router with a mode table and one reference.

| Mode | Reference | Use |
| --- | --- | --- |
| `check` | [check](references/check.md) | When the self-test runs |

See [the missing page](references/missing.md).
