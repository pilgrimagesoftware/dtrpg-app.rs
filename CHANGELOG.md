
## 1.1.0 - 2026-10-27

### Added
- Use NumberInput for the text scale and handle step events (#503)
- Run the on-match actions of smart collections (#387)
- Guard the render path against catalog scans
- Settle catalog search after a burst of keystrokes
- Finish the Agent & MCP page's remaining rows
- Update appearance page to match mockups
- Remove the dot from grid view thumbnails
- Add harness selector to the agent connect block (#522)
- Hint how to commit a file opener and let one change its application
- File reports through gh, attach the log and name screenshots


### Changed
- Change name of MCP crate from `dtrpg-mcp` to `libri-mcp`
- Split the smart collection editor's rule widgets into their own file


### Documentation
- Record remaining UI v2 items in NOTES.md


### Fixed
- Include dtrpg-mcp binary in release packages
- Keep System when a detail fetch cannot resolve it
- Add the tag when Enter is pressed in the manage-tags dialog
- Propagate filter toggles to controller and make filter column resizable
- Anchor status bar to bottom and span full width (issue #470)
- Keep add-file-opener button enabled and show full app path (#475)
- Reveal downloaded item dir in file manager (#481)
- Prevent filter-values scroll from passing through (#487)
- Limit window text selection to presses on selectable text
- Cap the text scale field at the settings field width
- Build the mirror tests on Windows
- Drop the PathBuf import only unix uses
- Stop materializing the catalog on every keystroke
- Allow adding multiple file openers and show full app paths
- Remove unused imports and dead code
- Keep inspector hidden when resizing sidebar; move library index to app data dir
- Use the correct nightly version, use 'make fmt-check'
- Add app dir path
- Change 'catalog' to 'library' and 'cache' to 'local_db'
- Update cache filenames
- Constant names that use the 'catalog' terminology
- Update more file and struct names
- Update paths, etc.
- Inception
- More paths and names
- Compilation errors
- Example code
- Clippy error
- Open the smart collection editor from the Catalog menu
- Keep the collections on show while signing in at startup
- Count the pages of downloaded files for the footer total and Pages column
- Open Report a Feature/Bug and About when the Settings window has focus
- Rebuild the menus and redraw Settings when the language changes
- Refresh a location's volume space after changing the default folder
- Show filter terms as chips inside the search box
- Make a search chip only when its term is committed
- Rearrange the files
- Mod and use references


### Performance
- Count smart collection matches once typing pauses
- Count smart collection matches off the UI thread
- Redraw only the filters pane while its divider is dragged



## 1.0.0 - 2026-10-22

### Added
- Renew sessions with the SDK's refresh_session
- Show token times in Settings and drop its About page
- Show in-progress downloads in the catalog
- Add native integration boundary
- Add SQLite+FTS5 library index, rewrite catalog/collections cache
- Extend LibraryApi with index-backed read methods
- Expose native file availability state
- Track offloaded file state
- Detect offloaded files during presence checks
- Show offloaded download state
- Wire multi-root placement into app
- Finish placement rule integration
- Add publisher rule creation controls
- Add typed placement rule controls
- Add root health and placement-aware space checks
- Add generic archive abstraction
- Render archive entries as a tree
- Expose archive operations through LibraryApi
- Add shared snapshot and tab state
- Add persistent detail tabs
- Add lazy archive contents tab
- Add selective archive extraction
- Add inline archive entry preview
- Add contents tree filtering and collapse state
- Add list_archive_entries tool
- Add list_archive_entries tool
- Track selected archive for multi-archive items
- Add a work queue for background jobs
- Load built-in themes from TOML data files
- Fix semantic state colors outside the themeable palette
- Report download progress
- Show a progress bar for running downloads
- Discover user themes from the themes directory
- Hot-reload user theme files while the app runs
- Materialize an offloaded file before opening it
- Add theme gallery to Settings > Appearance
- Store item tags in the library index
- Read and write a file's tags as extended attributes
- Reconcile a file's tags with the index
- Reconcile files' tags with the index on launch
- Add tag colors and the record of what a file's tags agreed on
- Read tags back from files and give tags colors
- Edit tags and filter the catalog by tag in the library controller
- Tag editor, tag sidebar section, color menu and Native Integration settings
- Search by tag with tag: and -tag:
- Tools to read and change tags
- Group the catalog by tag
- Remove an item's link from a collection when it leaves the collection
- Choose which list columns are shown, and add a Tags column
- Add content summary providers and storage
- Generate content summaries headlessly and expose them over MCP
- Run the catalog search box as an index SQL query
- Compute sidebar publisher and section counts with index SQL
- Sql-ordered catalog grouping with controller-side cache
- Inspector shows stored summaries, verify on disk, and materialize
- Add dump_filters example to inspect product filter categories
- Draw a menu bar and window buttons on Windows and Linux
- Open an item's inspector in its own window
- Clearer empty state and placeholder loading screens
- Add a download sheet with file and destination choice
- Follow the system setting for reopening windows at launch
- Search by publisher, system, type and availability
- Dock the inspector beside the catalog
- Add a Cover tab and a full-size cover viewer
- Show download status first in the list, with tooltips
- Size the list's columns to their widest item
- Count facets and treat same-field search terms as alternatives
- Add the filters column and filter chips
- Show a count bar under each filter value
- Add the smart collection rules, matching and store
- List smart collections in the sidebar and show their members
- Read system, type, format and genre from DriveThruRPG's product filters
- Choose which filter values the filters column lists
- Add the smart collection editor
- Show location badges, free space, title counts and a default location
- Larger remembered window, OS integration under Advanced, calmer filter chips
- Add Needs Attention, a collection cloud marker and the Insights placeholder
- Add the Insights page, library aggregates and CSV export
- Add agent scopes, a per-tool switch, an audit log and the Agent & MCP page
- Show a usage meter for each storage location in the status bar
- Rebuild the Storage page to match the design
- Edit the active theme file in Appearance
- Show the storage location in the Details table
- Record which location each downloaded file is in
- Shift-click the Download button to skip the sheet
- Add the location field and the Location facet
- Edit the summary settings, and summarize on download
- List the matching titles in the smart collection editor
- Offer create_collection, add_to_collection and download_item
- Choose the density on the Appearance page and keep it
- Make a smart collection from a manual one
- Add the collection: field, and use it to make a smart collection from a manual one
- Add and edit placement rules in one inline row
- Lay out Appearance, Account and File Openers like the other pages
- Show the cache details on the Insights page
- Save the Anthropic API key from the Summaries card
- Offer rule names in a scrolling popup


### Changed
- Expose platform boundary from library
- Invert the crate graph behind a LibraryApi facade **[BREAKING]**
- Route UI integration through core
- Centralize settings capability checks
- Centralize title bar platform check
- Split multi-root storage modules
- Defer inspector UI integration
- Consume shared snapshot data
- Remove legacy archive popover
- Share one Tokio runtime across services
- Add cover fetching service and queue result slots
- Run thumbnail fetches on the work queue
- Run file downloads on the work queue
- Run availability checks on the work queue
- Put avatar fetching behind a transport
- Replace closed ThemeKey enum with string ThemeId + catalog
- Move the download tests to their own file
- Split oversized Rust modules
- Split storage settings views
- Split remaining medium-sized modules
- Split additional oversized modules
- Keep stub fixtures out of normal builds
- Split oversized views/controllers and gate file size in CI
- Read the catalog from the index instead of an in-memory Vec
- Extract headless Library session for ADR 0002 Slice 1
- Move catalog-sync state machine into dtrpg_core::Library
- Move filtered/grouped query cache into dtrpg_core::Library
- Move thumbnail/download/check job orchestration into dtrpg_core::Library
- Move catalog mutations into dtrpg_core::Library
- Move the verifying-downloads set into dtrpg_core::Library
- Move collections list and tag-read guard into dtrpg_core::Library
- Keep index/mod.rs under the 500-line limit


### Documentation
- Record libri-v2-architecture decisions 0002-0006
- Status notes for ADR 0002 implementation
- Status notes for ADR 0003 implementation
- Accept crate inversion decision
- Accept platform integration decision
- Propose core work queue design
- Resolve open questions for core work queue
- Hand off the tags work and the follow-up fixes
- Record content-summaries status and the screenshot note in handoff notes
- Add next-step pointer to catalog-reads-from-index handoff notes
- Record screenshot audit results in NOTES.md
- Mark merged PRs and refresh open threads in NOTES.md
- Mark PR opened in group-by-sql tasks
- Mark #362 merged and archive group-by-sql openspec
- Openspec for ADR 0002 controller decomposition
- Record ADR 0002 Slice 1 merge and remaining slices
- Record ADR 0002 Slice 2 merge and remaining slices


### Fixed
- Refresh expired access token automatically (#280)
- Remove the redundant catalog 3-dot menu and Show in Finder row
- Don't let a disk check revert downloads that finish during it
- Persist offloaded file state
- Gate symlink test import behind cfg(unix)
- Streamline placement rule input
- Restore core files after merge
- Report real cloud placeholder state and fix Windows reveal
- Keep run_command out of the Windows build
- Show a plus on the add storage root button
- Collapse a nested if in the theme watcher
- Collapse a nested if in the theme watcher
- Collapse a nested if in the theme watcher
- Let the settings window be resized
- Load tags and read Finder's as soon as the saved catalog is in memory
- Link a downloaded item into a collection it is added to later
- Build the collection link tests on Windows
- Build the collection link tests on Windows
- Keep the real network out of the controller tests
- Launch the signed-out tags test without the live fetch
- Build the collection link tests on Windows
- Do not read a tag's color from a file
- Build the collection link tests on Windows
- Keep the View menu enabled and stop copying the library on every draw
- Add tooltips to the icon-only controls that lacked them
- Inspector layout and behavior fixes
- Tighten the spacing in the sidebar lists
- Drop the redundant status check from the inspector header
- Show progress and time out in the dump_filters example
- Show the loading state when a collection is opened before its items arrive
- Create the dump_filters timeout inside the runtime
- Keep the stored library when signing in and clear it on sign-out
- Show the stored catalog at startup and write text placeholders correctly
- Give the catalog view a column to fill so the content area shows
- Lay the grid out at the width the catalog area really has
- Stop scanning the catalog every frame and queue the stored covers
- Do not report a library of one when the total is not given
- Save sizes after a drag stops, ring the open title, put Insights under Analysis, draw a treemap
- Address settings window feedback
- Put the agent dot first and let Insights charts flow
- Make Contents work, add an Other details tab, fix grid spacing
- Pad the Files tab
- Open the download sheet from the catalog menus too
- Keep the download sheet's rows inside the sheet
- Show a rule the download sheet remembered
- Remember that Publishers and Tags were collapsed
- Say which archive Contents shows, and make it scroll smoothly
- Show the hide/show caret on every collapsible section
- A saved Anthropic key wins over ANTHROPIC_API_KEY
- Match the mockups' lower-left corner and downloading item



## 0.4.1 - 2026-10-17

### Fixed
- Enable Secret Service keyring backend



## 0.4.0 - 2026-10-16

### Added
- Open About as a standalone window instead of a modal dialog



## 0.3.0 - 2026-10-15

### Added
- Add bulk selection mode and bulk actions
- Migrate from gpui-component to gpui-kit (#233)
- Publish a versioned documentation site via GitHub Pages
- Make the bug/feature report dialog its own window
- Download all from sidebar smart filters and publishers


### Documentation
- Add ADR 0001 and OpenSpec proposal for gpui-kit 0.7 migration (#233)
- Check off task 0.2 (gpui-kit dependency added)
- Check off task 0.2 (gpui-kit dependency added)
- Amend 0001 for gpui-kit fork incompatibility, collapse plan to single rewrite
- Backfill documentation site with guides for core features


### Fixed
- Show bulk-action bar as soon as selection mode starts
- Confirm large bulk actions before dispatching
- Stop avatar photo squishing under gpui-kit's Img layout
- Force aspect_square() on avatar images to stop squish
- Map accent/list/group-box/switch/slider/link tokens
- Wrap title-bar avatar image in its own sized/clipped div
- Apply rounded_full() to the avatar img, not its wrapper div
- Pin title-bar avatar to flex_none, not flex-basis sizing
- Restore gpui-kit Avatar component for account avatars
- Cancel Button's internal padding on the title-bar avatar
- Pin Ruby version for the Pages build
- Handle fallible ZipFile::name() in zip 9.0
- Make activity panel item list scroll



## 0.2.4 - 2026-10-13

### Changed
- Split the library controller into concern modules


### Fixed
- Remove stale cover files when the cached format changes
- Do not start a retry or download after cancellation
- Present native file pickers asynchronously


### Performance
- Compute settings cache counts off the render path



## 0.2.3 - 2026-10-11

### Fixed
- Make Zip content preview popover list scroll for large archives
- Defer cache reload after settings change



## 0.2.2 - 2026-10-09

### Fixed
- Stop verifying a macOS .app bundle cargo-packager already deleted
- Stop duplicating catalog items on every sync tick



## 0.2.1 - 2026-10-05

### Fixed
- Use the ClientOptions builder, not a non-exhaustive struct literal
- Stop git-cliff from skipping fix:/feat: commits scoped "(release)"



## 0.2.0 - 2026-10-04

### Added
- Add Report a Feature/Bug menu item and dialog (#243)



## 0.1.3 - 2026-10-04

### Fixed
- Stop catalog title duplication and toolbar title clipping (#239)


# Changelog

All notable changes to this project are documented in this file.

prepare-release.yml prepends each release's entry above this line via
`git-cliff`, using `cliff.toml`'s template.
