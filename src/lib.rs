//! Pretty_duration takes a `Duration` and output a `String` in prettier way that make
//! sense for a human being.
//!
//! Provide customization with a short and long format. There is an optional configuration
//! to override terms for each time bins (year, month, day, hour, minute, second, millisecond)
//! allowing people using the library from a different language to adapt the output [^note].
//!
//! [^note]: The library let you specify the singular and plural form providing flexibility for
//! language that must have special rule. For example, the French word for month is `mois` with
//! an ending end for the singular and plural form.
mod pretty_duration_lib;
use std::time::Duration;

// Private because we do not want to expose outside the crate
use crate::pretty_duration_lib::PrettyDurationOptionsWithDefault;

// Public to expose these types to the consumer of the crate
pub use crate::pretty_duration_lib::{
    DurationBins, PrettyDurationLabels, PrettyDurationOptions, PrettyDurationOutputFormat,
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
const EXPANDED_SINGULAR_DEFAULT: PrettyDurationLabels = PrettyDurationLabels {
    year: "year",
    month: "month",
    day: "day",
    hour: "hour",
    minute: "minute",
    second: "second",
    millisecond: "millisecond",
};
const EXPANDED_PLURAL_DEFAULT: PrettyDurationLabels = PrettyDurationLabels {
    year: "years",
    month: "months",
    day: "days",
    hour: "hours",
    minute: "minutes",
    second: "seconds",
    millisecond: "milliseconds",
};

/// Main function of the pretty-duration library that takes a required [std::time::Duration] and
/// and optional [PrettyDurationOptions].
///
/// # Arguments
/// The first argument is a [std::time::Duration] that is the input to produce the [String] output.
///
/// The second argument is the optional configuration [PrettyDurationOptions] giving you the possibility
/// to decide the format (extended or compact) but also to decide the symbol and word for each [PrettyDurationLabels]
///
/// # Default
/// With an option set to [None], the function returns the duration in a compact format in US English symbol.
///
/// # Examples
///
/// ## No option outputs a string in a short format
/// ```rust
/// use std::time::Duration;
/// use pretty_duration::pretty_duration;
///    
/// let result = pretty_duration(&Duration::from_millis(1), None);
/// assert_eq!(result, "1ms");
/// ```
/// ## Option to have the extended long format
/// ```rust
/// use pretty_duration::pretty_duration;
/// use pretty_duration::PrettyDurationLabels;
/// use pretty_duration::PrettyDurationOptions;
/// use pretty_duration::PrettyDurationOutputFormat;
/// use std::time::Duration;
/// let result = pretty_duration(
///     &Duration::from_millis(43556556722),
///     Some(PrettyDurationOptions {
///         output_format: Some(PrettyDurationOutputFormat::Expanded),
///         singular_labels: None,
///         plural_labels: None,
///     }),
/// );
/// assert_eq!(
///     result,
///     "1 year 16 months 248 days 3 hours 2 minutes 36 seconds 722 milliseconds"
///);
/// ```
pub fn pretty_duration(duration: &Duration, options: Option<PrettyDurationOptions>) -> String {
    let options_with_default = set_default_options(options);
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
        &get_unit(
            options_with_default.singular_labels.year,
            options_with_default.plural_labels.year,
            duration_by_bin.years > 1,
        ),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.months.to_string(),
        &get_unit(
            options_with_default.singular_labels.month,
            options_with_default.plural_labels.month,
            duration_by_bin.months > 1,
        ),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.days.to_string(),
        &get_unit(
            options_with_default.singular_labels.day,
            options_with_default.plural_labels.day,
            duration_by_bin.days > 1,
        ),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.hours.to_string(),
        &get_unit(
            options_with_default.singular_labels.hour,
            options_with_default.plural_labels.hour,
            duration_by_bin.hours > 1,
        ),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.minutes.to_string(),
        &get_unit(
            options_with_default.singular_labels.minute,
            options_with_default.plural_labels.minute,
            duration_by_bin.minutes > 1,
        ),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.seconds.to_string(),
        &get_unit(
            options_with_default.singular_labels.second,
            options_with_default.plural_labels.second,
            duration_by_bin.seconds > 1,
        ),
        is_full_word,
    );
    try_adding(
        &mut result,
        duration_by_bin.milliseconds.to_string(),
        &get_unit(
            options_with_default.singular_labels.millisecond,
            options_with_default.plural_labels.millisecond,
            duration_by_bin.milliseconds > 1,
        ),
        is_full_word,
    );

    if result.len() == 0 {
        let separator = match is_full_word {
            true => " ",
            false => "",
        };
        return format!(
            "{}{}{}",
            "0", separator, options_with_default.singular_labels.millisecond
        );
    }
    return result.join(" ");
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

fn get_unit(singular_string: &str, plural_string: &str, is_plural: bool) -> String {
    return match is_plural {
        true => plural_string.to_string(),
        false => singular_string.to_string(),
    };
}

fn set_default_options(
    user_options: Option<PrettyDurationOptions>,
) -> PrettyDurationOptionsWithDefault {
    // Ensure if we the user passed nothing that we have a type with all options with no value
    let user_options2 = user_options.unwrap_or_else(|| PrettyDurationOptions {
        output_format: None,
        singular_labels: None,
        plural_labels: None,
    });

    let output_format = user_options2
        .output_format
        .unwrap_or_else(|| PrettyDurationOutputFormat::Compact);

    // Give default value to all options not defined by the user
    let default_options = PrettyDurationOptionsWithDefault {
        output_format,
        singular_labels: user_options2
            .singular_labels
            .unwrap_or_else(|| match output_format {
                PrettyDurationOutputFormat::Compact => COMPACT_DEFAULT,
                PrettyDurationOutputFormat::Expanded => EXPANDED_SINGULAR_DEFAULT,
                PrettyDurationOutputFormat::Colon => EXPANDED_SINGULAR_DEFAULT,
            }),
        plural_labels: user_options2
            .plural_labels
            .unwrap_or_else(|| match output_format {
                PrettyDurationOutputFormat::Compact => COMPACT_DEFAULT,
                PrettyDurationOutputFormat::Expanded => EXPANDED_PLURAL_DEFAULT,
                PrettyDurationOutputFormat::Colon => EXPANDED_PLURAL_DEFAULT,
            }),
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
mod test_get_unit {
    use super::*;

    #[test]
    fn test_get_unit_left_side() {
        let result = get_unit("unit1", "unit2", true);
        assert_eq!(result, "unit2")
    }

    #[test]
    fn test_get_unit_right_side() {
        let result = get_unit("unit1", "unit2", false);
        assert_eq!(result, "unit1")
    }
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
