//! What the Summaries settings offer to choose from, and how the month's use is
//! worded. No window and no I/O, so it can be tested directly.

use rust_i18n::t;

/// The models the Model menu lists, newest family first. The Anthropic provider
/// accepts any model name; these are the current ones, and a name saved by hand
/// in the settings file is added to the menu so it can still be seen.
pub const SUMMARY_MODELS: [&str; 4] = ["claude-fable-5-1",
                                       "claude-opus-5-5",
                                       "claude-sonnet-5-5",
                                       "claude-haiku-5-5"];

/// The monthly ceilings the ceiling menu lists, in tokens.
pub const TOKEN_CEILINGS: [u64; 5] = [100_000, 250_000, 500_000, 1_000_000, 2_500_000];

/// The models to list when `current` is the one in use: [`SUMMARY_MODELS`],
/// with `current` added at the end if it is not one of them.
///
/// # Examples
///
/// ```
/// use dtrpg_ui::view_models::summary_display::model_choices;
///
/// assert_eq!(model_choices("claude-opus-5-5").len(), 4);
/// assert_eq!(model_choices("my-model").last().map(String::as_str),
///            Some("my-model"));
/// ```
#[must_use]
pub fn model_choices(current: &str) -> Vec<String> {
    let mut choices: Vec<String> = SUMMARY_MODELS.iter()
                                                 .map(|model| (*model).to_owned())
                                                 .collect();
    if !choices.iter().any(|model| model == current) {
        choices.push(current.to_owned());
    }
    choices
}

/// The ceilings to list when `current` is the one in use: [`TOKEN_CEILINGS`],
/// with `current` put in its place if it is not one of them.
///
/// # Examples
///
/// ```
/// use dtrpg_ui::view_models::summary_display::ceiling_choices;
///
/// assert_eq!(ceiling_choices(500_000).len(), 5);
/// assert_eq!(ceiling_choices(300_000),
///            [100_000, 250_000, 300_000, 500_000, 1_000_000, 2_500_000]);
/// ```
#[must_use]
pub fn ceiling_choices(current: u64) -> Vec<u64> {
    let mut choices = TOKEN_CEILINGS.to_vec();
    if !choices.contains(&current) {
        choices.push(current);
        choices.sort_unstable();
    }
    choices
}

/// A token count with thousands separators, such as `500,000`.
///
/// # Examples
///
/// ```
/// use dtrpg_ui::view_models::summary_display::grouped;
///
/// assert_eq!(grouped(500_000), "500,000");
/// assert_eq!(grouped(12), "12");
/// assert_eq!(grouped(1_234_567), "1,234,567");
/// ```
#[must_use]
pub fn grouped(tokens: u64) -> String {
    let digits = tokens.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

/// A token count in a few characters, such as `18.4k` or `1.5M`.
///
/// # Examples
///
/// ```
/// use dtrpg_ui::view_models::summary_display::compact;
///
/// assert_eq!(compact(950), "950");
/// assert_eq!(compact(18_400), "18.4k");
/// assert_eq!(compact(500_000), "500k");
/// assert_eq!(compact(1_500_000), "1.5M");
/// ```
#[must_use]
pub fn compact(tokens: u64) -> String {
    let (value, suffix) = match tokens {
        0..=999 => return tokens.to_string(),
        1_000..=999_999 => (tokens as f64 / 1_000.0, "k"),
        _ => (tokens as f64 / 1_000_000.0, "M"),
    };
    let text = format!("{value:.1}");
    format!("{}{suffix}", text.strip_suffix(".0").unwrap_or(&text))
}

/// The line under the ceiling: what happens at the ceiling, and how much of it
/// this month's summaries have used.
#[must_use]
pub fn ceiling_note(used: u64, ceiling: u64) -> String {
    t!("agent.summary_ceiling_note",
       used = compact(used),
       ceiling = compact(ceiling)).to_string()
}

#[cfg(test)]
mod tests;
