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

Pick a color theme and a comfortable or compact density. Fonts are part of a
theme, so they are changed by editing the theme, not on this page. A shared
text-scale stepper zooms every
size-aware element in the window together. See
[Themes and appearance]({{ '/latest/themes/' | relative_url }}) for the full
theme list.

## Storage

Locations (each with its free space and how many titles it holds), the
placement rules that choose between them, and behavior: checking free space
before a download, mirroring collections as folders of symlinks, and how many
download (and thumbnail-load) operations run at once. See
[Storage roots]({{ '/latest/storage/' | relative_url }}) and
[Downloads]({{ '/latest/downloads/' | relative_url }}).

## File Openers

Override which application opens a given file extension, instead of
relying on your system's default. Add, edit, or remove per-extension
overrides here.

## Agent & MCP

Whether an AI agent may use Libri, and what it may do: three scopes (read, change,
download and spend), a switch for each tool, the config to copy into the agent, and
the latest calls it made. It is off until you turn it on. See
[Using Libri from an AI agent]({{ '/latest/mcp/' | relative_url }}).

### Summaries

The last section of the page sets how summaries of your titles are written. They
are shown in the inspector's Summary tab and searched along with the rest of the
title's text.

- **Provider** is Anthropic (it needs your own API key, which is kept in the
  system keychain, never in a settings file) or Local, which picks sentences from
  the text itself, offline and for free, at a lower quality.
- **Model** is the model Anthropic writes new summaries with.
- **Monthly token ceiling** stops summarizing for the rest of the month once the
  summaries made this month have used that many tokens. The line under it says how
  many have been used. Summaries you already have are kept.
- **Summarize automatically on download** writes a summary of a title when its
  last file finishes downloading, if it has none yet. It is off until you turn it
  on, because every summary costs tokens (the Local provider costs none). A summary
  that cannot be made, for example when the key is missing or the ceiling has been
  reached, is skipped without a message.

## Advanced

Cache details (size and location on disk) and maintenance actions:
opening the cache folder directly, or clearing it.

### Operating system

Libri works with your operating system where it can. On macOS and Linux, three
switches control whether tags move between Libri and your files' tags, and
whether that is checked at launch. On Windows and Linux a switch reopens
windows (such as an item's own window) where you left them; macOS follows its
own setting. At the bottom, **Platform abilities** lists what your system can do
(cloud file state, on-demand download, file tags, showing a file in the file
manager). See [Tags]({{ '/latest/tags/' | relative_url }}).

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

The settings window remembers its size and position.
