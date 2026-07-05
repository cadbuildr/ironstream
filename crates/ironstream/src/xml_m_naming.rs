// FILE: xml_m_naming.rs
// occt: XmlMNaming

/// XML serialization utilities for naming attributes.
/// Manages drivers for shape naming and geometric entity identification.
pub struct XmlMNaming;

impl XmlMNaming {
    /// Add all naming drivers to a driver table.
    /// This includes drivers for:
    /// - Named shapes
    /// - Naming attributes
    pub fn add_drivers() -> Vec<String> {
        vec![
            "XmlMNaming_NamedShapeDriver".to_string(),
            "XmlMNaming_NamingDriver".to_string(),
        ]
    }

    /// Get the number of naming drivers available.
    pub fn driver_count() -> usize {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_drivers_returns_correct_count() {
        let drivers = XmlMNaming::add_drivers();
        assert_eq!(drivers.len(), 2);
    }

    #[test]
    fn test_add_drivers_contains_named_shape() {
        let drivers = XmlMNaming::add_drivers();
        assert!(drivers.iter().any(|d| d.contains("NamedShape")));
    }

    #[test]
    fn test_add_drivers_contains_naming() {
        let drivers = XmlMNaming::add_drivers();
        assert!(drivers.iter().any(|d| d.contains("Naming") && !d.contains("NamedShape")));
    }

    #[test]
    fn test_driver_count() {
        assert_eq!(XmlMNaming::driver_count(), 2);
    }

    #[test]
    fn test_drivers_are_unique() {
        let drivers = XmlMNaming::add_drivers();
        let set: std::collections::HashSet<_> = drivers.iter().collect();
        assert_eq!(drivers.len(), set.len(), "All drivers should be unique");
    }

    #[test]
    fn test_all_drivers_have_driver_suffix() {
        let drivers = XmlMNaming::add_drivers();
        for driver in drivers {
            assert!(driver.ends_with("Driver"));
        }
    }

    #[test]
    fn test_all_drivers_start_with_xmlmnaming() {
        let drivers = XmlMNaming::add_drivers();
        for driver in drivers {
            assert!(driver.starts_with("XmlMNaming_"));
        }
    }
}
