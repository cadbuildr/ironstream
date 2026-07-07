// FILE: vrml_converter_hlr_shape.rs
// occt: VrmlConverter_HLRShape

#[derive(Clone, Debug)]
pub struct VrmlConverterHLRShape {
    shape_id: u32,
}

impl VrmlConverterHLRShape {
    pub fn new(shape_id: u32) -> Self {
        VrmlConverterHLRShape { shape_id }
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
        let shape = VrmlConverterHLRShape::new(100);
        assert_eq!(shape.shape_id(), 100);
    }
}
