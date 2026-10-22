use super::*;

/// Renders the item tier for a multi-item entry: a persistent, selectable
/// item list and an item metadata area that updates in place on selection.
///
/// See `catalog-entry-detail-view`'s persistent-item-list and
/// update-in-place requirements.
#[allow(clippy::too_many_arguments)]
pub(super) fn render_item_tier(item: &LibraryItem, storage_root_path: &Path,
                               entity: Entity<LibraryController>,
                               inspector: &Entity<InspectorController>, colors: &ColorTokens,
                               window: &mut Window, cx: &mut App)
                               -> impl IntoElement + 'static {
    let entry_id = Arc::clone(&item.id);
    let selected_ix = entity.read(cx).selected_item_file(&entry_id);
    let entry_dir = resolved_item_dir(&StorageConfig::load(), item);

    let table = item_list_table(inspector, &entity, &entry_id, entry_dir, window, cx);
    let table_for_prepaint = table.clone();

    // `DataTable` virtualizes its rows and sizes itself to its container
    // (`.size_full()` internally) — unlike the old stateless `Table`, it
    // renders nothing if given an unbounded height, which is what the
    // surrounding naturally-scrolling detail-tab content provides. Give it
    // an explicit height sized to the row count (capped so a very large
    // bundle scrolls internally instead of pushing the rest of the tab off
    // screen).
    let row_height = Size::default().table_row_height();
    let visible_rows = item.files.len().min(8) as f32;
    let table_height = row_height * (visible_rows + 1.0);
    let item_list =
        div().h(table_height)
             .child(DataTable::new(&table).bordered(false)
                                          .scrollbar_visible(true, false))
             // Captures the table's rendered width so the Name column can
             // fill whatever space Type/Size/Status don't use — see
             // `ItemListDelegate::table_width`. Only notifies when the width
             // changed, so an unchanged width doesn't re-notify every frame.
             .on_prepaint(move |bounds, _window, cx| {
                 table_for_prepaint.update(cx, |state, cx| {
                                       if state.delegate().table_width != Some(bounds.size.width) {
                                           state.delegate_mut().table_width =
                                               Some(bounds.size.width);
                                           cx.notify();
                                       }
                                   });
             });

    let selected_file = selected_ix.and_then(|ix| item.files.get(ix).map(|file| (ix, file)));

    div().flex()
         .flex_col()
         .gap(px(12.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(colors.text_primary)
                     .child(t!("detail.items_heading").to_string()))
         .child(div().text_sm()
                     .text_color(colors.text_tertiary)
                     .child(t!("detail.item_prompt_select").to_string())
                     .into_any_element())
         .child(item_list)
         .child(match selected_file {
                    Some((ix, file)) => render_item_metadata(item,
                                                             file,
                                                             ix,
                                                             storage_root_path,
                                                             colors,
                                                             cx).into_any_element(),
                    None => div().into_any_element(),
                })
}

/// Renders the details of a single selected item within a multi-item entry,
/// laid out like the Info tab's Details: name, format, file size, file id and
/// download location, as label and value rows under a heading.
///
/// The download location is the entry's shared item folder: per-file paths are
/// not tracked separately (see `LibraryItemFile`), so every file in a bundle
/// reports the same one.
///
/// `row_ix` is the file's position within the entry's `files` list — used to
/// key element ids, since the API has been observed to
/// reuse the same download id across genuinely distinct files within a
/// bundle (see `LibraryItem::dedupe_files`).
pub(super) fn render_item_metadata(item: &LibraryItem, file: &LibraryItemFile, row_ix: usize,
                                   storage_root_path: &Path, colors: &ColorTokens, cx: &App)
                                   -> impl IntoElement + 'static {
    let label_font_family = cx.global::<LibriTheme>().fonts.label_font.clone();
    let value_font_family = cx.global::<LibriTheme>().fonts.value_font.clone();
    let name_value = copyable_value(SharedString::from(format!("file-name-{row_ix}")),
                                    file.name.to_string(),
                                    &value_font_family);

    let file_size_value = {
        let entry_dir =
            dtrpg_core::data::storage::publisher_dir(storage_root_path, &item.publisher);
        let on_disk = crate::util::file_size::on_disk_file_size(&entry_dir, file.name.as_ref());
        crate::util::file_size::prefer_on_disk_size(format!("{:.1} {}",
                                                            file.size_mb,
                                                            t!("size.mb")),
                                                    on_disk)
    };

    let path_value = if file.downloaded {
        dtrpg_core::data::storage::publisher_dir(storage_root_path, &item.publisher).display()
                                                                                    .to_string()
    }
    else {
        value_or_dash("")
    };
    let rows = vec![(t!("detail.item_list_column_name").to_string(), name_value),
                    (t!("detail.field_format").to_string(),
                     styled_value(file.format.to_string(), &value_font_family)),
                    (t!("detail.field_file_size").to_string(),
                     styled_value(file_size_value, &value_font_family)),
                    (t!("detail.field_file_id").to_string(),
                     copyable_value(SharedString::from(format!("file-id-{row_ix}")),
                                    file.id.to_string(),
                                    &value_font_family)),
                    (t!("detail.field_download_location").to_string(),
                     copyable_value(SharedString::from(format!("file-path-{row_ix}")),
                                    path_value,
                                    &value_font_family))];
    render_section(t!("detail.file_details_heading").to_string(),
                   render_detail_rows(rows, colors, &label_font_family),
                   colors,
                   &label_font_family)
}
