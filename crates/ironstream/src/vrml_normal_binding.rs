// FILE: vrml_normal_binding.rs
// occt: Vrml_NormalBinding
//
// Faithful port of OCCT Vrml_NormalBinding
// (DataExchange/TKDEVRML/Vrml/Vrml_NormalBinding.hxx): the binding value
// of the VRML 1.0 NormalBinding node. C++ enumerators (declaration order):
// Vrml_DEFAULT, Vrml_PER_PART, Vrml_PER_PART_INDEXED, Vrml_PER_FACE,
// Vrml_PER_FACE_INDEXED, Vrml_PER_VERTEX, Vrml_PER_VERTEX_INDEXED.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlNormalBindingKind {
    VrmlDefault = 0,
    VrmlPerPart = 1,
    VrmlPerPartIndexed = 2,
    VrmlPerFace = 3,
    VrmlPerFaceIndexed = 4,
    VrmlPerVertex = 5,
    VrmlPerVertexIndexed = 6,
}

impl VrmlNormalBindingKind {
    /// All enumerators in C++ declaration order.
    pub fn values() -> [VrmlNormalBindingKind; 7] {
        [
            VrmlNormalBindingKind::VrmlDefault,
            VrmlNormalBindingKind::VrmlPerPart,
            VrmlNormalBindingKind::VrmlPerPartIndexed,
            VrmlNormalBindingKind::VrmlPerFace,
            VrmlNormalBindingKind::VrmlPerFaceIndexed,
            VrmlNormalBindingKind::VrmlPerVertex,
            VrmlNormalBindingKind::VrmlPerVertexIndexed,
        ]
    }

    /// The VRML 1.0 keyword emitted for this binding value.
    pub fn vrml_keyword(self) -> &'static str {
        match self {
            VrmlNormalBindingKind::VrmlDefault => "DEFAULT",
            VrmlNormalBindingKind::VrmlPerPart => "PER_PART",
            VrmlNormalBindingKind::VrmlPerPartIndexed => "PER_PART_INDEXED",
            VrmlNormalBindingKind::VrmlPerFace => "PER_FACE",
            VrmlNormalBindingKind::VrmlPerFaceIndexed => "PER_FACE_INDEXED",
            VrmlNormalBindingKind::VrmlPerVertex => "PER_VERTEX",
            VrmlNormalBindingKind::VrmlPerVertexIndexed => "PER_VERTEX_INDEXED",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_values_match_declaration_order() {
        assert_eq!(VrmlNormalBindingKind::VrmlDefault as i32, 0);
        assert_eq!(VrmlNormalBindingKind::VrmlPerPart as i32, 1);
        assert_eq!(VrmlNormalBindingKind::VrmlPerPartIndexed as i32, 2);
        assert_eq!(VrmlNormalBindingKind::VrmlPerFace as i32, 3);
        assert_eq!(VrmlNormalBindingKind::VrmlPerFaceIndexed as i32, 4);
        assert_eq!(VrmlNormalBindingKind::VrmlPerVertex as i32, 5);
        assert_eq!(VrmlNormalBindingKind::VrmlPerVertexIndexed as i32, 6);
    }

    #[test]
    fn keywords() {
        let vals = VrmlNormalBindingKind::values();
        assert_eq!(vals[0].vrml_keyword(), "DEFAULT");
        assert_eq!(vals[1].vrml_keyword(), "PER_PART");
        assert_eq!(vals[2].vrml_keyword(), "PER_PART_INDEXED");
        assert_eq!(vals[3].vrml_keyword(), "PER_FACE");
        assert_eq!(vals[4].vrml_keyword(), "PER_FACE_INDEXED");
        assert_eq!(vals[5].vrml_keyword(), "PER_VERTEX");
        assert_eq!(vals[6].vrml_keyword(), "PER_VERTEX_INDEXED");
    }
}
