// FILE: bin_mxcaf_doc.rs
// occt: BinMXCAFDoc

/// Factory for XCAF document drivers.
pub struct BinmxcafDoc;

impl BinmxcafDoc {
    /// Adds attribute drivers to a driver table.
    pub fn add_drivers() -> Vec<String> {
        vec![
            "ColorDriver".to_string(),
            "MaterialDriver".to_string(),
            "LayerDriver".to_string(),
            "ReferenceDriver".to_string(),
        ]
    }

    /// Returns supported driver types.
    pub fn supported_types() -> &'static [&'static str] {
        &["Color", "Material", "Layer", "Reference"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_drivers() {
        let drivers = BinmxcafDoc::add_drivers();
        assert!(drivers.len() > 0);
        assert!(drivers.contains(&"ColorDriver".to_string()));
    }

    #[test]
    fn test_supported_types() {
        let types = BinmxcafDoc::supported_types();
        assert!(types.contains(&"Color"));
    }
}
