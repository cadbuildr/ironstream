// FILE: rw_mesh_name_format.rs
// occt: RWMesh_NameFormat

/// Name format preference for XCAF shape labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RWMeshNameFormat {
    /// Omit the name
    Empty,
    /// Return Product name (e.g. from XCAFDoc_ShapeTool::GetReferredShape(),
    /// which could be shared by multiple Instances)
    Product,
    /// Return Instance name
    Instance,
    /// Return Instance name when available and Product name otherwise
    InstanceOrProduct,
    /// Return Product name when available and Instance name otherwise
    ProductOrInstance,
    /// Generate "Product [Instance]" name
    ProductAndInstance,
    /// Generate name combining Product+Instance+Ocaf (useful for debugging purposes)
    ProductAndInstanceAndOcaf,
}

impl RWMeshNameFormat {
    /// Returns a descriptive string for the format
    pub fn as_str(&self) -> &'static str {
        match self {
            RWMeshNameFormat::Empty => "Empty",
            RWMeshNameFormat::Product => "Product",
            RWMeshNameFormat::Instance => "Instance",
            RWMeshNameFormat::InstanceOrProduct => "InstanceOrProduct",
            RWMeshNameFormat::ProductOrInstance => "ProductOrInstance",
            RWMeshNameFormat::ProductAndInstance => "ProductAndInstance",
            RWMeshNameFormat::ProductAndInstanceAndOcaf => "ProductAndInstanceAndOcaf",
        }
    }
}

impl std::fmt::Display for RWMeshNameFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_as_str() {
        assert_eq!(RWMeshNameFormat::Empty.as_str(), "Empty");
        assert_eq!(RWMeshNameFormat::Product.as_str(), "Product");
        assert_eq!(RWMeshNameFormat::Instance.as_str(), "Instance");
        assert_eq!(RWMeshNameFormat::InstanceOrProduct.as_str(), "InstanceOrProduct");
        assert_eq!(RWMeshNameFormat::ProductOrInstance.as_str(), "ProductOrInstance");
        assert_eq!(RWMeshNameFormat::ProductAndInstance.as_str(), "ProductAndInstance");
        assert_eq!(
            RWMeshNameFormat::ProductAndInstanceAndOcaf.as_str(),
            "ProductAndInstanceAndOcaf"
        );
    }

    #[test]
    fn test_format_equality() {
        assert_eq!(RWMeshNameFormat::Empty, RWMeshNameFormat::Empty);
        assert_ne!(RWMeshNameFormat::Empty, RWMeshNameFormat::Product);
    }

    #[test]
    fn test_format_display() {
        assert_eq!(format!("{}", RWMeshNameFormat::Product), "Product");
        assert_eq!(format!("{}", RWMeshNameFormat::Instance), "Instance");
    }

    #[test]
    fn test_format_clone() {
        let fmt = RWMeshNameFormat::ProductAndInstance;
        let fmt_clone = fmt.clone();
        assert_eq!(fmt, fmt_clone);
    }
}
