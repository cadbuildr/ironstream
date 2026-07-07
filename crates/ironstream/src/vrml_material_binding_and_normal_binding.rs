// FILE: vrml_material_binding_and_normal_binding.rs
// occt: Vrml_MaterialBindingAndNormalBinding
//
// Faithful port of OCCT Vrml_MaterialBindingAndNormalBinding
// (DataExchange/TKDEVRML/Vrml/Vrml_MaterialBindingAndNormalBinding.hxx):
// combined enum representing both material and normal binding modes.
// C++ enumerators (declaration order): Vrml_OVERALL, Vrml_PER_PART,
// Vrml_PER_PART_INDEXED, Vrml_PER_FACE, Vrml_PER_FACE_INDEXED,
// Vrml_PER_VERTEX, Vrml_PER_VERTEX_INDEXED, Vrml_NONE.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlMaterialBindingAndNormalBindingKind {
    VrmlOverall = 0,
    VrmlPerPart = 1,
    VrmlPerPartIndexed = 2,
    VrmlPerFace = 3,
    VrmlPerFaceIndexed = 4,
    VrmlPerVertex = 5,
    VrmlPerVertexIndexed = 6,
    VrmlNone = 7,
}

impl VrmlMaterialBindingAndNormalBindingKind {
    /// All enumerators in C++ declaration order.
    pub fn values() -> [VrmlMaterialBindingAndNormalBindingKind; 8] {
        [
            VrmlMaterialBindingAndNormalBindingKind::VrmlOverall,
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerPart,
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerPartIndexed,
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerFace,
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerFaceIndexed,
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerVertex,
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerVertexIndexed,
            VrmlMaterialBindingAndNormalBindingKind::VrmlNone,
        ]
    }

    /// The VRML 1.0 keyword emitted for this binding value.
    pub fn vrml_keyword(self) -> &'static str {
        match self {
            VrmlMaterialBindingAndNormalBindingKind::VrmlOverall => "OVERALL",
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerPart => "PER_PART",
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerPartIndexed => "PER_PART_INDEXED",
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerFace => "PER_FACE",
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerFaceIndexed => "PER_FACE_INDEXED",
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerVertex => "PER_VERTEX",
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerVertexIndexed => "PER_VERTEX_INDEXED",
            VrmlMaterialBindingAndNormalBindingKind::VrmlNone => "NONE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_values_match_declaration_order() {
        assert_eq!(VrmlMaterialBindingAndNormalBindingKind::VrmlOverall as i32, 0);
        assert_eq!(VrmlMaterialBindingAndNormalBindingKind::VrmlPerPart as i32, 1);
        assert_eq!(
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerPartIndexed as i32,
            2
        );
        assert_eq!(VrmlMaterialBindingAndNormalBindingKind::VrmlPerFace as i32, 3);
        assert_eq!(
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerFaceIndexed as i32,
            4
        );
        assert_eq!(VrmlMaterialBindingAndNormalBindingKind::VrmlPerVertex as i32, 5);
        assert_eq!(
            VrmlMaterialBindingAndNormalBindingKind::VrmlPerVertexIndexed as i32,
            6
        );
        assert_eq!(VrmlMaterialBindingAndNormalBindingKind::VrmlNone as i32, 7);
    }

    #[test]
    fn keywords() {
        let vals = VrmlMaterialBindingAndNormalBindingKind::values();
        assert_eq!(vals[0].vrml_keyword(), "OVERALL");
        assert_eq!(vals[1].vrml_keyword(), "PER_PART");
        assert_eq!(vals[2].vrml_keyword(), "PER_PART_INDEXED");
        assert_eq!(vals[3].vrml_keyword(), "PER_FACE");
        assert_eq!(vals[4].vrml_keyword(), "PER_FACE_INDEXED");
        assert_eq!(vals[5].vrml_keyword(), "PER_VERTEX");
        assert_eq!(vals[6].vrml_keyword(), "PER_VERTEX_INDEXED");
        assert_eq!(vals[7].vrml_keyword(), "NONE");
    }
}
