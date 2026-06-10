//! `SubprocessArgsBuilder` — builder for [`SubprocessArgs`].

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::api::traits::isolation_profile::IsolationProfile;
use crate::api::types::subprocess::args::SubprocessArgs;

/// Builder for [`SubprocessArgs`].
///
/// Obtain via [`SubprocessArgs::builder()`]. All fields default to safe values;
/// call only the setters you need before calling [`build`](Self::build).
///
/// # Examples
///
/// ```rust
/// use swe_edge_egress_subprocess::SubprocessArgs;
///
/// let args = SubprocessArgs::builder()
///     .argv(vec!["grep".into(), "-r".into(), "TODO".into(), "src/".into()])
///     .allow_commands(vec!["grep".into()])
///     .timeout_ms(10_000)
///     .output_bytes_cap(65_536)
///     .build();
///
/// assert_eq!(&args.argv[0], "grep");
/// assert_eq!(args.timeout_ms, Some(10_000));
/// assert_eq!(args.output_bytes_cap, Some(65_536));
/// assert!(args.cwd.is_none());
/// assert!(args.isolation_profile.is_none());
/// ```
#[derive(Debug, Default)]
pub struct SubprocessArgsBuilder {
    argv: Vec<String>,
    cwd: Option<PathBuf>,
    env: HashMap<String, String>,
    allow_commands: Vec<String>,
    timeout_ms: Option<u64>,
    output_bytes_cap: Option<u64>,
    cpu_time_ms: Option<u64>,
    memory_bytes: Option<u64>,
    isolation_profile: Option<Arc<dyn IsolationProfile>>,
}

impl SubprocessArgsBuilder {
    /// Set `argv`.  `argv[0]` is the binary; remaining entries are arguments.
    pub fn argv(mut self, argv: Vec<String>) -> Self {
        self.argv = argv;
        self
    }

    /// Set the working directory for the child process.
    pub fn cwd(mut self, cwd: impl Into<PathBuf>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    /// Replace the environment map for the child process.
    pub fn env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    /// Set the list of allowed command basenames.
    pub fn allow_commands(mut self, commands: Vec<String>) -> Self {
        self.allow_commands = commands;
        self
    }

    /// Set the wall-clock timeout in milliseconds.
    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    /// Set the combined stdout + stderr byte cap.
    pub fn output_bytes_cap(mut self, cap: u64) -> Self {
        self.output_bytes_cap = Some(cap);
        self
    }

    /// Set the CPU time limit in milliseconds. `0` = unlimited.
    pub fn cpu_time_ms(mut self, ms: u64) -> Self {
        self.cpu_time_ms = Some(ms);
        self
    }

    /// Set the maximum virtual address space in bytes. `0` = unlimited.
    pub fn memory_bytes(mut self, bytes: u64) -> Self {
        self.memory_bytes = Some(bytes);
        self
    }

    /// Set the OS-level isolation profile.
    pub fn isolation_profile(mut self, profile: Arc<dyn IsolationProfile>) -> Self {
        self.isolation_profile = Some(profile);
        self
    }

    /// Consume the builder and return a [`SubprocessArgs`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_egress_subprocess::SubprocessArgs;
    /// let args = SubprocessArgs::builder()
    ///     .argv(vec!["echo".into(), "ok".into()])
    ///     .allow_commands(vec!["echo".into()])
    ///     .build();
    /// assert_eq!(args.argv.len(), 2);
    /// ```
    pub fn build(self) -> SubprocessArgs {
        SubprocessArgs {
            argv: self.argv,
            cwd: self.cwd,
            env: self.env,
            allow_commands: self.allow_commands,
            timeout_ms: self.timeout_ms,
            output_bytes_cap: self.output_bytes_cap,
            cpu_time_ms: self.cpu_time_ms,
            memory_bytes: self.memory_bytes,
            isolation_profile: self.isolation_profile,
        }
    }
}
