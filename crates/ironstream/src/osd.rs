// FILE: osd.rs
// occt: OSD

/// Operating System interface module.
/// Provides cross-platform abstractions for OS operations.
pub mod osd {
    /// Get current working directory.
    pub fn current_dir() -> std::io::Result<std::path::PathBuf> {
        std::env::current_dir()
    }

    /// Get home directory.
    pub fn home_dir() -> Option<std::path::PathBuf> {
        std::env::var_os("HOME")
            .and_then(|h| if h.is_empty() { None } else { Some(h.into()) })
    }

    /// Get temp directory.
    pub fn temp_dir() -> std::path::PathBuf {
        std::env::temp_dir()
    }

    /// Get environment variable.
    pub fn get_env(name: &str) -> Option<String> {
        std::env::var(name).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::osd::*;

    #[test]
    fn test_current_dir() {
        let dir = current_dir();
        assert!(dir.is_ok());
    }

    #[test]
    fn test_temp_dir() {
        let dir = temp_dir();
        assert!(dir.is_absolute());
    }
}
