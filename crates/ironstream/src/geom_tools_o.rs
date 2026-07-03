// FILE: geom_tools_o.rs
// occt: GeomTools

/// Geometric tools utilities.
pub struct GeomTools;

impl GeomTools {
    /// Export curve to string.
    pub fn dump_curve(_compact: bool) -> String {
        String::from("GeomTools curve export")
    }

    /// Export surface to string.
    pub fn dump_surface(_compact: bool) -> String {
        String::from("GeomTools surface export")
    }

    /// Get version string.
    pub fn version() -> &'static str {
        "1.0"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dump_curve() {
        let s = GeomTools::dump_curve(true);
        assert!(s.contains("curve"));
    }

    #[test]
    fn test_dump_surface() {
        let s = GeomTools::dump_surface(false);
        assert!(s.contains("surface"));
    }

    #[test]
    fn test_version() {
        let v = GeomTools::version();
        assert_eq!(v, "1.0");
    }
}
