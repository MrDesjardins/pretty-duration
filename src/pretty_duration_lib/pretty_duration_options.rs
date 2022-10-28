#[derive(Copy, Clone)]
pub enum PrettyDurationOutputFormat {
    Compact,
    Expanded,
    Colon,
}

/// Structure that hold the text to display for the singular and plural form of each unit
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
/// Options to customize the output [String]
#[derive(Clone)]
pub struct PrettyDurationOptions {
    /// Output format [PrettyDurationOutputFormat]
    pub output_format: Option<PrettyDurationOutputFormat>,
    /// Label to use when a unit of time must be singular
    pub singular_labels: Option<PrettyDurationLabels>,
    // Label to use when a unit of time must be plural
    pub plural_labels: Option<PrettyDurationLabels>,
}

// Need to fine a way to do like in TypeScript with Required<PrettyDurationOptions>
// Private structure that define all fields from [PrettyDurationOptions] without
// being optional.
#[derive(Clone)]
pub struct PrettyDurationOptionsWithDefault {
    pub output_format: PrettyDurationOutputFormat,
    pub singular_labels: PrettyDurationLabels,
    pub plural_labels: PrettyDurationLabels,
}
