// FILE: xml_m_data_xtd.rs
// occt: XmlMDataXtd

use std::sync::Mutex;
use std::sync::atomic::{AtomicI32, Ordering};

/// Global document version for XML serialization of extended data attributes.
static DOCUMENT_VERSION: AtomicI32 = AtomicI32::new(-1);

/// XML serialization utilities for extended (Xtd) data attributes.
/// Manages drivers for geometry, constraints, patterns, presentations, and positioning.
pub struct XmlMDataXtd;

impl XmlMDataXtd {
    /// Add all extended data drivers to a driver table.
    /// This includes drivers for:
    /// - Geometry (points, lines, planes, cylinders, etc.)
    /// - Constraints (distance, angle, parallel, etc.)
    /// - Pattern standards (linear, circular arrays)
    /// - Triangulations
    /// - Presentations
    /// - Positions
    pub fn add_drivers() -> Vec<String> {
        vec![
            "XmlMDataXtd_GeometryDriver".to_string(),
            "XmlMDataXtd_ConstraintDriver".to_string(),
            "XmlMDataXtd_PatternStdDriver".to_string(),
            "XmlMDataXtd_TriangulationDriver".to_string(),
            "XmlMDataXtd_PresentationDriver".to_string(),
            "XmlMDataXtd_PositionDriver".to_string(),
        ]
    }

    /// Set the document version for serialization compatibility.
    pub fn set_document_version(version: i32) {
        DOCUMENT_VERSION.store(version, Ordering::SeqCst);
    }

    /// Get the current document version.
    pub fn document_version() -> i32 {
        DOCUMENT_VERSION.load(Ordering::SeqCst)
    }

    /// Check if a specific version is set.
    pub fn has_version() -> bool {
        DOCUMENT_VERSION.load(Ordering::SeqCst) >= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_drivers_returns_all_drivers() {
        let drivers = XmlMDataXtd::add_drivers();
        assert_eq!(drivers.len(), 6);
    }

    #[test]
    fn test_add_drivers_contains_geometry() {
        let drivers = XmlMDataXtd::add_drivers();
        assert!(drivers.iter().any(|d| d.contains("Geometry")));
    }

    #[test]
    fn test_add_drivers_contains_constraint() {
        let drivers = XmlMDataXtd::add_drivers();
        assert!(drivers.iter().any(|d| d.contains("Constraint")));
    }

    #[test]
    fn test_initial_document_version() {
        XmlMDataXtd::set_document_version(-1);
        assert_eq!(XmlMDataXtd::document_version(), -1);
        assert!(!XmlMDataXtd::has_version());
    }

    #[test]
    fn test_set_and_get_document_version() {
        XmlMDataXtd::set_document_version(3);
        assert_eq!(XmlMDataXtd::document_version(), 3);
        assert!(XmlMDataXtd::has_version());
    }

    #[test]
    fn test_version_persistence() {
        XmlMDataXtd::set_document_version(5);
        let ver1 = XmlMDataXtd::document_version();
        let ver2 = XmlMDataXtd::document_version();
        assert_eq!(ver1, ver2);
        assert_eq!(ver1, 5);
    }

    #[test]
    fn test_version_update() {
        XmlMDataXtd::set_document_version(1);
        assert_eq!(XmlMDataXtd::document_version(), 1);
        XmlMDataXtd::set_document_version(2);
        assert_eq!(XmlMDataXtd::document_version(), 2);
    }

    #[test]
    fn test_drivers_are_distinct() {
        let drivers = XmlMDataXtd::add_drivers();
        let set: std::collections::HashSet<_> = drivers.iter().collect();
        assert_eq!(drivers.len(), set.len(), "All drivers should be unique");
    }
}
