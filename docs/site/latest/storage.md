---
title: Storage roots
version: latest
slug: storage
---

# Storage roots

Libri keeps the original download location as the **Default** storage root.
Additional roots can be added from **Settings → Downloads**. A root must
already exist and be writable; Libri does not recreate a missing external
volume.

## The Storage page

**Settings → Storage** lists each location with:

- an **EXTERNAL** badge for a drive that can be unplugged or reached over a
  network, or a **CLOUD** badge for a folder that a cloud service (iCloud
  Drive, OneDrive, Dropbox, Google Drive) keeps in step. Libri recognises these
  from the folder's path, so a badge is a good guess, not a guarantee;
- how much space is free on its volume, such as `582.0 MB free of 994.0 MB`; and
- how many downloaded titles it holds, such as `6 titles here`.

**Default location** picks where downloads go when no rule matches. Rules read
in plain words, such as `Paizo → Archive Vault`.

## Download placement

Downloads continue to use the default root unless a saved placement rule
matches. The core storage configuration supports rules for:

- publisher name;
- collection membership; and
- minimum file size in megabytes.

Rules are evaluated in order. The first matching rule selects its named root.
If the target root is unavailable or no rule matches, Libri falls back to the
Default root.

Adding or removing a root does not move existing files. To relocate existing
content, move it manually while Libri is closed, then verify the root and file
presence from the library view.
