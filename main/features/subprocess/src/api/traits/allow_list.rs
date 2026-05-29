//! `AllowList` — command allow-list interface.

/// Checks whether a command is permitted to run.
pub trait AllowList: Send + Sync {
    /// Returns `true` if `command` is permitted.
    fn is_allowed(&self, command: &str) -> bool;
    /// Normalise a command path to the basename used for comparison.
    fn normalise(&self, command: &str) -> String;
}
