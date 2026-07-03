// FILE: standard_version_info.rs
// occt: Standard_VersionInfo

//! Version information for IronStream.

/// Development version string.
/// "" for official release, "dev" for development version.
pub fn occt_development_version() -> &'static str {
    "dev"
}

/// Version as a double "major.minor".
pub fn occt_version_double() -> f64 {
    8.0
}

/// Version as a string "major.minor".
pub fn occt_version_string() -> &'static str {
    "8.0"
}

/// Complete version as a string "major.minor.maintenance".
pub fn occt_version_string_complete() -> &'static str {
    "8.0.0"
}

/// Extended version as a string "major.minor.maintenance.devext".
pub fn occt_version_string_extended() -> &'static str {
    "8.0.0-dev"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_development_version() {
        let ver = occt_development_version();
        assert!(!ver.is_empty());
    }

    #[test]
    fn test_version_double() {
        let ver = occt_version_double();
        assert!(ver > 0.0);
    }

    #[test]
    fn test_version_string() {
        let ver = occt_version_string();
        assert!(ver.contains('.'));
    }

    #[test]
    fn test_version_string_complete() {
        let ver = occt_version_string_complete();
        assert!(ver.contains('.'));
    }

    #[test]
    fn test_version_string_extended() {
        let ver = occt_version_string_extended();
        assert!(!ver.is_empty());
    }

    #[test]
    fn test_version_consistency() {
        let major_minor = occt_version_string();
        let complete = occt_version_string_complete();
        assert!(complete.starts_with(major_minor));
    }
}
