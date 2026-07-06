// FILE: vrml_material_binding.rs
// occt: Vrml_MaterialBinding
//
// Faithful port of OCCT Vrml_MaterialBinding
// (DataExchange/TKDEVRML/Vrml/Vrml_MaterialBinding.hxx): the binding value
// of the VRML 1.0 MaterialBinding node. C++ enumerators (declaration order):
// Vrml_DEFAULT, Vrml_PER_PART, Vrml_PER_PART_INDEXED, Vrml_PER_FACE,
// Vrml_PER_FACE_INDEXED, Vrml_PER_VERTEX, Vrml_PER_VERTEX_INDEXED.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlMaterialBindingKind {
    VrmlDefault = 0,
    VrmlPerPart = 1,
    VrmlPerPartIndexed = 2,
    VrmlPerFace = 3,
    VrmlPerFaceIndexed = 4,
    VrmlPerVertex = 5,
    VrmlPerVertexIndexed = 6,
}

impl VrmlMaterialBindingKind {
    /// All enumerators in C++ declaration order.
    pub fn values() -> [VrmlMaterialBindingKind; 7] {
        [
            VrmlMaterialBindingKind::VrmlDefault,
            VrmlMaterialBindingKind::VrmlPerPart,
            VrmlMaterialBindingKind::VrmlPerPartIndexed,
            VrmlMaterialBindingKind::VrmlPerFace,
            VrmlMaterialBindingKind::VrmlPerFaceIndexed,
            VrmlMaterialBindingKind::VrmlPerVertex,
            VrmlMaterialBindingKind::VrmlPerVertexIndexed,
        ]
    }

    /// The VRML 1.0 keyword emitted for this binding value.
    pub fn vrml_keyword(self) -> &'static str {
        match self {
            VrmlMaterialBindingKind::VrmlDefault => "DEFAULT",
            VrmlMaterialBindingKind::VrmlPerPart => "PER_PART",
            VrmlMaterialBindingKind::VrmlPerPartIndexed => "PER_PART_INDEXED",
            VrmlMaterialBindingKind::VrmlPerFace => "PER_FACE",
            VrmlMaterialBindingKind::VrmlPerFaceIndexed => "PER_FACE_INDEXED",
            VrmlMaterialBindingKind::VrmlPerVertex => "PER_VERTEX",
            VrmlMaterialBindingKind::VrmlPerVertexIndexed => "PER_VERTEX_INDEXED",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_values_match_declaration_order() {
        assert_eq!(VrmlMaterialBindingKind::VrmlDefault as i32, 0);
        assert_eq!(VrmlMaterialBindingKind::VrmlPerPart as i32, 1);
        assert_eq!(VrmlMaterialBindingKind::VrmlPerPartIndexed as i32, 2);
        assert_eq!(VrmlMaterialBindingKind::VrmlPerFace as i32, 3);
        assert_eq!(VrmlMaterialBindingKind::VrmlPerFaceIndexed as i32, 4);
        assert_eq!(VrmlMaterialBindingKind::VrmlPerVertex as i32, 5);
        assert_eq!(VrmlMaterialBindingKind::VrmlPerVertexIndexed as i32, 6);
    }

    #[test]
    fn keywords() {
        let vals = VrmlMaterialBindingKind::values();
        assert_eq!(vals[0].vrml_keyword(), "DEFAULT");
        assert_eq!(vals[1].vrml_keyword(), "PER_PART");
        assert_eq!(vals[2].vrml_keyword(), "PER_PART_INDEXED");
        assert_eq!(vals[3].vrml_keyword(), "PER_FACE");
        assert_eq!(vals[4].vrml_keyword(), "PER_FACE_INDEXED");
        assert_eq!(vals[5].vrml_keyword(), "PER_VERTEX");
        assert_eq!(vals[6].vrml_keyword(), "PER_VERTEX_INDEXED");
    }
}
