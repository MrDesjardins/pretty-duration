use pretty_duration::pretty_duration;
use pretty_duration::PrettyDurationLabels;
use std::time::Duration;

use pretty_duration::PrettyDurationOptions;
use pretty_duration::PrettyDurationOutputFormat;

// -------------------------
// No Option
// -------------------------
#[test]
fn pretty_duration_no_option() {
    let result = pretty_duration(&Duration::from_millis(1), None);
    assert_eq!(result, "1ms");
}

// -------------------------
// Output format Compact
// -------------------------
#[test]
fn pretty_duration_compact_large_duration() {
    let result = pretty_duration(
        &Duration::from_millis(31556956789),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Compact),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "1y 11mon 109d 5h 49m 16s 789ms");
}

#[test]
fn pretty_duration_compact_minutes_second_duration() {
    let result = pretty_duration(
        &Duration::from_millis(301000),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Compact),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "5m 1s");
}

#[test]
fn pretty_duration_compact_hours_minutes_duration() {
    let result = pretty_duration(
        &Duration::from_millis(3661001),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Compact),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "1h 1m 1s 1ms");
}

#[test]
fn pretty_duration_compact_zerovalue() {
    let result = pretty_duration(
        &Duration::from_millis(0),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Compact),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "0ns");
}

// -------------------------
// Output format Expanded
// -------------------------

#[test]
fn pretty_duration_expanded_large_duration() {
    let result = pretty_duration(
        &Duration::from_millis(31556956789),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Expanded),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(
        result,
        "1 year 11 months 109 days 5 hours 49 minutes 16 seconds 789 milliseconds"
    );
}

#[test]
fn pretty_duration_expanded_minutes_second_duration() {
    let result = pretty_duration(
        &Duration::from_millis(301000),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Expanded),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "5 minutes 1 second");
}

#[test]
fn pretty_duration_expanded_hours_minutes_duration() {
    let result = pretty_duration(
        &Duration::from_millis(3661001),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Expanded),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "1 hour 1 minute 1 second 1 millisecond");
}

#[test]
fn pretty_duration_expanded_zerovalue() {
    let result = pretty_duration(
        &Duration::from_millis(0),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Expanded),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "0 nanosecond");
}

// -------------------------
// Provide alternative word for expanded
// -------------------------
#[test]
fn pretty_duration_expanded_custom_words() {
    let result = pretty_duration(
        &Duration::from_millis(31556956789),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Expanded),
            singular_labels: Some(PrettyDurationLabels {
                year: "année",
                month: "mois", // Not the `s` here in singular form
                day: "jour",
                hour: "heure",
                minute: "minute",
                second: "seconde",
                millisecond: "milliseconde",
                microsecond: "microseconde",
                nanosecond: "nanoseconde",
            }),
            plural_labels: Some(PrettyDurationLabels {
                year: "années",
                month: "mois",
                day: "jours",
                hour: "heures",
                minute: "minutes",
                second: "secondes",
                millisecond: "millisecondes",
                microsecond: "microsecondes",
                nanosecond: "nanosecondes",
            }),
        }),
    );
    assert_eq!(
        result,
        "1 année 11 mois 109 jours 5 heures 49 minutes 16 secondes 789 millisecondes"
    );
}

// -------------------------
// Test microseconds and nanoseconds
// -------------------------
#[test]
fn pretty_duration_compact_microseconds() {
    let result = pretty_duration(
        &Duration::from_micros(1234),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Compact),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "1ms 234μs");
}

#[test]
fn pretty_duration_compact_nanoseconds() {
    let result = pretty_duration(
        &Duration::from_nanos(1234567),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Compact),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "1ms 234μs 567ns");
}

#[test]
fn pretty_duration_expanded_microseconds() {
    let result = pretty_duration(
        &Duration::from_micros(1),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Expanded),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "1 microsecond");
}

#[test]
fn pretty_duration_expanded_nanoseconds() {
    let result = pretty_duration(
        &Duration::from_nanos(999),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Expanded),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "999 nanoseconds");
}

#[test]
fn pretty_duration_expanded_all_units() {
    let result = pretty_duration(
        &Duration::from_nanos(3661001234567),
        Some(PrettyDurationOptions {
            output_format: Some(PrettyDurationOutputFormat::Expanded),
            singular_labels: None,
            plural_labels: None,
        }),
    );
    assert_eq!(result, "1 hour 1 minute 1 second 1 millisecond 234 microseconds 567 nanoseconds");
}
