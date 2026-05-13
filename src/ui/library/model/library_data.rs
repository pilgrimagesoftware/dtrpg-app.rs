//! Data transformation helpers for library presentation.

use crate::services::LibraryItem;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibraryViewMode {
    FlatList,
    TreeByPublisher,
    TreeByProductType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FilterScope {
    ChildOnly,
    RootAndChild,
    RootOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortMethod {
    AtoZ,
    ZtoA,
    MostRecentlyAdded,
    MostRecentlyUpdated,
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub root_label: String,
    pub children: Vec<LibraryItem>,
}

pub fn next_sort(current: SortMethod) -> SortMethod {
    match current {
        SortMethod::AtoZ => SortMethod::ZtoA,
        SortMethod::ZtoA => SortMethod::MostRecentlyAdded,
        SortMethod::MostRecentlyAdded => SortMethod::MostRecentlyUpdated,
        SortMethod::MostRecentlyUpdated => SortMethod::AtoZ,
    }
}

pub fn sort_label(sort: SortMethod) -> &'static str {
    match sort {
        SortMethod::AtoZ => "A-Z",
        SortMethod::ZtoA => "Z-A",
        SortMethod::MostRecentlyAdded => "Most recently added",
        SortMethod::MostRecentlyUpdated => "Most recently updated",
    }
}

pub fn mode_label(mode: LibraryViewMode) -> &'static str {
    match mode {
        LibraryViewMode::FlatList => "Flat list",
        LibraryViewMode::TreeByPublisher => "Tree by publisher",
        LibraryViewMode::TreeByProductType => "Tree by product type",
    }
}



pub fn filter_presets() -> [&'static str; 4] {
    ["", "atlas", "sandbox", "tabletop"]
}

pub fn sorted_filtered_flat_items(
    items: &[LibraryItem],
    query: &str,
    sort: SortMethod,
) -> Vec<LibraryItem> {
    let mut result = items.to_vec();
    let query = query.to_lowercase();

    if !query.is_empty() {
        result.retain(|item| {
            item.title.to_lowercase().contains(&query)
                || item.publisher.to_lowercase().contains(&query)
                || item.product_type.to_lowercase().contains(&query)
        });
    }

    sort_items(&mut result, sort);
    result
}

pub fn grouped_items(
    items: &[LibraryItem],
    mode: LibraryViewMode,
    query: &str,
    scope: FilterScope,
    outer_sort: SortMethod,
    inner_sort: SortMethod,
) -> Vec<TreeNode> {
    let mut groups = std::collections::BTreeMap::<String, Vec<LibraryItem>>::new();

    for item in items.iter().cloned() {
        let key = match mode {
            LibraryViewMode::TreeByPublisher => item.publisher.clone(),
            LibraryViewMode::TreeByProductType => item.product_type.clone(),
            LibraryViewMode::FlatList => item.publisher.clone(),
        };
        groups.entry(key).or_default().push(item);
    }

    let query = query.to_lowercase();
    let mut nodes = Vec::new();

    for (root_label, mut children) in groups {
        let root_matches = query.is_empty() || root_label.to_lowercase().contains(&query);

        if !query.is_empty() {
            match scope {
                FilterScope::ChildOnly => {
                    children.retain(|item| item.title.to_lowercase().contains(&query));
                }
                FilterScope::RootAndChild => {
                    children.retain(|item| root_matches || item.title.to_lowercase().contains(&query));
                }
                FilterScope::RootOnly => {
                    if !root_matches {
                        children.clear();
                    }
                }
            }
        }

        if children.is_empty() {
            continue;
        }

        sort_items(&mut children, inner_sort);
        nodes.push(TreeNode {
            root_label,
            children,
        });
    }

    nodes.sort_by(|a, b| compare_roots(&a.root_label, &b.root_label, outer_sort));
    nodes
}

fn sort_items(items: &mut [LibraryItem], sort: SortMethod) {
    items.sort_by(|a, b| compare_items(a, b, sort));
}

fn compare_items(a: &LibraryItem, b: &LibraryItem, sort: SortMethod) -> std::cmp::Ordering {
    match sort {
        SortMethod::AtoZ => a.title.cmp(&b.title),
        SortMethod::ZtoA => b.title.cmp(&a.title),
        SortMethod::MostRecentlyAdded => b.added_order.cmp(&a.added_order),
        SortMethod::MostRecentlyUpdated => b.updated_order.cmp(&a.updated_order),
    }
}

fn compare_roots(a: &str, b: &str, sort: SortMethod) -> std::cmp::Ordering {
    match sort {
        SortMethod::AtoZ => a.cmp(b),
        SortMethod::ZtoA => b.cmp(a),
        SortMethod::MostRecentlyAdded => b.cmp(a),
        SortMethod::MostRecentlyUpdated => b.cmp(a),
    }
}
