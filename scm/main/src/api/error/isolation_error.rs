//! `IsolationError` — errors returned by [`IsolationProfile`] hooks.
//!
//! [`IsolationProfile`]: crate::api::traits::isolation_profile::IsolationProfile

/// Errors returned by [`IsolationProfile`] hooks.
///
/// [`IsolationProfile`]: crate::api::traits::isolation_profile::IsolationProfile
#[derive(Debug, thiserror::Error)]
pub enum IsolationError {
    /// The profile is not supported on the current platform.
    #[error("profile '{profile}' is not supported on this platform")]
    UnsupportedPlatform {
        /// The profile name that is unsupported.
        profile: String,
    },

    /// The `seccomp-bpf` filter failed to compile or load.
    #[error("seccomp filter failed for profile '{profile}': {reason}")]
    SeccompFailed {
        /// The profile name that failed.
        profile: String,
        /// The underlying error message.
        reason: String,
    },

    /// A Windows Job Object operation failed.
    #[error("Job Object operation failed for profile '{profile}': {reason}")]
    JobObjectFailed {
        /// The profile name that failed.
        profile: String,
        /// The underlying error message.
        reason: String,
    },

    /// The named profile is not registered in the `IsolationProfileRegistry`.
    #[error("profile '{profile}' is unknown; add it to subprocess_policy.toml")]
    UnknownProfile {
        /// The profile name that was requested but not found.
        profile: String,
    },
}
