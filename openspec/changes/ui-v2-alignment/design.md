# Design: gap analysis against the Libri v2 mockups

Status key: **Built** (exists in code), **Partial** (exists, differs from the
mockup), **Missing** (nothing found). The app was not run; see the proposal.

## Gap matrix

| Mockup | Status | What exists | What is missing or different |
| --- | --- | --- | --- |
| `library-list`, `library-covers-1/2` (library) | Partial | List, thumbs and grid presentations (`CatalogPresentation`); grouping by publisher or tag; sort menu; selection-mode toggle; search; sidebar with publishers, collections and tags | The **facet column** (Availability, System, Type, Location, Tag, each with counts and a bar) and the **Filters** button with an active-count badge. Search **tokens** shown as chips (`system:`, `tag:`). The sliders button between Sort and select. Sidebar entries **Needs Attention** and the **Analysis → Insights** section. Collection rows with a cloud marker and a count |
| Status bar (every screen) | Partial | Counts and size, theme tooltip, activity and notification indicators | Per-location **usage meters** (Local, Archive, iCloud), an **Agent** state indicator, an inline "N downloading" |
| `inspector-info`, `-files-1/2`, `-contents`, `-summary` (inspector) | Partial | Tabs Info (named Overview in code), Files, Collections, Contents and Summary; lazy archive contents; the tags block with its platform note; a Details table (System, Released, Format, Size, Type, Pages, Added, Updated); the Read and Download buttons | The Summary tab showed the description, not the stored summary (fixed in slice 4). **Materialize** and **Verify on disk** (slice 4). The Location row in Details (needs slice 1). A **docked right-hand pane**: the mockups show the inspector beside the list, while the app opens it as a full tab on double-click and a popover on single-click. That is a layout change and is not part of slice 4. A summary "Redo" action and themes chips (no themes data is stored) |
| `download-sheet` | Partial | A pending-download confirmation (free-space warning); storage placement rules | A real **download sheet**: a file checklist with All/None, a **destination picker** listing each storage root with its free space, and "Remember this choice for <publisher>" (which writes a placement rule) |
| `smart-collection` | Missing | Collections are manual DriveThruRPG product lists | **Smart collections**: a rule editor (all/any, field, operator, value), a live "Matches N of M titles" preview, an "equivalent query" line, and on-match actions (auto-download, mirror as symlinks) |
| `upgrade-a-collection` | Missing | | Seeding a smart collection from a manual one. **Design asset problem:** this screenshot shows the same "Edit smart collection" content as `smart-collection.png`, so it appears to be a mis-capture. Recapture before building |
| `insights-1/2` | Missing | Sidebar section counts and publisher counts in core | The whole **Insights** view: library at a glance, titles by publisher and by system, by type, availability, disk usage by publisher (treemap), titles added over time, storage roots, Recompute and Export CSV |
| `empty-state`, `loading-state` | Partial | An empty-status view and skeleton rows exist in `catalog_view` | Copy and recovery actions ("Clear a filter token, or save this search as a smart collection"). Unverified: skeleton layout against the mockup |
| `settings-appearance-1/2` | Built (verify) | Theme gallery with duplicate, import, reveal folder and reload; themes as data files (ADR 0005) | Unverified: the live theme-file (TOML) preview and the typography and density controls |
| `settings-storage` | Partial | Multiple roots and placement rules (publisher, collection, size); concurrency setting; collection symlink option | Per-location **capacity and title counts**, EXTERNAL and CLOUD badges, human-readable rules ("Paizo → Archive Vault, 1 title moved on next sync") instead of "root N", **Default location**, and the behaviour toggles (check free space, move files when a rule changes, keep a manifest in each location) |
| `settings-native` | Partial | Native integration page; tag projection and cloud-placeholder handling in core | The **capability matrix** by platform (macOS, Windows, Linux) and the cloud-placeholder toggles (treat as present, materialize before opening, warn before an agent materializes). Unverified: which toggles exist |
| `settings-agent-1/2` | Missing | `dtrpg-mcp` crate; summary settings in core | The whole **Agent & MCP** page: enable, transport, bind address, permission scopes (Read, Change, Download and spend), tools table, connect-an-agent config with Copy and Rotate token, recent activity log, and summaries (provider, model, monthly token ceiling, summarize on download) |
| `windows-linux-chrome` | Missing | A title bar with account menu | The Windows and Linux **chrome**: an in-window menu bar (File, Edit, View, Library, Window, Help) and right-hand caption buttons |

## Foundations that several screens share

These are the reason the slices are ordered the way they are. Building a screen
before its foundation means building it twice.

1. **Availability states.** The design shows On this device, Placeholder, Not
   downloaded, Downloading and Failed. `ItemStatus` has only `Downloaded` and
   `Cloud`. Placeholder exists per file (`offloaded`) and Downloading is
   transient (`ItemActivity`), but **Failed** is not persisted anywhere. The
   facet counts, the Insights availability chart and Needs Attention all need
   one defined set of states.
2. **Per-file location.** The Location facet, "N titles here" in Storage, and
   the Local/Archive/iCloud meters all need to know which storage root a file
   is on. Today location is derived from placement rules, not recorded.
3. **System and Type.** The model has `line` and `kind`. The mockups use
   "System" (D&D 5E, Pathfinder 2E) and "Type" (Core Rules, Adventure,
   Setting). A decision is needed on whether `line` is System and `kind` is
   Type, and whether the DriveThruRPG data gives clean values.
4. **A query language.** Search parses only `tag:` and `-tag:`
   (`dtrpg_core::util::search`). The mockups use `system:`, `type:` and
   negation in search chips, in the smart-collection "equivalent query", and in
   the MCP `search_library` tool. One parser should feed the facets, the chips,
   smart collections and MCP.
5. **Facet queries.** Counts per facet value for the current filter, computed in
   the index and exposed through `Library`, with no I/O on the render path.

## MCP surface against the design

The design lists 11 tools and three scopes. `dtrpg-mcp` has 9 tools and a single
"writes allowed" switch.

- Present: list items, get item, list tags, get tags, set tags, set tag color,
  get summary, generate summary, list archive entries.
- Missing: `search_library`, `list_collections`, `get_library_stats` (the same
  aggregates as Insights), `create_collection`, `add_to_collection`,
  `download_item`.
- Missing: the Read / Change / Download-and-spend scopes, the per-tool toggles,
  and the activity log.

## Proposed slices

Ordered by dependency. Each becomes its own issue and OpenSpec change.

1. **Domain foundations** (core): availability states including a persisted
   Failed state, per-file location, the System/Type decision, and a facet query
   API on `Library`. Unblocks slices 3, 6 and 7.
2. **Query language** (core): tokens for system, type, tag, publisher, location
   and availability, with negation, shared by search, facets, smart collections
   and MCP.
3. **Library chrome** (UI): the Filters button and facet column, token chips in
   the search field, the extra toolbar control, Needs Attention, collection row
   markers, and the status bar's per-root meters, Agent indicator and
   downloads.
4. **Inspector parity** (UI): Info, Files, Contents and Summary tabs, Read,
   Download and Materialize actions, Verify on disk, the Details table.
5. **Download sheet** (UI + core): file checklist, destination picker with free
   space, remember-for-publisher writing a placement rule.
6. **Smart collections** (core + UI): rule model and persistence, matching via
   slice 2, sidebar rendering, the editor sheet, on-match actions, and the
   upgrade-a-collection sheet once its mockup is recaptured.
7. **Insights** (core + UI): aggregate projections in the index, the view and its
   charts, CSV export; reused by MCP `get_library_stats`.
8. **Settings parity** (UI): Agent & MCP page (needs the MCP scopes and activity
   log from slice 9), Storage page parity, Native capability matrix, and a check
   of the Appearance page.
9. **MCP completion** (mcp + core): the missing tools, scopes, per-tool toggles
   and an audit log.
10. **Windows and Linux chrome** (UI): in-window menu bar and caption buttons.
11. **Empty and loading states** (UI): copy and actions, skeleton parity.

Slices 4, 10 and 11 do not depend on the foundations and can start at any time.

## Decisions

- **System** is the game system (D&D 5E, Pathfinder 2E). **Type** is the
  content type (Core Rules, Adventure, Setting). `format` is the file type
  (PDF, EPUB) and is not a facet in the mockups.
- What the code holds today: `LibraryItem.line` is **never populated**
  (`services/sdk/library/mapping.rs` hard-codes `""`), so System has no data
  yet. `LibraryItem.kind` is the product's first top-level DriveThruRPG filter
  category (`parent_filter_id == 0`), which fits Type. The SDK returns a list of
  filter categories per product; slice 1 starts with a spike to capture real
  responses and find which categories carry the game system and the content
  type. The only fixture in the repo is invented.
- **Needs Attention** means an item with a failed download, for now. It will be
  expanded later (for example unavailable items or missing files).

## Inspector pane (#402)

The mockups dock the inspector beside the list. The main window has no tabs any
more: the docked pane and the pop-out window replace the detail tab.

- **Single-click** selects an item, and the pane shows it. The old popover is
  gone.
- **Double-click** pops the item out into its own window. The pop-out window
  uses the same layout as the pane, without the pane's pop-out and close
  buttons.
- **Read** on an item with several files shows its Files tab, where the file is
  chosen.
- The pane opens and closes from **View > Show Inspector** (Alt+Cmd+I) and its
  close button. Its width is dragged at its left edge. Both are remembered.
- With nothing selected the pane asks for a selection.
- The tab strip, the Window menu's tab items and Cmd+0 to Cmd+9 are removed.
  The Collections tab folds into Info, which now lists the description, tags,
  collections, details, Opens With and the collapsed other details.
- Left for later: a toolbar button for the pane, choosing the opener from
  Opens With (it only shows the current one), the Location row (needs slice 1),
  and the Cover tab and full-size cover viewer. The last of these followed in
  the next change: the Cover tab, and a viewer window opened by double-clicking
  the thumbnail or the cover.

## Open questions

- The mockups are light-theme only. Is a dark pass of the screens needed before
  building, or is the theme-token system enough?
- Several mockups carry content the app may not have data for (pages, "Released"
  date). Confirm which fields the DriveThruRPG API provides. Slice 1's spike
  covers System and Type; the rest still needs checking.
- Issue types, project, milestone, size and effort could not be set through the
  integration used to file the issues, and still need setting.
