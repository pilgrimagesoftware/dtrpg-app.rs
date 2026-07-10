## 1. Shared image-format sniffing helper

- [x] 1.1 Create `crates/dtrpg-ui/src/util/image_format.rs` with a UI-independent `ImageKind` enum (`Jpeg`, `Png`, `Webp`, `Gif`, `Bmp`), a `sniff(bytes: &[u8]) -> ImageKind` function (byte patterns ported from `ui::library::cover::sniff_image_format`, default `Jpeg` preserved), and an `ImageKind::extension(&self) -> &'static str` method returning `"jpg"`/`"png"`/`"webp"`/`"gif"`/`"bmp"`
- [x] 1.2 Register the new module in `crates/dtrpg-ui/src/util/mod.rs`
- [x] 1.3 Add unit tests for `sniff` covering each of the five known magic-byte patterns plus the unknown/default-to-Jpeg case

## 2. Wire the UI-layer decoder to the shared helper

- [x] 2.1 In `crates/dtrpg-ui/src/ui/library/cover.rs`, remove the private `sniff_image_format` function
- [x] 2.2 Replace its call site in `CoverCache::insert` with `util::image_format::sniff(&bytes)` mapped to `gpui::ImageFormat` via a small local `match`
- [x] 2.3 Confirm existing cover-rendering tests still pass unchanged (no behavior change expected)

## 3. Extension-aware disk cache

- [x] 3.1 In `crates/dtrpg-ui/src/data/cover_cache.rs`, define the ordered list of known extensions (`["jpg", "png", "webp", "gif", "bmp"]`) as a constant
- [x] 3.2 Update `save_cached_cover` to sniff the format via `util::image_format::sniff(bytes)` and write to `{hash}.{ext}` instead of `{hash}.cover`
- [x] 3.3 Update `load_cached_cover` to check `{hash}.{ext}` for each known extension in order, returning the first hit (no directory scan)
- [x] 3.4 Update/extend existing tests in `cover_cache.rs` (round-trip, distinct-ids, empty-file) to assert the correct extension is used for each format, and add a case confirming a lookup succeeds regardless of which extension the file was written with
- [x] 3.5 Update the module doc comment to describe the extension-aware filename scheme (no longer "format is not encoded in the filename")

## 4. Build and quality gates

- [x] 4.1 `cargo build --workspace --all-features`
- [x] 4.2 `cargo test --workspace --all-features`
- [x] 4.3 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] 4.4 `cargo fmt --all -- --check`

## 5. Manual verification

- [x] 5.1 Delete the local covers cache directory, launch the app, let thumbnails load, and confirm `~/Library/Caches/com.pilgrimagesoftware.dtrpg/covers/` now contains files with real extensions (`.jpg`/`.png`/etc.) instead of `.cover`
- [x] 5.2 Relaunch and confirm those thumbnails load from disk (no network activity) and display correctly
