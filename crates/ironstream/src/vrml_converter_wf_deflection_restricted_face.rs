// FILE: vrml_converter_wf_deflection_restricted_face.rs
// occt: VrmlConverter_WFDeflectionRestrictedFace

#[derive(Clone, Debug)]
pub struct VrmlConverterWFDeflectionRestrictedFace {
    face_id: u32,
    deflection: f64,
}

impl VrmlConverterWFDeflectionRestrictedFace {
    pub fn new(face_id: u32, deflection: f64) -> Self {
        VrmlConverterWFDeflectionRestrictedFace {
            face_id,
            deflection,
        }
    }

    pub fn face_id(&self) -> u32 {
        self.face_id
    }

    pub fn deflection(&self) -> f64 {
        self.deflection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let face = VrmlConverterWFDeflectionRestrictedFace::new(1, 0.01);
        assert_eq!(face.face_id(), 1);
        assert_eq!(face.deflection(), 0.01);
    }
}
