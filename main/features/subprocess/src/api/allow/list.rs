//! Allow-list interface — types for command permission checking.
//!
//! [`core::allow::list`] implements the normalisation and membership check
//! used by [`DefaultProcessRunner`].
//!
//! [`DefaultProcessRunner`]: crate::core::default::runner::DefaultProcessRunner

/// Normalised command name used in allow-list comparisons.
pub type NormalisedCommand = String;
