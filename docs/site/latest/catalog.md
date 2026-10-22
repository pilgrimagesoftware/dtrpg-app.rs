---
title: Browsing your catalog
version: latest
slug: catalog
---

# Browsing your catalog

The main window's catalog tab lists everything in your DriveThruRPG library.
It's always open and can't be closed.

## Layouts

Switch between three presentations from the toolbar's layout switcher:

- **Grid** (default) - cover thumbnails in a card grid.
- **Thumbs** - a denser thumbnail row list.
- **List** - a text-only table, with resizable, sortable columns.

## Sorting and grouping

The toolbar's sort dropdown sorts by title (default), publisher, date added,
or page count. In List layout, clicking a column header sorts by that column
instead (shown as "custom" sort in the toolbar). The sort dropdown and the
View menu also group the catalog by publisher or by [tag]({{ '/latest/tags/' | relative_url }}#grouping-by-tag),
one at a time.

## Choosing the list columns

In List layout, right-click any column heading to show or hide columns. The choices are
Publisher, System, Pages, Size, Added and Tags; the title, the download status and the open
button are always there. Libri remembers which columns you chose.

Each column is as wide as its widest item, but never more than 60% of the table's width, so
one very long title cannot push the other columns out of view. The widths follow the items on
show, so they change as you search or filter. A width you drag a column to replaces its fitted
width for that column only, lasts until you quit, and survives showing and hiding other
columns. The Tags column lists an item's tags separated by commas.
It does not sort, because a list of tags has no order to sort by. Publisher, System,
Pages, Size and Added are shown until you choose otherwise.

## Navigation and filtering

The sidebar has three sections:

- **Smart filters** - All Titles, Recently Updated, On This Device, In the
  Cloud, and Needs Attention. Needs Attention lists the titles whose last
  download failed, and a title leaves it when a download of it succeeds. It
  starts empty each time Libri opens. Insights is listed after it but is greyed
  out until it is built.
- **Publishers** - every publisher represented in your library, each
  filtering the catalog to just their titles.
- **Collections** - your custom collections (see
  [Collections]({{ '/latest/collections/' | relative_url }})). A cloud
  marker beside a collection's count means some of its titles are only in the
  cloud; hover it to see how many.

The toolbar's search box filters the current section's titles as you type. It also finds items by tag and by
other fields: see [Searching by field](#searching-by-field) below and [Tags]({{ '/latest/tags/' | relative_url }}).

## Searching by field

Type a field, a colon and a value to keep only the items whose field has exactly that value. Capital letters
do not matter. Put a minus sign in front to keep only the items that do not have it, and put a value with
spaces in quotes.

| Field | Finds items whose | Example |
| --- | --- | --- |
| `tag` | tags include the value | `tag:solo` |
| `publisher` | publisher is the value | `publisher:"free league"` |
| `system` | game systems include the value | `system:"pathfinder ogl 1e"` |
| `type` | product types include the value | `type:"core rulebooks"` |
| `format` | formats include the value | `format:pdf` |
| `genre` | genres include the value | `genre:horror` |
| `availability` | download state is `downloaded` or `cloud` | `-availability:cloud` |

Combine terms with each other and with ordinary words: `type:adventure -tag:solo moria`. An item has to meet
every term, except that several terms for the same field (`type:adventure type:core`) are alternatives: an
item needs only one of them. Tag terms and terms with a minus sign must all hold. A word before a colon that is not one of these fields, such as `location:cloud`, is searched
for as ordinary text.

`system`, `type`, `format` and `genre` read the filters DriveThruRPG puts on each product (its
Rule System, Product Type, Format and Genre). A product can have several of each, and a nested
rule system counts at every level: a Pathfinder OGL 1E product matches `system:"pathfinder ogl 1e"`,
`system:"pathfinder ogl / orc"` and `system:"pathfinder / starfinder"`. A product DriveThruRPG has
not filed under a category matches no value in it, so the filters column and these terms are
emptier for libraries whose products carry few filters.

The first time you start a version that reads these filters, Libri fetches the library again to
get them, because titles stored earlier do not have them.

## When there is nothing to show

While the catalog loads for the first time, grey placeholders stand in for the
titles: rows in the thumbnail layout and cards in the grid. The list layout
draws its own.

If your search and section leave no titles, the catalog says "No titles match
these filters" and tells you what to change: clear the search (the x in the
search box), go back to all titles, or both. When a section other than All
Titles is chosen, a **Show all titles** button goes back to it in one click. An
empty library says so instead.

## Viewing an item

The inspector sits beside the catalog and shows the item you have selected. It
opens when you select an item and closes when nothing is selected.

- **Single-click** a catalog entry to select it and see it in the inspector.
- **Double-click** to open it in a window of its own (see
  [Opening an item in its own window](#opening-an-item-in-its-own-window)).
- The close button at the top right of the inspector clears the selection and
  closes it.
- Drag the inspector's left edge to make it wider or narrower. Libri remembers
  the width.

### The inspector

At the top are the cover, the title, the publisher and the item's game system
and type (for example "D&D 5E · Adventure"), when the catalog has them. Under
that are the buttons: **Read** and **Download…**, which show icons (hover for
their names), and a **⋮** menu with Show in Finder (Explorer, Files), Remove
download and Refresh thumbnail. Once an item is downloaded, its button shows a
check mark and only reports that; hold Shift while clicking it, or use the ⋮
menu, to remove the download. Its tabs are:

- **Info** - the description, the item's tags, the collections it belongs to,
  a table of its details (the Type row is the content category DriveThruRPG
  files it under), and which application opens the item's file type. That
  application is the system default unless you set one in Settings > File
  Openers, which the hint under it links to. A collapsed "Other details" section holds the ids.
- **Files** - each file with its size and download state, with a count beside
  the tab's name. For an item with several files this is the list you pick
  from. **Verify on disk** checks the files against the disk right away and
  corrects the catalog if a file was moved or deleted outside Libri.
- **Contents** - what is inside an archive file, loaded when you open the tab.
- **Summary** - the item's saved content summary, with the model that wrote it,
  when, and its length in tokens. Items without a summary say so. The tab only
  shows summaries; they are written elsewhere for now, for example by an agent
  through the MCP server.
- **Cover** - the cover as wide as the inspector. An item with no image shows
  its generated cover.

Double-click the small cover at the top of the inspector, or the cover on the
Cover tab, to open it in a window of its own at the largest size the window
allows. Resize the window to suit; press Escape to close it. The full-size image
is fetched the first time you ask for it, so it can take a moment to appear.

Reading an item that has several files shows its Files tab, so you can choose
which file to open.

On macOS and Windows, a file that the system has moved out to the cloud (it
shows a dashed cloud) gets a **Materialize** button next to Download. It asks
the system for the local copy and waits for it, then checks the file against
the disk. Opening the file with **Read** does the same automatically.

### Opening an item in its own window

Double-click an item, or press the arrow-out button at the top right of the
inspector (hover it for "Open in its own window"), to move the item into a
separate window, so you can keep it beside the catalog or beside another item.
The window has the same layout, details and actions as the inspector. Its
button at the top right puts the item back: the window closes and the item is
selected in the main window, which shows it in the inspector. If the inspector
was showing that item when you opened the window, it closes.

- Opening an item that already has a window brings that window to the front
  instead of opening a second one.
- Libri remembers these windows. Where you leave one, and which items have one,
  carry over to the next launch: quit with a window open and it reopens in the
  same place. Closing a window yourself means Libri forgets it.
- Whether the windows reopen at launch follows your system. On macOS it
  follows System Settings > Desktop & Dock > "Close windows when quitting an
  application": with that ticked, Libri starts without them. On Windows and
  Linux there is no such system setting, so Settings > Advanced > Operating system >
  Windows has **Reopen windows at launch** (on by default). When windows are
  not to reopen, Libri also forgets the remembered ones, so turning the
  setting back on later does not bring back old windows.
- If an item leaves your library, its window closes.
  Libri waits for the catalog to finish loading first, so a window is not
  closed just because its item has not arrived yet.

## Bulk selection

Enter bulk-selection mode from the toolbar to check multiple items at once.
A bulk-action bar appears below the toolbar once at least one item is
checked, including a "Select Matches" control that checks every item whose
title, publisher, or game line/series matches a text pattern you type.
