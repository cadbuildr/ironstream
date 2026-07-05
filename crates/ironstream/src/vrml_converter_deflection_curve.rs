// FILE: vrml_converter_deflection_curve.rs
// occt: VrmlConverter_DeflectionCurve

#[derive(Clone, Debug)]
pub struct VrmlConverterDeflectionCurve {
    deflection: f64,
}

impl VrmlConverterDeflectionCurve {
    pub fn new(deflection: f64) -> Self {
        VrmlConverterDeflectionCurve { deflection }
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
        let curve = VrmlConverterDeflectionCurve::new(0.01);
        assert_eq!(curve.deflection(), 0.01);
    }
}
