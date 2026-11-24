---
id: AS-REC-2026-11-24-SESSION-CAP-PREP
kind: session
title: Session capability family — consumer inventory and prep evidence
summary: "Inventory orchestration/workspace consumers; prepare evidence for conduct and session-view family. Kitty and Herdr Core are preferred future references; current Herdr remains operational. No backend removal."
status: active
created: 2026-11-24
updated: 2026-11-24
related: [AS-ADR-0003, AS-ARCH-SKILL-CAPABILITY-EVOLUTION, AS-REC-2026-11-24-SKILL-SYSTEM-PLANNING]
---

# Session capability prep (2026-11-24)

## Intent

Prepare evidence for the future `conduct` rename target and the session capability family (`session-view` and siblings) without changing working Herdr bindings or removing tmux fallback documentation prematurely.

## Consumer inventory (main @ `cf86b6a…`)

Full associated-file audit: run evidence `associated-file-audit.md`.

| Skill | In-repo mention files | Role for migration |
| --- | --- | --- |
| `orchestration` | 13 | Future `conduct` rename target (D1); keep Herdr/tmux as **mappings**, not taxonomy |
| `workspace` + leaf skills | 48+ overlapping | Stay Git/checkout identity; optional association to session family |
| `orchestration` modes `herdr` / `tmux` | skill references | Preserve until session-view mappings prove equivalent |

## Preferred future references (not installed)

- Kitty sessions / remote control — view/layout/navigation examples
- Herdr Core — PTY/session lifecycle/client examples
- Current Herdr TUI — **operational** Workbench binding today

## Guardrails

- Do not remove working backend behavior in this prep
- Do not put concrete backend commands in general `conduct`
- Do not treat AS-ADR-0003 as shipped registry or pin/install authority
