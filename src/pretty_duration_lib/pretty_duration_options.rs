#[derive(Copy, Clone)]
pub enum PrettyDurationOutputFormat {
    Compact,
    Expanded,
    Colon,
}

#[derive(Clone)]
pub struct PrettyDurationLabels {
    pub year: &'static str,
    pub month: &'static str,
    pub day: &'static str,
    pub hour: &'static str,
    pub minute: &'static str,
    pub second: &'static str,
    pub millisecond: &'static str,
}
/// Options to change how the produced output
#[derive(Clone)]
pub struct PrettyDurationOptions {
    pub output_format: Option<PrettyDurationOutputFormat>,
    pub compact_labels: Option<PrettyDurationLabels>,
    pub expanded_labels: Option<PrettyDurationLabels>,
}

// Need to fine a way to do like in TypeScript with Required<PrettyDurationOptions>
#[derive(Clone)]
pub struct PrettyDurationOptionsWithDefault {
    pub output_format: PrettyDurationOutputFormat,
    pub compact_labels: PrettyDurationLabels,
    pub expanded_labels: PrettyDurationLabels,
}
