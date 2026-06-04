//! `ApplicationConfigBuilder` — the api-level config builder contract.
//!
//! Counterpart for `core::swe::application_config_builder::ApplicationConfigBuilder`.

/// Application-level config builder — a type alias for the configbuilder's
/// concrete builder type. Returned by [`SubprocessSvc::config_builder`].
///
/// [`SubprocessSvc::config_builder`]: crate::SubprocessSvc::config_builder
pub type ApplicationConfigBuilder = swe_edge_configbuilder::ConfigBuilderImpl;
