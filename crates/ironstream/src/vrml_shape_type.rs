// FILE: vrml_shape_type.rs
// occt: Vrml_ShapeType
//
// Faithful port of OCCT Vrml_ShapeType
// (DataExchange/TKDEVRML/Vrml/Vrml_ShapeType.hxx): the shapeType hint of the
// VRML 1.0 ShapeHints node. C++ enumerators (declaration order):
// Vrml_UNKNOWN_SHAPE_TYPE, Vrml_SOLID.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlShapeTypeKind {
    VrmlUnknownShapeType = 0,
    VrmlSolid = 1,
}

impl VrmlShapeTypeKind {
    /// All enumerators in C++ declaration order.
    pub fn values() -> [VrmlShapeTypeKind; 2] {
        [
            VrmlShapeTypeKind::VrmlUnknownShapeType,
            VrmlShapeTypeKind::VrmlSolid,
        ]
    }

    /// The VRML 1.0 keyword the ShapeHints writer emits for this value
    /// (UNKNOWN_SHAPE_TYPE is the default and is suppressed there).
    pub fn vrml_keyword(self) -> &'static str {
        match self {
            VrmlShapeTypeKind::VrmlUnknownShapeType => "UNKNOWN_SHAPE_TYPE",
            VrmlShapeTypeKind::VrmlSolid => "SOLID",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_values_match_declaration_order() {
        assert_eq!(VrmlShapeTypeKind::VrmlUnknownShapeType as i32, 0);
        assert_eq!(VrmlShapeTypeKind::VrmlSolid as i32, 1);
    }

    #[test]
    fn keywords() {
        let vals = VrmlShapeTypeKind::values();
        assert_eq!(vals[0].vrml_keyword(), "UNKNOWN_SHAPE_TYPE");
        assert_eq!(vals[1].vrml_keyword(), "SOLID");
    }
}
