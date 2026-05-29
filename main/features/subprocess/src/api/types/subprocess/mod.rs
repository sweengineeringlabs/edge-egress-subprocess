//! Subprocess value object types — args, config, result, and factory.

pub mod subprocess_args;
pub mod subprocess_args_builder;
pub mod subprocess_config;
pub mod subprocess_config_builder;
pub mod subprocess_result;
pub mod subprocess_svc;

pub use subprocess_args::SubprocessArgs;
pub use subprocess_args_builder::SubprocessArgsBuilder;
pub use subprocess_config::SubprocessConfig;
pub use subprocess_config_builder::SubprocessConfigBuilder;
pub use subprocess_result::SubprocessResult;
pub use subprocess_svc::SubprocessSvc;
