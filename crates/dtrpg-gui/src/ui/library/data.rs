//! Data model, filtering, sorting, and stub catalog for the Libri library view.

use std::sync::Arc;

// ── Enumerations ─────────────────────────────────────────────────────────────

/// Download state of a catalog item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemStatus {
    Downloaded,
    Cloud,
}

/// How the catalog is displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CatalogPresentation {
    List,
    Thumbs,
    #[default]
    Grid,
}

/// Active filter applied in the sidebar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidebarFilter {
    AllTitles,
    RecentlyAdded,
    OnDevice,
    InCloud,
    Publisher(Arc<str>),
}

impl Default for SidebarFilter {
    fn default() -> Self {
        Self::AllTitles
    }
}

/// Field used to sort the catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortMethod {
    #[default]
    Title,
    Publisher,
    DateAdded,
    PageCount,
}

// ── LibraryItem ───────────────────────────────────────────────────────────────

/// A single item in the RPG catalog.
#[derive(Debug, Clone)]
pub struct LibraryItem {
    /// Stable unique identifier (e.g. `"b1"`).
    pub id: Arc<str>,
    pub title: Arc<str>,
    pub publisher: Arc<str>,
    /// Game line / series name.
    pub line: Arc<str>,
    /// Category tag: Core, Supplement, Adventure, Bestiary, Setting …
    pub kind: Arc<str>,
    /// File format string, e.g. `"PDF"` or `"PDF + EPUB"`.
    pub format: Arc<str>,
    pub pages: u32,
    pub size_mb: f64,
    pub year: u32,
    /// Relative ordering — lower means more recently added.
    pub added_order: u32,
    pub status: ItemStatus,
    /// Hex color string for the generative cover background, e.g. `"#1C2A44"`.
    pub color: Arc<str>,
    pub desc: Arc<str>,
    /// Optional URL for a real cover thumbnail.
    pub cover_url: Option<Arc<str>>,
}

impl LibraryItem {
    fn new(
        id: &str,
        title: &str,
        publisher: &str,
        line: &str,
        kind: &str,
        format: &str,
        pages: u32,
        size_mb: f64,
        year: u32,
        added_order: u32,
        status: ItemStatus,
        color: &str,
        desc: &str,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            publisher: publisher.into(),
            line: line.into(),
            kind: kind.into(),
            format: format.into(),
            pages,
            size_mb,
            year,
            added_order,
            status,
            color: color.into(),
            desc: desc.into(),
            cover_url: None,
        }
    }
}

// ── Stub catalog ──────────────────────────────────────────────────────────────

/// Returns the full stub catalog matching the design prototype data.
#[must_use]
pub fn stub_catalog() -> Vec<LibraryItem> {
    use ItemStatus::{Cloud, Downloaded};
    vec![
        // Wizards of the Coast — D&D 5e
        LibraryItem::new("b1",  "Player's Handbook",            "Wizards of the Coast", "Dungeons & Dragons 5e", "Core",       "PDF",       320, 96.0,  2014, 412, Downloaded, "#1C2A44", "Core rules for players: species, classes, spells, equipment."),
        LibraryItem::new("b2",  "Dungeon Master's Guide",       "Wizards of the Coast", "Dungeons & Dragons 5e", "Core",       "PDF",       320, 88.0,  2014, 410, Downloaded, "#6B2230", "Tools and guidance for building and running campaigns."),
        LibraryItem::new("b3",  "Monster Manual",               "Wizards of the Coast", "Dungeons & Dragons 5e", "Bestiary",   "PDF",       352, 104.0, 2014, 408, Downloaded, "#20402F", "Hundreds of monsters with lore, stats, and tactics."),
        LibraryItem::new("b4",  "Curse of Strahd",              "Wizards of the Coast", "Dungeons & Dragons 5e", "Adventure",  "PDF",       256, 72.0,  2016, 220, Downloaded, "#34234A", "Gothic horror campaign in the cursed land of Barovia."),
        LibraryItem::new("b5",  "Xanathar's Guide to Everything","Wizards of the Coast", "Dungeons & Dragons 5e", "Supplement","PDF",       192, 58.0,  2017,  96, Cloud,      "#15403F", "New subclasses, spells, and rules for players and DMs."),
        LibraryItem::new("b6",  "Tasha's Cauldron of Everything","Wizards of the Coast", "Dungeons & Dragons 5e", "Supplement","PDF + EPUB",192, 61.0,  2020,  41, Cloud,      "#45264A", "Customizable origins, subclasses, feats, and group patrons."),
        // Paizo — Pathfinder 2e
        LibraryItem::new("b7",  "Pathfinder Player Core",       "Paizo",                "Pathfinder 2e",          "Core",       "PDF",       464, 132.0, 2023, 188, Downloaded, "#7E2230", "Everything to build and play a Pathfinder character."),
        LibraryItem::new("b8",  "Pathfinder GM Core",           "Paizo",                "Pathfinder 2e",          "Core",       "PDF",       336,  98.0, 2023, 180, Downloaded, "#2E3A45", "Rules and advice for running the game as GM."),
        LibraryItem::new("b9",  "Pathfinder Monster Core",      "Paizo",                "Pathfinder 2e",          "Bestiary",   "PDF",       360, 118.0, 2024,  64, Cloud,      "#1B3A33", "Over four hundred monsters for any encounter."),
        LibraryItem::new("b10", "Abomination Vaults",           "Paizo",                "Pathfinder 2e",          "Adventure",  "PDF",       256,  79.0, 2021, 150, Downloaded, "#232529", "A three-part dungeon delve into a sunken stronghold."),
        // Free League
        LibraryItem::new("b11", "Mörk Borg",                   "Free League",          "Mörk Borg",              "Core",       "PDF",        96,  44.0, 2020,  73, Downloaded, "#C9A02C", "Doom-metal art-punk dungeon crawler; the world is ending."),
        LibraryItem::new("b12", "The One Ring",                 "Free League",          "Middle-earth",           "Core",       "PDF",       240,  86.0, 2022, 132, Downloaded, "#8A5A1E", "Adventure in the lands and ages of Tolkien's Middle-earth."),
        LibraryItem::new("b13", "Alien: The Roleplaying Game",  "Free League",          "Alien RPG",              "Core",       "PDF",       392, 124.0, 2019, 210, Downloaded, "#182338", "Sci-fi horror and survival in the corporate frontier."),
        LibraryItem::new("b14", "Tales from the Loop",          "Free League",          "Tales from the Loop",    "Core",       "PDF",       192,  64.0, 2017, 305, Cloud,      "#16494E", "Kids solving mysteries in an alternate '80s of machines."),
        LibraryItem::new("b15", "Vaesen",                       "Free League",          "Vaesen",                 "Core",       "PDF",       192,  70.0, 2020, 118, Downloaded, "#1B3A33", "Nordic folk horror; hunt the creatures of myth."),
        LibraryItem::new("b16", "Forbidden Lands",              "Free League",          "Forbidden Lands",        "Core",       "PDF",       232,  81.0, 2018, 240, Cloud,      "#7C3A24", "Open-world survival sandbox of raiders and explorers."),
        LibraryItem::new("b17", "Blade Runner: The Roleplaying Game","Free League",     "Blade Runner",           "Core",       "PDF + EPUB",280,  99.0, 2022,  88, Downloaded, "#2A2750", "Neo-noir replicant detective work in 2037 Los Angeles."),
        LibraryItem::new("b18", "Twilight: 2000",               "Free League",          "Twilight: 2000",         "Core",       "PDF",       232,  77.0, 2021, 162, Cloud,      "#46491F", "Open-world survival in the aftermath of a European war."),
        // Chaosium
        LibraryItem::new("b19", "Call of Cthulhu: Keeper Rulebook","Chaosium",          "Call of Cthulhu 7e",     "Core",       "PDF",       448, 128.0, 2014, 360, Downloaded, "#20402F", "Investigative cosmic horror in the Lovecraftian tradition."),
        LibraryItem::new("b20", "Pulp Cthulhu",                 "Chaosium",             "Call of Cthulhu 7e",     "Supplement", "PDF",       280,  92.0, 2016, 190, Cloud,      "#7A2E3A", "Two-fisted action heroics against the Mythos."),
        LibraryItem::new("b21", "RuneQuest: Roleplaying in Glorantha","Chaosium",       "RuneQuest",              "Core",       "PDF",       448, 140.0, 2018, 215, Downloaded, "#7C3A24", "Bronze-age myth and magic in the world of Glorantha."),
        LibraryItem::new("b22", "Masks of Nyarlathotep",        "Chaosium",             "Call of Cthulhu 7e",     "Adventure",  "PDF",       656, 188.0, 2018, 134, Downloaded, "#232529", "Globe-spanning campaign to stop an apocalyptic cult."),
        // Modiphius
        LibraryItem::new("b23", "Dune: Adventures in the Imperium","Modiphius",         "Dune",                   "Core",       "PDF",       336, 110.0, 2021, 158, Downloaded, "#8A5A1E", "Houses, intrigue, and survival on the desert planet."),
        LibraryItem::new("b24", "Star Trek Adventures",         "Modiphius",            "Star Trek",              "Core",       "PDF",       368, 116.0, 2017, 300, Cloud,      "#1C2A44", "Explore strange new worlds in the final frontier."),
        LibraryItem::new("b25", "Conan: Adventures in an Age Undreamed Of","Modiphius", "Conan",                  "Core",       "PDF",       368, 121.0, 2017, 276, Cloud,      "#7C3A24", "Sword-and-sorcery across the Hyborian Age."),
        LibraryItem::new("b26", "Achtung! Cthulhu",             "Modiphius",            "Achtung! Cthulhu",       "Core",       "PDF",       360, 113.0, 2021, 144, Downloaded, "#2E3A45", "Secret-war pulp horror against occult Nazi forces."),
        // R. Talsorian
        LibraryItem::new("b27", "Cyberpunk RED",                "R. Talsorian Games",   "Cyberpunk",              "Core",       "PDF",       456, 134.0, 2020, 126, Downloaded, "#7E2230", "Style over substance in the dark future of Night City."),
        // Monte Cook Games
        LibraryItem::new("b28", "Numenera Discovery",           "Monte Cook Games",     "Numenera",               "Core",       "PDF",       416, 122.0, 2018, 232, Cloud,      "#15403F", "Science-fantasy a billion years in Earth's future."),
        LibraryItem::new("b29", "Cypher System Rulebook",       "Monte Cook Games",     "Cypher System",          "Core",       "PDF",       416, 119.0, 2019, 198, Downloaded, "#2A2750", "The flexible engine behind Numenera, for any genre."),
        LibraryItem::new("b30", "Invisible Sun",                "Monte Cook Games",     "Invisible Sun",          "Setting",    "PDF",       240,  84.0, 2018, 250, Cloud,      "#45264A", "Surreal magic and hidden worlds for dedicated tables."),
        // Pelgrane Press
        LibraryItem::new("b31", "13th Age",                     "Pelgrane Press",       "13th Age",               "Core",       "PDF",       320,  95.0, 2013, 340, Downloaded, "#7A2E3A", "d20 fantasy built around your character's unique edge."),
        LibraryItem::new("b32", "Trail of Cthulhu",             "Pelgrane Press",       "GUMSHOE",                "Core",       "PDF",       240,  71.0, 2008, 355, Cloud,      "#232529", "Clue-driven 1930s investigative horror."),
        LibraryItem::new("b33", "Night's Black Agents",         "Pelgrane Press",       "GUMSHOE",                "Core",       "PDF",       232,  73.0, 2012, 168, Downloaded, "#182338", "Burned spies versus a vampyric conspiracy."),
        // Renegade
        LibraryItem::new("b34", "Vampire: The Masquerade 5th Ed.","Renegade Game Studios","World of Darkness",    "Core",       "PDF",       416, 130.0, 2018, 102, Downloaded, "#6B2230", "Personal horror and intrigue among the undead."),
        // Onyx Path
        LibraryItem::new("b35", "Scion: Origin",                "Onyx Path",            "Scion",                  "Core",       "PDF",       240,  82.0, 2019, 222, Cloud,      "#8A5A1E", "Mortal children of the gods in the modern world."),
        LibraryItem::new("b36", "Chronicles of Darkness",       "Onyx Path",            "World of Darkness",      "Core",       "PDF",       320,  97.0, 2015, 260, Cloud,      "#232529", "A modern world of secret horrors lurking beneath."),
        // Goodman Games
        LibraryItem::new("b37", "Dungeon Crawl Classics",       "Goodman Games",        "DCC RPG",                "Core",       "PDF",       488, 142.0, 2012, 175, Downloaded, "#8A5A1E", "Old-school sword-and-sorcery with deadly funnels."),
        LibraryItem::new("b38", "Mutant Crawl Classics",        "Goodman Games",        "MCC RPG",                "Core",       "PDF",       320, 101.0, 2017, 192, Cloud,      "#46491F", "Post-apocalyptic gonzo science-fantasy crawling."),
        // Kobold Press
        LibraryItem::new("b39", "Tome of Beasts",               "Kobold Press",         "Dungeons & Dragons 5e",  "Bestiary",   "PDF",       432, 126.0, 2016, 138, Downloaded, "#1B3A33", "More than four hundred new monsters for 5e."),
        LibraryItem::new("b40", "Tales of the Valiant",         "Kobold Press",         "Tales of the Valiant",   "Core",       "PDF",       360, 108.0, 2024,  58, Cloud,      "#1C2A44", "A standalone evolution of the 5e ruleset."),
        // Cubicle 7
        LibraryItem::new("b41", "Warhammer Fantasy Roleplay",   "Cubicle 7",            "WFRP 4e",                "Core",       "PDF",       368, 117.0, 2018, 205, Downloaded, "#7A2E3A", "Grim, perilous adventure in the Old World."),
        LibraryItem::new("b42", "Wrath & Glory",                "Cubicle 7",            "Warhammer 40,000",       "Core",       "PDF",       456, 138.0, 2020, 120, Cloud,      "#2E3A45", "Heroic action in the grim darkness of the far future."),
        // Magpie Games
        LibraryItem::new("b43", "Avatar Legends",               "Magpie Games",         "Avatar",                 "Core",       "PDF",       320,  99.0, 2022, 110, Downloaded, "#15403F", "Bend the elements and grow as a balanced hero."),
        LibraryItem::new("b44", "Masks: A New Generation",      "Magpie Games",         "Powered by the Apocalypse","Core",     "PDF",       224,  68.0, 2016, 248, Cloud,      "#2A2750", "Teen superheroes finding out who they really are."),
        // Evil Hat
        LibraryItem::new("b45", "Blades in the Dark",           "Evil Hat",             "Forged in the Dark",     "Core",       "PDF",       336,  94.0, 2017,  84, Downloaded, "#232529", "Daring scoundrels building a crew in a haunted city."),
        LibraryItem::new("b46", "Fate Core System",             "Evil Hat",             "Fate",                   "Core",       "PDF",       302,  89.0, 2013, 330, Cloud,      "#34234A", "A flexible, narrative engine for any setting."),
    ]
}

// ── Filtering ─────────────────────────────────────────────────────────────────

/// The threshold (in `added_order`) below which an item counts as recently added.
const RECENTLY_ADDED_THRESHOLD: u32 = 90;

/// Returns `true` if `item` passes the given sidebar filter.
#[must_use]
pub fn item_matches_filter(item: &LibraryItem, filter: &SidebarFilter) -> bool {
    match filter {
        SidebarFilter::AllTitles => true,
        SidebarFilter::RecentlyAdded => item.added_order <= RECENTLY_ADDED_THRESHOLD,
        SidebarFilter::OnDevice => item.status == ItemStatus::Downloaded,
        SidebarFilter::InCloud => item.status == ItemStatus::Cloud,
        SidebarFilter::Publisher(name) => item.publisher.as_ref() == name.as_ref(),
    }
}

/// Returns `true` if `item` contains `query` in title, publisher, or game line
/// (case-insensitive).
#[must_use]
pub fn item_matches_query(item: &LibraryItem, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let q = query.to_lowercase();
    item.title.to_lowercase().contains(&q)
        || item.publisher.to_lowercase().contains(&q)
        || item.line.to_lowercase().contains(&q)
}

// ── Sorting ───────────────────────────────────────────────────────────────────

/// Sorts `items` in place according to `method`.
pub fn sort_items(items: &mut [LibraryItem], method: SortMethod) {
    match method {
        SortMethod::Title => items.sort_by(|a, b| a.title.cmp(&b.title)),
        SortMethod::Publisher => items.sort_by(|a, b| {
            a.publisher
                .cmp(&b.publisher)
                .then_with(|| a.title.cmp(&b.title))
        }),
        SortMethod::DateAdded => items.sort_by_key(|i| i.added_order),
        SortMethod::PageCount => items.sort_by(|a, b| {
            b.pages.cmp(&a.pages).then_with(|| a.title.cmp(&b.title))
        }),
    }
}

// ── Publisher aggregation ─────────────────────────────────────────────────────

/// A publisher entry with the count of its items in the current result set.
#[derive(Debug, Clone)]
pub struct PublisherEntry {
    pub name: Arc<str>,
    pub count: usize,
}

/// Returns publisher entries sorted by count descending, then name ascending.
#[must_use]
pub fn publisher_entries(items: &[LibraryItem]) -> Vec<PublisherEntry> {
    let mut map: std::collections::HashMap<Arc<str>, usize> =
        std::collections::HashMap::new();
    for item in items {
        *map.entry(Arc::clone(&item.publisher)).or_insert(0) += 1;
    }
    let mut entries: Vec<PublisherEntry> = map
        .into_iter()
        .map(|(name, count)| PublisherEntry { name, count })
        .collect();
    entries.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.cmp(&b.name)));
    entries
}

// ── Grouped view ──────────────────────────────────────────────────────────────

/// A publisher group containing its items (already filtered and sorted).
#[derive(Debug, Clone)]
pub struct PublisherGroup {
    pub publisher: Arc<str>,
    pub items: Vec<LibraryItem>,
}

/// Partitions `items` into publisher groups in the same order as `publisher_entries`.
#[must_use]
pub fn group_by_publisher(items: Vec<LibraryItem>) -> Vec<PublisherGroup> {
    let entries = publisher_entries(&items);
    let mut map: std::collections::HashMap<Arc<str>, Vec<LibraryItem>> =
        std::collections::HashMap::new();
    for item in items {
        map.entry(Arc::clone(&item.publisher))
            .or_default()
            .push(item);
    }
    entries
        .into_iter()
        .filter_map(|e| {
            map.remove(&e.name).map(|group_items| PublisherGroup {
                publisher: e.name,
                items: group_items,
            })
        })
        .collect()
}

// ── Smart section counts ──────────────────────────────────────────────────────

/// Counts for each smart sidebar section given the full catalog.
#[derive(Debug, Clone, Copy)]
pub struct SectionCounts {
    pub all: usize,
    pub recently_added: usize,
    pub on_device: usize,
    pub in_cloud: usize,
}

/// Computes smart section counts from the full catalog.
#[must_use]
pub fn section_counts(catalog: &[LibraryItem]) -> SectionCounts {
    SectionCounts {
        all: catalog.len(),
        recently_added: catalog
            .iter()
            .filter(|i| i.added_order <= RECENTLY_ADDED_THRESHOLD)
            .count(),
        on_device: catalog
            .iter()
            .filter(|i| i.status == ItemStatus::Downloaded)
            .count(),
        in_cloud: catalog
            .iter()
            .filter(|i| i.status == ItemStatus::Cloud)
            .count(),
    }
}

// ── Footer totals ─────────────────────────────────────────────────────────────

/// Formats `bytes` as a human-readable size string (GB or MB).
#[must_use]
pub fn format_total_size(items: &[LibraryItem]) -> String {
    let total_mb: f64 = items.iter().map(|i| i.size_mb).sum();
    if total_mb >= 1024.0 {
        format!("{:.1} GB", total_mb / 1024.0)
    } else {
        format!("{:.0} MB", total_mb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_catalog_has_expected_count() {
        assert_eq!(stub_catalog().len(), 46);
    }

    #[test]
    fn section_counts_sum_to_total() {
        let catalog = stub_catalog();
        let counts = section_counts(&catalog);
        assert_eq!(counts.on_device + counts.in_cloud, counts.all);
    }

    #[test]
    fn item_matches_query_case_insensitive() {
        let item = stub_catalog().remove(0); // Player's Handbook
        assert!(item_matches_query(&item, "player"));
        assert!(item_matches_query(&item, "PLAYER"));
        assert!(!item_matches_query(&item, "starfinder"));
    }

    #[test]
    fn publisher_entries_ordered_count_desc() {
        let catalog = stub_catalog();
        let entries = publisher_entries(&catalog);
        for w in entries.windows(2) {
            assert!(
                w[0].count >= w[1].count,
                "publisher entries not sorted by count desc"
            );
        }
    }
}
