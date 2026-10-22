---
title: Using Libri from an AI agent
version: latest
slug: mcp
---

# Using Libri from an AI agent

Libri has a small [Model Context Protocol](https://modelcontextprotocol.io/) (MCP)
server, the `dtrpg-mcp` program in this repository, that lets an AI agent look at your
library and, if you allow it, change it. It reads Libri's own copy of your library on
your computer. It does not talk to DriveThruRPG, it never holds your sign-in, and it
opens no network port: it talks to the agent over standard input and output only.

## Turning it on

What an agent may do is decided in **Settings > Agent & MCP**, not on the command
line.

- **Allow agents to use Libri** is off until you turn it on. While it is off, every
  call is refused.
- Once it is on, only the **Read the library** scope is on. **Change the library**
  and **Download and spend** stay off until you widen access.
- Each tool also has its own switch, for when you want a scope on but one tool off.

The server reads these settings on every call, so a change applies to the very next
call. A call that is refused says which switch is off.

Copy the config from the same page into your agent's MCP settings. It runs
`libri-mcp` (or `cargo run -p dtrpg-mcp` from a checkout).

## What it can read (Read scope)

- `list_items` lists the items in your library.
- `get_item` gets one item by its id.
- `search_library` searches the way Libri's search box does, with words and
  `field:value` terms. See [Searching by field]({{ '/latest/catalog/' | relative_url }}#searching-by-field).
- `list_tags` lists every tag with its color and how many items have it.
- `get_item_tags` lists the tags of one item.
- `get_summary` gets the saved summary of an item, or `null`.
- `list_collections` lists your collections and how many titles each lists, as of the
  last time Libri loaded them.
- `get_library_stats` gives the same figures as [Insights]({{ '/latest/insights/' | relative_url }}).
- `list_archive_entries` lists what is inside a downloaded archive file.

## What it can change (Change scope)

- `set_tags` gives an item exactly the tags you pass it, replacing the ones it had. A
  tag that already exists keeps its spelling and its color; a new tag has no color.
- `set_tag_color` sets a tag's color to gray, green, purple, blue, yellow, red or
  orange, or clears it.

A tag it cannot store, such as one containing a comma, or an item that is not in the
library, is reported as an error and changes nothing.

## What spends money (Download and spend scope)

- `summarize_item` writes and saves a summary of a downloaded item, using the provider
  chosen in Libri's settings. It spends tokens against the monthly ceiling set there.

## The activity log

Every call, allowed or refused, is written to a log: the time, the tool, a few words
about what it was for (an item id, a tag name or search text, never a credential),
its scope, and whether it was refused. **Settings > Agent & MCP > Recent activity**
shows the latest ten. The log keeps the newest 1,000 calls in `mcp_audit.jsonl` in
Libri's data folder.

## Not available yet

`create_collection`, `add_to_collection` and `download_item` are in the design but
are not offered. They need a signed-in session, and this server deliberately holds no
credentials. They will come when Libri can run them on the agent's behalf from the
app itself.

## When a change shows up

The server changes Libri's library, not your files and not the window that is open.
The new tags reach an item's files the next time Libri starts or you reload your
library, the same way a tag changed in Libri does. A Libri window that is already open
shows the new tags after it reloads them. See [Tags]({{ '/latest/tags/' | relative_url }}).

## Upgrading from `--allow-write`

The `--allow-write` flag is gone. If you used it, turn on **Change the library** (and
**Download and spend** for summaries) in the settings.
