---
id: AS-REC-2026-11-24-ISSUE-12-CLOSEOUT
kind: session
title: Issue #12 validator gaps — evidence for authorized close
summary: "Map M0 review gaps A–C on check.sh to the regression coverage merged through Agents #24 at cf86b6a…; recommend closing GitHub issue #12 only through the authorized host workflow. Source on main is not a pin or install."
status: active
created: 2026-11-24
updated: 2026-11-24
related: [AS-STATE, AS-REC-2026-11-15-M0, AS-REC-2026-11-24-INTEGRATION-DELIVERY]
---

# Issue #12 validator closeout evidence (2026-11-24)

## Intent

Confirm whether [GitHub issue #12](https://github.com/luckgrid/agents/issues/12) (three `scripts/check.sh` gaps from M0’s independent review) is fully addressed by the validator work landed via squash-merge [#24](https://github.com/luckgrid/agents/pull/24) at `cf86b6ad3298c4468d1a11c07c3613670f991740`, and prepare an evidence-backed closeout for the **authorized** issue workflow. This record does **not** close the issue from the guest.

## Gaps and coverage

| Gap (issue #12) | Fix on `main` @ `cf86b6a…` | Self-test cases (must FAIL or pass as named) |
| --- | --- | --- |
| **A** Multi-line link target not checked | `links()` scans across continued lines (not one line at a time) | `multiline-link` → FAIL missing file; `multiline-link-across-paragraph` → pass (blank line breaks the link) |
| **B** YAML `#` comment treated as value | `strip_yaml_comment` on plain/quoted scalars before field checks | `yaml-inline-comments` → pass; `yaml-comment-only-description` → FAIL description missing; `yaml-plain-apostrophe-comment` → pass |
| **C** Mode table without outer pipes not found | `tables()` accepts unpiped and mixed-pipe mode tables | `mode-table-without-outer-pipes` → pass; `mode-table-mixed-pipes` → pass; related malformed/prose cases present |

Additional related hardening (wrong-width outer-piped rows, unpiped prose ending a table) shipped in the same validator line and is covered by self-test cases beyond the original three bullets; those extras are not substitutes for A–C.

## What was checked (this session)

| Command | Tip | Result |
| --- | --- | --- |
| `./scripts/check.sh --self-test` | `cf86b6ad3298c4468d1a11c07c3613670f991740` (also on PR #23 tip after ff) | exit 0; `ok: self-test, 50 cases as expected`; A–C cases listed above present in output |
| `./scripts/check.sh` | same | exit 0; `ok: 11 skills checked` |

Evidence copy: `~/…/state/runs/20261124-agents-closeout-m5/evidence/issue-12-self-test.txt` (when written with this closeout).

## Disposition

- **Source resolution:** the three originally reported gaps have matching fixes and regression coverage on `main` through #24.
- **Issue close:** recommend the owner (or authorized host workflow) close #12 with a comment linking this record and tip `cf86b6a…`. The guest has **no** GitHub write credential and must not claim the issue closed.
- **Not claimed:** Workbench `agent_skills_commit` pin, U16 install of agents skills, or any runtime admission.

## What's next

1. Owner closes #12 via the authorized issue workflow with the evidence above.
2. Keep first-pin / install as separate Workstation-owned steps.
