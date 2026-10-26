# Tasks

## 1. Value order in the facet queries

- [ ] 1.1 Change the `ORDER BY` of the System, Type, Format, Genre, Tag, and Location facet queries to case-insensitive name (`ulower`), then exact name, and verify with a test over mixed-case values, including one that pins where an accented first letter sorts
- [ ] 1.2 Order Availability as Downloaded then Cloud regardless of counts, and verify with a test where Cloud has more items
- [ ] 1.3 Add a test that checking a value (changing the other facets' counts) leaves every remaining value in the same relative order

## 2. Section order

- [ ] 2.1 Reorder `FACET_SECTIONS` to Availability, Type, System, Genre, Format, Location, Tags with a comment stating the rule, and verify with a unit test on the constant's order
- [ ] 2.2 Verify a section with no values is skipped without moving the others (test on `render_facet_section` inputs or the section filter)

## 3. Verification and docs

- [ ] 3.1 Verify `FiltersPaneView` redraws only on a real facet change after this order lands (test that equal facets do not notify and a changed value list does)
- [ ] 3.2 Update the filters section of `docs/site/` to describe the order, and verify the page builds
- [ ] 3.3 Run `make fmt`, `make size-check`, clippy with `-D warnings`, and `cargo test --workspace`

## Workflow follow-up

- Rebase onto PR 528 if it has not merged.
- Archive the change after the PR merges.
