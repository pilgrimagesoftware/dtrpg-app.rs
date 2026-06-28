## Context

`section_title_for(filter: &SidebarFilter) -> &str` in `toolbar_view.rs` returns a static string slice. The `Publisher(Arc<str>)` arm ignores the inner name and returns `"Publisher"`. The return type must become `String` to support a dynamic label.

## Goals / Non-Goals

**Goals:**
- Return `"Publisher: <name>"` when the publisher filter is active.
- Keep all other filter titles unchanged.

**Non-Goals:**
- Truncating long publisher names (not a concern; names come from the API and are typically short).
- Changing the sidebar label or any other rendering site.

## Decisions

### Change return type to `String`

`section_title_for` is a private helper called once at line 51 to produce a `String` local (`let title = section_title_for(filter).to_string()`). Changing the return type to `String` eliminates the `.to_string()` call at the use site and allows the `Publisher` arm to use `format!`. The non-publisher arms return `"…".to_string()` — minimal overhead for a function called once per render frame.

### Format: `"Publisher: <name>"`

The literal prefix `"Publisher"` is preserved so the title remains recognizable without context. A colon-space separator is the standard label pattern in prose and UI strings. No localization is in scope.
