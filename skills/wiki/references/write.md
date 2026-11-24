# Write

Use the shared template authority, wiki-nest's `wiki-structure.md`, for the
complete page profile. Follow a target wiki's README and existing conventions
where they do not conflict with that authority.

- Create only folders that have content beyond their README. Keep a page a
  single page until it has useful child pages of its own.
- Give each non-README page the required frontmatter: stable unique `id`,
  `kind`, `title`, concise `summary`, `status`, plain `created` and `updated`
  dates, and `related` page IDs. Optional fields must follow the template.
- `related` contains IDs, never paths or slugs. Use the wiki's ID prefix and
  check that each ID is unique.
- Put dated episodes in `records/YYYY-MM-DD-<focus>.md`; the date is when the
  episode began, not when a file was committed. Put a type in `kind`, not the
  filename. Group a kind only when the template's threshold is met.
- Put present position, open gates, and next steps in `records/STATE.md`.
  Other pages link to STATE instead of repeating mutable status.
- Keep proposals marked proposed and accepted decisions distinct. Do not
  upgrade a proposal, report, or hypothesis into a decision or fact.
- Update indexes and links required by the wiki, without rewriting historical
  evidence. Preserve stable IDs across moves and renames.

Run [lint](lint.md) after writing. For an episode, also use [record](record.md);
for a decision, use [decide](decide.md).
