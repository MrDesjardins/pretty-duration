mod pretty_duration_lib;
use std::time::Duration;

pub use crate::pretty_duration_lib::DurationBins;
pub use crate::pretty_duration_lib::{
    PrettyDurationLabels, PrettyDurationOptions, PrettyDurationOptionsWithDefault,
    PrettyDurationOutputFormat,
};

const COMPACT_DEFAULT: PrettyDurationLabels = PrettyDurationLabels {
    year: "y",
    month: "mon",
    day: "d",
    hour: "h",
    minute: "m",
    second: "s",
    millisecond: "ms",
};
const EXPANDED_DEFAULT: PrettyDurationLabels = PrettyDurationLabels {
    year: "year",
    month: "month",
    day: "day",
    hour: "hour",
    minute: "minute",
    second: "second",
    millisecond: "millisecond",
};
/// Take a duration with option
/// # Examples
/// ```rust
///
/// ```
pub fn pretty_duration(duration: &Duration, options: Option<PrettyDurationOptions>) -> String {
    let options_with_default = set_default_options(options);
    let active_labels: PrettyDurationLabels = match options_with_default.output_format {
        PrettyDurationOutputFormat::Compact => options_with_default.compact_labels,
        PrettyDurationOutputFormat::Expanded => options_with_default.expanded_labels,
        PrettyDurationOutputFormat::Colon => options_with_default.expanded_labels,
    };
    let ms = duration.as_millis();
    let duration_by_bin = extract_bins(&ms);

    let mut result: Vec<String> = Vec::new();
    let is_full_word = matches!(
        options_with_default.output_format,
        PrettyDurationOutputFormat::Expanded
    );
    try_adding(
        &mut result,
        duration_by_bin.years.to_string(),
        &pluralize(active_labels.year, duration_by_bin.years > 1, is_full_word),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.months.to_string(),
        &pluralize(
            active_labels.month,
            duration_by_bin.months > 1,
            is_full_word,
        ),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.days.to_string(),
        &pluralize(active_labels.day, duration_by_bin.days > 1, is_full_word),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.hours.to_string(),
        &pluralize(active_labels.hour, duration_by_bin.hours > 1, is_full_word),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.minutes.to_string(),
        &pluralize(
            active_labels.minute,
            duration_by_bin.minutes > 1,
            is_full_word,
        ),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.seconds.to_string(),
        &pluralize(
            active_labels.second,
            duration_by_bin.seconds > 1,
            is_full_word,
        ),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.milliseconds.to_string(),
        &pluralize(
            active_labels.millisecond,
            duration_by_bin.milliseconds > 1,
            is_full_word,
        ),
        is_full_word,
    );

    if result.len() == 0 {
        let separator = match is_full_word {
            true => " ",
            false => "",
        };
        return format!("{}{}{}", "0", separator, active_labels.millisecond);
    }
    return result.join(" ");
}

fn pluralize(word: &str, plural: bool, is_full_word: bool) -> String {
    if plural && is_full_word {
        return format!("{}s", word);
    }
    return word.to_string();
}

fn try_adding(result: &mut Vec<String>, value: String, unit: &str, is_full_word: bool) -> () {
    let separator = match is_full_word {
        true => " ",
        false => "",
    };
    if value != "0" {
        result.push(value + separator + unit);
    }
}

fn set_default_options(
    user_options: Option<PrettyDurationOptions>,
) -> PrettyDurationOptionsWithDefault {
    // Ensure if we the user passed nothing that we have a type with all options with no value
    let user_options2 = user_options.unwrap_or_else(|| PrettyDurationOptions {
        output_format: None,
        compact_labels: None,
        expanded_labels: None,
    });

    // Give default value to all options not defined by the user
    let default_options = PrettyDurationOptionsWithDefault {
        output_format: user_options2
            .output_format
            .unwrap_or_else(|| PrettyDurationOutputFormat::Compact),
        compact_labels: user_options2
            .compact_labels
            .unwrap_or_else(|| COMPACT_DEFAULT),
        expanded_labels: user_options2
            .expanded_labels
            .unwrap_or_else(|| EXPANDED_DEFAULT),
    };

    // Return all configurations with user first, then default value when not specified
    return default_options;
}

/// Convert a millisecond number into bins of time
fn extract_bins(ms: &u128) -> DurationBins {
    return DurationBins {
        years: (ms / 31556926000) as u16,
        months: (ms / 2629800000) as u8,
        days: (ms / 86400000) as u8,
        hours: ((ms / 3600000) % 24) as u8,
        minutes: ((ms / 60000) % 60) as u8,
        seconds: ((ms / 1000) % 60) as u8,
        milliseconds: ((ms) % 1000) as u16,
    };
}

#[cfg(test)]
mod test_extract_bins {
    use super::*;

    #[test]
    fn test_extract_bins_huge() {
        let result = extract_bins(&31556956789);
        assert_eq!(result.years, 1, "Year mismatch");
        assert_eq!(result.months, 11, "Month mismatch");
        assert_eq!(result.days, 109, "Day mismatch");
        assert_eq!(result.hours, 5, "Hour mismatch");
        assert_eq!(result.minutes, 49, "Minute mismatch");
        assert_eq!(result.seconds, 16, "Second mismatch");
        assert_eq!(result.milliseconds, 789, "Millisecond mismatch");
    }
    #[test]
    fn test_extract_bins_zero() {
        let result = extract_bins(&0);
        assert_eq!(result.years, 0, "Year mismatch");
        assert_eq!(result.months, 0, "Month mismatch");
        assert_eq!(result.days, 0, "Day mismatch");
        assert_eq!(result.hours, 0, "Hour mismatch");
        assert_eq!(result.minutes, 0, "Minute mismatch");
        assert_eq!(result.seconds, 0, "Second mismatch");
        assert_eq!(result.milliseconds, 0, "Millisecond mismatch");
    }
}

#[cfg(test)]
mod test_pluralize {
    use super::*;

    #[test]
    fn test_pluralize_plural_not_full_word() {
        let result = pluralize("word", true, false);
        assert_eq!(result, "word");
    }
    #[test]
    fn test_pluralize_plural_full_word() {
        let result = pluralize("word", true, true);
        assert_eq!(result, "words");
    }
    #[test]
    fn test_pluralize_singular_not_full_word() {
        let result = pluralize("word", false, false);
        assert_eq!(result, "word");
    }
    #[test]
    fn test_pluralize_singular_full_word() {
        let result = pluralize("word", false, true);
        assert_eq!(result, "word");
    }
}
