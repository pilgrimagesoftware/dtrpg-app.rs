## Context

Count-noun strings throughout the UI are currently constructed with hard-coded plural forms via `format!()` calls in `toolbar_view.rs` and `sidebar_view.rs`. There is no central utility for count formatting, so every call site makes its own choice — and currently all choose the plural form unconditionally.

## Goals / Non-Goals

**Goals:**
- Introduce a single `pluralize` utility that all count-noun formatting routes through.
- Correct all existing call sites to use it.
- Design the utility signature so a future i18n layer can replace it with a single, well-scoped edit.

**Non-Goals:**
- Actual i18n / l10n integration (string tables, locale switching) — that is future work.
- Pluralization rules for languages other than English.
- Handling irregular plurals beyond what callers supply as arguments.

## Decisions

**Utility signature: `pluralize(count, singular, plural) -> String`**

The function takes the singular and plural forms as arguments rather than deriving the plural automatically (e.g., appending "s"). Auto-derivation fails for irregular nouns ("title" → "titles" is fine, but "library" → "libraries" is not). Accepting both forms keeps the function trivial and leaves irregular handling to the caller, which is the right place.

Alternative considered: a macro that infers the plural. Rejected — too much magic for a problem that is solved cleanly by passing two strings.

**Module location: `crates/dtrpg-ui/src/util/pluralize.rs`**

Matches the existing `util/` convention (see `util/matching.rs`, `util/sort.rs`). Exposed via `util/mod.rs`.

**No trait, no struct** — a free function is sufficient. The i18n replacement point is the function body, not a trait object, keeping the call sites unchanged when the implementation grows.

## Risks / Trade-offs

[New call sites added after this change] may forget to use `pluralize` → Mitigation: the function is the only place that formats a count with a noun, so code review catches this; it is also documented in its module.

[Future irregular plurals] may be awkward to pass at every call site → Mitigation: acceptable at English scale; if i18n grows to need message catalogs the function signature is replaced wholesale.

## Migration Plan

No migration needed — this is a pure UI text change with no data model or API surface involvement. No rollback strategy required beyond reverting the commit.
