//! Application configuration builder.

#[allow(
    dead_code,
    reason = "SEA core/ counterpart anchor for api/types/application_config_builder — exercised in tests only"
)]
pub(crate) struct ApplicationConfigBuilder;

impl ApplicationConfigBuilder {
    #[allow(
        dead_code,
        reason = "SEA core/ counterpart anchor — exercised in tests only"
    )]
    pub(crate) fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_config_builder_new() {
        let _ = ApplicationConfigBuilder::new();
    }
}
