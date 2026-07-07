// FILE: t_data_xtd_shape.rs
// occt: TDataXtd_Shape

/// GUID for TDataXtd_Shape attribute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandardGUID {
    data: [u8; 16],
}

impl StandardGUID {
    pub fn new(data: [u8; 16]) -> Self {
        StandardGUID { data }
    }

    pub fn get_id() -> Self {
        // "2a96b620-ec8b-11d0-bee7-080009dc3333"
        let bytes: [u8; 16] = [0x2a, 0x96, 0xb6, 0x20, 0xec, 0x8b, 0x11, 0xd0,
                               0xbe, 0xe7, 0x08, 0x00, 0x09, 0xdc, 0x33, 0x33];
        StandardGUID { data: bytes }
    }
}

/// A placeholder for a TopoDS_Shape. In a full port, this would be the actual shape type.
#[derive(Clone, Debug, Default)]
pub struct TopodsShape {
    // Simplified: we don't have access to OCCT's full TopoDS implementation.
    // This would normally contain topology data.
    is_null: bool,
}

impl TopodsShape {
    pub fn null() -> Self {
        TopodsShape { is_null: true }
    }

    pub fn is_null(&self) -> bool {
        self.is_null
    }

    pub fn nullify(&mut self) {
        self.is_null = true;
    }
}

/// A Shape attribute associated with a NamedShape in the framework.
/// Mirrors OCCT's TDataXtd_Shape.
#[derive(Clone, Debug, Default)]
pub struct TDataXtdShape {
    // Empty attribute; inherits from TDataStd_GenericEmpty
}

impl TDataXtdShape {
    /// Creates a new TDataXtd_Shape attribute.
    pub fn new() -> Self {
        TDataXtdShape {}
    }

    /// Returns the ID of the attribute.
    pub fn get_id() -> StandardGUID {
        StandardGUID::get_id()
    }

    /// Returns the ID (instance method).
    pub fn id(&self) -> StandardGUID {
        Self::get_id()
    }

    /// Dumps the attribute as a string.
    pub fn dump(&self) -> String {
        "Shape".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        let shape = TDataXtdShape::new();
        // Just verify it was created without panicking
        assert_eq!(shape.dump(), "Shape");
    }

    #[test]
    fn test_get_id() {
        let id1 = TDataXtdShape::get_id();
        let id2 = TDataXtdShape::get_id();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_id_method() {
        let shape = TDataXtdShape::new();
        let id = shape.id();
        assert_eq!(id, TDataXtdShape::get_id());
    }

    #[test]
    fn test_topods_shape_null() {
        let shape = TopodsShape::null();
        assert!(shape.is_null());
    }

    #[test]
    fn test_topods_shape_nullify() {
        let mut shape = TopodsShape::default();
        shape.nullify();
        assert!(shape.is_null());
    }

    #[test]
    fn test_dump() {
        let shape = TDataXtdShape::new();
        assert_eq!(shape.dump(), "Shape");
    }
}
