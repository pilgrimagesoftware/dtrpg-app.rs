---
title: Settings
version: latest
slug: settings
---

# Settings

Settings opens as its own window, separate from the main library window, so
you can keep working in the catalog while it's open.

## Account

Sign in with your DriveThruRPG email and password, see who you're signed in
as, and log out. Signing in is what populates your catalog in the first
place.

While signed in, the page also shows when your current access token was
acquired and when it expires. The app renews an expired token on its own the
next time it talks to DriveThruRPG, so the expiry time can be in the past
without anything being wrong; the acquired time then moves forward after the
renewal. You never need to sign out just to refresh a token.

## Appearance

Pick one of six color themes, a comfortable or compact density, and a font
for each of four independent roles (body, value, label, monospace) from any
font installed on your system. A shared text-scale stepper zooms every
size-aware element in the window together. See
[Themes and appearance]({{ '/latest/themes/' | relative_url }}) for the full
theme list.

## Downloads

Where files save to, how many download (and thumbnail-load) operations run
at once, whether collections get mirrored into a `collections/` folder of
symlinks, and how many days an item stays in the sidebar's Recently Updated
section. See [Downloads]({{ '/latest/downloads/' | relative_url }}).

## File Openers

Override which application opens a given file extension, instead of
relying on your system's default. Add, edit, or remove per-extension
overrides here.

## Native Integration

How Libri works with your operating system. On macOS and Linux, three switches
control whether tags move between Libri and your files' tags, and whether that is
checked at launch. A list of platform abilities above them shows what your
system can do (cloud file state, on-demand download, file tags, showing a file in
the file manager); anything marked *Not on this platform* is unavailable here. See [Tags]({{ '/latest/tags/' | relative_url }}).

## Advanced

Cache details (size and location on disk) and maintenance actions:
opening the cache folder directly, or clearing it.

### Filter values

DriveThruRPG puts filters on every product, such as its format and genre, and some of the
values are useful to you while others are not. The **Filter values** section lists every
format, genre and product type your library has as a chip, so you choose which ones the
[filters column]({{ '/latest/catalog/' | relative_url }}#filtering-with-the-filters-column) lists.

- A lit chip is shown. Ghosted chips are hidden and listed after the lit ones. Each group is in
  alphabetical order.
- Click a chip to move it to the other group.
- A hidden value is only left out of the filters column. It still works if you type it into the
  search, such as `format:"creation method"`, and it still shows as a chip under the toolbar while
  it is applied.
- Libri starts with a short list of values it hides, such as `Human-Created Without AI` under
  Format, because they say little about a title. Show one by clicking it. Libri keeps only what you
  change, so a value you never touched follows Libri's own list, even when that list improves
  in a later version.

The section appears once your library has been loaded with filters.

The app name, version, and build details are in the About window (the app
menu's "About Libri"), not in Settings.
