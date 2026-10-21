//! What a smart collection is made of.

use serde::{Deserialize, Serialize};

use crate::util::search::Field;

/// Whether an item has to meet every rule or only one of them.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchMode {
    /// The item meets every rule ("all of the following").
    #[default]
    All,
    /// The item meets at least one rule ("any of the following").
    Any,
}

/// How a rule compares the field with its value.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    /// The field has the value.
    #[default]
    Is,
    /// The field does not have the value.
    IsNot,
}

/// One test in a smart collection, such as "tag is not running".
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Rule {
    /// What the rule tests.
    #[serde(with = "field_keyword")]
    pub field:    Field,
    /// How the field is compared with the value.
    pub operator: Operator,
    /// The value to compare with. A rule whose value is blank is a rule still
    /// being typed, and is ignored.
    pub value:    String,
}

/// A collection that finds its own members.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SmartCollection {
    /// The collection's identity, which the store gives it when it is added.
    pub id:                   u64,
    /// The name shown in the sidebar.
    pub name:                 String,
    /// Whether an item has to meet every rule or only one.
    pub mode:                 MatchMode,
    /// The rules, in the order the user wrote them.
    pub rules:                Vec<Rule>,
    /// Whether items that start to match are downloaded without asking.
    pub download_new_matches: bool,
    /// Whether the members are mirrored as symlinks in a folder on disk.
    pub mirror_as_symlinks:   bool,
}

/// Reads and writes a [`Field`] as its search keyword (`type`, `tag`, ...),
/// which is also how it is typed and shown.
mod field_keyword {
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    use crate::util::search::Field;

    /// Writes `field` as its keyword.
    pub fn serialize<S: Serializer>(field: &Field, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(field.keyword())
    }

    /// Reads a field from its keyword.
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Field, D::Error> {
        let keyword = String::deserialize(deserializer)?;
        Field::from_keyword(&keyword).ok_or_else(|| D::Error::custom(format!("unknown field: {keyword}")))
    }
}
