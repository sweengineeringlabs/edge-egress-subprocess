//! Value objects — subprocess invocation inputs, policy, and outcomes.

pub mod subprocess_args;
pub mod subprocess_args_builder;
pub mod subprocess_config;
pub mod subprocess_config_builder;
pub mod subprocess_result;

pub use subprocess_args::SubprocessArgs;
pub use subprocess_args_builder::SubprocessArgsBuilder;
pub use subprocess_config::SubprocessConfig;
pub use subprocess_config_builder::SubprocessConfigBuilder;
pub use subprocess_result::SubprocessResult;
