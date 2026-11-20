# request

Build the files a reviewer needs. The requester writes; reviewers only read.

## Steps

1. **Pin the range.** Resolve `base` and `head` to full 40-hex SHAs with Git. Confirm base is an ancestor of head and the checkout is clean at head. Record both.
2. **Choose lenses.** Spec, standards and correctness always; add security, tests, lean or operability by risk. One lens per reviewer turn, so several lenses mean several turns (or reviewers) over the same range.
3. **Write the briefing pack** to a file: the same text goes to every reviewer, word for word.
   - What changed, in a short summary of the files and intent (not the raw diff).
   - The spec: the issue, plan or decision the change answers, quoted or linked by locator.
   - The standards: the repository rules that apply (README, wiki decisions).
   - Not included: the author's session, reasoning or opinions of the change.
4. **Write the request file** next to it, with these fields:

| Field | Content |
| --- | --- |
| Repository | Locator such as `luckgrid/agents`, never a machine path |
| Base, head | Full SHAs from step 1 |
| Lens | Exactly one, from the lens list |
| Briefing | Location of the briefing file |
| Findings file | Where the reviewer writes its result |

5. **Keep the markers out.** Neither the briefing, the request file nor the fixed start prompt may contain the result markers defined by `council-review`'s `review` reference. Check with a text search before handing over.
6. **Hand over by file.** The start prompt is fixed: it names `council-review` and the request file's location. The requester does not paste findings, hints or opinions into it.

If the head moves after the request is written, write a new request; don't edit the old one in place.
