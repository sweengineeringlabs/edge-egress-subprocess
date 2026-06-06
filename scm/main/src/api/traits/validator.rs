//! `Validator` — subprocess configuration validation trait.

use crate::api::error::error::Error;

/// Validates subprocess configuration before use.
pub trait Validator: Send + Sync {
    /// Returns `Ok(())` if the configuration is valid.
    fn validate(&self) -> Result<(), Error>;
}
