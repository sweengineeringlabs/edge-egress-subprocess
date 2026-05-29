//! Allow-NormalisedCommand basename normalisation and membership check.
//!
//! Normalisation rules:
//! - Strip any directory prefix (`/usr/bin/echo` → `echo`)
//! - On Windows: strip `.exe` suffix and lowercase (`Echo.EXE` → `echo`)
//! - On Unix: preserve case (`Echo` stays `Echo`)

/// Command allow-NormalisedCommand checker with basename normalisation.
pub(crate) struct NormalisedCommand;

impl NormalisedCommand {
    /// Normalise a command path to the basename used for allow-NormalisedCommand comparison.
    pub(crate) fn normalise(command: &str) -> String {
        let base = std::path::Path::new(command)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(command);

        #[cfg(windows)]
        {
            let lowered = base.to_lowercase();
            let without_exe = lowered.strip_suffix(".exe").unwrap_or(&lowered);
            without_exe.to_string()
        }
        #[cfg(not(windows))]
        {
            base.to_string()
        }
    }

    /// Returns `true` when `command` normalises to a name present in `allow_commands`.
    ///
    /// Both sides are normalised before comparison so callers can NormalisedCommand `echo` and
    /// match `/usr/bin/echo` (Unix) or `Echo.EXE` (Windows).
    pub(crate) fn is_allowed(command: &str, allow_commands: &[String]) -> bool {
        let normalised = Self::normalise(command);
        allow_commands
            .iter()
            .any(|a| Self::normalise(a) == normalised)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalise_strips_directory_prefix() {
        assert_eq!(NormalisedCommand::normalise("/usr/bin/echo"), "echo");
    }

    #[test]
    fn test_normalise_absolute_windows_path_strips_prefix() {
        let result = NormalisedCommand::normalise("C:\\Windows\\System32\\cmd.exe");
        assert!(result == "cmd.exe" || result == "cmd");
    }

    #[test]
    fn test_normalise_bare_name_unchanged_on_unix() {
        #[cfg(not(windows))]
        assert_eq!(NormalisedCommand::normalise("echo"), "echo");
    }

    #[test]
    fn test_normalise_strips_exe_and_lowercases_on_windows() {
        #[cfg(windows)]
        assert_eq!(NormalisedCommand::normalise("Echo.EXE"), "echo");
    }

    #[test]
    fn test_is_allowed_exact_match_returns_true() {
        let allowed = vec!["echo".to_string()];
        assert!(NormalisedCommand::is_allowed("echo", &allowed));
    }

    #[test]
    fn test_is_allowed_path_prefix_stripped_returns_true() {
        let allowed = vec!["echo".to_string()];
        assert!(NormalisedCommand::is_allowed("/usr/bin/echo", &allowed));
    }

    #[test]
    fn test_is_allowed_not_present_returns_false() {
        let allowed = vec!["echo".to_string()];
        assert!(!NormalisedCommand::is_allowed("rm", &allowed));
    }

    #[test]
    fn test_is_allowed_empty_allow_list_blocks_all() {
        assert!(!NormalisedCommand::is_allowed("echo", &[]));
    }

    #[test]
    fn test_is_allowed_multiple_entries_matches_any() {
        let allowed = vec!["echo".to_string(), "cat".to_string()];
        assert!(NormalisedCommand::is_allowed("cat", &allowed));
    }
}
