//! Default `Validator` implementation.

use crate::api::error::error::Error;
use crate::api::traits::validator::Validator;

/// Default implementation of the [`Validator`] trait.
#[derive(Debug, Default)]
pub(crate) struct DefaultProcessValidator;

impl Validator for DefaultProcessValidator {
    fn validate(&self) -> Result<(), Error> {
        tracing::debug!("validating swe_edge_egress_subprocess");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_succeeds() {
        let v = DefaultProcessValidator;
        assert!(v.validate().is_ok());
    }

    #[test]
    fn test_default_creates_process_validator() {
        let _v = DefaultProcessValidator::default();
    }
}
