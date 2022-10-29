use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pretty_duration::pretty_duration;
use pretty_duration::PrettyDurationLabels;
use pretty_duration::PrettyDurationOptions;
use pretty_duration::PrettyDurationOutputFormat;
use std::time::Duration;

fn all_benchmarks(c: &mut Criterion) {
    c.bench_function("pretty_duration with default options", |b| {
        b.iter(|| pretty_duration(black_box(&Duration::from_millis(31556956789)), None))
    });

    c.bench_function("pretty_duration with full custom options", |b| {
        b.iter(|| {
            pretty_duration(
                black_box(&Duration::from_millis(31556956789)),
                black_box(Some(PrettyDurationOptions {
                    output_format: Some(PrettyDurationOutputFormat::Expanded),
                    singular_labels: Some(PrettyDurationLabels {
                        year: "année",
                        month: "mois", // Not the `s` here in singular form
                        day: "jour",
                        hour: "heure",
                        minute: "minute",
                        second: "seconde",
                        millisecond: "milliseconde",
                    }),
                    plural_labels: Some(PrettyDurationLabels {
                        year: "années",
                        month: "mois",
                        day: "jours",
                        hour: "heures",
                        minute: "minutes",
                        second: "secondes",
                        millisecond: "millisecondes",
                    }),
                })),
            )
        })
    });
}
criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(2000);
    targets = all_benchmarks
);
criterion_main!(benches);
