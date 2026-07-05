// FILE: vrml_api_representation_of_shape.rs
// occt: VrmlAPI_RepresentationOfShape

#[derive(Clone, Debug)]
pub struct VrmlApiRepresentationOfShape {
    shape_id: u32,
}

impl VrmlApiRepresentationOfShape {
    pub fn new(shape_id: u32) -> Self {
        VrmlApiRepresentationOfShape { shape_id }
    }

    pub fn shape_id(&self) -> u32 {
        self.shape_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let r = VrmlApiRepresentationOfShape::new(42);
        assert_eq!(r.shape_id(), 42);
    }
}
