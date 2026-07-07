// FILE: vrml_converter_wf_deflection_shape.rs
// occt: VrmlConverter_WFDeflectionShape

#[derive(Clone, Debug)]
pub struct VrmlConverterWFDeflectionShape {
    shape_id: u32,
    deflection: f64,
}

impl VrmlConverterWFDeflectionShape {
    pub fn new(shape_id: u32, deflection: f64) -> Self {
        VrmlConverterWFDeflectionShape {
            shape_id,
            deflection,
        }
    }

    pub fn shape_id(&self) -> u32 {
        self.shape_id
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
        let shape = VrmlConverterWFDeflectionShape::new(42, 0.01);
        assert_eq!(shape.shape_id(), 42);
        assert_eq!(shape.deflection(), 0.01);
    }
}
