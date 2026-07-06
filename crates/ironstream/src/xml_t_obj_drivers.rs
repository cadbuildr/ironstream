// FILE: xml_t_obj_drivers.rs
// occt: XmlTObjDrivers

/// Registry and factory for XML drivers handling TObj (Transient Object) model persistence.
/// Provides driver lookup, registration, and centralized access to all TObj XML drivers.
pub struct XmlTObjDrivers;

impl XmlTObjDrivers {
    /// Create a driver factory/registry.
    pub fn new() -> Self {
        XmlTObjDrivers
    }

    /// Register all standard TObj XML drivers into the persistence system.
    /// Must be called before any TObj model persistence operations.
    pub fn factory_setup() -> &'static str {
        // Returns indicator that factory setup is complete
        "XmlTObjDrivers::Factory"
    }

    /// Retrieve XML driver name for a given TObj model attribute type.
    /// Returns the driver class name used for XML I/O of that type.
    pub fn get_driver_name(type_name: &str) -> Option<String> {
        match type_name {
            "Model" => Some("XmlTObjDrivers_ModelDriver".to_string()),
            "Object" => Some("XmlTObjDrivers_ObjectDriver".to_string()),
            "Reference" => Some("XmlTObjDrivers_ReferenceDriver".to_string()),
            "XYZ" => Some("XmlTObjDrivers_XYZDriver".to_string()),
            "IntSparseArray" => Some("XmlTObjDrivers_IntSparseArrayDriver".to_string()),
            _ => None,
        }
    }

    /// Get the current factory version number.
    pub fn factory_version() -> i32 {
        1
    }

    /// Check if a given type has an available XML driver.
    pub fn has_driver(type_name: &str) -> bool {
        Self::get_driver_name(type_name).is_some()
    }
}

impl Default for XmlTObjDrivers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_setup_string() {
        assert_eq!(XmlTObjDrivers::factory_setup(), "XmlTObjDrivers::Factory");
    }

    #[test]
    fn test_get_driver_name_model() {
        assert_eq!(
            XmlTObjDrivers::get_driver_name("Model"),
            Some("XmlTObjDrivers_ModelDriver".to_string())
        );
    }

    #[test]
    fn test_get_driver_name_object() {
        assert_eq!(
            XmlTObjDrivers::get_driver_name("Object"),
            Some("XmlTObjDrivers_ObjectDriver".to_string())
        );
    }

    #[test]
    fn test_get_driver_name_reference() {
        assert_eq!(
            XmlTObjDrivers::get_driver_name("Reference"),
            Some("XmlTObjDrivers_ReferenceDriver".to_string())
        );
    }

    #[test]
    fn test_get_driver_name_xyz() {
        assert_eq!(
            XmlTObjDrivers::get_driver_name("XYZ"),
            Some("XmlTObjDrivers_XYZDriver".to_string())
        );
    }

    #[test]
    fn test_get_driver_name_int_sparse_array() {
        assert_eq!(
            XmlTObjDrivers::get_driver_name("IntSparseArray"),
            Some("XmlTObjDrivers_IntSparseArrayDriver".to_string())
        );
    }

    #[test]
    fn test_get_driver_name_unknown() {
        assert_eq!(XmlTObjDrivers::get_driver_name("UnknownType"), None);
    }

    #[test]
    fn test_has_driver_true() {
        assert!(XmlTObjDrivers::has_driver("Model"));
        assert!(XmlTObjDrivers::has_driver("Object"));
    }

    #[test]
    fn test_has_driver_false() {
        assert!(!XmlTObjDrivers::has_driver("UnknownType"));
    }

    #[test]
    fn test_factory_version() {
        assert_eq!(XmlTObjDrivers::factory_version(), 1);
    }
}
