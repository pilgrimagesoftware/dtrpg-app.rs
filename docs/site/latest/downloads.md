---
title: Downloads
version: latest
slug: downloads
---

# Downloads

Queue a catalog item's files for download from the inspector.

## Choosing files and a location

The **Download…** button in the inspector, and **Download** in the right-click
menu of a title in the catalog (list, grid and thumbnails), open the download
sheet. **Shift-click** the inspector's Download… button to skip the sheet and
download with its defaults: every file you do not have yet, to the location your
rules pick, without saving a rule. The button's tooltip says so. The cloud icon beside a single file in the Files tab still queues just
that file.
It lists the item's files with their formats and sizes; the files you do not
have yet start checked, and a file you already have shows a check mark and
cannot be selected again. **All** and **None** change the selection, and the
bottom of the sheet shows how many files and how much space you have chosen.

Below the files, the sheet lists your storage locations with their free
space. The one your placement rules would pick for this title is chosen to
begin with. If the chosen files are larger than what is free on the chosen
location, the sheet says so, but still lets you go ahead.

**Remember this choice for the publisher** saves a placement rule that sends
every title from that publisher to the chosen location. It is off for the
usual location. Choosing a different location turns it on and locks it,
because Libri finds your downloaded files through your placement rules, so a
location the rules don't know about would make the files look missing. The
new rule goes first in the list and replaces any earlier rule for the same
publisher; see [Storage]({{ '/latest/storage/' | relative_url }}).

Downloading from the catalog's context menu still queues
every missing file straight away, without the sheet.

## Following a download in the catalog

An item with a file queued or downloading shows a small spinner in the
catalog's status column (list view) or next to its publisher (grid and
thumbnail views), in place of the usual cloud or downloaded marker.
Hovering it shows "Downloading…". It switches to the downloaded marker when
the last file lands, or back to the cloud marker if you cancel. A muted
spinner means something different: Libri is checking the item's files
against what is on disk.

## Download progress

A running download appears in the activity panel with a progress bar that
fills as the file arrives. If the server doesn't say how large the file is,
the bar stays indeterminate instead of guessing. When a download retries
after a failure, its label says so, and the bar starts over for the new
attempt.

## Where files go

Downloads save to your default location, shown first under Locations in
Settings > Storage. Open its ⋮ menu to show it in the file manager or to change
its folder. Changing the folder doesn't move files you've already downloaded.

Settings > Storage > Behavior has a "Check free space before downloading"
switch, on by default. When it is on, Libri warns before a download that would
not fit in the free space of its location. It also has a "Mirror collections as
folders of symlinks" switch. When it's on,
downloading an item that belongs to one or more collections also creates a
symlink for it under a `collections/` subfolder, organized by collection,
so the same downloaded file is reachable there without duplicating the
data on disk. If you add an item you have already downloaded to another
collection, the symlink for that collection is created then. Removing an item
from a collection removes its symlink from that collection's folder. Only a
symlink that points at the item's downloaded file is removed: a file or a link
to somewhere else with the same name is left alone. A link you made yourself
that points at the same file looks just the same, and is removed too. The
collection's folder stays, even when it is empty. On Windows this needs
Developer Mode (or an elevated process) - if symlink creation fails, the toggle
has no effect rather than erroring.

## Concurrency

Settings > Storage > Behavior's "Max concurrent downloads" field is a shared limit
for both thumbnail loading and file downloads, not file downloads alone.
Libri dispatches queued files to fill as many of that many concurrency
slots as are free, rather than downloading everything at once or strictly
one at a time.

A download you start goes ahead of thumbnails still waiting to load, so a
large catalog loading its covers doesn't hold up a file you asked for.
Thumbnails already loading finish first; nothing is interrupted.

## Retries and disk space

A transient failure (a dropped connection, a 5xx response) retries with
backoff rather than failing the download outright. Libri also checks
available disk space before starting a download and warns if it's low,
rather than letting a transfer fail partway through from running out of
room.

## Checking availability

Catalog items occasionally get re-checked against the server to confirm
they're still available for download, on a cooldown so repeatedly viewing
the same item's details doesn't trigger a check every time.

Checks run one at a time, and they don't count against the concurrency
limit above, so a busy download queue never holds one up. A check you ask
for, by opening an item's details or choosing Check Item Availability,
goes ahead of the automatic background ones.
