## Architecture Decision Records

Numbered sequentially (`NNNN-kebab-title.md`). Each record is immutable once
merged — a changed decision gets a new record that supersedes the old one
(noted in both), not an edit to history.

Use `0000-template.md` as the starting point for a new record.

## Index

| ADR | Title | Status |
|---|---|---|
| [0001](0001-migrate-to-gpui-kit.md) | Migrate to gpui-kit | Accepted (amended) |
| [0002](0002-invert-crate-graph-and-add-library-facade.md) | Invert crate graph and add library facade | Accepted |
| [0003](0003-sqlite-library-index.md) | SQLite library index | Accepted |
| [0004](0004-platform-integration-layer.md) | Platform integration layer | Accepted (tag rule superseded by 0008) |
| [0005](0005-themes-as-data-files.md) | Themes as data files | Accepted |
| [0006](0006-container-format-abstraction.md) | Container format abstraction | Accepted |
| [0007](0007-core-work-queue.md) | Core work queue | Accepted |
| [0008](0008-reconcile-tags-by-which-side-changed.md) | Reconcile tags by which side changed | Accepted (one sentence superseded by 0009) |
| [0009](0009-do-not-read-tag-colors-from-files.md) | Do not read tag colors from files | Accepted |
| [0010](0010-guard-the-render-path-against-catalog-scans.md) | Guard the render path against catalog scans | Accepted |

Add a row here when a record is added, and update the Status column when a
later record supersedes part of an earlier one.
