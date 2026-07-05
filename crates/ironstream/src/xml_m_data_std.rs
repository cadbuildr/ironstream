// FILE: xml_m_data_std.rs
// occt: XmlMDataStd

/// XML serialization utilities for standard data attributes.
/// Manages drivers for standard TDataStd attributes including:
/// - Scalar types (Integer, Real, String, Boolean)
/// - Arrays (IntegerArray, RealArray)
/// - Collections (IntegerList, RealList)
/// - Named data containers
pub struct XmlMDataStd;

impl XmlMDataStd {
    /// Add all standard data attribute drivers to driver table.
    pub fn add_drivers() -> Vec<String> {
        vec![
            "XmlMDataStd_IntegerDriver".to_string(),
            "XmlMDataStd_RealDriver".to_string(),
            "XmlMDataStd_AsciiStringDriver".to_string(),
            "XmlMDataStd_IntegerArrayDriver".to_string(),
            "XmlMDataStd_RealArrayDriver".to_string(),
            "XmlMDataStd_BooleanArrayDriver".to_string(),
            "XmlMDataStd_IntegerListDriver".to_string(),
            "XmlMDataStd_NamedDataDriver".to_string(),
        ]
    }

    /// Get count of standard data drivers.
    pub fn driver_count() -> usize {
        8
    }

    /// Check if a specific driver type is registered.
    pub fn has_driver(driver_name: &str) -> bool {
        let drivers = Self::add_drivers();
        drivers.iter().any(|d| d == driver_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_drivers_returns_correct_count() {
        let drivers = XmlMDataStd::add_drivers();
        assert_eq!(drivers.len(), 8);
    }

    #[test]
    fn test_driver_count() {
        assert_eq!(XmlMDataStd::driver_count(), 8);
    }

    #[test]
    fn test_add_drivers_contains_integer() {
        let drivers = XmlMDataStd::add_drivers();
        assert!(drivers.iter().any(|d| d.contains("Integer")));
    }

    #[test]
    fn test_add_drivers_contains_real() {
        let drivers = XmlMDataStd::add_drivers();
        assert!(drivers.iter().any(|d| d.contains("Real")));
    }

    #[test]
    fn test_add_drivers_contains_string() {
        let drivers = XmlMDataStd::add_drivers();
        assert!(drivers.iter().any(|d| d.contains("String")));
    }

    #[test]
    fn test_add_drivers_contains_array() {
        let drivers = XmlMDataStd::add_drivers();
        assert!(drivers.iter().any(|d| d.contains("Array")));
    }

    #[test]
    fn test_add_drivers_contains_list() {
        let drivers = XmlMDataStd::add_drivers();
        assert!(drivers.iter().any(|d| d.contains("List")));
    }

    #[test]
    fn test_has_driver_integer() {
        assert!(XmlMDataStd::has_driver("XmlMDataStd_IntegerDriver"));
    }

    #[test]
    fn test_has_driver_real() {
        assert!(XmlMDataStd::has_driver("XmlMDataStd_RealDriver"));
    }

    #[test]
    fn test_has_driver_nonexistent() {
        assert!(!XmlMDataStd::has_driver("XmlMDataStd_NonExistentDriver"));
    }

    #[test]
    fn test_drivers_are_unique() {
        let drivers = XmlMDataStd::add_drivers();
        let set: std::collections::HashSet<_> = drivers.iter().collect();
        assert_eq!(drivers.len(), set.len(), "All drivers should be unique");
    }

    #[test]
    fn test_all_drivers_end_with_driver() {
        let drivers = XmlMDataStd::add_drivers();
        for driver in drivers {
            assert!(driver.ends_with("Driver"));
        }
    }

    #[test]
    fn test_all_drivers_start_with_xmlmdatastd() {
        let drivers = XmlMDataStd::add_drivers();
        for driver in drivers {
            assert!(driver.starts_with("XmlMDataStd_"));
        }
    }
}
