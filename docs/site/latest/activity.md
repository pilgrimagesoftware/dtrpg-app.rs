---
title: Activity and alerts
version: latest
slug: activity
---

# Activity and alerts

Libri surfaces background operations - catalog syncs, thumbnail loads,
downloads - through two related but distinct panels, both reachable from
the status bar at the bottom of the main window.

## Activity panel

An overlay listing in-progress and recently-completed background
operations. Entries here expire on a timer once an operation finishes, so
the panel stays focused on what's happening now or just happened.

## Alert history

A separate overlay reading a durable, capped log of past error activity -
unlike the activity panel's expiring list, entries here stick around so you
can review a failure from earlier in the session after the activity panel
has already cleared it. Each entry supports copying its error message.

## Status bar indicators

The status bar runs along the bottom of the main window. On the left it says how
many titles are on show of how many there are ("18 of 18 titles") and their total
size. On the right are the **Agent** indicator, the activity button, the language,
the theme picker and the notification indicator (which opens alert history).

Under the sidebar, level with the status bar, a footer gives the library's totals in
two lines: titles and size, then pages and how many storage locations they are
across. It is as wide as the sidebar, and follows it when you drag it.

The activity button is a download arrow. While titles are downloading it also says
how many ("2 downloading"). Click it to open the activity panel, which lists
everything in progress, not only downloads. The language shows as a short code
such as EN.

Beside the totals, each storage location, marked with a drive icon, has a small meter of how much of its
volume Libri's files fill; hover it for the figures (used by Libri, and free). The
meters are measured in the background at startup, when the locations change, and
when downloads finish, so they can lag a little behind the disk. The **Agent**
indicator, a dot then an icon then the word, shows a dot that is green when agents may use Libri and grey when they may not; see
[Using Libri from an AI agent]({{ '/latest/mcp/' | relative_url }}).
