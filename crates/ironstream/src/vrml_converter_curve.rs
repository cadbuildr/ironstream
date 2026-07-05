// FILE: vrml_converter_curve.rs
// occt: VrmlConverter_Curve

#[derive(Clone, Debug)]
pub struct VrmlConverterCurve {
    curve_type: String,
}

impl VrmlConverterCurve {
    pub fn new(curve_type: &str) -> Self {
        VrmlConverterCurve {
            curve_type: curve_type.to_string(),
        }
    }

    pub fn curve_type(&self) -> &str {
        &self.curve_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let curve = VrmlConverterCurve::new("line");
        assert_eq!(curve.curve_type(), "line");
    }
}
