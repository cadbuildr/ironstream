// FILE: vrml_converter_projector.rs
// occt: VrmlConverter_Projector

#[derive(Clone, Debug)]
pub struct VrmlConverterProjector {
    scale: f64,
}

impl VrmlConverterProjector {
    pub fn new(scale: f64) -> Self {
        VrmlConverterProjector { scale }
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let proj = VrmlConverterProjector::new(1.0);
        assert_eq!(proj.scale(), 1.0);
    }
}
