# review

## Output

Produce, in this order: a short human summary, then one result block.

Where it goes depends on the request's **output mode** (`emit` or `file`) as `council-review`'s router describes. Either way the **content** is the same.

**Verdict:** `PASS` when no blocker or major finding stands; otherwise `CHANGES`.

**Findings:** at most ten, most severe first. Each has:

| Field | Meaning |
| --- | --- |
| `id` | `F1`, `F2`, … unique in this block |
| `severity` | `blocker` (wrong or unsafe, must fix), `major` (should fix before merge), `minor` (worth fixing, can follow), `nit` (taste) |
| `file` | Repository-relative path |
| `line` | Line in the **head** revision |
| `reason` | What is wrong, with the evidence you read |

If more than ten qualify, report the ten most severe and say in the summary that more were left out.

## The result block

One JSON object between these two marker lines, each alone on its line:

```text
=====COUNCIL-RESULT-BEGIN-7f3a=====
=====COUNCIL-RESULT-END-7f3a=====
```

Between them:

```json
{
  "base": "<40-hex SHA>",
  "head": "<40-hex SHA>",
  "lens": "<one lens>",
  "verdict": "PASS",
  "findings": [
    {"id": "F1", "severity": "major", "file": "path/to/file", "line": 12, "reason": "…"}
  ]
}
```

`base`, `head` and `lens` repeat the request's; `findings` is `[]` when there are none. `PASS` with any blocker or major in `findings` is invalid.

## Why the markers are defined here

The markers above exist only in this reference. Fixed prompts and request files never contain them, so a reviewer echoing its input can't produce a block. A reader of a findings file or pane capture takes the **last** well-formed block, and discards it if `base`, `head` or `lens` don't match the request, or if the schema is invalid. Don't write the markers anywhere else, including in your summary: refer to them as "the result block".
