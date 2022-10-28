// Add the modules (files) but do not expose to avoid long path when using
mod pretty_duration_bins;
mod pretty_duration_options;

// Expose direct struct and enum for shorter path when using
pub use pretty_duration_bins::DurationBins;
pub use pretty_duration_options::*;
