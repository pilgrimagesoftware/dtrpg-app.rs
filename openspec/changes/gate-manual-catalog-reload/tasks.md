## 1. Gate Manual Reload on the Freshness Policy

- [x] 1.1 In `dtrpg-ui/src/controllers/library.rs`'s `reload_catalog`, change the `self.start_load_inner(cx, true)` call to `self.start_load_inner(cx, false)` so the auto-load freshness/count-match policy applies.
- [x] 1.2 Update `reload_catalog`'s doc comment (currently "Forces a full live catalog fetch, bypassing the auto-load policy") to describe the new policy-gated behavior, keeping the existing note about `reload_cooldown_active`'s 60-second throttle.

## 2. Spec Sync

- [x] 2.1 Confirm the `catalog-menu` spec's Reload requirement (updated in this change) matches the shipped behavior.
- [x] 2.2 Confirm the `catalog-auto-load-policy` spec's new "User-initiated reload shares the freshness policy" requirement matches the shipped behavior.

## 3. Verification

- [x] 3.1 Run `cargo check -p dtrpg-ui` and confirm zero errors
- [x] 3.2 Run `cargo test -p dtrpg-ui` and confirm all existing tests pass
- [ ] 3.3 Run the app with a fresh, count-matching cache; select "Catalog > Reload"; confirm no full paginated fetch occurs (only a lightweight count check) and the catalog is unchanged
- [ ] 3.4 Force the cache stale (e.g. edit the cache metadata's saved timestamp, or wait past the freshness window) or clear it; select "Catalog > Reload"; confirm a full paginated fetch runs and the catalog refreshes
- [ ] 3.5 Select "Catalog > Reload" twice within 60 seconds; confirm the second invocation is a silent no-op regardless of cache freshness (double-click throttle still applies)
