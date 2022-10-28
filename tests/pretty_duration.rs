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
            compact_labels: None,
            expanded_labels: None,
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
            compact_labels: None,
            expanded_labels: None,
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
            compact_labels: None,
            expanded_labels: None,
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
            compact_labels: None,
            expanded_labels: None,
        }),
    );
    assert_eq!(result, "0ms");
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
            compact_labels: None,
            expanded_labels: None,
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
            compact_labels: None,
            expanded_labels: None,
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
            compact_labels: None,
            expanded_labels: None,
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
            compact_labels: None,
            expanded_labels: None,
        }),
    );
    assert_eq!(result, "0 millisecond");
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
            compact_labels: None,
            expanded_labels: Some(PrettyDurationLabels {
                year: "année",
                month: "mois",
                day: "jour",
                hour: "heure",
                minute: "minute",
                second: "seconde",
                millisecond: "milliseconde",
            }),
        }),
    );
    assert_eq!(
        result,
        "1 année 11 moiss 109 jours 5 heures 49 minutes 16 secondes 789 millisecondes"
    );
    // ⚠️⚠️ That is NOT good (moiss), but will take care for particular rules later
}
