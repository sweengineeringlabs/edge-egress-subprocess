//! `NormalisedCommand` — normalised command name used in allow-list comparisons.

/// A command basename after normalisation (directory prefix and `.exe` suffix stripped).
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor (Rule 121) — intentionally unused"
)]
pub type NormalisedCommand = String;
