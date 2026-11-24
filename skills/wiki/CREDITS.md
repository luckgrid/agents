# Credits

This module is written in Luckgrid's own words and takes ideas only from the
sources below. No source text is copied.

| Source | Commit or version | Ideas used |
| --- | --- | --- |
| [Karpathy's LLM wiki gist](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f) | Gist | Preserve raw sources, maintain a wiki, follow its schema |
| [kfchou/wiki-skills](https://github.com/kfchou/wiki-skills) | `98126172` | Separate query, lint, audit, and integration workflows |
| [vanillaflava/llm-wiki-skills](https://github.com/vanillaflava/llm-wiki-skills) | `ff6f7be1` | Turn sessions into records; keep indexes and links coherent |
| [tobi/qmd](https://github.com/tobi/qmd), `skills/qmd` | `86d2365d` | Search first, retrieve full pages, then cite evidence |
| Luckgrid's wiki-nest `wiki-structure.md` (private) | Current shared template authority | Folder and record structure, frontmatter, evidence and decision rules, and checks |

Luckgrid's public module reference list records these sources and their
licences in `wiki/resources/skill-references.md`. The shared template remains
the authority for Luckgrid wikis; consult the target wiki's README for its
local scope and rules.
