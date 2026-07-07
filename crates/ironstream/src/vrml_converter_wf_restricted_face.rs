// FILE: vrml_converter_wf_restricted_face.rs
// occt: VrmlConverter_WFRestrictedFace

#[derive(Clone, Debug)]
pub struct VrmlConverterWFRestrictedFace {
    face_id: u32,
}

impl VrmlConverterWFRestrictedFace {
    pub fn new(face_id: u32) -> Self {
        VrmlConverterWFRestrictedFace { face_id }
    }

    pub fn face_id(&self) -> u32 {
        self.face_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let face = VrmlConverterWFRestrictedFace::new(99);
        assert_eq!(face.face_id(), 99);
    }
}
