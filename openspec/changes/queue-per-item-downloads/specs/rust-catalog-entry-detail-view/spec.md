## MODIFIED Requirements

### Requirement: Rust expanded detail tab MUST render a persistent item list for multi-item entries

`render_detail_tab_content` MUST render a persistent, scrollable item list using
`gpui-component`'s `DataTable` or `Table` primitives (per this repo's UI policy) when the entry's
`files` array contains more than one entry. Each row MUST show item name, item type, and a
download action/status affordance for that specific item.

#### Scenario: Rendering the item list

- **WHEN** the expanded detail tab is shown for a multi-item entry
- **THEN** the tab renders a `DataTable`/`Table`-based item list showing all items, each with
  name, type, and a per-item download action/status affordance

#### Scenario: Single-item entry renders no item list

- **WHEN** the expanded detail tab is shown for a single-item entry
- **THEN** no item list is rendered; item metadata is shown inline in the entry tier

## ADDED Requirements

### Requirement: Rust item list rows MUST support downloading a single item independently

Each row in the multi-item entry's item list MUST expose a download action for that specific
file, independent of the entry-level download action and of every other row's download state.

#### Scenario: Downloading a single item from the list

- **WHEN** the user triggers the download action on one row in the item list
- **THEN** only that file is enqueued for download; sibling rows' download state is unaffected

#### Scenario: Item already downloaded

- **WHEN** a row's file is already downloaded
- **THEN** the row's affordance reflects the downloaded state (e.g. a checkmark) instead of a
  download action

#### Scenario: Item queued or downloading

- **WHEN** a row's file is queued or actively downloading
- **THEN** the row reflects that in-progress state; cancelling that download is done from the
  activity panel rather than a duplicate cancel control on the row

### Requirement: Rust entry-level download status MUST reflect aggregate per-item state

`catalog_view.rs` and `detail_panel_view.rs` MUST derive a multi-item entry's entry-level
download button/status affordance from its files' individual downloaded state rather than a
single independently-set flag: the entry MUST show as fully downloaded only when every one of
its files is downloaded.

#### Scenario: No items downloaded

- **WHEN** none of a multi-item entry's files are downloaded
- **THEN** the entry-level status/button reflects the not-downloaded (Cloud) state

#### Scenario: Some but not all items downloaded

- **WHEN** at least one but not all of a multi-item entry's files are downloaded
- **THEN** the entry-level status/button does not report the entry as fully Downloaded

#### Scenario: All items downloaded

- **WHEN** every file in a multi-item entry is downloaded
- **THEN** the entry-level status/button reflects the Downloaded state, matching existing
  single-item behavior
