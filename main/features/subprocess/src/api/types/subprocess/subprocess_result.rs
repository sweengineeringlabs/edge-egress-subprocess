//! `SubprocessResult` — outcome of a single subprocess invocation.

/// Outcome of a single subprocess invocation.
///
/// Never `Err` — all failure modes are represented as variants so callers can
/// match exhaustively without `?`. The `SubprocessRunner::run` future always
/// resolves to one of these variants; it never panics.
///
/// # Examples
///
/// ```rust
/// use swe_edge_egress_subprocess::SubprocessResult;
///
/// fn handle(result: SubprocessResult) -> String {
///     match result {
///         SubprocessResult::Completed { exit_code, stdout, stderr } => {
///             format!("exit={exit_code} out={stdout} err={stderr}")
///         }
///         SubprocessResult::Denied { command } => {
///             format!("command '{}' is not in the allow-list", command)
///         }
///         SubprocessResult::TimedOut { timeout_ms } => {
///             format!("killed after {}ms", timeout_ms)
///         }
///         SubprocessResult::SpawnFailed { reason } => {
///             format!("OS error: {}", reason)
///         }
///         SubprocessResult::IsolationFailed { profile, reason } => {
///             format!("isolation '{}' failed: {}", profile, reason)
///         }
///     }
/// }
///
/// let msg = handle(SubprocessResult::Completed {
///     exit_code: 0,
///     stdout: "hello\n".to_string(),
///     stderr: String::new(),
/// });
/// assert!(msg.contains("exit=0"));
///
/// let msg = handle(SubprocessResult::Denied { command: "rm".to_string() });
/// assert!(msg.contains("rm"));
/// ```
#[derive(Debug)]
pub enum SubprocessResult {
    /// The process exited within the deadline.
    Completed {
        /// OS exit code.  `-1` when the process was terminated by a signal.
        exit_code: i32,
        /// Captured stdout, truncated to `output_bytes_cap` bytes.
        stdout: String,
        /// Captured stderr, truncated to `output_bytes_cap - stdout.len()` bytes.
        stderr: String,
    },
    /// `argv[0]` was not in `allow_commands` after normalisation, or `argv` was empty.
    Denied {
        /// The normalised command name that was denied.  Empty when `argv` was empty.
        command: String,
    },
    /// The process did not exit within `timeout_ms` and has been killed.
    TimedOut {
        /// The timeout that expired, in milliseconds.
        timeout_ms: u64,
    },
    /// The OS rejected the spawn (binary not found, permission denied, etc.).
    SpawnFailed {
        /// The underlying OS error message.
        reason: String,
    },
    /// The isolation profile's `configure` or `apply` hook failed.
    ///
    /// The child was killed before it could execute any work.
    IsolationFailed {
        /// The profile name that failed.
        profile: String,
        /// The underlying error message.
        reason: String,
    },
}
