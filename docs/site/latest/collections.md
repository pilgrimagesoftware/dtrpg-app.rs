---
title: Collections
version: latest
slug: collections
---

# Collections

Collections are custom groups of catalog items you define yourself,
independent of publisher or any other built-in grouping.

## Managing an item's collections

Open the "Manage Collections" dialog from a catalog item (the inspector's
Info tab) to see and change which collections that item belongs to.
The dialog also lets you create a new collection inline, without leaving it
to set one up first.

## Browsing by collection

Every collection you create appears in the sidebar's Collections section,
alongside the built-in smart filters and publisher list. Selecting one
filters the catalog to just its members, the same way selecting a publisher
does.

## Deleting a collection

Deleting a collection removes the grouping, not the items in it - the
underlying catalog entries stay in your library and under any other
collection or publisher grouping they also belong to.

## Smart collections

A smart collection finds its own members. Instead of choosing titles by hand, you
write rules, and every title that meets them is in the collection. The members are
worked out again whenever the library or its tags change, so there is nothing to keep
up to date.

### Making one

- Choose **Catalog > New Smart Collection...**, or
- Filter the catalog with the filters column or the search box, then press
  **Save as smart collection** beside the filter chips. Each `field:value` term becomes
  a rule. Ordinary words in the search are not turned into a rule.
- Right-click a manual collection in the sidebar and choose **Make a smart
  collection...**. The editor opens with the collection's name filled in and one rule,
  **collection is** the manual collection's name, so the smart collection starts with
  the same titles. Add rules to narrow it, or remove that rule and use others. The
  smart collection follows the manual one: when you add a title to the manual
  collection, the smart collection gains it too. The manual collection is left as it is.

The editor has a name, a list of rules and a line that says how many titles match
right now, for example "Matches 17 of 18 titles right now". It changes as you type.
Under the count, the matching titles are listed alphabetically, up to 50 of them, in a
box that scrolls. When more match, the last line says how many are not shown.

Each rule has a field (tag, publisher, system, type, format, genre or availability), "is" or "is
not", and a value. Capital letters do not matter. A rule with no value is ignored while
you are still typing it. Choose whether a title has to meet **all** of the rules or
**any** of them.

A collection with no rule that has a value has no members, so a new collection does not
start out holding the whole library.

The line at the foot of the editor shows the **equivalent query**: the search that finds
the same titles. It is shown when the search box can say it. The search reads two
positive terms for the same field as alternatives (`type:adventure type:core` finds
either), so a collection that needs both, or that mixes fields under "any", has no
equivalent query.

The two **On match** switches do something with the titles a smart collection holds:

- **Download new matches automatically** queues the download of a title when it starts to
  match, for example after you tag it or a library update brings it in. Titles the
  collection already holds when you turn the switch on are not downloaded. Libri
  remembers what matched only while it runs, so a title that starts to match while Libri
  is closed is not downloaded.
- **Mirror to a folder of symlinks** keeps a folder named after the collection under
  `smart-collections` in your storage location. It holds a symlink to each downloaded
  file of each member, and is updated when the library, the tags, or the downloads change.
  Libri only removes symlinks from that folder, never other files. Renaming the collection,
  turning the switch off, or deleting the collection removes the old folder's links.

### Using them

Smart collections appear in their own **Smart collections** section of the sidebar, each
with a folder-with-gear icon and the number of titles in it. Select one to see its members.
Right-click one to edit or delete it. Deleting removes the collection only, not the
titles in it.

Unlike a manual collection, you cannot drop titles onto a smart collection, because the
rules decide what is in it.

Libri keeps smart collections in `smart_collections.toml` in its preferences folder.
