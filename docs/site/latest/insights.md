---
title: Insights
version: latest
slug: insights
---

# Insights

**Insights** (in the sidebar, under the smart filters) shows figures about your
whole library. It is worked out from Libri's local index, so it works offline,
and it never changes your library.

## What it shows

- **Library at a glance** - titles (and how many were added this month), total
  size, pages, how many titles are downloaded, publishers and game systems.
- **Titles by publisher** and **Titles by system** - the top eight of each,
  with a bar for each. A title is counted under its most specific system, such
  as *Pathfinder OGL 1E* rather than *Pathfinder / Starfinder*.
- **By type** - a ring of the product types (core rules, adventures, settings).
- **Availability** - a ring of what is on this machine and what is not: on
  device, not downloaded, downloading now, and failed. A failed title is also
  listed under **Needs Attention**.
- **Disk usage by publisher** - each publisher's share of the total size.
- **Titles added over time** - a chart by the month you acquired them.
- **Storage locations** - for each location, what Libri uses there and how
  much space is free.

## Recompute and export

The figures are worked out when you first open Insights and kept until you
choose **Recompute**. The work runs in the background, so you can keep using the
catalog, and the old figures stay on screen until the new ones arrive.

**Export CSV** saves every figure in one file with three columns: `section`,
`label` and `value`. A spreadsheet can filter it by section (`glance`,
`publisher`, `system`, `type`, `disk_mb`, `added`, `root_used_mb`,
`root_free_mb`).

## Differences from the design

Disk usage is shown as bars rather than a treemap, and there is no "placeholder"
availability state yet.
