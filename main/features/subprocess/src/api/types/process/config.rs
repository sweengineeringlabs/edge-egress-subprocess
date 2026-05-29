//! `ProcessConfig` — TOML-loaded policy for subprocess execution.

use std::collections::HashMap;
use std::path::PathBuf;

use swe_edge_configbuilder::ConfigSection;

use crate::api::types::process::process_args::{ProcessArgs, DEFAULT_OUTPUT_BYTES_CAP, DEFAULT_TIMEOUT_MS};

/// Static subprocess policy loaded from the `[process]` TOML section.
///
/// These fields are constant across all calls — they define what the service
/// is allowed to run.  Per-call `argv` is supplied at the call site via
/// [`ProcessConfig::with_argv`].
///
/// # TOML example
///
/// ```toml
/// [process]
/// allow_commands   = ["ffmpeg", "convert"]
/// timeout_ms       = 10000
/// output_bytes_cap = 2097152
/// cpu_time_ms      = 60000   # 60 s CPU time; 0 = unlimited
/// memory_bytes     = 536870912  # 512 MiB; 0 = unlimited
/// cwd              = "/var/data/jobs"
///
/// [process.env]
/// TMPDIR = "/var/tmp"
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessConfig {
    /// Command basenames the runner is permitted to spawn.
    /// An empty list blocks all commands.
    #[serde(default)]
    pub allow_commands: Vec<String>,

    /// Wall-clock deadline in milliseconds applied to every invocation.
    #[serde(default = "ProcessConfig::default_timeout_ms")]
    pub timeout_ms: u64,

    /// Maximum combined stdout + stderr bytes buffered per invocation.
    #[serde(default = "ProcessConfig::default_output_bytes_cap")]
    pub output_bytes_cap: u64,

    /// Working directory for the child process.
    /// `None` → inherits the parent's working directory.
    #[serde(default)]
    pub cwd: Option<PathBuf>,

    /// Environment variables overlaid on a cleared environment.
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// CPU time limit in milliseconds (user + system).
    /// Enforced via `setrlimit(RLIMIT_CPU)` on Linux. `0` = unlimited.
    /// `None` → no limit applied.
    #[serde(default)]
    pub cpu_time_ms: Option<u64>,

    /// Maximum virtual address space in bytes.
    /// Enforced via `setrlimit(RLIMIT_AS)` on Linux. `0` = unlimited.
    /// `None` → no limit applied.
    #[serde(default)]
    pub memory_bytes: Option<u64>,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            allow_commands: Vec::new(),
            timeout_ms: DEFAULT_TIMEOUT_MS,
            output_bytes_cap: DEFAULT_OUTPUT_BYTES_CAP,
            cwd: None,
            env: HashMap::new(),
            cpu_time_ms: None,
            memory_bytes: None,
        }
    }
}

impl ConfigSection for ProcessConfig {
    fn section_name() -> &'static str {
        "process"
    }
}

impl ProcessConfig {
    /// Default wall-clock timeout — used by serde `default`.
    pub fn default_timeout_ms() -> u64 {
        DEFAULT_TIMEOUT_MS
    }

    /// Default combined stdout+stderr byte cap — used by serde `default`.
    pub fn default_output_bytes_cap() -> u64 {
        DEFAULT_OUTPUT_BYTES_CAP
    }

    /// Combine this policy with a per-call `argv` to produce a [`ProcessArgs`].
    ///
    /// `isolation_profile` defaults to `None`; callers that need OS-level
    /// isolation set it via `ProcessArgs::isolation_profile` after calling this.
    pub fn with_argv(&self, argv: Vec<String>) -> ProcessArgs {
        ProcessArgs {
            argv,
            cwd: self.cwd.clone(),
            env: self.env.clone(),
            allow_commands: self.allow_commands.clone(),
            timeout_ms: Some(self.timeout_ms),
            output_bytes_cap: Some(self.output_bytes_cap),
            cpu_time_ms: self.cpu_time_ms,
            memory_bytes: self.memory_bytes,
            isolation_profile: None,
        }
    }
}
