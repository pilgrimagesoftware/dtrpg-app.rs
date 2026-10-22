use super::*;

/// Renders a text value with an appear-on-hover copy button, in the app's
/// dedicated value font.
///
/// `field_id` must be unique within the surrounding view — it doubles as the
/// hover group name and the copy button's element id.
pub(super) fn copyable_value(field_id: SharedString, value: impl Into<SharedString>,
                             value_font_family: &str)
                             -> AnyElement {
    let value: SharedString = value.into();

    div()
        .id(SharedString::from(format!("{field_id}-row")))
        .group(field_id.clone())
        .flex()
        .items_center()
        .gap(px(6.0))
        .child(
            selectable_text(
                SharedString::from(format!("{field_id}-text")),
                value.clone(),
            )
            .font_family(value_font_family.to_string()),
        )
        .child(
            div()
                .invisible()
                .group_hover(field_id.clone(), |d| d.visible())
                .child(
                    Clipboard::new(SharedString::from(format!("{field_id}-copy")))
                        .value(value)
                        .tooltip(t!("detail.copy_tooltip").to_string()),
                ),
        )
        .into_any_element()
}

/// Renders a `DescriptionList` label in the app's dedicated label font and in
/// small caps, distinguishing field names from their values (rendered in the
/// dedicated value font — see [`styled_value`]).
pub(super) fn styled_label(label: impl Into<SharedString>, label_font_family: &str) -> AnyElement {
    div().font_family(label_font_family.to_string())
         .child(small_caps_text(label))
         .into_any_element()
}

/// Renders a `DescriptionList` value in the app's dedicated value font, for
/// plain text values that don't need [`copyable_value`]'s copy button.
pub(super) fn styled_value(value: impl Into<SharedString>, value_font_family: &str) -> AnyElement {
    div().font_family(value_font_family.to_string())
         .child(value.into())
         .into_any_element()
}

pub(super) fn platform_reveal_label() -> std::borrow::Cow<'static, str> {
    match Platform.kind() {
        PlatformKind::MacOs => t!("detail.show_in_finder"),
        PlatformKind::Windows => t!("detail.show_in_explorer"),
        PlatformKind::Linux => t!("detail.show_in_files"),
    }
}

/// Renders a metadata value, falling back to an em dash when the underlying
/// data is empty (e.g. the API did not report a game system/line for this
/// item).
pub(super) fn value_or_dash(value: &str) -> String {
    if value.trim().is_empty() {
        "\u{2014}".to_string()
    }
    else {
        value.to_string()
    }
}

/// Width of the label column in the details table, in pixels.
const DETAILS_LABEL_WIDTH: f32 = 110.0;

/// Renders the details table: plain labels on the left, values on the right,
/// in the order System, Type, Format, Pages, Size, Released, Added, Updated,
/// Location. Rows the catalog has nothing for are left out; `location` is the
/// name of the storage location holding a downloaded title.
pub(super) fn render_metadata_table(item: &LibraryItem, location: Option<String>,
                                    storage_root_path: &Path, colors: &ColorTokens,
                                    label_font_family: &str, value_font_family: &str)
                                    -> impl IntoElement + 'static + use<> {
    let file_size_label = if item.files.len() > 1 {
        t!("detail.field_total_file_size").to_string()
    }
    else {
        t!("detail.field_file_size").to_string()
    };

    // Three display modes depending on how many of the entry's files are
    // downloaded: none → catalog (cloud) size only; all → on-disk (local)
    // size only, since it's the exact figure once every file is present;
    // some → both, so the user can see progress against the total.
    let file_size_value = {
        let catalog_mb: f64 = if item.files.is_empty() {
            item.size_mb
        }
        else {
            item.files.iter().map(|f| f.size_mb).sum()
        };
        let catalog_str = format!("{:.0} {}", catalog_mb, t!("size.mb"));

        if item.files.is_empty() {
            catalog_str
        }
        else {
            let entry_dir =
                dtrpg_core::data::storage::publisher_dir(storage_root_path, &item.publisher);
            let downloaded_count = item.files.iter().filter(|f| f.downloaded).count();
            let resolved: Vec<u64> =
                item.files
                    .iter()
                    .filter(|f| f.downloaded)
                    .filter_map(|f| {
                        crate::util::file_size::on_disk_file_size(&entry_dir, f.name.as_ref())
                    })
                    .collect();

            if downloaded_count == 0 {
                catalog_str
            }
            else if downloaded_count == item.files.len() && resolved.len() == item.files.len() {
                crate::util::file_size::format_bytes(resolved.iter().sum())
            }
            else {
                let on_disk_bytes = if resolved.is_empty() {
                    None
                }
                else {
                    Some(resolved.iter().sum())
                };
                crate::util::file_size::with_on_disk_suffix(catalog_str, on_disk_bytes)
            }
        }
    };

    let text_value = |id: &'static str, value: String| {
        selectable_text(id, value).font_family(value_font_family.to_string())
                                  .into_any_element()
    };

    let mut rows: Vec<(String, AnyElement)> =
        vec![(t!("detail.field_system").to_string(),
              text_value("detail-field-system", value_or_dash(&item.line))),
             (t!("detail.field_category").to_string(),
              text_value("detail-field-kind", item.kind.to_string())),
             (t!("detail.field_format").to_string(),
              text_value("detail-field-format", item.format.to_string()))];

    // The DriveThruRPG order-product API does not always report a page count;
    // omit the row entirely rather than showing a misleading "0".
    if item.pages > 0 {
        rows.push((t!("detail.field_pages").to_string(),
                   text_value("detail-field-pages", item.pages.to_string())));
    }
    rows.push((file_size_label, text_value("detail-field-file-size", file_size_value)));
    rows.push((t!("detail.field_released").to_string(),
               text_value("detail-field-released", item.year.to_string())));
    if let Some(ts) = item.date_added {
        rows.push((t!("detail.field_added").to_string(),
                   render_relative_date_value(&item.id, "added", ts, value_font_family)));
    }
    if let Some(ts) = item.date_updated {
        rows.push((t!("detail.field_updated").to_string(),
                   render_relative_date_value(&item.id, "updated", ts, value_font_family)));
    }
    if let Some(location) = location {
        rows.push((t!("detail.field_location").to_string(),
                   text_value("detail-field-location", location)));
    }

    let label_color = colors.text_secondary;
    let value_color = colors.text_primary;
    div().flex()
         .flex_col()
         .gap(px(6.0))
         .children(rows.into_iter().map(|(label, value)| {
                                       div().flex()
                                            .items_baseline()
                                            .gap(px(12.0))
                                            .text_sm()
                                            .child(div().flex_none()
                                                        .w(px(DETAILS_LABEL_WIDTH))
                                                        .font_family(label_font_family.to_string())
                                                        .text_color(label_color)
                                                        .child(label))
                                            .child(div().flex_1()
                                                        .min_w_0()
                                                        .text_color(value_color)
                                                        .child(value))
                                   }))
}
