// FILE: vrml_converter_wf_shape.rs
// occt: VrmlConverter_WFShape

#[derive(Clone, Debug)]
pub struct VrmlConverterWFShape {
    shape_id: u32,
}

impl VrmlConverterWFShape {
    pub fn new(shape_id: u32) -> Self {
        VrmlConverterWFShape { shape_id }
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
        let shape = VrmlConverterWFShape::new(77);
        assert_eq!(shape.shape_id(), 77);
    }
}
