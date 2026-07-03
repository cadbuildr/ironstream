// FILE: tkb_rep_pch.rs
// occt: TKBRep_pch

/// Precompiled header include for TKBRep module.
/// This module provides boundary representation functionality including:
/// - BRepGraph: Structured B-rep graph with layers and views
/// - BRepLProp: Local properties of B-rep entities
/// - BRepTools: Utilities for B-rep shapes
/// - BRep: Core B-rep representation classes
/// - BinTools: Binary serialization of shapes

pub struct TKBRepPch;

impl TKBRepPch {
    /// Get the module name.
    pub fn module_name() -> &'static str {
        "TKBRep"
    }

    /// Get the module version.
    pub fn module_version() -> &'static str {
        "1.0.0"
    }

    /// Get the module description.
    pub fn module_description() -> &'static str {
        "Boundary Representation (B-Rep) kernel for IronStream"
    }

    /// Get all included components.
    pub fn components() -> Vec<&'static str> {
        vec![
            "BRepGraph",
            "BRepLProp",
            "BRepTools",
            "BRep",
            "BinTools",
            "TopTools",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_name() {
        assert_eq!(TKBRepPch::module_name(), "TKBRep");
    }

    #[test]
    fn test_module_version() {
        assert_eq!(TKBRepPch::module_version(), "1.0.0");
    }

    #[test]
    fn test_components() {
        let components = TKBRepPch::components();
        assert!(components.contains(&"BRepGraph"));
        assert!(components.contains(&"BRep"));
        assert!(components.contains(&"BinTools"));
    }
}
