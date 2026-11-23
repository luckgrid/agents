# Lint

Check both the edited page and its relationships to the rest of the wiki.

1. Run the wiki's Markdown formatter/linter. Luckgrid's shared template pins
   rumdl 0.3.2 and uses `rumdl check <wiki-path>`.
2. Check relative links and anchors, `related:` IDs, duplicate IDs, and any
   index that the change affects. Verify links after moves and renames.
3. Inspect frontmatter against the shared template: stable unique ID, valid
   kind and status, plain dates, and IDs (not paths) in `related`.
4. Check that a page is filed by its role and that records preserve their own
   dates and evidence. Confirm proposals, decisions, historical facts, and
   delivery/status claims remain correctly labeled.
5. Read the rendered diff for private runtime details and unsupported claims.

`uwiki validate` is optional under the shared template until it ships as a
standalone CLI. Do not report its absence as a defect. If unavailable, use the
checks above and the wiki's own scripts; do not install tools as part of lint.
