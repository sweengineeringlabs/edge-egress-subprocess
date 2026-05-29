//! `ProcessArgs` — inputs for a single subprocess invocation.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::api::traits::isolation_profile::IsolationProfile;

/// Default combined stdout+stderr byte cap — 1 MiB.
pub(crate) const DEFAULT_OUTPUT_BYTES_CAP: u64 = 1_048_576;

/// Default per-call wall-clock timeout — 30 seconds.
pub(crate) const DEFAULT_TIMEOUT_MS: u64 = 30_000;

/// All inputs to a single subprocess invocation.
///
/// Built with [`ProcessArgs::builder()`].  All fields have safe defaults;
/// callers override only what they need.
#[derive(Debug, Clone)]
pub struct ProcessArgs {
    /// Command + arguments.  `argv[0]` is the binary basename or absolute path.
    /// An empty list causes [`ProcessResult::Denied`] with an empty command string.
    pub argv: Vec<String>,

    /// Working directory for the child process.
    /// `None` → inherits the parent's working directory.
    pub cwd: Option<PathBuf>,

    /// Environment variables overlaid on a cleared environment.
    /// The child never inherits the parent's environment unless the caller
    /// explicitly copies entries — fail-closed by default.
    pub env: HashMap<String, String>,

    /// Basenames the child's `argv[0]` must match after normalisation (strip
    /// directory prefix; strip `.exe` and lowercase on Windows).
    /// An empty list blocks all commands.
    pub allow_commands: Vec<String>,

    /// Wall-clock deadline for the child process in milliseconds.
    /// On expiry the child receives `SIGKILL` (Unix) / `TerminateProcess` (Windows).
    /// `None` → [`DEFAULT_TIMEOUT_MS`].
    pub timeout_ms: Option<u64>,

    /// Maximum combined stdout + stderr bytes buffered.
    /// Output beyond this cap is silently truncated; the exit code is still
    /// captured.  `None` → [`DEFAULT_OUTPUT_BYTES_CAP`].
    pub output_bytes_cap: Option<u64>,

    /// CPU time limit in milliseconds (user + system).
    ///
    /// Enforced via `setrlimit(RLIMIT_CPU)` on Linux. `0` = unlimited.
    /// `None` → no limit applied.
    pub cpu_time_ms: Option<u64>,

    /// Maximum virtual address space in bytes.
    ///
    /// Enforced via `setrlimit(RLIMIT_AS)` on Linux. `0` = unlimited.
    /// `None` → no limit applied.
    pub memory_bytes: Option<u64>,

    /// OS-level isolation profile applied around the subprocess invocation.
    ///
    /// `None` → no isolation (equivalent to `NoopIsolator`).
    /// Resolved at startup from `IsolationProfileRegistry`; supply `None` in
    /// unit tests that do not need isolation.
    pub isolation_profile: Option<Arc<dyn IsolationProfile>>,
}

impl ProcessArgs {
    /// Returns a builder pre-seeded with empty/default values.
    pub fn builder() -> crate::api::types::process::process_args_builder::ProcessArgsBuilder {
        crate::api::types::process::process_args_builder::ProcessArgsBuilder::default()
    }
}
